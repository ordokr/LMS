use leptos::*;
use crate::models::notification::{Notification, NotificationType};
use crate::services::notification::{NotificationService, NotificationsPage};

#[component]
pub fn AllNotifications() -> impl IntoView {
    // State for notifications
    let (notifications, set_notifications) = create_signal(Vec::<Notification>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    let (total_pages, set_total_pages) = create_signal(1);
    
    // Load notifications for current page
    let load_notifications = move |page: usize| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match NotificationService::get_all(page, 20).await {
                Ok(page_data) => {
                    set_notifications.set(page_data.notifications);
                    set_total_pages.set(page_data.total_pages);
                    set_current_page.set(page_data.page);
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
        load_notifications(1);
    });
    
    // Function to mark a notification as read
    let mark_as_read = move |id: i64| {
        spawn_local(async move {
            if let Err(e) = NotificationService::mark_as_read(id).await {
                log::error!("Failed to mark notification as read: {}", e);
            } else {
                // Update local state to reflect the change
                set_notifications.update(|notifications| {
                    for notif in notifications.iter_mut() {
                        if notif.id == id {
                            notif.read = true;
                        }
                    }
                });
            }
        });
    };
    
    // Mark all as read
    let mark_all_as_read = move |_| {
        spawn_local(async move {
            if let Err(e) = NotificationService::mark_all_as_read().await {
                log::error!("Failed to mark all notifications as read: {}", e);
            } else {
                // Update local state to reflect the change
                set_notifications.update(|notifications| {
                    for notif in notifications.iter_mut() {
                        notif.read = true;
                    }
                });
            }
        });
    };
    
    // Navigate to page
    let go_to_page = move |page: usize| {
        if page != current_page() && page > 0 && page <= total_pages() {
            load_notifications(page);
        }
    };
    
    view! {
        <div class="all-notifications">
            <div class="d-flex justify-content-between align-items-center mb-4">
                <h1>"Notifications"</h1>
                <button class="btn btn-outline-secondary" on:click=mark_all_as_read>
                    "Mark all as read"
                </button>
            </div>
            
            {move || if loading() {
                view! {
                    <div class="d-flex justify-content-center p-5">
                        <div class="spinner-border" role="status">
                            <span class="visually-hidden">"Loading notifications..."</span>
                        </div>
                    </div>
                }
            } else if let Some(err) = error() {
                view! { <div class="alert alert-danger">{err}</div> }
            } else if notifications().is_empty() {
                view! {
                    <div class="text-center p-5">
                        <i class="bi bi-bell-slash mb-3 d-block" style="font-size: 3rem;"></i>
                        <h3>"No notifications yet"</h3>
                        <p class="text-muted">"You'll receive notifications when someone interacts with you on the forum."</p>
                    </div>
                }
            } else {
                view! {
                    <div class="notification-list">
                        {notifications().into_iter().map(|notification| {
                            let notif_id = notification.id;
                            let is_unread = !notification.read;
                            
                            view! {
                                <div class=format!("notification-item card mb-3 {}", if is_unread { "border-primary" } else { "" })>
                                    <div class="card-body">
                                        <div class="d-flex">
                                            <div class="notification-icon me-3">
                                                {match notification.notification_type {
                                                    NotificationType::Reply => view! { <i class="bi bi-reply fs-4 text-primary"></i> },
                                                    NotificationType::Mention => view! { <i class="bi bi-at fs-4 text-info"></i> },
                                                    NotificationType::Quote => view! { <i class="bi bi-quote fs-4 text-secondary"></i> },
                                                    NotificationType::Like => view! { <i class="bi bi-heart fs-4 text-danger"></i> },
                                                    NotificationType::Solution => view! { <i class="bi bi-check-circle fs-4 text-success"></i> },
                                                    NotificationType::Welcome => view! { <i class="bi bi-stars fs-4 text-warning"></i> },
                                                    NotificationType::Message => view! { <i class="bi bi-envelope fs-4 text-primary"></i> },
                                                    NotificationType::System => {
                                                        if let Some(icon) = &notification.data.icon {
                                                            let color = notification.data.color.clone().unwrap_or_else(|| "secondary".to_string());
                                                            view! { <i class={format!("bi bi-{} fs-4 text-{}", icon, color)}></i> }
                                                        } else {
                                                            view! { <i class="bi bi-info-circle fs-4 text-info"></i> }
                                                        }
                                                    }
                                                }}
                                            </div>
                                            <div class="notification-content flex-grow-1">
                                                <div class="d-flex justify-content-between align-items-start mb-2">
                                                    <h5 class="mb-0">{notification.data.title.clone()}</h5>
                                                    <small class="text-muted">{format_relative_time(notification.created_at)}</small>
                                                </div>
                                                <p class="mb-2">{notification.data.message.clone()}</p>
                                                <div class="d-flex justify-content-between">
                                                    <a href={notification.data.link.clone()} class="btn btn-sm btn-primary">
                                                        {notification.data.action_text.clone().unwrap_or_else(|| "View".to_string())}
                                                    </a>
                                                    {if is_unread {
                                                        view! {
                                                            <button class="btn btn-sm btn-outline-secondary" on:click=move |_| mark_as_read(notif_id)>
                                                                "Mark as read"
                                                            </button>
                                                        }
                                                    } else {
                                                        view! {}
                                                    }}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    
                    {move || if total_pages() > 1 {
                        view! {
                            <nav aria-label="Notification pagination">
                                <ul class="pagination justify-content-center">
                                    <li class=format!("page-item {}", if current_page() <= 1 { "disabled" } else { "" })>
                                        <button class="page-link" on:click=move |_| go_to_page(current_page() - 1)>
                                            "Previous"
                                        </button>
                                    </li>
                                    
                                    {(1..=total_pages()).map(|page| {
                                        view! {
                                            <li class=format!("page-item {}", if page == current_page() { "active" } else { "" })>
                                                <button class="page-link" on:click=move |_| go_to_page(page)>
                                                    {page}
                                                </button>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                    
                                    <li class=format!("page-item {}", if current_page() >= total_pages() { "disabled" } else { "" })>
                                        <button class="page-link" on:click=move |_| go_to_page(current_page() + 1)>
                                            "Next"
                                        </button>
                                    </li>
                                </ul>
                            </nav>
                        }
                    } else {
                        view! {}
                    }}
                }
            }}
        </div>
    }
}

// Helper function to format relative time
fn format_relative_time(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 30 {
        date.format("%b %d, %Y").to_string()
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