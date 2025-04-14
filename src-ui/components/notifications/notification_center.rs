use leptos::*;
use crate::models::notification::Notification;
use crate::services::notification_service::NotificationService;
use crate::components::notifications::integration_notification::IntegrationNotification;

#[component]
pub fn NotificationCenter() -> impl IntoView {
    // State
    let (notifications, set_notifications) = create_signal(Vec::<Notification>::new());
    let (show_all, set_show_all) = create_signal(false);
    let (loading, set_loading) = create_signal(false);
    
    // Load notifications
    let load_notifications = move || {
        set_loading.set(true);
        
        spawn_local(async move {
            match NotificationService::get_notifications().await {
                Ok(notifs) => {
                    set_notifications.set(notifs);
                },
                Err(e) => {
                    log::error!("Failed to load notifications: {}", e);
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Dismiss notification
    let dismiss_notification = move |id: String| {
        spawn_local(async move {
            if let Err(e) = NotificationService::dismiss_notification(&id).await {
                log::error!("Failed to dismiss notification: {}", e);
            } else {
                // Remove from local state
                set_notifications.update(|notifs| {
                    notifs.retain(|n| n.id != id);
                });
            }
        });
    };
    
    // Dismiss all notifications
    let dismiss_all = move |_| {
        spawn_local(async move {
            if let Err(e) = NotificationService::dismiss_all_notifications().await {
                log::error!("Failed to dismiss all notifications: {}", e);
            } else {
                // Clear local state
                set_notifications.set(Vec::new());
            }
        });
    };
    
    // Toggle show all
    let toggle_show_all = move |_| {
        set_show_all.update(|val| *val = !*val);
    };
    
    // Load notifications on component mount
    create_effect(move |_| {
        load_notifications();
    });
    
    // Set up notification polling
    create_effect(move |_| {
        let interval_id = window().set_interval_with_callback_and_timeout_and_arguments_0(
            move || {
                load_notifications();
            },
            30000, // Poll every 30 seconds
        ).unwrap();
        
        on_cleanup(move || {
            window().clear_interval_with_handle(interval_id);
        });
    });
    
    view! {
        <div class="notification-center">
            <div class="notification-header">
                <h3 class="notification-title">"Notifications"</h3>
                
                <div class="notification-actions">
                    <button 
                        class="btn btn-sm btn-secondary"
                        on:click=toggle_show_all
                        disabled=loading.get()
                    >
                        {move || if show_all.get() { "Show Recent" } else { "Show All" }}
                    </button>
                    
                    <button 
                        class="btn btn-sm btn-danger"
                        on:click=dismiss_all
                        disabled=move || loading.get() || notifications.get().is_empty()
                    >
                        "Dismiss All"
                    </button>
                </div>
            </div>
            
            <div class="notification-list">
                {move || {
                    let notifs = notifications.get();
                    
                    if notifs.is_empty() {
                        view! {
                            <div class="empty-notifications">
                                <p>"No notifications to display."</p>
                            </div>
                        }
                    } else {
                        let filtered_notifs = if show_all.get() {
                            notifs
                        } else {
                            notifs.into_iter().take(5).collect::<Vec<_>>()
                        };
                        
                        view! {
                            <For
                                each=move || filtered_notifs.clone()
                                key=|notif| notif.id.clone()
                                let:notification
                            >
                                <IntegrationNotification 
                                    notification=notification
                                    on_dismiss=dismiss_notification
                                />
                            </For>
                            
                            {move || {
                                let total = notifications.get().len();
                                let shown = filtered_notifs.len();
                                
                                if !show_all.get() && total > shown {
                                    view! {
                                        <div class="notification-more">
                                            <button class="btn btn-link" on:click=toggle_show_all>
                                                {"Show "}{total - shown}{" more notifications"}
                                            </button>
                                        </div>
                                    }
                                } else {
                                    view! { <></> }
                                }
                            }}
                        }
                    }
                }}
            </div>
        </div>
    }
}
