use leptos::*;
use crate::models::notification::Notification;
use crate::services::notification_service::NotificationService;
use crate::components::notifications::notification_toast::NotificationToast;

#[component]
pub fn NotificationToastContainer() -> impl IntoView {
    // State
    let (toasts, set_toasts) = create_signal(Vec::<Notification>::new());
    
    // Add a new toast
    let add_toast = move |notification: Notification| {
        set_toasts.update(|t| {
            t.push(notification);
        });
    };
    
    // Remove a toast
    let remove_toast = move |id: String| {
        set_toasts.update(|t| {
            t.retain(|n| n.id != id);
        });
    };
    
    // Listen for notification events
    create_effect(move |_| {
        // In a real application, you would set up an event listener
        // to receive notifications from the backend
        // For now, we'll just poll for new notifications
        
        let interval_id = window().set_interval_with_callback_and_timeout_and_arguments_0(
            move || {
                spawn_local(async move {
                    if let Ok(notifications) = NotificationService::get_unread_notifications().await {
                        for notification in notifications {
                            // Mark as read immediately
                            let _ = NotificationService::mark_notification_read(&notification.id).await;
                            
                            // Add to toasts
                            add_toast(notification);
                        }
                    }
                });
            },
            5000, // Poll every 5 seconds
        ).unwrap();
        
        on_cleanup(move || {
            window().clear_interval_with_handle(interval_id);
        });
    });
    
    view! {
        <div class="notification-toast-container">
            <For
                each=move || toasts.get()
                key=|toast| toast.id.clone()
                let:toast
            >
                <NotificationToast 
                    notification=toast
                    on_dismiss=remove_toast
                />
            </For>
        </div>
    }
}
