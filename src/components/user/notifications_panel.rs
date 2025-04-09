use leptos::*;
use crate::models::notifications::{Notification, NotificationSummary};
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::format_date_for_display;

#[component]
pub fn NotificationsPanel(
    user_id: String,
) -> impl IntoView {
    // State
    let (notifications, set_notifications) = create_signal(Vec::<NotificationSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (unread_count, set_unread_count) = create_signal(0);
    
    // Load notifications
    let load_notifications = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, Vec<NotificationSummary>>(
                "get_user_notification_summaries", 
                &(user_id.clone(), Some(1), Some(20), None::<bool>)
            ).await {
                Ok(fetched_notifications) => {
                    set_notifications.set(fetched_notifications);
                    set_loading.set(false);
                    
                    // Update unread count
                    if let Ok(count) = invoke::<_, i64>(
                        "get_unread_notification_count",
                        &user_id
                    ).await {
                        set_unread_count.set(count as i32);
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load notifications: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load notifications on mount
    create_effect(move |_| {
        load_notifications();
    });
    
    // Mark notification as read
    let mark_as_read = move |notification_id: String| {
        spawn_local(async move {
            if let Ok(_) = invoke::<_, ()>("mark_notification_read", &notification_id).await {
                // Update notification in the list
                set_notifications.update(|list| {
                    list.iter_mut()
                        .filter(|n| n.id == notification_id)
                        .for_each(|n| n.read = true);
                });
                
                // Update unread count
                set_unread_count.update(|count| if *count > 0 { *count -= 1 });
            }
        });
    };
    
    // Mark all as read
    let mark_all_read = move |_| {
        spawn_local(async move {
            if let Ok(_) = invoke::<_, ()>("mark_all_notifications_read", &user_id).await {
                // Update all notifications in the list
                set_notifications.update(|list| {
                    list.iter_mut().for_each(|n| n.read = true);
                });
                
                // Reset unread count
                set_unread_count.set(0);
            }
        });
    };

    view! {
        <div class="notifications-panel">
            <div class="notifications-header">
                <h2>{"Notifications "}<span class="notification-count">{unread_count}</span></h2>
                <button class="mark-all-read" on:click=mark_all_read>
                    "Mark all as read"
                </button>
            </div>
            
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() && notifications.get().is_empty() {
                    view! { <div class="loading-state">"Loading notifications..."</div> }
                } else if notifications.get().is_empty() {
                    view! { <div class="empty-state">"No notifications"</div> }
                } else {
                    view! {
                        <div class="notifications-list">
                            {notifications.get().into_iter().map(|notification| {
                                let notification_id = notification.id.clone();
                                let is_unread = !notification.read;
                                
                                let notification_icon = get_notification_icon(&notification.notification_type);
                                let notification_url = notification.url.clone();
                                
                                view! {
                                    <div 
                                        class={format!("notification-item{}", if is_unread { " unread" } else { "" })}
                                        on:click=move |_| {
                                            if is_unread {
                                                mark_as_read(notification_id.clone());
                                            }
                                            
                                            // Navigate to URL if provided
                                            if let Some(url) = &notification_url {
                                                window_location_assign(url);
                                            }
                                        }
                                    >
                                        <div class="notification-icon">{notification_icon}</div>
                                        
                                        <div class="notification-content">
                                            <div class="notification-title">{notification.title}</div>
                                            <div class="notification-body">{notification.body}</div>
                                            
                                            <div class="notification-meta">
                                                {if let Some(actor_name) = notification.actor_name {
                                                    view! {
                                                        <span class="notification-actor">{actor_name}</span>
                                                        <span class="notification-separator">{"·"}</span>
                                                    }
                                                } else {
                                                    view! { <span></span> }
                                                }}
                                                <span class="notification-date">
                                                    {format_date_for_display(Some(&notification.created_at))}
                                                </span>
                                            </div>
                                        </div>
                                        
                                        {if is_unread {
                                            view! { <div class="unread-indicator"></div> }
                                        } else {
                                            view! { <div></div> }
                                        }}
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Helper to get notification icon
fn get_notification_icon(notification_type: &crate::models::notifications::NotificationType) -> &'static str {
    use crate::models::notifications::NotificationType;
    
    match notification_type {
        NotificationType::Mentioned => "@",
        NotificationType::Replied => "💬",
        NotificationType::Liked => "❤️",
        NotificationType::PrivateMessage => "✉️",
        NotificationType::BadgeAwarded => "🏆",
        NotificationType::TopicUpdated => "🔔",
        NotificationType::Announcement => "📢",
        NotificationType::AssignmentGraded => "✓",
        NotificationType::DueDateReminder => "⏰",
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