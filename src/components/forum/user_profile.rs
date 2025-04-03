use leptos::*;
use leptos_router::use_params_map;
use crate::models::user::User;
use crate::models::forum::{Topic, Post};
use crate::services::user::UserService;
use crate::services::forum::ForumService;

#[component]
pub fn UserProfile() -> impl IntoView {
    // Get user ID from route params
    let params = use_params_map();
    let user_id = move || {
        params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0))
    };

    // State signals
    let (user, set_user) = create_signal(None::<User>);
    let (recent_topics, set_recent_topics) = create_signal(Vec::<Topic>::new());
    let (recent_posts, set_recent_posts) = create_signal(Vec::<Post>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load user data
    create_effect(move |_| {
        let id = user_id();
        if id <= 0 {
            set_error.set(Some("Invalid user ID".to_string()));
            set_loading.set(false);
            return;
        }
        
        spawn_local(async move {
            match UserService::get_user(id).await {
                Ok(user_data) => {
                    set_user.set(Some(user_data));
                    
                    // Load user's recent topics
                    match ForumService::get_user_topics(id, 5).await {
                        Ok(topics) => set_recent_topics.set(topics),
                        Err(e) => log::error!("Failed to load user topics: {}", e)
                    }
                    
                    // Load user's recent posts
                    match ForumService::get_user_posts(id, 5).await {
                        Ok(posts) => set_recent_posts.set(posts),
                        Err(e) => log::error!("Failed to load user posts: {}", e)
                    }
                    
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load user: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    // Get the current user for permission checks
    let auth_state = use_context::<AuthState>();
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    let is_own_profile = move || current_user_id() > 0 && current_user_id() == user_id();
    
    view! {
        <div class="user-profile">
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if let Some(u) = user() {
                view! {
                    <div class="row">
                        <!-- User Info Sidebar -->
                        <div class="col-md-4 mb-4">
                            <div class="card">
                                <div class="card-body text-center">
                                    <div class="user-avatar mb-3">
                                        <img src={u.avatar_url.unwrap_or_else(|| format!("https://ui-avatars.com/api/?name={}&background=random", u.name))}
                                             alt="User Avatar"
                                             class="rounded-circle img-fluid"
                                             style="width: 120px; height: 120px; object-fit: cover;"
                                        />
                                    </div>
                                    
                                    <h3 class="mb-1">{u.name}</h3>
                                    <div class="text-muted">
                                        {u.role.unwrap_or_else(|| "Member".to_string())}
                                        {u.title.map(|t| view! { <span> · {t}</span> })}
                                    </div>
                                    
                                    <div class="mt-3">
                                        <div class="badge bg-primary me-1">{format!("{} topics", u.topic_count.unwrap_or(0))}</div>
                                        <div class="badge bg-info me-1">{format!("{} posts", u.post_count.unwrap_or(0))}</div>
                                        {u.solution_count.map(|count| {
                                            if count > 0 {
                                                view! { <div class="badge bg-success">{format!("{} solutions", count)}</div> }
                                            } else {
                                                view! {}
                                            }
                                        })}
                                    </div>
                                    
                                    {move || {
                                        if is_own_profile() {
                                            view! {
                                                <div class="mt-3">
                                                    <a href="/profile/edit" class="btn btn-sm btn-outline-secondary">
                                                        <i class="bi bi-pencil me-1"></i>
                                                        "Edit Profile"
                                                    </a>
                                                </div>
                                            }
                                        } else {
                                            view! {}
                                        }
                                    }}
                                </div>
                                
                                <div class="card-body border-top">
                                    <div class="mb-3">
                                        <h6>"About"</h6>
                                        <p class="mb-0">
                                            {u.bio.unwrap_or_else(|| "This user has not added a bio yet.".to_string())}
                                        </p>
                                    </div>
                                    
                                    <div class="mb-0">
                                        <h6>"Member Since"</h6>
                                        <p class="mb-0">{format_date(u.created_at)}</p>
                                    </div>
                                </div>
                            </div>
                            
                            {u.badges.map(|badges| {
                                if !badges.is_empty() {
                                    view! {
                                        <div class="card mt-4">
                                            <div class="card-header">"Badges"</div>
                                            <div class="card-body">
                                                <div class="d-flex flex-wrap gap-2">
                                                    {badges.into_iter().map(|badge| {
                                                        view! {
                                                            <div class="badge-item" title={badge.description}>
                                                                <span class={format!("badge rounded-pill bg-{}", badge.color)}>
                                                                    <i class={format!("bi bi-{} me-1", badge.icon)}></i>
                                                                    {badge.name}
                                                                </span>
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    view! {}
                                }
                            })}
                        </div>
                        
                        <!-- User Activity -->
                        <div class="col-md-8">
                            <!-- Activity Tabs -->
                            <ul class="nav nav-tabs" id="userActivityTabs" role="tablist">
                                <li class="nav-item" role="presentation">
                                    <button class="nav-link active" id="topics-tab" data-bs-toggle="tab" 
                                            data-bs-target="#topics" type="button" role="tab">
                                        "Topics"
                                    </button>
                                </li>
                                <li class="nav-item" role="presentation">
                                    <button class="nav-link" id="posts-tab" data-bs-toggle="tab" 
                                            data-bs-target="#posts" type="button" role="tab">
                                        "Posts"
                                    </button>
                                </li>
                            </ul>
                            
                            <!-- Tab Content -->
                            <div class="tab-content p-3 border border-top-0 rounded-bottom" id="userActivityTabsContent">
                                <!-- Topics Tab -->
                                <div class="tab-pane fade show active" id="topics" role="tabpanel">
                                    <h4 class="mb-3">"Recent Topics"</h4>
                                    {move || {
                                        if recent_topics().is_empty() {
                                            view! {
                                                <div class="text-muted text-center py-4">
                                                    <i class="bi bi-chat-square-text mb-3 d-block" style="font-size: 2rem;"></i>
                                                    <p>"This user hasn't created any topics yet."</p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="list-group">
                                                    {recent_topics().into_iter().map(|topic| {
                                                        view! {
                                                            <a href={format!("/forum/topics/{}", topic.id)} class="list-group-item list-group-item-action">
                                                                <div class="d-flex w-100 justify-content-between">
                                                                    <h5 class="mb-1">{topic.title}</h5>
                                                                    <small>{format_relative_date(topic.created_at)}</small>
                                                                </div>
                                                                <p class="mb-1">{topic.excerpt.unwrap_or_default()}</p>
                                                                <small class="text-muted">
                                                                    "In " <a href={format!("/forum/categories/{}", topic.category_id)}>
                                                                        {topic.category_name.unwrap_or_else(|| "Unknown".to_string())}
                                                                    </a>
                                                                    " · " {format!("{} replies", topic.reply_count.unwrap_or(0))}
                                                                </small>
                                                            </a>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        }
                                    }}
                                    
                                    <div class="text-end mt-3">
                                        <a href={format!("/users/{}/topics", u.id)} class="btn btn-outline-primary btn-sm">
                                            "View All Topics"
                                        </a>
                                    </div>
                                </div>
                                
                                <!-- Posts Tab -->
                                <div class="tab-pane fade" id="posts" role="tabpanel">
                                    <h4 class="mb-3">"Recent Posts"</h4>
                                    {move || {
                                        if recent_posts().is_empty() {
                                            view! {
                                                <div class="text-muted text-center py-4">
                                                    <i class="bi bi-chat-right-text mb-3 d-block" style="font-size: 2rem;"></i>
                                                    <p>"This user hasn't made any posts yet."</p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="list-group">
                                                    {recent_posts().into_iter().map(|post| {
                                                        view! {
                                                            <a href={format!("/forum/topics/{}#post-{}", post.topic_id, post.id)} class="list-group-item list-group-item-action">
                                                                <div class="d-flex w-100 justify-content-between">
                                                                    <h5 class="mb-1">{post.topic_title.unwrap_or_else(|| "Reply".to_string())}</h5>
                                                                    <small>{format_relative_date(post.created_at)}</small>
                                                                </div>
                                                                <p class="mb-1 post-excerpt">
                                                                    {post.excerpt.unwrap_or_else(|| truncate(&post.content, 150))}
                                                                </p>
                                                                {post.is_solution.map(|is_solution| {
                                                                    if is_solution {
                                                                        view! {
                                                                            <span class="badge bg-success">"Solution"</span>
                                                                        }
                                                                    } else {
                                                                        view! {}
                                                                    }
                                                                })}
                                                            </a>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        }
                                    }}
                                    
                                    <div class="text-end mt-3">
                                        <a href={format!("/users/{}/posts", u.id)} class="btn btn-outline-primary btn-sm">
                                            "View All Posts"
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                view! {
                    <div class="alert alert-danger">
                        {move || error().unwrap_or_else(|| "User not found".to_string())}
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to format dates
fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%B %d, %Y").to_string()
}

// Helper function to format relative dates
fn format_relative_date(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 365 {
        format!("{} years ago", diff.num_days() / 365)
    } else if diff.num_days() > 30 {
        format!("{} months ago", diff.num_days() / 30)
    } else if diff.num_days() > 0 {
        format!("{} days ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{} hours ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{} minutes ago", diff.num_minutes())
    } else {
        "just now".to_string()
    }
}

// Helper function to truncate text
fn truncate(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let mut truncated = text.chars().take(max_length).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}