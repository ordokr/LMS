use leptos::*;
use crate::models::forum::{Category, Topic};
use crate::services::forum::ForumService;
use chrono::Utc;

#[component]
pub fn ForumThreads(
    #[prop()] category_id: i64,
) -> impl IntoView {
    let (category, set_category) = create_signal(None::<Category>);
    let (topics, set_topics) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load category and its topics
    create_effect(move |_| {
        spawn_local(async move {
            // Get category details
            match ForumService::get_category(category_id).await {
                Ok(cat) => {
                    set_category.set(Some(cat));
                    
                    // Get topics for this category
                    match ForumService::get_topics(Some(category_id)).await {
                        Ok(topic_list) => {
                            set_topics.set(topic_list);
                            set_loading.set(false);
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to load topics: {}", e)));
                            set_loading.set(false);
                        }
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load category: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    // Optional: Check authentication status for creating new topics
    let auth_state = use_context::<AuthState>();
    let is_authenticated = move || auth_state.map(|state| state.is_authenticated()).unwrap_or(false);
    
    view! {
        <div class="forum-threads">
            {move || if let Some(cat) = category() {
                view! {
                    <div class="d-flex justify-content-between align-items-center mb-4">
                        <div>
                            <h1 class="mb-1">
                                <span class="category-color me-2" 
                                      style={format!("background-color: {}", cat.color.clone().unwrap_or_else(|| "#0088cc".to_string()))}>
                                </span>
                                {cat.name}
                            </h1>
                            {cat.description.clone().map(|desc| {
                                view! { <p class="text-muted">{desc}</p> }
                            })}
                        </div>
                        <div class="d-flex gap-2">
                            <a href="/forum" class="btn btn-outline-secondary">"All Categories"</a>
                            {move || {
                                if is_authenticated() {
                                    view! {
                                        <a href={format!("/forum/categories/{}/topics/new", category_id)}
                                           class="btn btn-primary">"New Topic"</a>
                                    }
                                } else {
                                    view! { <a href="/login" class="btn btn-outline-primary">"Log in to Post"</a> }
                                }
                            }}
                        </div>
                    </div>
                }
            } else {
                view! { <div class="skeleton-header mb-4 placeholder-glow">
                    <div class="placeholder col-7 mb-2" style="height: 2rem;"></div>
                    <div class="placeholder col-5" style="height: 1rem;"></div>
                </div> }
            }}
            
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if let Some(err) = error() {
                view! { <div class="alert alert-danger">{err}</div> }
            } else if topics().is_empty() {
                view! { 
                    <div class="empty-state text-center p-5">
                        <div class="mb-3">
                            <i class="bi bi-chat-square-text" style="font-size: 3rem;"></i>
                        </div>
                        <h3>"No topics yet"</h3>
                        <p class="text-muted">"Be the first to start a conversation in this category!"</p>
                        {move || {
                            if is_authenticated() {
                                view! {
                                    <a href={format!("/forum/categories/{}/topics/new", category_id)} 
                                       class="btn btn-primary">"Start the first topic"</a>
                                }
                            } else {
                                view! {
                                    <a href="/login" class="btn btn-outline-primary">"Log in to start the first topic"</a>
                                }
                            }
                        }}
                    </div>
                }
            } else {
                view! {
                    <div class="table-responsive">
                        <table class="table table-hover topic-list">
                            <thead>
                                <tr>
                                    <th class="topic-col">"Topic"</th>
                                    <th class="replies-col text-center">"Replies"</th>
                                    <th class="views-col text-center">"Views"</th>
                                    <th class="activity-col">"Activity"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {topics().into_iter().map(|topic| {
                                    view! {
                                        <tr class={format!("topic-row {}{}", 
                                            if topic.pinned { "pinned " } else { "" }, 
                                            if topic.locked { "locked" } else { "" }
                                        )}>
                                            <td class="topic-details">
                                                <div class="d-flex flex-wrap gap-2 mb-1">
                                                    {if topic.pinned {
                                                        view! { <span class="badge bg-info">"Pinned"</span> }
                                                    } else {
                                                        view! {}
                                                    }}
                                                    {if topic.locked {
                                                        view! { <span class="badge bg-secondary">"Locked"</span> }
                                                    } else {
                                                        view! {}
                                                    }}
                                                </div>
                                                <a href={format!("/forum/topics/{}", topic.id)} class="topic-title">
                                                    {topic.title}
                                                </a>
                                                
                                                <div class="topic-meta small text-muted mt-1">
                                                    <span class="topic-author">
                                                        "Started by " 
                                                        <a href={format!("/users/{}", topic.author_id)}>
                                                            {topic.author_name.clone().unwrap_or_else(|| "Unknown".to_string())}
                                                        </a>
                                                    </span>
                                                    <span class="topic-created-at ms-2">
                                                        {format_relative_date(topic.created_at)}
                                                    </span>
                                                </div>
                                            </td>
                                            <td class="text-center">
                                                <span class="badge rounded-pill bg-light text-dark">
                                                    {topic.reply_count.unwrap_or(0)}
                                                </span>
                                            </td>
                                            <td class="text-center">
                                                {topic.view_count.unwrap_or(0)}
                                            </td>
                                            <td class="topic-activity">
                                                {if let Some(date) = topic.last_post_date {
                                                    view! {
                                                        <div>
                                                            <div class="small fw-bold">{format_relative_date(date)}</div>
                                                            {topic.last_poster.as_ref().map(|user| {
                                                                view! {
                                                                    <div class="small">
                                                                        "by "
                                                                        <a href={format!("/users/{}", user.id)}>
                                                                            {user.name.clone()}
                                                                        </a>
                                                                    </div>
                                                                }
                                                            })}
                                                        </div>
                                                    }
                                                } else {
                                                    view! {
                                                        <span class="small text-muted">"No replies yet"</span>
                                                    }
                                                }}
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to format relative dates
fn format_relative_date(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = Utc::now();
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