use leptos::*;
use crate::models::notification::{Notification, NotificationType, NotificationSummary};
use crate::services::notification::NotificationService;
use crate::utils::auth::AuthState;
use web_sys::MouseEvent;
use chrono::Utc;

#[component]
pub fn NotificationDropdown() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (summary, set_summary) = create_signal(None::<NotificationSummary>);
    let (loading, set_loading) = create_signal(false);
    let (dropdown_open, set_dropdown_open) = create_signal(false);
    
    // Unread count
    let unread_count = create_memo(move |_| {
        summary().map(|s| s.unread_count).unwrap_or(0)
    });
    
    // Recent notifications for the dropdown
    let recent_notifications = create_memo(move |_| {
        summary()
            .map(|s| s.recent_notifications)
            .unwrap_or_default()
    });
    
    // Load notifications on login/init
    create_effect(move |_| {
        if !is_logged_in() {
            return;
        }
        
        let user_id = current_user_id();
        if user_id == 0 {
            return;
        }
        
        // Load notification summary
        load_notification_summary(user_id);
        
        // Set up polling for new notifications (every minute)
        let user_id_clone = user_id;
        spawn_local(async move {
            loop {
                leptos::timeout(60000, move || {
                    if is_logged_in() {
                        load_notification_summary(user_id_clone);
                    }
                }).await;
            }
        });
    });
    
    // Load notification summary
    let load_notification_summary = move |user_id: i64| {
        set_loading.set(true);
        
        spawn_local(async move {
            match NotificationService::get_notification_summary(user_id).await {
                Ok(notification_summary) => {
                    set_summary.set(Some(notification_summary));
                },
                Err(_) => {
                    // Handle error (could set an error state)
                }
            }
            set_loading.set(false);
        });
    };
    
    // Mark notification as read
    let mark_as_read = move |ev: MouseEvent, notification_id: i64| {
        let user_id = current_user_id();
        
        // Don't close dropdown when clicking inside
        ev.stop_propagation();
        
        spawn_local(async move {
            match NotificationService::mark_as_read(user_id, notification_id).await {
                Ok(_) => {
                    // Refresh notification summary to update unread count
                    load_notification_summary(user_id);
                },
                Err(_) => {
                    // Handle error
                }
            }
        });
    };
    
    // Mark all as read
    let mark_all_as_read = move |ev: MouseEvent| {
        let user_id = current_user_id();
        
        // Don't close dropdown when clicking inside
        ev.stop_propagation();
        
        spawn_local(async move {
            match NotificationService::mark_all_as_read(user_id).await {
                Ok(_) => {
                    // Refresh notification summary to update unread count
                    load_notification_summary(user_id);
                },
                Err(_) => {
                    // Handle error
                }
            }
        });
    };
    
    // Toggle dropdown
    let toggle_dropdown = move |ev: MouseEvent| {
        ev.stop_propagation();
        set_dropdown_open.update(|open| *open = !*open);
    };
    
    // Close dropdown when clicking outside
    create_effect(move |_| {
        if dropdown_open() {
            let document = web_sys::window().unwrap().document().unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let target = event.target().unwrap();
                let dropdown_element = document.get_element_by_id("notificationDropdown");
                
                if let Some(dropdown) = dropdown_element {
                    if !dropdown.contains(Some(&target)) {
                        set_dropdown_open.set(false);
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            
            on_cleanup(move || {
                document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            });
        }
    });
    
    // Format time ago
    let format_time_ago = |date: chrono::DateTime<chrono::Utc>| -> String {
        let now = Utc::now();
        let diff = now.signed_duration_since(date);
        
        if diff.num_days() > 365 {
            format!("{} year{} ago", diff.num_days() / 365, if diff.num_days() / 365 > 1 { "s" } else { "" })
        } else if diff.num_days() > 30 {
            format!("{} month{} ago", diff.num_days() / 30, if diff.num_days() / 30 > 1 { "s" } else { "" })
        } else if diff.num_days() > 0 {
            format!("{} day{} ago", diff.num_days(), if diff.num_days() > 1 { "s" } else { "" })
        } else if diff.num_hours() > 0 {
            format!("{} hour{} ago", diff.num_hours(), if diff.num_hours() > 1 { "s" } else { "" })
        } else if diff.num_minutes() > 0 {
            format!("{} minute{} ago", diff.num_minutes(), if diff.num_minutes() > 1 { "s" } else { "" })
        } else {
            "just now".to_string()
        }
    };
    
    // Get notification icon
    let get_notification_icon = |notification_type: &NotificationType| -> &'static str {
        match notification_type {
            NotificationType::Reply => "bi-reply",
            NotificationType::Mention => "bi-at",
            NotificationType::Quote => "bi-quote",
            NotificationType::PrivateMessage => "bi-envelope",
            NotificationType::GroupMention => "bi-people",
            NotificationType::Reaction => "bi-emoji-smile",
            NotificationType::TopicCreated => "bi-plus-circle",
            NotificationType::AdminNotification => "bi-shield",
            NotificationType::PostEdited => "bi-pencil",
            NotificationType::TopicMoved => "bi-arrow-right",
            NotificationType::BadgeAwarded => "bi-award",
            NotificationType::WelcomeNotification => "bi-hand-wave",
            NotificationType::SystemNotification => "bi-info-circle",
            NotificationType::TaggedUser => "bi-tag",
        }
    };
    
    // Get notification URL
    let get_notification_url = |notification: &Notification| -> String {
        if let Some(topic_id) = notification.data.topic_id {
            if let Some(post_id) = notification.data.post_id {
                return format!("/topic/{}/post/{}", topic_id, post_id);
            }
            return format!("/topic/{}", topic_id);
        } else if notification.notification_type == NotificationType::PrivateMessage {
            if let Some(from_user_id) = notification.data.from_user_id {
                return format!("/messages/{}", from_user_id);
            }
        } else if notification.notification_type == NotificationType::BadgeAwarded {
            return "/user/badges".to_string();
        }
        
        // Default to notifications page
        format!("/notifications")
    };

    view! {
        // Only render if user is logged in
        {move || if is_logged_in() {
            view! {
                <div id="notificationDropdown" class="dropdown">
                    <button 
                        class="btn btn-link nav-link position-relative px-2" 
                        on:click=toggle_dropdown
                        aria-expanded=move || dropdown_open()
                    >
                        <i class="bi bi-bell fs-5"></i>
                        
                        // Show unread count badge
                        {move || if unread_count() > 0 {
                            view! {
                                <span class="position-absolute top-0 start-75 translate-middle badge rounded-pill bg-danger">
                                    {unread_count()}
                                    <span class="visually-hidden">"unread notifications"</span>
                                </span>
                            }
                        } else {
                            view! { <></> }
                        }}
                    </button>
                    
                    // Dropdown menu
                    <div class=format!("dropdown-menu dropdown-menu-end notification-dropdown p-0 shadow-lg {}", 
                                     if dropdown_open() { "d-block" } else { "d-none" })
                         style="width: 320px; max-height: 500px;">
                         
                        <div class="dropdown-header d-flex justify-content-between align-items-center p-3 border-bottom">
                            <h6 class="mb-0">"Notifications"</h6>
                            <div>
                                <button 
                                    class="btn btn-sm btn-outline-secondary" 
                                    title="Mark all as read"
                                    disabled=move || unread_count() == 0
                                    on:click=mark_all_as_read
                                >
                                    <i class="bi bi-check-all"></i>
                                </button>
                                <a 
                                    href="/notifications" 
                                    class="btn btn-sm btn-link text-decoration-none"
                                    on:click=move |_| set_dropdown_open.set(false)
                                >
                                    "See all"
                                </a>
                            </div>
                        </div>
                        
                        <div class="notification-list overflow-auto" style="max-height: 400px;">
                            {move || if loading() {
                                view! {
                                    <div class="text-center p-3">
                                        <div class="spinner-border spinner-border-sm" role="status">
                                            <span class="visually-hidden">"Loading..."</span>
                                        </div>
                                    </div>
                                }
                            } else if recent_notifications().is_empty() {
                                view! {
                                    <div class="text-center p-4">
                                        <i class="bi bi-bell-slash mb-2 d-block" style="font-size: 2rem;"></i>
                                        <p class="text-muted mb-0">"No notifications"</p>
                                    </div>
                                }
                            } else {
                                view! {
                                    <For
                                        each=move || recent_notifications()
                                        key=|notification| notification.id
                                        children=move |notification: Notification| {
                                            let notification_id = notification.id;
                                            let notification_url = get_notification_url(&notification);
                                            let icon_class = get_notification_icon(&notification.notification_type);
                                            let is_read = notification.read;
                                            let time_ago = format_time_ago(notification.created_at);
                                            
                                            view! {
                                                <a 
                                                    href={notification_url}
                                                    class=format!("notification-item d-flex p-3 border-bottom text-decoration-none {}", 
                                                              if is_read { "bg-white" } else { "bg-light" })
                                                    on:click=move |ev| {
                                                        if !is_read {
                                                            mark_as_read(ev, notification_id);
                                                        }
                                                    }
                                                >
                                                    // Avatar or icon
                                                    <div class="flex-shrink-0">
                                                        {match notification.data.from_user_avatar.clone() {
                                                            Some(avatar) => view! {
                                                                <img 
                                                                    src={avatar} 
                                                                    alt="User Avatar" 
                                                                    class="rounded-circle" 
                                                                    width="40" 
                                                                    height="40"
                                                                />
                                                            },
                                                            None => view! {
                                                                <div class="notification-icon-wrapper rounded-circle bg-light d-flex align-items-center justify-content-center" 
                                                                     style="width: 40px; height: 40px;">
                                                                    <i class=format!("bi {} text-primary", icon_class)></i>
                                                                </div>
                                                            }
                                                        }}
                                                    </div>
                                                    
                                                    // Content
                                                    <div class="flex-grow-1 ms-3">
                                                        <div class="d-flex justify-content-between align-items-center mb-1">
                                                            <span class="fw-bold text-body">
                                                                {notification.data.from_username.clone().unwrap_or_else(|| "System".to_string())}
                                                            </span>
                                                            <small class="text-muted">{time_ago}</small>
                                                        </div>
                                                        <p class="mb-1 text-body-secondary">{notification.data.message.clone()}</p>
                                                        
                                                        {notification.data.topic_title.clone().map(|title| {
                                                            view! {
                                                                <small class="text-truncate d-block text-muted">
                                                                    <i class="bi bi-chat-text me-1"></i>
                                                                    {title}
                                                                </small>
                                                            }
                                                        })}
                                                    </div>
                                                    
                                                    // Unread indicator
                                                    {if !is_read {
                                                        view! {
                                                            <span class="position-absolute top-50 end-0 translate-middle-y me-3">
                                                                <span class="bg-primary rounded-circle d-block" style="width: 8px; height: 8px;"></span>
                                                            </span>
                                                        }
                                                    } else {
                                                        view! { <></> }
                                                    }}
                                                </a>
                                            }
                                        }
                                    />
                                }
                            }}
                        </div>
                        
                        <div class="dropdown-footer border-top p-2 text-center">
                            <a 
                                href="/notifications" 
                                class="text-decoration-none small"
                                on:click=move |_| set_dropdown_open.set(false)
                            >
                                "View all notifications"
                            </a>
                        </div>
                    </div>
                </div>
            }
        } else {
            view! { <></> }
        }}
    }
}