use leptos::*;
use crate::models::user::{TopicSubscription, BookmarkedTopic};
use crate::services::user::UserService;
use crate::utils::auth::AuthState;

#[component]
pub fn UserSubscriptions() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (subscriptions, set_subscriptions) = create_signal(Vec::<TopicSubscription>::new());
    let (bookmarks, set_bookmarks) = create_signal(Vec::<BookmarkedTopic>::new());
    let (loading_subscriptions, set_loading_subscriptions) = create_signal(true);
    let (loading_bookmarks, set_loading_bookmarks) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Filter and pagination
    let (subscription_filter, set_subscription_filter) = create_signal("all".to_string());
    let (bookmark_filter, set_bookmark_filter) = create_signal("all".to_string());
    let (subscription_search, set_subscription_search) = create_signal(String::new());
    let (bookmark_search, set_bookmark_search) = create_signal(String::new());
    
    // Filter subscriptions
    let filtered_subscriptions = create_memo(move |_| {
        let mut filtered = subscriptions.get();
        
        // Apply notification level filter
        if subscription_filter() != "all" {
            filtered = filtered.into_iter()
                .filter(|s| s.notification_level == subscription_filter())
                .collect();
        }
        
        // Apply search filter if any
        if !subscription_search().is_empty() {
            let search_term = subscription_search().to_lowercase();
            filtered = filtered.into_iter()
                .filter(|s| {
                    s.topic_title.to_lowercase().contains(&search_term) || 
                    s.category_name.as_ref().map_or(false, |c| c.to_lowercase().contains(&search_term))
                })
                .collect();
        }
        
        filtered
    });
    
    // Filter bookmarks
    let filtered_bookmarks = create_memo(move |_| {
        let mut filtered = bookmarks.get();
        
        // Apply category filter
        if bookmark_filter() != "all" {
            filtered = filtered.into_iter()
                .filter(|b| b.category_name.as_ref().map_or(false, |c| *c == bookmark_filter()))
                .collect();
        }
        
        // Apply search filter if any
        if !bookmark_search().is_empty() {
            let search_term = bookmark_search().to_lowercase();
            filtered = filtered.into_iter()
                .filter(|b| {
                    b.topic_title.to_lowercase().contains(&search_term) || 
                    b.note.as_ref().map_or(false, |n| n.to_lowercase().contains(&search_term))
                })
                .collect();
        }
        
        filtered
    });
    
    // Available categories for filtering
    let bookmark_categories = create_memo(move |_| {
        let mut categories = bookmarks.get().into_iter()
            .filter_map(|b| b.category_name)
            .collect::<Vec<String>>();
        categories.sort();
        categories.dedup();
        categories
    });
    
    // Load user subscriptions and bookmarks
    create_effect(move |_| {
        if !is_logged_in() {
            set_loading_subscriptions.set(false);
            set_loading_bookmarks.set(false);
            return;
        }
        
        let user_id = current_user_id();
        if user_id == 0 {
            set_loading_subscriptions.set(false);
            set_loading_bookmarks.set(false);
            return;
        }
        
        // Load subscriptions
        spawn_local(async move {
            match UserService::get_topic_subscriptions(user_id).await {
                Ok(user_subscriptions) => {
                    set_subscriptions.set(user_subscriptions);
                    set_loading_subscriptions.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load subscriptions: {}", e)));
                    set_loading_subscriptions.set(false);
                }
            }
        });
        
        // Load bookmarks
        spawn_local(async move {
            match UserService::get_bookmarks(user_id).await {
                Ok(user_bookmarks) => {
                    set_bookmarks.set(user_bookmarks);
                    set_loading_bookmarks.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load bookmarks: {}", e)));
                    set_loading_bookmarks.set(false);
                }
            }
        });
    });
    
    // Update subscription notification level
    let update_subscription = move |topic_id: i64, level: String| {
        let user_id = current_user_id();
        if user_id == 0 {
            return;
        }
        
        spawn_local(async move {
            match UserService::update_topic_subscription(user_id, topic_id, level).await {
                Ok(updated_subscription) => {
                    // Update the subscription in the list
                    set_subscriptions.update(|subs| {
                        let mut updated = subs.clone();
                        if let Some(idx) = updated.iter().position(|s| s.topic_id == topic_id) {
                            updated[idx] = updated_subscription;
                        }
                        *subs = updated;
                    });
                    set_success.set(Some("Subscription updated successfully".to_string()));
                    
                    // Clear success message after a delay
                    spawn_local(async {
                        leptos::timeout(3000, move || {
                            set_success.set(None);
                        }).await;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update subscription: {}", e)));
                }
            }
        });
    };
    
    // Unsubscribe from topic
    let unsubscribe_topic = move |topic_id: i64| {
        if !window().confirm_with_message("Are you sure you want to unsubscribe from this topic?").unwrap_or(false) {
            return;
        }
        
        let user_id = current_user_id();
        
        spawn_local(async move {
            match UserService::unsubscribe_from_topic(user_id, topic_id).await {
                Ok(_) => {
                    // Remove the subscription from the list
                    set_subscriptions.update(|subs| {
                        let mut updated = subs.clone();
                        updated.retain(|s| s.topic_id != topic_id);
                        *subs = updated;
                    });
                    set_success.set(Some("Unsubscribed from topic successfully".to_string()));
                    
                    // Clear success message after a delay
                    spawn_local(async {
                        leptos::timeout(3000, move || {
                            set_success.set(None);
                        }).await;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to unsubscribe from topic: {}", e)));
                }
            }
        });
    };
    
    // Update bookmark note
    let update_bookmark_note = move |bookmark_id: i64, note: String| {
        let user_id = current_user_id();
        
        spawn_local(async move {
            match UserService::update_bookmark_note(user_id, bookmark_id, note).await {
                Ok(updated_bookmark) => {
                    // Update the bookmark in the list
                    set_bookmarks.update(|bks| {
                        let mut updated = bks.clone();
                        if let Some(idx) = updated.iter().position(|b| b.id == bookmark_id) {
                            updated[idx] = updated_bookmark;
                        }
                        *bks = updated;
                    });
                    set_success.set(Some("Bookmark note updated successfully".to_string()));
                    
                    // Clear success message after a delay
                    spawn_local(async {
                        leptos::timeout(3000, move || {
                            set_success.set(None);
                        }).await;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update bookmark note: {}", e)));
                }
            }
        });
    };
    
    // Remove bookmark
    let remove_bookmark = move |bookmark_id: i64| {
        if !window().confirm_with_message("Are you sure you want to remove this bookmark?").unwrap_or(false) {
            return;
        }
        
        let user_id = current_user_id();
        
        spawn_local(async move {
            match UserService::remove_bookmark(user_id, bookmark_id).await {
                Ok(_) => {
                    // Remove the bookmark from the list
                    set_bookmarks.update(|bks| {
                        let mut updated = bks.clone();
                        updated.retain(|b| b.id != bookmark_id);
                        *bks = updated;
                    });
                    set_success.set(Some("Bookmark removed successfully".to_string()));
                    
                    // Clear success message after a delay
                    spawn_local(async {
                        leptos::timeout(3000, move || {
                            set_success.set(None);
                        }).await;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to remove bookmark: {}", e)));
                }
            }
        });
    };
    
    // Format date
    let format_date = |date: chrono::DateTime<chrono::Utc>| -> String {
        // Format as "Jan 1, 2023"
        date.format("%b %e, %Y").to_string()
    };

    view! {
        <div class="user-subscriptions">
            {move || if !is_logged_in() {
                view! {
                    <div class="alert alert-warning">
                        "You must be logged in to view your subscriptions and bookmarks"
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"My Topics"</h1>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                        
                        <ul class="nav nav-tabs mb-4" role="tablist">
                            <li class="nav-item" role="presentation">
                                <button
                                    class="nav-link active"
                                    id="subscriptions-tab"
                                    data-bs-toggle="tab"
                                    data-bs-target="#subscriptions"
                                    type="button"
                                    role="tab"
                                    aria-controls="subscriptions"
                                    aria-selected="true"
                                >
                                    <i class="bi bi-bell me-1"></i>
                                    "Subscriptions"
                                </button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button
                                    class="nav-link"
                                    id="bookmarks-tab"
                                    data-bs-toggle="tab"
                                    data-bs-target="#bookmarks"
                                    type="button"
                                    role="tab"
                                    aria-controls="bookmarks"
                                    aria-selected="false"
                                >
                                    <i class="bi bi-bookmark me-1"></i>
                                    "Bookmarks"
                                </button>
                            </li>
                        </ul>
                        
                        <div class="tab-content">
                            // Subscriptions Tab
                            <div class="tab-pane fade show active" id="subscriptions" role="tabpanel" aria-labelledby="subscriptions-tab">
                                <div class="card mb-4">
                                    <div class="card-header">
                                        <div class="d-flex justify-content-between align-items-center">
                                            <h5 class="mb-0">"Topic Subscriptions"</h5>
                                            <span class="badge bg-secondary">
                                                {move || filtered_subscriptions().len().to_string() + " topics"}
                                            </span>
                                        </div>
                                    </div>
                                    <div class="card-body">
                                        <div class="row mb-3">
                                            <div class="col-md-6">
                                                <div class="input-group">
                                                    <input 
                                                        type="text" 
                                                        class="form-control" 
                                                        placeholder="Search topics..." 
                                                        prop:value=move || subscription_search()
                                                        on:input=move |ev| set_subscription_search.set(event_target_value(&ev))
                                                    />
                                                    <button 
                                                        class="btn btn-outline-secondary" 
                                                        type="button"
                                                        on:click=move |_| set_subscription_search.set(String::new())
                                                    >
                                                        <i class="bi bi-x"></i>
                                                    </button>
                                                </div>
                                            </div>
                                            <div class="col-md-6">
                                                <select
                                                    class="form-select"
                                                    prop:value=move || subscription_filter()
                                                    on:change=move |ev| set_subscription_filter.set(event_target_value(&ev))
                                                >
                                                    <option value="all">"All Notification Levels"</option>
                                                    <option value="watching">"Watching"</option>
                                                    <option value="tracking">"Tracking"</option>
                                                    <option value="normal">"Normal"</option>
                                                    <option value="muted">"Muted"</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        {move || if loading_subscriptions() {
                                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                                        } else if filtered_subscriptions().is_empty() {
                                            view! {
                                                <div class="text-center p-4">
                                                    <i class="bi bi-bell-slash mb-3 d-block" style="font-size: 3rem;"></i>
                                                    <h4>"No Subscriptions Found"</h4>
                                                    <p class="text-muted">
                                                        "You're not subscribed to any topics yet, or no topics match your filter."
                                                    </p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="table-responsive">
                                                    <table class="table table-hover align-middle">
                                                        <thead>
                                                            <tr>
                                                                <th>"Topic"</th>
                                                                <th>"Category"</th>
                                                                <th>"Notification Level"</th>
                                                                <th>"Last Activity"</th>
                                                                <th>"Actions"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            {filtered_subscriptions().into_iter().map(|subscription| {
                                                                let topic_id = subscription.topic_id;
                                                                let topic_url = format!("/topic/{}", subscription.topic_id);
                                                                
                                                                view! {
                                                                    <tr>
                                                                        <td>
                                                                            <a href={&topic_url} class="fw-medium text-decoration-none">
                                                                                {&subscription.topic_title}
                                                                            </a>
                                                                            {subscription.unread_count.map(|count| {
                                                                                if count > 0 {
                                                                                    view! {
                                                                                        <span class="badge bg-primary ms-2">{count} " new"</span>
                                                                                    }
                                                                                } else {
                                                                                    view! {}
                                                                                }
                                                                            })}
                                                                        </td>
                                                                        <td>
                                                                            {subscription.category_name.map(|category| {
                                                                                view! {
                                                                                    <span 
                                                                                        class="badge"
                                                                                        style=format!("background-color: {}", subscription.category_color.unwrap_or_else(|| "#6c757d".to_string()))
                                                                                    >
                                                                                        {category}
                                                                                    </span>
                                                                                }
                                                                            })}
                                                                        </td>
                                                                        <td>
                                                                            <select
                                                                                class="form-select form-select-sm"
                                                                                prop:value={&subscription.notification_level}
                                                                                on:change=move |ev| update_subscription(topic_id, event_target_value(&ev))
                                                                            >
                                                                                <option value="watching">"Watching"</option>
                                                                                <option value="tracking">"Tracking"</option>
                                                                                <option value="normal">"Normal"</option>
                                                                                <option value="muted">"Muted"</option>
                                                                            </select>
                                                                        </td>
                                                                        <td>
                                                                            <small class="text-muted">
                                                                                {format_date(subscription.last_activity_at)}
                                                                            </small>
                                                                        </td>
                                                                        <td>
                                                                            <div class="d-flex gap-2">
                                                                                <a 
                                                                                    href={&topic_url} 
                                                                                    class="btn btn-sm btn-outline-primary"
                                                                                    title="View Topic"
                                                                                >
                                                                                    <i class="bi bi-eye"></i>
                                                                                </a>
                                                                                <button 
                                                                                    class="btn btn-sm btn-outline-danger"
                                                                                    title="Unsubscribe"
                                                                                    on:click=move |_| unsubscribe_topic(topic_id)
                                                                                >
                                                                                    <i class="bi bi-x-circle"></i>
                                                                                </button>
                                                                            </div>
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
                                </div>
                            </div>
                            
                            // Bookmarks Tab
                            <div class="tab-pane fade" id="bookmarks" role="tabpanel" aria-labelledby="bookmarks-tab">
                                <div class="card mb-4">
                                    <div class="card-header">
                                        <div class="d-flex justify-content-between align-items-center">
                                            <h5 class="mb-0">"Bookmarked Topics"</h5>
                                            <span class="badge bg-secondary">
                                                {move || filtered_bookmarks().len().to_string() + " bookmarks"}
                                            </span>
                                        </div>
                                    </div>
                                    <div class="card-body">
                                        <div class="row mb-3">
                                            <div class="col-md-6">
                                                <div class="input-group">
                                                    <input 
                                                        type="text" 
                                                        class="form-control" 
                                                        placeholder="Search bookmarks..." 
                                                        prop:value=move || bookmark_search()
                                                        on:input=move |ev| set_bookmark_search.set(event_target_value(&ev))
                                                    />
                                                    <button 
                                                        class="btn btn-outline-secondary" 
                                                        type="button"
                                                        on:click=move |_| set_bookmark_search.set(String::new())
                                                    >
                                                        <i class="bi bi-x"></i>
                                                    </button>
                                                </div>
                                            </div>
                                            <div class="col-md-6">
                                                <select
                                                    class="form-select"
                                                    prop:value=move || bookmark_filter()
                                                    on:change=move |ev| set_bookmark_filter.set(event_target_value(&ev))
                                                >
                                                    <option value="all">"All Categories"</option>
                                                    {move || bookmark_categories().into_iter().map(|category| {
                                                        view! {
                                                            <option value={category.clone()}>{category}</option>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </select>
                                            </div>
                                        </div>
                                        
                                        {move || if loading_bookmarks() {
                                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                                        } else if filtered_bookmarks().is_empty() {
                                            view! {
                                                <div class="text-center p-4">
                                                    <i class="bi bi-bookmark mb-3 d-block" style="font-size: 3rem;"></i>
                                                    <h4>"No Bookmarks Found"</h4>
                                                    <p class="text-muted">
                                                        "You haven't bookmarked any topics yet, or no bookmarks match your filter."
                                                    </p>
                                                    <p class="text-muted">
                                                        "Look for the bookmark icon on topics to save them for later."
                                                    </p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="row row-cols-1 g-4">
                                                    {filtered_bookmarks().into_iter().map(|bookmark| {
                                                        let bookmark_id = bookmark.id;
                                                        let topic_url = format!("/topic/{}", bookmark.topic_id);
                                                        let post_url = if let Some(post_id) = bookmark.post_id {
                                                            format!("/topic/{}/post/{}", bookmark.topic_id, post_id)
                                                        } else {
                                                            topic_url.clone()
                                                        };
                                                        
                                                        // For edit functionality
                                                        let (editing, set_editing) = create_signal(false);
                                                        let (edit_note, set_edit_note) = create_signal(bookmark.note.clone().unwrap_or_default());
                                                        
                                                        // Clone values for edit handlers
                                                        let bookmark_id_for_save = bookmark_id;
                                                        
                                                        let save_note = move |_| {
                                                            update_bookmark_note(bookmark_id_for_save, edit_note());
                                                            set_editing.set(false);
                                                        };
                                                        
                                                        let cancel_edit = move |_| {
                                                            set_edit_note.set(bookmark.note.clone().unwrap_or_default());
                                                            set_editing.set(false);
                                                        };
                                                        
                                                        view! {
                                                            <div class="col">
                                                                <div class="card h-100">
                                                                    <div class="card-body">
                                                                        <div class="d-flex justify-content-between align-items-start mb-2">
                                                                            <h5 class="card-title">
                                                                                <a href={&post_url} class="text-decoration-none">
                                                                                    {&bookmark.topic_title}
                                                                                </a>
                                                                            </h5>
                                                                            <div class="dropdown">
                                                                                <button class="btn btn-sm btn-outline-secondary" id=format!("bookmark-dropdown-{}", bookmark_id) type="button" data-bs-toggle="dropdown" aria-expanded="false">
                                                                                    <i class="bi bi-three-dots"></i>
                                                                                </button>
                                                                                <ul class="dropdown-menu" aria-labelledby=format!("bookmark-dropdown-{}", bookmark_id)>
                                                                                    <li>
                                                                                        <a class="dropdown-item" href={&post_url}>
                                                                                            <i class="bi bi-eye me-2"></i>
                                                                                            "View"
                                                                                        </a>
                                                                                    </li>
                                                                                    <li>
                                                                                        <button 
                                                                                            class="dropdown-item" 
                                                                                            on:click=move |_| set_editing.set(true)
                                                                                        >
                                                                                            <i class="bi bi-pencil me-2"></i>
                                                                                            "Edit Note"
                                                                                        </button>
                                                                                    </li>
                                                                                    <li><hr class="dropdown-divider"/></li>
                                                                                    <li>
                                                                                        <button 
                                                                                            class="dropdown-item text-danger" 
                                                                                            on:click=move |_| remove_bookmark(bookmark_id)
                                                                                        >
                                                                                            <i class="bi bi-trash me-2"></i>
                                                                                            "Remove Bookmark"
                                                                                        </button>
                                                                                    </li>
                                                                                </ul>
                                                                            </div>
                                                                        </div>
                                                                        
                                                                        {bookmark.category_name.map(|category| {
                                                                            view! {
                                                                                <div class="mb-2">
                                                                                    <span 
                                                                                        class="badge"
                                                                                        style=format!("background-color: {}", bookmark.category_color.unwrap_or_else(|| "#6c757d".to_string()))
                                                                                    >
                                                                                        {category}
                                                                                    </span>
                                                                                </div>
                                                                            }
                                                                        })}
                                                                        
                                                                        {move || if editing() {
                                                                            view! {
                                                                                <div class="mb-3">
                                                                                    <textarea 
                                                                                        class="form-control mb-2" 
                                                                                        rows="3" 
                                                                                        placeholder="Add a note (optional)"
                                                                                        prop:value=move || edit_note()
                                                                                        on:input=move |ev| set_edit_note.set(event_target_value(&ev))
                                                                                    ></textarea>
                                                                                    <div class="d-flex justify-content-end gap-2">
                                                                                        <button class="btn btn-sm btn-outline-secondary" on:click=cancel_edit>
                                                                                            "Cancel"
                                                                                        </button>
                                                                                        <button class="btn btn-sm btn-primary" on:click=save_note>
                                                                                            "Save Note"
                                                                                        </button>
                                                                                    </div>
                                                                                </div>
                                                                            }
                                                                        } else {
                                                                            match &bookmark.note {
                                                                                Some(note) if !note.is_empty() => {
                                                                                    view! {
                                                                                        <div class="card-text mb-3">
                                                                                            <small class="text-muted d-block mb-1">"Your note:"</small>
                                                                                            <div class="note-content p-2 bg-light rounded">
                                                                                                {note}
                                                                                            </div>
                                                                                        </div>
                                                                                    }
                                                                                },
                                                                                _ => view! {
                                                                                    <div class="card-text mb-3">
                                                                                        <small class="text-muted fst-italic">
                                                                                            "No note added"
                                                                                        </small>
                                                                                    </div>
                                                                                }
                                                                            }
                                                                        }}
                                                                        
                                                                        <div class="d-flex justify-content-between align-items-center mt-auto">
                                                                            <small class="text-muted">
                                                                                "Bookmarked "
                                                                                {format_date(bookmark.created_at)}
                                                                            </small>
                                                                            <a href={post_url} class="btn btn-sm btn-outline-primary">
                                                                                "Go to Topic"
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}