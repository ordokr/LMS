use leptos::*;
use crate::models::notification::{Notification, NotificationType};
use crate::services::notification::NotificationService;
use crate::utils::auth::AuthState;
use chrono::Utc;

#[component]
pub fn NotificationsPage() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (notifications, set_notifications) = create_signal(Vec::<Notification>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (filter, set_filter) = create_signal("all".to_string());
    let (read_filter, set_read_filter) = create_signal("all".to_string());
    let (current_page, set_current_page) = create_signal(1);
    let (total_pages, set_total_pages) = create_signal(1);
    let per_page = 20;
    
    // Filtered notifications
    let filtered_notifications = create_memo(move |_| {
        let mut filtered = notifications.get();
        
        // Apply type filter
        if filter() != "all" {
            filtered = filtered.into_iter()
                .filter(|n| {
                    match n.notification_type {
                        NotificationType::Reply if filter() == "replies" => true,
                        NotificationType::Mention if filter() == "mentions" => true,
                        NotificationType::Quote if filter() == "quotes" => true,
                        NotificationType::PrivateMessage if filter() == "messages" => true,
                        NotificationType::Reaction if filter() == "reactions" => true,
                        _ if filter() == "other" => {
                            !matches!(n.notification_type, 
                                NotificationType::Reply | 
                                NotificationType::Mention | 
                                NotificationType::Quote | 
                                NotificationType::PrivateMessage |
                                NotificationType::Reaction)
                        },
                        _ => false
                    }
                })
                .collect();
        }
        
        // Apply read filter
        if read_filter() == "read" {
            filtered = filtered.into_iter().filter(|n| n.read).collect();
        } else if read_filter() == "unread" {
            filtered = filtered.into_iter().filter(|n| !n.read).collect();
        }
        
        filtered
    });
    
    // Load notifications
    create_effect(move |_| {
        if !is_logged_in() {
            set_loading.set(false);
            return;
        }
        
        let user_id = current_user_id();
        if user_id == 0 {
            set_loading.set(false);
            return;
        }
        
        let page = current_page();
        
        set_loading.set(true);
        
        spawn_local(async move {
            match NotificationService::get_notifications(user_id, page, per_page).await {
                Ok(loaded_notifications) => {
                    set_notifications.set(loaded_notifications);
                    // In a real app, you'd get total count from the API response headers
                    // Here we're just setting it to 5 pages for demonstration
                    set_total_pages.set(5);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load notifications: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Mark notification as read
    let mark_as_read = move |notification_id: i64| {
        let user_id = current_user_id();
        
        spawn_local(async move {
            match NotificationService::mark_as_read(user_id, notification_id).await {
                Ok(updated_notification) => {
                    // Update the notification in the list
                    set_notifications.update(|notifications| {
                        let mut updated = notifications.clone();
                        if let Some(idx) = updated.iter().position(|n| n.id == notification_id) {
                            updated[idx] = updated_notification;
                        }
                        *notifications = updated;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to mark notification as read: {}", e)));
                }
            }
        });
    };
    
    // Mark all as read
    let mark_all_as_read = move |_| {
        let user_id = current_user_id();
        
        spawn_local(async move {
            match NotificationService::mark_all_as_read(user_id).await {
                Ok(_) => {
                    // Update all notifications in the list
                    set_notifications.update(|notifications| {
                        let mut updated = notifications.clone();
                        for notification in updated.iter_mut() {
                            notification.read = true;
                        }
                        *notifications = updated;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to mark all notifications as read: {}", e)));
                }
            }
        });
    };
    
    // Delete notification
    let delete_notification = move |notification_id: i64| {
        let user_id = current_user_id();
        
        if !window().confirm_with_message("Are you sure you want to delete this notification?").unwrap_or(false) {
            return;
        }
        
        spawn_local(async move {
            match NotificationService::delete_notification(user_id, notification_id).await {
                Ok(_) => {
                    // Remove the notification from the list
                    set_notifications.update(|notifications| {
                        let mut updated = notifications.clone();
                        updated.retain(|n| n.id != notification_id);
                        *notifications = updated;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete notification: {}", e)));
                }
            }
        });
    };
    
    // Format date
    let format_date = |date: chrono::DateTime<chrono::Utc>| -> String {
        date.format("%b %d, %Y %H:%M").to_string()
    };
    
    // Time ago
    let time_ago = |date: chrono::DateTime<chrono::Utc>| -> String {
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
    
    // Get notification type text
    let get_notification_type_text = |notification_type: &NotificationType| -> &'static str {
        match notification_type {
            NotificationType::Reply => "Reply",
            NotificationType::Mention => "Mention",
            NotificationType::Quote => "Quote",
            NotificationType::PrivateMessage => "Private Message",
            NotificationType::GroupMention => "Group Mention",
            NotificationType::Reaction => "Reaction",
            NotificationType::TopicCreated => "Topic Created",
            NotificationType::AdminNotification => "Admin Notification",
            NotificationType::PostEdited => "Post Edited",
            NotificationType::TopicMoved => "Topic Moved",
            NotificationType::BadgeAwarded => "Badge Awarded",
            NotificationType::WelcomeNotification => "Welcome",
            NotificationType::SystemNotification => "System",
            NotificationType::TaggedUser => "Tagged",
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
        format!("#")
    };

    view! {
        <div class="notifications-page">
            {move || if !is_logged_in() {
                view! {
                    <div class="alert alert-warning">
                        "You must be logged in to view your notifications"
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"Notifications"</h1>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        
                        <div class="card">
                            <div class="card-header">
                                <div class="d-flex justify-content-between align-items-center flex-wrap">
                                    <div class="d-flex gap-2 flex-wrap">
                                        <div class="btn-group">
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if filter() == "all" { "active" } else { "" })
                                                on:click=move |_| set_filter.set("all".to_string())
                                            >
                                                "All"
                                            </button>
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if filter() == "mentions" { "active" } else { "" })
                                                on:click=move |_| set_filter.set("mentions".to_string())
                                            >
                                                <i class="bi bi-at me-1"></i> "Mentions"
                                            </button>
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if filter() == "replies" { "active" } else { "" })
                                                on:click=move |_| set_filter.set("replies".to_string())
                                            >
                                                <i class="bi bi-reply me-1"></i> "Replies"
                                            </button>
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if filter() == "messages" { "active" } else { "" })
                                                on:click=move |_| set_filter.set("messages".to_string())
                                            >
                                                <i class="bi bi-envelope me-1"></i> "Messages"
                                            </button>
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if filter() == "other" { "active" } else { "" })
                                                on:click=move |_| set_filter.set("other".to_string())
                                            >
                                                "Other"
                                            </button>
                                        </div>
                                        
                                        <div class="btn-group ms-0 ms-md-2 mt-2 mt-md-0">
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if read_filter() == "all" { "active" } else { "" })
                                                on:click=move |_| set_read_filter.set("all".to_string())
                                            >
                                                "All"
                                            </button>
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if read_filter() == "unread" { "active" } else { "" })
                                                on:click=move |_| set_read_filter.set("unread".to_string())
                                            >
                                                "Unread"
                                            </button>
                                            <button 
                                                class=format!("btn btn-sm btn-outline-secondary {}", if read_filter() == "read" { "active" } else { "" })
                                                on:click=move |_| set_read_filter.set("read".to_string())
                                            >
                                                "Read"
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="mt-2 mt-md-0">
                                        <button 
                                            class="btn btn-outline-primary btn-sm"
                                            on:click=mark_all_as_read
                                        >
                                            <i class="bi bi-check-all me-1"></i>
                                            "Mark all as read"
                                        </button>
                                    </div>
                                </div>
                            </div>
                            <div class="card-body p-0">
                                {move || if loading() {
                                    view! {
                                        <div class="d-flex justify-content-center p-5">
                                            <div class="spinner-border" role="status">
                                                <span class="visually-hidden">"Loading..."</span>
                                            </div>
                                        </div>
                                    }
                                } else if filtered_notifications().is_empty() {
                                    view! {
                                        <div class="text-center p-5">
                                            <i class="bi bi-bell-slash mb-3 d-block" style="font-size: 3rem;"></i>
                                            <h4>"No notifications found"</h4>
                                            <p class="text-muted">
                                                "You don't have any notifications matching your current filters."
                                            </p>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="list-group list-group-flush">
                                            <For
                                                each=move || filtered_notifications()
                                                key=|notification| notification.id
                                                children=move |notification: Notification| {
                                                    let notification_id = notification.id;
                                                    let notification_url = get_notification_url(&notification);
                                                    let icon_class = get_notification_icon(&notification.notification_type);
                                                    let type_text = get_notification_type_text(&notification.notification_type);
                                                    let is_read = notification.read;
                                                    let time_ago_text = time_ago(notification.created_at);
                                                    let date_formatted = format_date(notification.created_at);
                                                    
                                                    view! {
                                                        <div 
                                                            class=format!("list-group-item notification-item d-flex py-3 px-3 {}", 
                                                                       if is_read { "bg-white" } else { "bg-light" })
                                                        >
                                                            // Avatar or icon
                                                            <div class="flex-shrink-0 me-3">
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
                                                            <div class="flex-grow-1">
                                                                <div class="d-flex justify-content-between align-items-start">
                                                                    <div>
                                                                        <span class="fw-bold me-2">
                                                                            {notification.data.from_username.clone().unwrap_or_else(|| "System".to_string())}
                                                                        </span>
                                                                        <span class="badge bg-secondary me-2">{type_text}</span>
                                                                        <small class="text-muted" title={date_formatted}>{time_ago_text}</small>
                                                                    </div>
                                                                    <div class="d-flex">
                                                                        {if !is_read {
                                                                            view! {
                                                                                <button 
                                                                                    class="btn btn-sm btn-outline-secondary me-1"
                                                                                    title="Mark as read"
                                                                                    on:click=move |_| mark_as_read(notification_id)
                                                                                >
                                                                                    <i class="bi bi-check"></i>
                                                                                </button>
                                                                            }
                                                                        } else {
                                                                            view! { <></> }
                                                                        }}
                                                                        
                                                                        <button 
                                                                            class="btn btn-sm btn-outline-danger"
                                                                            title="Delete notification"
                                                                            on:click=move |_| delete_notification(notification_id)
                                                                        >
                                                                            <i class="bi bi-trash"></i>
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                                
                                                                <p class="mb-1 mt-2">{notification.data.message.clone()}</p>
                                                                
                                                                {notification.data.topic_title.clone().map(|title| {
                                                                    view! {
                                                                        <div class="d-flex align-items-center mt-2">
                                                                            <i class="bi bi-chat-text me-2 text-muted"></i>
                                                                            <a href={notification_url} class="text-decoration-none stretched-link">
                                                                                {title}
                                                                            </a>
                                                                            
                                                                            {notification.data.category_name.clone().map(|category| {
                                                                                let category_color = notification.data.category_color.clone().unwrap_or_else(|| "#6c757d".to_string());
                                                                                
                                                                                view! {
                                                                                    <>
                                                                                        <span class="mx-2">•</span>
                                                                                        <span 
                                                                                            class="badge"
                                                                                            style=format!("background-color: {}", category_color)
                                                                                        >
                                                                                            {category}
                                                                                        </span>
                                                                                    </>
                                                                                }
                                                                            })}
                                                                        </div>
                                                                    }
                                                                })}
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    }
                                }}
                                
                                // Pagination
                                {move || if !filtered_notifications().is_empty() && total_pages() > 1 {
                                    view! {
                                        <div class="card-footer">
                                            <nav aria-label="Notifications pagination">
                                                <ul class="pagination justify-content-center mb-0">
                                                    <li class=format!("page-item {}", if current_page() == 1 { "disabled" } else { "" })>
                                                        <button 
                                                            class="page-link" 
                                                            on:click=move |_| set_current_page.update(|p| *p = (*p - 1).max(1))
                                                        >
                                                            "Previous"
                                                        </button>
                                                    </li>
                                                    
                                                    <For
                                                        each=move || (1..=total_pages().min(7)).collect::<Vec<usize>>()
                                                        key=|&page| page
                                                        children=move |page: usize| {
                                                            view! {
                                                                <li class=format!("page-item {}", if current_page() == page { "active" } else { "" })>
                                                                    <button 
                                                                        class="page-link" 
                                                                        on:click=move |_| set_current_page.set(page)
                                                                    >
                                                                        {page.to_string()}
                                                                    </button>
                                                                </li>
                                                            }
                                                        }
                                                    />
                                                    
                                                    <li class=format!("page-item {}", if current_page() == total_pages() { "disabled" } else { "" })>
                                                        <button 
                                                            class="page-link" 
                                                            on:click=move |_| set_current_page.update(|p| *p = (*p + 1).min(total_pages()))
                                                        >
                                                            "Next"
                                                        </button>
                                                    </li>
                                                </ul>
                                            </nav>
                                        </div>
                                    }
                                } else {
                                    view! { <></> }
                                }}
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}