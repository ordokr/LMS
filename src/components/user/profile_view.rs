use leptos::*;
use crate::models::user::User;
use crate::models::user::profile::UserProfile;
use crate::models::forum::topic::TopicSummary;
use crate::models::forum::post::PostSummary;
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::format_date_for_display;

#[component]
pub fn UserProfileView(
    username: String,
    #[prop(optional)] default_tab: Option<String>,
) -> impl IntoView {
    // State
    let (user, set_user) = create_signal(None::<User>);
    let (profile, set_profile) = create_signal(None::<UserProfile>);
    let (topics, set_topics) = create_signal(Vec::<TopicSummary>::new());
    let (posts, set_posts) = create_signal(Vec::<PostSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (active_tab, set_active_tab) = create_signal(default_tab.unwrap_or_else(|| "summary".to_string()));
    let (is_following, set_is_following) = create_signal(false);
    
    // Set up tab options
    let tab_options = vec![
        ("summary", "Summary"),
        ("activity", "Activity"),
        ("topics", "Topics"),
        ("replies", "Replies"),
    ];
    
    // Load user profile
    create_effect(move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, (User, UserProfile)>("get_user_profile", &username).await {
                Ok((fetched_user, fetched_profile)) => {
                    set_user.set(Some(fetched_user.clone()));
                    set_profile.set(Some(fetched_profile));
                    
                    // Load topics
                    if let Ok(user_topics) = invoke::<_, Vec<TopicSummary>>(
                        "get_user_topics", 
                        &(fetched_user.id, Some(1), Some(5))
                    ).await {
                        set_topics.set(user_topics);
                    }
                    
                    // Load posts
                    if let Ok(user_posts) = invoke::<_, Vec<PostSummary>>(
                        "get_user_posts", 
                        &(fetched_user.id, Some(1), Some(5))
                    ).await {
                        set_posts.set(user_posts);
                    }
                    
                    // Check if current user follows this user
                    if let Some(current_user_id) = get_current_user_id() {
                        if let Ok(follows) = invoke::<_, bool>(
                            "check_follows_user", 
                            &(current_user_id, fetched_user.id)
                        ).await {
                            set_is_following.set(follows);
                        }
                    }
                    
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load user profile: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Follow/unfollow user
    let toggle_follow = move |_| {
        if let Some(current_user_id) = get_current_user_id() {
            if let Some(u) = user.get() {
                let user_id = u.id.clone();
                let currently_following = is_following.get();
                
                spawn_local(async move {
                    let result = if currently_following {
                        invoke::<_, ()>("unfollow_user", &(current_user_id, user_id)).await
                    } else {
                        invoke::<_, ()>("follow_user", &(current_user_id, user_id)).await
                    };
                    
                    if result.is_ok() {
                        set_is_following.set(!currently_following);
                    }
                });
            }
        } else {
            // Redirect to login
            window_location_assign("/login?redirect_to=/users/${username}");
        }
    };
    
    view! {
        <div class="user-profile-page">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() {
                    view! { <div class="loading-state">"Loading user profile..."</div> }
                } else {
                    match (user.get(), profile.get()) {
                        (Some(u), Some(p)) => {
                            view! {
                                <div class="profile-header">
                                    <div class="profile-header-main">
                                        <div class="avatar-container">
                                            {match u.avatar_url {
                                                Some(url) => view! {
                                                    <img src={url} alt="User avatar" class="user-avatar-large" />
                                                },
                                                None => view! {
                                                    <div class="user-avatar-large default-avatar">
                                                        {u.display_name.chars().next().unwrap_or('?')}
                                                    </div>
                                                }
                                            }}
                                        </div>
                                        
                                        <div class="user-info-container">
                                            <h1 class="user-display-name">{u.display_name}</h1>
                                            <div class="user-username">{"@"}{u.username}</div>
                                            
                                            {match p.tag_line {
                                                Some(tag) if !tag.is_empty() => view! {
                                                    <div class="user-tagline">{tag}</div>
                                                },
                                                _ => view! { <span></span> }
                                            }}
                                            
                                            <div class="user-stats">
                                                <div class="stat-item">
                                                    <span class="stat-value">{p.posts_count}</span>
                                                    <span class="stat-label">"posts"</span>
                                                </div>
                                                <div class="stat-item">
                                                    <span class="stat-value">{p.created_topics_count}</span>
                                                    <span class="stat-label">"topics"</span>
                                                </div>
                                                <div class="stat-item">
                                                    <span class="stat-value">{p.likes_received}</span>
                                                    <span class="stat-label">"likes"</span>
                                                </div>
                                                <a href={format!("/users/{}/followers", u.username)} class="stat-item stat-link">
                                                    <span class="stat-value">{p.followers_count}</span>
                                                    <span class="stat-label">"followers"</span>
                                                </a>
                                                
                                                <a href={format!("/users/{}/following", u.username)} class="stat-item stat-link">
                                                    <span class="stat-value">{p.following_count}</span>
                                                    <span class="stat-label">"following"</span>
                                                </a>
                                            </div>
                                        </div>
                                        
                                        <div class="profile-actions">
                                            {if u.id != get_current_user_id().unwrap_or_default() {
                                                view! {
                                                    <button 
                                                        class={if is_following.get() { "unfollow-button" } else { "follow-button" }}
                                                        on:click=toggle_follow
                                                    >
                                                        {if is_following.get() { "Unfollow" } else { "Follow" }}
                                                    </button>
                                                    <button class="message-button">"Message"</button>
                                                }
                                            } else {
                                                view! {
                                                    <a href="/settings/profile" class="edit-profile-button">"Edit Profile"</a>
                                                }
                                            }}
                                        </div>
                                    </div>
                                    
                                    <div class="profile-nav">
                                        <ul class="profile-nav-tabs">
                                            {tab_options.iter().map(|(value, label)| {
                                                let tab_value = value.to_string();
                                                let is_active = move || active_tab.get() == tab_value;
                                                
                                                view! {
                                                    <li 
                                                        class:active=is_active
                                                        on:click=move |_| set_active_tab.set(tab_value.clone())
                                                    >
                                                        {label}
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                </div>
                                
                                <div class="profile-content">
                                    {move || match active_tab.get().as_str() {
                                        "summary" => view! {
                                            <div class="profile-summary">
                                                <div class="profile-bio">
                                                    <h3>"About"</h3>
                                                    {match p.bio {
                                                        Some(ref bio) if !bio.is_empty() => view! { <div class="bio-content">{bio}</div> },
                                                        _ => view! { <div class="bio-empty">"No bio provided"</div> }
                                                    }}
                                                </div>
                                                
                                                <div class="profile-details">
                                                    <h3>"Details"</h3>
                                                    <ul class="details-list">
                                                        <li>
                                                            <span class="detail-label">"Member since:"</span>
                                                            <span class="detail-value">{format_date_for_display(Some(&u.created_at))}</span>
                                                        </li>
                                                        
                                                        {if let Some(location) = &p.location {
                                                            if !location.is_empty() {
                                                                view! {
                                                                    <li>
                                                                        <span class="detail-label">"Location:"</span>
                                                                        <span class="detail-value">{location}</span>
                                                                    </li>
                                                                }
                                                            } else {
                                                                view! { <span></span> }
                                                            }
                                                        } else {
                                                            view! { <span></span> }
                                                        }}
                                                        
                                                        {if let Some(website) = &p.website {
                                                            if !website.is_empty() {
                                                                view! {
                                                                    <li>
                                                                        <span class="detail-label">"Website:"</span>
                                                                        <a href={website} target="_blank" class="detail-link">{website}</a>
                                                                    </li>
                                                                }
                                                            } else {
                                                                view! { <span></span> }
                                                            }
                                                        } else {
                                                            view! { <span></span> }
                                                        }}
                                                    </ul>
                                                </div>
                                                
                                                <div class="profile-recent-topics">
                                                    <h3>"Recent Topics"</h3>
                                                    {if topics.get().is_empty() {
                                                        view! { <div class="empty-list">"No topics created yet"</div> }
                                                    } else {
                                                        view! {
                                                            <ul class="topics-list">
                                                                {topics.get().into_iter().map(|topic| {
                                                                    view! {
                                                                        <li class="topic-item">
                                                                            <a href={format!("/topics/{}", topic.id)} class="topic-title">
                                                                                {topic.title}
                                                                            </a>
                                                                            <div class="topic-meta">
                                                                                <span class="topic-category">{topic.category_name}</span>
                                                                                <span class="topic-date">
                                                                                    {format_date_for_display(Some(&topic.created_at))}
                                                                                </span>
                                                                            </div>
                                                                        </li>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </ul>
                                                            <a href="#" class="view-all-link" on:click=move |_| set_active_tab.set("topics".to_string())>
                                                                "View all topics"
                                                            </a>
                                                        }
                                                    }}
                                                </div>
                                                
                                                <div class="profile-recent-posts">
                                                    <h3>"Recent Replies"</h3>
                                                    {if posts.get().is_empty() {
                                                        view! { <div class="empty-list">"No posts created yet"</div> }
                                                    } else {
                                                        view! {
                                                            <ul class="posts-list">
                                                                {posts.get().into_iter().map(|post| {
                                                                    view! {
                                                                        <li class="post-item">
                                                                            <div class="post-topic">
                                                                                <a href={format!("/topics/{}", post.topic_id)} class="post-topic-link">
                                                                                    {post.topic_title}
                                                                                </a>
                                                                            </div>
                                                                            <div class="post-excerpt">
                                                                                {truncate_post_content(&post.raw, 150)}
                                                                            </div>
                                                                            <div class="post-meta">
                                                                                <span class="post-date">
                                                                                    {format_date_for_display(Some(&post.created_at))}
                                                                                </span>
                                                                                <span class="post-likes">
                                                                                    {"♥ "}{post.like_count}
                                                                                </span>
                                                                            </div>
                                                                        </li>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </ul>
                                                            <a href="#" class="view-all-link" on:click=move |_| set_active_tab.set("replies".to_string())>
                                                                "View all replies"
                                                            </a>
                                                        }
                                                    }}
                                                </div>
                                            </div>
                                        },
                                        "activity" => view! {
                                            <UserActivityFeed user_id={u.id.clone()} />
                                        },
                                        "topics" => view! {
                                            <UserTopicsList user_id={u.id.clone()} />
                                        },
                                        "replies" => view! {
                                            <UserPostsList user_id={u.id.clone()} />
                                        },
                                        _ => view! { <div>"Unknown tab"</div> }
                                    }}
                                </div>
                            }
                        },
                        _ => view! { <div class="error-state">"User profile not found"</div> }
                    }
                }
            }}
        </div>
    }
}

// Helper function to get current user ID from auth context
fn get_current_user_id() -> Option<String> {
    // This would be implemented to get the current authenticated user
    // For example: use_context::<AuthContext>().map(|ctx| ctx.user_id.clone())
    None
}

// Helper to truncate post content for display
fn truncate_post_content(content: &str, max_length: usize) -> String {
    if content.len() <= max_length {
        content.to_string()
    } else {
        format!("{}...", &content[0..max_length])
    }
}

// Helper to redirect to a different page
fn window_location_assign(path: &str) {
    use wasm_bindgen::JsValue;
    
    if let Ok(window) = web_sys::window() {
        let _ = window.location().assign(&JsValue::from_str(path));
    }
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: serde::Serialize + ?Sized,
    R: for<'de> serde::de::DeserializeOwned,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}