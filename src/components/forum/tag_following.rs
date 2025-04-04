use leptos::*;
use crate::models::forum::tag::{Tag, FollowedTag};
use crate::services::forum::ForumService;
use crate::services::auth::AuthService;
use crate::components::common::alerts::Alert;

#[component]
pub fn TagFollowing() -> impl IntoView {
    // State signals
    let (followed_tags, set_followed_tags) = create_signal(Vec::<FollowedTag>::new());
    let (all_tags, set_all_tags) = create_signal(Vec::<Tag>::new());
    let (loading, set_loading) = create_signal(true);
    let (alert, set_alert) = create_signal(None::<(String, String)>);
    let (show_add_modal, set_show_add_modal) = create_signal(false);
    
    // Load followed tags
    create_effect(move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            let tags_result = ForumService::get_followed_tags().await;
            let all_tags_result = ForumService::get_tags().await;
            
            match (tags_result, all_tags_result) {
                (Ok(tags), Ok(all)) => {
                    set_followed_tags.set(tags);
                    set_all_tags.set(all);
                },
                (Err(e), _) | (_, Err(e)) => {
                    set_alert.set(Some(("danger".to_string(), format!("Failed to load tags: {}", e))));
                }
            }
            
            set_loading.set(false);
        });
    });
    
    // Toggle tag following
    let toggle_follow = move |tag_id: i64| {
        spawn_local(async move {
            let is_followed = followed_tags.get().iter().any(|t| t.tag.id == tag_id);
            let result = if is_followed {
                ForumService::unfollow_tag(tag_id).await
            } else {
                ForumService::follow_tag(tag_id).await
            };
            
            match result {
                Ok(_) => {
                    if is_followed {
                        // Remove from followed tags
                        set_followed_tags.update(|tags| {
                            tags.retain(|t| t.tag.id != tag_id);
                        });
                        set_alert.set(Some(("success".to_string(), "Tag unfollowed successfully".to_string())));
                    } else {
                        // Add to followed tags
                        if let Some(tag) = all_tags.get().iter().find(|t| t.id == tag_id).cloned() {
                            set_followed_tags.update(|tags| {
                                tags.push(FollowedTag {
                                    tag,
                                    notification_level: "normal".to_string(),
                                    followed_at: chrono::Utc::now(),
                                });
                            });
                        }
                        set_alert.set(Some(("success".to_string(), "Tag followed successfully".to_string())));
                    }
                },
                Err(e) => {
                    set_alert.set(Some(("danger".to_string(), format!("Failed to update tag: {}", e))));
                }
            }
        });
    };
    
    // Update notification level
    let update_notification_level = move |tag_id: i64, level: String| {
        spawn_local(async move {
            match ForumService::update_tag_notification_level(tag_id, &level).await {
                Ok(_) => {
                    // Update in the list
                    set_followed_tags.update(|tags| {
                        if let Some(tag) = tags.iter_mut().find(|t| t.tag.id == tag_id) {
                            tag.notification_level = level;
                        }
                    });
                },
                Err(e) => {
                    set_alert.set(Some(("danger".to_string(), format!("Failed to update notification level: {}", e))));
                }
            }
        });
    };
    
    // Filtered tags for modal
    let available_tags = create_memo(move |_| {
        let followed_ids: Vec<i64> = followed_tags.get().iter().map(|t| t.tag.id).collect();
        all_tags.get()
            .into_iter()
            .filter(|tag| !followed_ids.contains(&tag.id))
            .collect::<Vec<_>>()
    });

    view! {
        <div class="tag-following">
            <div class="d-flex justify-content-between align-items-center mb-3">
                <h3>"Followed Tags"</h3>
                <button 
                    class="btn btn-primary btn-sm"
                    on:click=move |_| set_show_add_modal.set(true)
                >
                    <i class="bi bi-plus"></i> "Follow New Tags"
                </button>
            </div>
            
            {move || if let Some((variant, message)) = alert() {
                view! { <Alert variant=variant message=message on_close=move || set_alert.set(None) /> }
            } else {
                view! {}
            }}
            
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if followed_tags().is_empty() {
                view! { 
                    <div class="alert alert-info">
                        <i class="bi bi-info-circle me-2"></i>
                        "You aren't following any tags yet. Follow tags to receive notifications and easily find topics that interest you."
                    </div>
                }
            } else {
                view! {
                    <div class="list-group mb-4">
                        {followed_tags().into_iter().map(|followed| {
                            let tag_id = followed.tag.id;
                            let notification_level = followed.notification_level.clone();
                            
                            view! {
                                <div class="list-group-item">
                                    <div class="d-flex justify-content-between align-items-center">
                                        <div class="d-flex align-items-center">
                                            <span 
                                                class="tag-preview me-2"
                                                style={format!("background-color: {}; color: white;", 
                                                    followed.tag.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                            >
                                                {followed.tag.icon.clone().map(|icon| {
                                                    view! { <i class={format!("bi bi-{} me-1", icon)}></i> }
                                                })}
                                                {&followed.tag.name}
                                            </span>
                                            
                                            <small class="text-muted ms-2">
                                                {format!("Following since {}", 
                                                    followed.followed_at.format("%b %d, %Y"))}
                                            </small>
                                        </div>
                                        
                                        <div class="d-flex align-items-center">
                                            <div class="dropdown me-2">
                                                <button 
                                                    class="btn btn-sm btn-outline-secondary dropdown-toggle" 
                                                    type="button"
                                                    id=format!("notificationDropdown{}", tag_id)
                                                    data-bs-toggle="dropdown"
                                                    aria-expanded="false"
                                                >
                                                    {match notification_level.as_str() {
                                                        "muted" => "Muted",
                                                        "high" => "High Priority",
                                                        _ => "Normal"
                                                    }}
                                                </button>
                                                <ul class="dropdown-menu">
                                                    <li>
                                                        <a 
                                                            class="dropdown-item" 
                                                            href="javascript:void(0)"
                                                            class:active=move || notification_level == "normal"
                                                            on:click=move |_| update_notification_level(tag_id, "normal".to_string())
                                                        >
                                                            <i class="bi bi-bell me-2"></i>
                                                            "Normal"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a 
                                                            class="dropdown-item" 
                                                            href="javascript:void(0)"
                                                            class:active=move || notification_level == "high"
                                                            on:click=move |_| update_notification_level(tag_id, "high".to_string())
                                                        >
                                                            <i class="bi bi-bell-fill me-2"></i>
                                                            "High Priority"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a 
                                                            class="dropdown-item" 
                                                            href="javascript:void(0)"
                                                            class:active=move || notification_level == "muted"
                                                            on:click=move |_| update_notification_level(tag_id, "muted".to_string())
                                                        >
                                                            <i class="bi bi-bell-slash me-2"></i>
                                                            "Muted"
                                                        </a>
                                                    </li>
                                                </ul>
                                            </div>
                                            
                                            <button 
                                                class="btn btn-sm btn-outline-danger"
                                                on:click=move |_| toggle_follow(tag_id)
                                            >
                                                <i class="bi bi-x-lg"></i>
                                                "Unfollow"
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="mt-2">
                                        <a 
                                            href={format!("/forum/tags/{}", followed.tag.id)} 
                                            class="text-decoration-none me-3"
                                        >
                                            <i class="bi bi-list-ul me-1"></i>
                                            "View Topics"
                                        </a>
                                        <a 
                                            href={format!("/forum/search?tags={}", followed.tag.name)} 
                                            class="text-decoration-none"
                                        >
                                            <i class="bi bi-search me-1"></i>
                                            "Search in Tag"
                                        </a>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            }}
            
            {move || if show_add_modal() {
                let (filter, set_filter) = create_signal(String::new());
                
                let filtered_available_tags = create_memo(move |_| {
                    let search = filter().to_lowercase();
                    if search.is_empty() {
                        available_tags.get()
                    } else {
                        available_tags
                            .get()
                            .into_iter()
                            .filter(|t| t.name.to_lowercase().contains(&search))
                            .collect()
                    }
                });
                
                view! {
                    <div class="modal fade show" 
                         style="display: block; background-color: rgba(0,0,0,0.5);"
                         tabindex="-1" 
                         aria-labelledby="followTagsModalLabel">
                        <div class="modal-dialog">
                            <div class="modal-content">
                                <div class="modal-header">
                                    <h5 class="modal-title" id="followTagsModalLabel">"Follow Tags"</h5>
                                    <button 
                                        type="button" 
                                        class="btn-close"
                                        aria-label="Close"
                                        on:click=move |_| set_show_add_modal.set(false)
                                    ></button>
                                </div>
                                <div class="modal-body">
                                    <div class="mb-3">
                                        <label for="tagFilter" class="form-label">"Find Tags"</label>
                                        <input 
                                            type="text" 
                                            class="form-control" 
                                            id="tagFilter"
                                            placeholder="Type to filter tags..."
                                            prop:value=move || filter()
                                            on:input=move |ev| set_filter.set(event_target_value(&ev))
                                        />
                                    </div>
                                    
                                    <div class="tag-list" style="max-height: 300px; overflow-y: auto;">
                                        {move || if filtered_available_tags().is_empty() {
                                            view! { 
                                                <div class="alert alert-info">
                                                    "No matching tags found or you're already following all available tags."
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="list-group">
                                                    {filtered_available_tags().into_iter().map(|tag| {
                                                        let tag_id = tag.id;
                                                        
                                                        view! {
                                                            <div class="list-group-item d-flex justify-content-between align-items-center">
                                                                <div>
                                                                    <span 
                                                                        class="tag-preview me-2"
                                                                        style={format!("background-color: {}; color: white;", 
                                                                            tag.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                                                    >
                                                                        {tag.icon.clone().map(|icon| {
                                                                            view! { <i class={format!("bi bi-{} me-1", icon)}></i> }
                                                                        })}
                                                                        {&tag.name}
                                                                    </span>
                                                                    <small class="text-muted">
                                                                        {format!("{} topics", tag.topic_count.unwrap_or(0))}
                                                                    </small>
                                                                </div>
                                                                <button 
                                                                    class="btn btn-sm btn-primary"
                                                                    on:click=move |_| toggle_follow(tag_id)
                                                                >
                                                                    <i class="bi bi-plus-lg me-1"></i>
                                                                    "Follow"
                                                                </button>
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        }}
                                    </div>
                                </div>
                                <div class="modal-footer">
                                    <button 
                                        type="button" 
                                        class="btn btn-secondary"
                                        on:click=move |_| set_show_add_modal.set(false)
                                    >
                                        "Close"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                view! {}
            }}
        </div>
    }
}