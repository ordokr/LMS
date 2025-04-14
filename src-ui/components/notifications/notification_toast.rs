use leptos::*;
use crate::models::notification::Notification;
use std::time::Duration;

#[component]
pub fn NotificationToast(
    #[prop(into)] notification: Notification,
    #[prop(into)] on_dismiss: Callback<String>,
    #[prop(default = 5000)] auto_dismiss_ms: u32,
) -> impl IntoView {
    // State
    let (is_dismissing, set_is_dismissing) = create_signal(false);
    
    // Determine notification type class
    let type_class = match notification.notification_type.as_str() {
        "success" => "notification-success",
        "error" => "notification-error",
        "warning" => "notification-warning",
        "info" => "notification-info",
        _ => "notification-info",
    };
    
    // Determine icon based on notification type
    let icon = match notification.notification_type.as_str() {
        "success" => "icon-success",
        "error" => "icon-error",
        "warning" => "icon-warning",
        "info" => "icon-info",
        _ => "icon-info",
    };
    
    // Handle dismiss button click
    let notification_id = notification.id.clone();
    let handle_dismiss = move |_| {
        set_is_dismissing.set(true);
        
        // Wait for animation to complete before actually dismissing
        let notification_id = notification_id.clone();
        set_timeout(
            move || {
                on_dismiss.call(notification_id);
            },
            Duration::from_millis(300),
        );
    };
    
    // Auto-dismiss after specified time
    let notification_id = notification.id.clone();
    create_effect(move |_| {
        if auto_dismiss_ms > 0 {
            set_timeout(
                move || {
                    set_is_dismissing.set(true);
                    
                    // Wait for animation to complete before actually dismissing
                    let notification_id = notification_id.clone();
                    set_timeout(
                        move || {
                            on_dismiss.call(notification_id);
                        },
                        Duration::from_millis(300),
                    );
                },
                Duration::from_millis(auto_dismiss_ms as u64),
            );
        }
    });
    
    view! {
        <div class=format!("notification-toast {} {}", type_class, if is_dismissing.get() { "slide-out" } else { "" })>
            <div class="notification-icon">
                <i class=icon></i>
            </div>
            
            <div class="notification-content">
                <div class="notification-title">{notification.title.clone()}</div>
                <div class="notification-message">{notification.message.clone()}</div>
                
                {move || if let Some(action_url) = &notification.action_url {
                    view! {
                        <div class="notification-action">
                            <a href={action_url.clone()} class="notification-action-link">
                                {notification.action_text.clone().unwrap_or_else(|| "View".to_string())}
                            </a>
                        </div>
                    }
                } else {
                    view! { <></> }
                }}
            </div>
            
            <button class="notification-dismiss" on:click=handle_dismiss>
                <i class="icon-close"></i>
            </button>
        </div>
    }
}
