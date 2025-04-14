use leptos::*;
use crate::models::notification::Notification;

#[component]
pub fn IntegrationNotification(
    #[prop(into)] notification: Notification,
    #[prop(into)] on_dismiss: Callback<String>,
) -> impl IntoView {
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
    let handle_dismiss = move |_| {
        on_dismiss.call(notification.id.clone());
    };
    
    view! {
        <div class=format!("notification-item {}", type_class)>
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
