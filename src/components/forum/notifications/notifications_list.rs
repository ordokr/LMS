use leptos::*;
use crate::models::notification::Notification;
use crate::services::notification::NotificationService;
use crate::utils::auth::AuthState;
use crate::utils::formatting::{format_datetime, format_relative_time};

#[component]
pub fn NotificationsList() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (notifications, set_notifications) = create_signal(Vec::<Notification>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    let (filter, set_filter) = create_signal("all".to_string());
    
    // Filter notifications
    let filtered_notifications = create_memo(move |_| {
        let mut filtered = notifications.get();
        
        match filter().as_str() {
            "unread" => {
                filtered = filtered.into_iter()
                    .filter(|n| !n.read)
                    .collect();
            },
            "mentions" => {
                filtered = filtered.into_iter()
                    .filter(|n| n.notification_type == "mention")
                    .collect();
            },
            "replies" => {
                filtered = filtered.into_iter()
                    .filter(|n| n.notification_type == "reply")
                    .collect();
            },
            "quotes" => {
                filtered = filtered.into_iter()
                    .filter(|n| n.notification_type == "quote")
                    .collect();
            },
            "likes" => {
                filtered = filtered.into_iter()
                    .filter(|n| n.notification_type == "like")
                    .collect();
            },
            "messages" => {
                filtered = filtered.into_iter()
                    .filter(|n| n.notification_type == "message" || n.notification_type == "group_message")
                    .collect();
            },
            _ => {} // "all" - no filtering
        }
        
        filtered
    });
    
    // Load notifications
    let load_notifications = move || {
        if !is_logged_in() {
            set_loading.set(false);
            return;
        }
        
        let user_id = current_user_id();
        if user_id == 0 {
            set_loading.set(false);
            return;
        }
        
        set_loading.set(true);
        
        spawn_local(async move {
            match NotificationService::get_notifications(user_id).await {
                Ok(user_notifications) => {
                    set_notifications.set(user_notifications);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load notifications: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        load_notifications();
    });
    
    // Mark notification as read
    let mark_as_read = move |notification_id: i64| {
        let user_id = current_user_id();
        
        spawn_local(async move {
            if let Ok(_) = NotificationService::mark_notification_read(user_id, notification_id).await {
                // Update the notification in the list
                set_notifications.update(|notifs| {
                    let mut updated = notifs.clone();
                    if let Some(idx) = updated.iter().position(|n| n.id == notification_id) {
                        updated[idx].read = true;
                    }
                    *notifs = updated;
                });
            }
        });
    };
    
    // Mark all as read
    let mark_all_as_read = move |_| {
        let user_id = current_user_id();
        
        spawn_local(async move {
            match NotificationService::mark_all_notifications_read(user_id).await {
                Ok(_) => {
                    // Update all notifications in the list
                    set_notifications.update(|notifs| {
                        let mut updated = notifs.clone();
                        for notification in &mut updated {
                            notification.read = true;
                        }
                        *notifs = updated;
                    });
                    set_success.set(Some("All notifications marked as read".to_string()));
                    
                    // Clear success message after 3 seconds
                    spawn_local(async {
                        leptos::timeout(3000, move || {
                            set_success.set(None);
                        }).await;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to mark all notifications as read: {}", e)));
                }
            }
        });
    };
    
    // Get notification icon based on type
    let get_notification_icon = |notification_type: &str| -> &'static str {
        match notification_type {
            "mention" => "bi-at",
            "reply" => "bi-reply",
            "quote" => "bi-quote",
            "like" => "bi-heart",
            "message" => "bi-envelope",
            "follow" => "bi-person-plus",
            "group_mention" => "bi-people",
            "group_message" => "bi-people-fill",
            "achievement" => "bi-trophy",
            "admin" => "bi-shield",
            _ => "bi-bell"
        }
    };
    
    // Get notification type display name
    let get_notification_type_display = |notification_type: &str| -> &'static str {
        match notification_type {
            "mention" => "Mention",
            "reply" => "Reply",
            "quote" => "Quote",
            "like" => "Like",
            "message" => "Message",
            "follow" => "Follow",
            "group_mention" => "Group Mention",
            "group_message" => "Group Message",
            "achievement" => "Achievement",
            "admin" => "Admin",
            _ => "Notification"
        }
    };
    
    // Get badge color for notification type
    let get_notification_badge_color = |notification_type: &str| -> &'static str {
        match notification_type {
            "mention" => "bg-primary",
            "reply" => "bg-success",
            "quote" => "bg-info",
            "like" => "bg-danger",
            "message" => "bg-warning",
            "follow" => "bg-secondary",
            "group_mention" => "bg-primary",
            "group_message" => "bg-warning",
            "achievement" => "bg-info",
            "admin" => "bg-danger",
            _ => "bg-secondary"
        }
    };

    view! {
        <div class="notifications-list">
            {move || if !is_logged_in() {
                view! {
                    <div class="alert alert-warning">
                        "You must be logged in to view your notifications"
                    </div>
                }
            } else {
                view! {
                    <div>
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h1>"Notifications"</h1>
                            
                            <div>
                                <a href="/notification-preferences" class="btn btn-outline-secondary me-2">
                                    <i class="bi bi-gear me-1"></i>
                                    "Notification Settings"
                                </a>
                                
                                <button 
                                    class="btn btn-outline-primary"
                                    on:click=mark_all_as_read
                                >
                                    <i class="bi bi-check-all me-1"></i>
                                    "Mark all as read"
                                </button>
                            </div>
                        </div>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                        
                        <div class="card mb-4">
                            <div class="card-body p-0">
                                <div class="nav nav-pills nav-fill border-bottom p-2">
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "all"
                                        on:click=move |_| set_filter.set("all".to_string())
                                    >
                                        "All"
                                    </button>
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "unread"
                                        on:click=move |_| set_filter.set("unread".to_string())
                                    >
                                        "Unread"
                                    </button>
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "mentions"
                                        on:click=move |_| set_filter.set("mentions".to_string())
                                    >
                                        <i class="bi bi-at me-1"></i>
                                        "Mentions"
                                    </button>
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "replies"
                                        on:click=move |_| set_filter.set("replies".to_string())
                                    >
                                        <i class="bi bi-reply me-1"></i>
                                        "Replies"
                                    </button>
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "quotes"
                                        on:click=move |_| set_filter.set("quotes".to_string())
                                    >
                                        <i class="bi bi-quote me-1"></i>
                                        "Quotes"
                                    </button>
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "likes"
                                        on:click=move |_| set_filter.set("likes".to_string())
                                    >
                                        <i class="bi bi-heart me-1"></i>
                                        "Likes"
                                    </button>
                                    <button 
                                        class="nav-link" 
                                        class:active=move || filter() == "messages"
                                        on:click=move |_| set_filter.set("messages".to_string())
                                    >
                                        <i class="bi bi-envelope me-1"></i>
                                        "Messages"
                                    </button>
                                </div>
                                
                                {move || if loading() {
                                    view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                                } else if filtered_notifications().is_empty() {
                                    view! {
                                        <div class="text-center p-5">
                                            <i class="bi bi-bell-slash mb-3 d-block" style="font-size: 3rem;"></i>
                                            <h4 class="mb-3">"No Notifications"</h4>
                                            <p class="text-muted">
                                                "You don't have any notifications matching your filter."
                                            </p>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="list-group list-group-flush">
                                            {filtered_notifications().into_iter().map(|notification| {
                                                let notification_id = notification.id;
                                                let notification_url = notification.url.clone();
                                                let is_read = notification.read;
                                                let notification_type = notification.notification_type.clone();
                                                
                                                view! {
                                                    <div 
                                                        class="list-group-item"
                                                        class:bg-light=!is_read
                                                    >
                                                        <div class="d-flex">
                                                            <div class="icon-column text-center me-3" style="width: 40px;">
                                                                <i class={format!("bi {} fs-4", get_notification_icon(&notification_type))}></i>
                                                            </div>
                                                            <div class="content-column flex-grow-1">
                                                                <div class="d-flex justify-content-between align-items-center mb-1">
                                                                    <div>
                                                                        <span class={format!("badge {} me-2", get_notification_badge_color(&notification_type))}>
                                                                            {get_notification_type_display(&notification_type)}
                                                                        </span>
                                                                        {(!is_read).then(|| view! {
                                                                            <span class="badge bg-secondary">"Unread"</span>
                                                                        })}
                                                                    </div>
                                                                    <small class="text-muted">
                                                                        {format_datetime(notification.created_at)}
                                                                    </small>
                                                                </div>
                                                                <div class="notification-content mb-2" inner_html={&notification.content}></div>
                                                                <div class="d-flex justify-content-between align-items-center">
                                                                    <a 
                                                                        href={&notification_url} 
                                                                        class="btn btn-sm btn-outline-primary"
                                                                        on:click=move |_| {
                                                                            if !is_read {
                                                                                mark_as_read(notification_id);
                                                                            }
                                                                        }
                                                                    >
                                                                        "View"
                                                                    </a>
                                                                    
                                                                    {(!is_read).then(|| view! {
                                                                        <button 
                                                                            class="btn btn-sm btn-outline-secondary" 
                                                                            on:click=move |_| mark_as_read(notification_id)
                                                                        >
                                                                            "Mark as read"
                                                                        </button>
                                                                    })}
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
                }
            }}
        </div>
    }
}