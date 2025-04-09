# Notifications API Reference

This document describes the Tauri command API for notifications in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_notifications` | `get_notifications(user_id: string, status?: NotificationStatus, limit?: number, offset?: number)` | Retrieves notifications for a user | Implemented |
| `get_unread_notification_count` | `get_unread_notification_count(user_id: string)` | Gets count of unread notifications | Implemented |
| `create_notification` | `create_notification(notification_create: NotificationCreate)` | Creates a new notification | Implemented |
| `mark_notifications_as_read` | `mark_notifications_as_read(notification_ids: string[], user_id: string)` | Marks specific notifications as read | Implemented |
| `mark_all_notifications_as_read` | `mark_all_notifications_as_read(user_id: string)` | Marks all notifications as read | Implemented |
| `delete_notification` | `delete_notification(notification_id: string, user_id: string)` | Deletes a notification | Implemented |

## Data Types

### Notification

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub status: NotificationStatus,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationCreate {
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    Unread,
    Read,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    Discussion,
    Assignment,
    Submission,
    Grade,
    Announcement,
    CourseEnrollment,
    CourseUpdate,
    SystemMessage,
    DiscoursePost,
    DiscourseReply,
    DiscourseMessage,
}
```

// In your Leptos component
use crate::models::notification::{Notification, NotificationStatus, NotificationType};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn NotificationCenter(user_id: String) -> impl IntoView {
    // Fetch notifications for this user
    let notifications = create_resource(
        || user_id.clone(),
        |user_id| async move {
            invoke::<_, Vec<Notification>>("get_notifications", &serde_json::json!({
                "user_id": user_id,
                "status": Some(NotificationStatus::Unread),
                "limit": Some(10),
                "offset": Some(0)
            })).await.ok()
        }
    );
    
    // Get unread count
    let unread_count = create_resource(
        || user_id.clone(),
        |user_id| async move {
            invoke::<_, u32>("get_unread_notification_count", &serde_json::json!({
                "user_id": user_id
            })).await.ok()
        }
    );
    
    // Mark all as read action
    let mark_all_read = create_action(move |_| {
        let user_id = user_id.clone();
        
        async move {
            match invoke::<_, u32>("mark_all_notifications_as_read", &serde_json::json!({
                "user_id": user_id
            })).await {
                Ok(count) => {
                    // Refresh notifications
                    notifications.refetch();
                    unread_count.refetch();
                    Ok(count)
                },
                Err(e) => Err(e.to_string())
            }
        }
    });
    
    // Delete notification action
    let delete_notification = create_action(move |notification_id: &String| {
        let notification_id = notification_id.clone();
        let user_id = user_id.clone();
        
        async move {
            match invoke::<_, bool>("delete_notification", &serde_json::json!({
                "notification_id": notification_id,
                "user_id": user_id
            })).await {
                Ok(_) => {
                    // Refresh notifications
                    notifications.refetch();
                    unread_count.refetch();
                    Ok(())
                },
                Err(e) => Err(e.to_string())
            }
        }
    });
    
    view! {
        <div class="notification-center">
            <div class="notification-header">
                <h2>"Notifications"</h2>
                <span class="unread-count">
                    {move || unread_count.get().map(|count| {
                        count.unwrap_or(0).to_string()
                    })}
                </span>
                <button 
                    on:click=move |_| mark_all_read.dispatch(())
                    disabled=move || unread_count
                        .get()
                        .map(|count| count.unwrap_or(0) == 0)
                        .unwrap_or(true)
                >
                    "Mark All as Read"
                </button>
            </div>
            
            <Suspense fallback=move || view! { <p>"Loading notifications..."</p> }>
                {move || notifications.get().map(|maybe_notifications| match maybe_notifications {
                    Some(list) if !list.is_empty() => {
                        view! {
                            <ul class="notifications-list">
                                {list.into_iter().map(|notification| {
                                    let id = notification.id.clone();
                                    view! {
                                        <li class=format!("notification-item {}", notification.notification_type.to_string().to_lowercase())>
                                            <div class="notification-content">
                                                <h3>{&notification.title}</h3>
                                                <p>{&notification.message}</p>
                                                <span class="timestamp">{&notification.created_at}</span>
                                            </div>
                                            <div class="notification-actions">
                                                <button 
                                                    class="delete-btn"
                                                    on:click=move |_| delete_notification.dispatch(id.clone())
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }
                    },
                    _ => view! { <p>"No notifications found."</p> }
                })}
            </Suspense>
        </div>
    }
}

.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::notifications::get_notifications,
    api::notifications::get_unread_notification_count,
    api::notifications::create_notification,
    api::notifications::mark_notifications_as_read,
    api::notifications::mark_all_notifications_as_read,
    api::notifications::delete_notification,
])
```