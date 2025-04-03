use leptos::*;
use crate::models::notification::Notification;
use crate::services::notification::NotificationService;

#[component]
pub fn NotificationIndicator() -> impl IntoView {
    // Auth state to check if user is logged in
    let auth_state = use_context::<AuthState>();
    let is_authenticated = move || auth_state.map(|state| state.is_authenticated()).unwrap_or(false);
    
    let (unread_count, set_unread_count) = create_signal(0);
    let (recent_notifications, set_recent_notifications) = create_signal(Vec::<Notification>::new());
    let (loading, set_loading) = create_signal(false);
    let (dropdown_open, set_dropdown_open) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // Function to load notifications
    let load_notifications = move || {
        if !is_authenticated() {
            return;
        }
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            // Get unread count
            match NotificationService::get_unread_count().await {
                Ok(count) => set_unread_count.set(count as i32),
                Err(e) => log::error!("Failed to load unread count: {}", e),
            }
            
            // Get recent notifications
            match NotificationService::get_recent(5).await {
                Ok(notifications) => set_recent_notifications.set(notifications),
                Err(e) => set_error.set(Some(format!("Failed to load notifications: {}", e))),
            }
            
            set_loading.set(false);
        });
    };
    
    // Load notifications on component mount if authenticated
    create_effect(move |_| {
        if is_authenticated() {
            load_notifications();
        }
    });
    
    // Setup event listener for real-time notifications
    create_effect(move |_| {
        if is_authenticated() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            
            // Create event listener for new notifications
            let handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
                // Reload notifications
                load_notifications();
            }) as Box<dyn FnMut(_)>);
            
            document.add_event_listener_with_callback(
                "new-notification",
                handler.as_ref().unchecked_ref()
            ).unwrap();
            
            // Store handler to prevent it from being garbage collected
            use_context::<RwSignal<Vec<Closure<dyn FnMut(_)>>>>()
                .map(|signal| {
                    signal.update(|handlers| {
                        handlers.push(handler);
                    });
                });
        }
    });
    
    // Function to mark a notification as read
    let mark_as_read = move |id: i64| {
        spawn_local(async move {
            match NotificationService::mark_as_read(id).await {
                Ok(_) => {
                    // Update local state to reflect the change
                    set_recent_notifications.update(|notifications| {
                        for notif in notifications.iter_mut() {
                            if notif.id == id && !notif.read {
                                notif.read = true;
                                set_unread_count.update(|count| *count = (*count - 1).max(0));
                            }
                        }
                    });
                },
                Err(e) => log::error!("Failed to mark notification as read: {}", e),
            }
        });
    };
    
    // Mark all as read
    let mark_all_as_read = move |_| {
        spawn_local(async move {
            match NotificationService::mark_all_as_read().await {
                Ok(_) => {
                    set_unread_count.set(0);
                    set_recent_notifications.update(|notifications| {
                        for notif in notifications.iter_mut() {
                            notif.read = true;
                        }
                    });
                },
                Err(e) => log::error!("Failed to mark all notifications as read: {}", e),
            }
        });
    };
    
    // Toggle dropdown
    let toggle_dropdown = move |_| set_dropdown_open.update(|open| *open = !*open);
    
    view! {
        {move || if is_authenticated() {
            view! {
                <div class="dropdown notification-indicator">
                    <button 
                        class="btn btn-link position-relative"
                        on:click=toggle_dropdown
                        aria-expanded=move || dropdown_open().to_string()
                    >
                        <i class="bi bi-bell fs-5"></i>
                        {move || if unread_count() > 0 {
                            view! {
                                <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">
                                    {unread_count()}
                                </span>
                            }
                        } else {
                            view! {}
                        }}
                    </button>
                    
                    <div class=move || format!("dropdown-menu notification-dropdown p-0 {}",
                                               if dropdown_open() { "show" } else { "" })
                         style="width: 350px; max-height: 500px; overflow-y: auto; right: 0; left: auto;">
                        
                        <div class="dropdown-header d-flex justify-content-between align-items-center p-3 border-bottom">
                            <h6 class="mb-0">"Notifications"</h6>
                            {move || if unread_count() > 0 {
                                view! {
                                    <button class="btn btn-sm btn-link text-decoration-none" on:click=mark_all_as_read>
                                        "Mark all as read"
                                    </button>
                                }
                            } else {
                                view! {}
                            }}
                        </div>
                        
                        {move || if loading() {
                            view! {
                                <div class="d-flex justify-content-center p-3">
                                    <div class="spinner-border spinner-border-sm" role="status">
                                        <span class="visually-hidden">"Loading notifications..."</span>
                                    </div>
                                </div>
                            }
                        } else if let Some(err) = error() {
                            view! {
                                <div class="dropdown-item text-danger p-3">{err}</div>
                            }
                        } else if recent_notifications().is_empty() {
                            view! {
                                <div class="dropdown-item text-center text-muted p-3">
                                    <i class="bi bi-bell-slash mb-2 d-block" style="font-size: 1.5rem;"></i>
                                    "No notifications yet"
                                </div>
                            }
                        } else {
                            view! {
                                <div>
                                    {recent_notifications().into_iter().map(|notification| {
                                        let notif_id = notification.id;
                                        let is_unread = !notification.read;
                                        
                                        view! {
                                            <a 
                                                href={notification.data.link.clone()}
                                                class=format!("dropdown-item p-3 border-bottom notification-item {}", 
                                                           if is_unread { "unread" } else { "" })
                                                style=format!("background-color: {}", if is_unread { "rgba(var(--bs-primary-rgb), 0.05)" } else { "" })
                                                on:click=move |_| mark_as_read(notif_id)
                                            >
                                                <div class="d-flex">
                                                    <div class="notification-icon me-3">
                                                        {match notification.notification_type {
                                                            NotificationType::Reply => view! { <i class="bi bi-reply text-primary"></i> },
                                                            NotificationType::Mention => view! { <i class="bi bi-at text-info"></i> },
                                                            NotificationType::Quote => view! { <i class="bi bi-quote text-secondary"></i> },
                                                            NotificationType::Like => view! { <i class="bi bi-heart text-danger"></i> },
                                                            NotificationType::Solution => view! { <i class="bi bi-check-circle text-success"></i> },
                                                            NotificationType::Welcome => view! { <i class="bi bi-stars text-warning"></i> },
                                                            NotificationType::Message => view! { <i class="bi bi-envelope text-primary"></i> },
                                                            NotificationType::System => {
                                                                if let Some(icon) = &notification.data.icon {
                                                                    let color = notification.data.color.clone().unwrap_or_else(|| "secondary".to_string());
                                                                    view! { <i class={format!("bi bi-{} text-{}", icon, color)}></i> }
                                                                } else {
                                                                    view! { <i class="bi bi-info-circle text-info"></i> }
                                                                }
                                                            }
                                                        }}
                                                    </div>
                                                    <div class="notification-content">
                                                        <div class="notification-title fw-bold">
                                                            {notification.data.title.clone()}
                                                        </div>
                                                        <div class="notification-message small text-muted">
                                                            {notification.data.message.clone()}
                                                        </div>
                                                        <div class="notification-time small text-muted mt-1">
                                                            {format_relative_time(notification.created_at)}
                                                        </div>
                                                    </div>
                                                </div>
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()}
                                    
                                    <div class="dropdown-item text-center p-2">
                                        <a href="/notifications" class="btn btn-link text-decoration-none">
                                            "View all notifications"
                                        </a>
                                    </div>
                                </div>
                            }
                        }}
                    </div>
                </div>
            }
        } else {
            view! {}
        }}
    }
}

// Helper function to format relative time
fn format_relative_time(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 30 {
        date.format("%b %d").to_string()
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