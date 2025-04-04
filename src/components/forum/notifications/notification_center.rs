use leptos::*;
use crate::models::notification::Notification;
use crate::services::notification::NotificationService;
use crate::utils::auth::AuthState;
use crate::utils::formatting::format_relative_time;

#[component]
pub fn NotificationCenter() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (notifications, set_notifications) = create_signal(Vec::<Notification>::new());
    let (loading, set_loading) = create_signal(true);
    let (unread_count, set_unread_count) = create_signal(0);
    let (error, set_error) = create_signal(None::<String>);
    let (filter, set_filter) = create_signal("all".to_string());
    let (show_dropdown, set_show_dropdown) = create_signal(false);
    
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
            _ => {} // "all" - no filtering
        }
        
        filtered
    });
    
    // Count unread notifications
    create_effect(move |_| {
        let count = notifications.get()
            .iter()
            .filter(|n| !n.read)
            .count();
        set_unread_count.set(count as i32);
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
    
    // Toggle dropdown
    let toggle_dropdown = move |_| {
        set_show_dropdown.update(|show| *show = !*show);
    };
    
    // Close dropdown when clicking outside
    let document = window().document().unwrap();
    document.add_event_listener_with_callback("click", &Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let target = event.target().unwrap();
        let dropdown_el = document.get_element_by_id("notification-dropdown");
        
        if let Some(dropdown) = dropdown_el {
            if !dropdown.contains(Some(&target)) && target.node_name() != "BUTTON" {
                set_show_dropdown.set(false);
            }
        }
    }) as Box<dyn FnMut(_)>)).unwrap();
    
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
            if let Ok(_) = NotificationService::mark_all_notifications_read(user_id).await {
                // Update all notifications in the list
                set_notifications.update(|notifs| {
                    let mut updated = notifs.clone();
                    for notification in &mut updated {
                        notification.read = true;
                    }
                    *notifs = updated;
                });
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
    
    // Poll for new notifications every minute
    use_interval(60_000, move || {
        if is_logged_in() {
            load_notifications();
        }
    });

    view! {
        <div class="notification-center position-relative">
            <button 
                class="btn position-relative" 
                class:btn-primary=move || unread_count() > 0
                class:btn-outline-secondary=move || unread_count() == 0
                on:click=toggle_dropdown
            >
                <i class="bi bi-bell"></i>
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
            
            <div 
                id="notification-dropdown"
                class="notification-dropdown dropdown-menu dropdown-menu-end p-0" 
                class:show=move || show_dropdown()
                style="width: 400px; max-height: 500px;"
            >
                <div class="notification-header d-flex justify-content-between align-items-center p-3 border-bottom">
                    <h6 class="mb-0">
                        "Notifications"
                        {move || if unread_count() > 0 {
                            view! {
                                <span class="badge bg-primary ms-2">{unread_count()}</span>
                            }
                        } else {
                            view! {}
                        }}
                    </h6>
                    
                    <div class="btn-group btn-group-sm">
                        <button 
                            type="button" 
                            class="btn btn-outline-secondary" 
                            class:active=move || filter() == "all"
                            on:click=move |_| set_filter.set("all".to_string())
                        >
                            "All"
                        </button>
                        <button 
                            type="button" 
                            class="btn btn-outline-secondary" 
                            class:active=move || filter() == "unread"
                            on:click=move |_| set_filter.set("unread".to_string())
                        >
                            "Unread"
                        </button>
                        <button 
                            type="button" 
                            class="btn btn-outline-secondary" 
                            class:active=move || filter() == "mentions"
                            on:click=move |_| set_filter.set("mentions".to_string())
                        >
                            "Mentions"
                        </button>
                    </div>
                </div>
                
                <div class="notification-body overflow-auto" style="max-height: 400px;">
                    {move || if loading() {
                        view! { <div class="d-flex justify-content-center p-3"><div class="spinner-border spinner-sm" role="status"></div></div> }
                    } else if filtered_notifications().is_empty() {
                        view! {
                            <div class="text-center p-4">
                                <i class="bi bi-bell-slash mb-2 d-block" style="font-size: 2rem;"></i>
                                <p class="text-muted mb-0">"No notifications to display"</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="list-group list-group-flush">
                                {filtered_notifications().into_iter().map(|notification| {
                                    let notification_id = notification.id;
                                    let notification_url = notification.url.clone();
                                    let is_read = notification.read;
                                    
                                    view! {
                                        <a 
                                            href={&notification_url}
                                            class="list-group-item list-group-item-action p-3"
                                            class:unread=!is_read
                                            class:bg-light=!is_read
                                            on:click=move |_| {
                                                if !is_read {
                                                    mark_as_read(notification_id);
                                                }
                                            }
                                        >
                                            <div class="d-flex w-100 align-items-start">
                                                <div class="notification-icon me-3">
                                                    <i class={format!("bi {}", get_notification_icon(&notification.notification_type))}></i>
                                                </div>
                                                <div class="notification-content flex-grow-1 min-width-0">
                                                    <div class="d-flex justify-content-between mb-1">
                                                        <small class="text-muted">
                                                            {format_relative_time(notification.created_at)}
                                                        </small>
                                                        {(!is_read).then(|| view! {
                                                            <span class="badge bg-primary">New</span>
                                                        })}
                                                    </div>
                                                    <p class="mb-1" inner_html={&notification.content}></p>
                                                </div>
                                            </div>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                </div>
                
                <div class="notification-footer d-flex justify-content-between p-2 border-top bg-light">
                    <button 
                        class="btn btn-sm btn-outline-secondary"
                        on:click=mark_all_as_read
                        disabled=move || unread_count() == 0
                    >
                        "Mark all as read"
                    </button>
                    
                    <a href="/notifications" class="btn btn-sm btn-outline-primary">
                        "See all"
                    </a>
                </div>
            </div>
        </div>
    }
}