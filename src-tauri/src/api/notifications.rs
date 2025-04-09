use crate::models::notification::{Notification, NotificationCreate, NotificationStatus, NotificationType};
use crate::db::notification_repository::NotificationRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

/// Gets notifications for a user
///
/// # Arguments
/// * `user_id` - ID of the user
/// * `status` - Optional filter by notification status
/// * `limit` - Optional limit for number of notifications
/// * `offset` - Optional offset for pagination
///
/// # Returns
/// * `Vec<Notification>` - List of notifications for the user
#[tauri::command]
#[instrument(skip(notification_repo), err)]
pub async fn get_notifications(
    user_id: String,
    status: Option<NotificationStatus>,
    limit: Option<u32>,
    offset: Option<u32>,
    notification_repo: State<'_, Arc<dyn NotificationRepository + Send + Sync>>
) -> Result<Vec<Notification>, String> {
    info!(
        event = "api_call", 
        endpoint = "get_notifications", 
        user_id = %user_id,
        status = ?status,
        limit = ?limit,
        offset = ?offset
    );
    
    match notification_repo.get_notifications_for_user(&user_id, status, limit, offset).await {
        Ok(notifications) => {
            info!(
                event = "api_success", 
                endpoint = "get_notifications",
                user_id = %user_id,
                count = notifications.len()
            );
            Ok(notifications)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_notifications", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets the count of unread notifications for a user
///
/// # Arguments
/// * `user_id` - ID of the user
///
/// # Returns
/// * `u32` - Count of unread notifications
#[tauri::command]
#[instrument(skip(notification_repo), err)]
pub async fn get_unread_notification_count(
    user_id: String,
    notification_repo: State<'_, Arc<dyn NotificationRepository + Send + Sync>>
) -> Result<u32, String> {
    info!(event = "api_call", endpoint = "get_unread_notification_count", user_id = %user_id);
    
    match notification_repo.get_unread_count(&user_id).await {
        Ok(count) => {
            info!(
                event = "api_success", 
                endpoint = "get_unread_notification_count",
                user_id = %user_id,
                count = count
            );
            Ok(count)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_unread_notification_count", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new notification
///
/// # Arguments
/// * `notification_create` - Notification creation data
///
/// # Returns
/// * `Notification` - The created notification
#[tauri::command]
#[instrument(skip(notification_repo), err)]
pub async fn create_notification(
    notification_create: NotificationCreate,
    notification_repo: State<'_, Arc<dyn NotificationRepository + Send + Sync>>
) -> Result<Notification, String> {
    info!(
        event = "api_call", 
        endpoint = "create_notification",
        user_id = %notification_create.user_id,
        notification_type = ?notification_create.notification_type
    );
    
    // Generate ID and create full notification object
    let notification = Notification {
        id: Uuid::new_v4().to_string(),
        user_id: notification_create.user_id,
        title: notification_create.title,
        message: notification_create.message,
        notification_type: notification_create.notification_type,
        status: NotificationStatus::Unread,
        reference_id: notification_create.reference_id,
        reference_type: notification_create.reference_type,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    match notification_repo.create_notification(notification).await {
        Ok(created) => {
            info!(
                event = "api_success", 
                endpoint = "create_notification", 
                notification_id = %created.id,
                user_id = %created.user_id
            );
            Ok(created)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "create_notification", error = %e);
            Err(format!("Failed to create notification: {}", e))
        }
    }
}

/// Marks notifications as read
///
/// # Arguments
/// * `notification_ids` - IDs of the notifications to mark as read
/// * `user_id` - ID of the user (for security)
///
/// # Returns
/// * `u32` - Number of notifications updated
#[tauri::command]
#[instrument(skip(notification_repo), err)]
pub async fn mark_notifications_as_read(
    notification_ids: Vec<String>,
    user_id: String,
    notification_repo: State<'_, Arc<dyn NotificationRepository + Send + Sync>>
) -> Result<u32, String> {
    info!(
        event = "api_call", 
        endpoint = "mark_notifications_as_read",
        user_id = %user_id,
        notification_count = notification_ids.len()
    );
    
    match notification_repo.mark_as_read(&notification_ids, &user_id).await {
        Ok(count) => {
            info!(
                event = "api_success", 
                endpoint = "mark_notifications_as_read",
                user_id = %user_id,
                updated_count = count
            );
            Ok(count)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "mark_notifications_as_read", error = %e);
            Err(format!("Failed to update notifications: {}", e))
        }
    }
}

/// Marks all notifications as read for a user
///
/// # Arguments
/// * `user_id` - ID of the user
///
/// # Returns
/// * `u32` - Number of notifications updated
#[tauri::command]
#[instrument(skip(notification_repo), err)]
pub async fn mark_all_notifications_as_read(
    user_id: String,
    notification_repo: State<'_, Arc<dyn NotificationRepository + Send + Sync>>
) -> Result<u32, String> {
    info!(event = "api_call", endpoint = "mark_all_notifications_as_read", user_id = %user_id);
    
    match notification_repo.mark_all_as_read(&user_id).await {
        Ok(count) => {
            info!(
                event = "api_success", 
                endpoint = "mark_all_notifications_as_read",
                user_id = %user_id,
                updated_count = count
            );
            Ok(count)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "mark_all_notifications_as_read", error = %e);
            Err(format!("Failed to update notifications: {}", e))
        }
    }
}

/// Deletes a notification
///
/// # Arguments
/// * `notification_id` - ID of the notification to delete
/// * `user_id` - ID of the user (for security)
///
/// # Returns
/// * `bool` - Whether the deletion was successful
#[tauri::command]
#[instrument(skip(notification_repo), err)]
pub async fn delete_notification(
    notification_id: String,
    user_id: String,
    notification_repo: State<'_, Arc<dyn NotificationRepository + Send + Sync>>
) -> Result<bool, String> {
    info!(
        event = "api_call", 
        endpoint = "delete_notification",
        notification_id = %notification_id,
        user_id = %user_id
    );
    
    match notification_repo.delete_notification(&notification_id, &user_id).await {
        Ok(deleted) => {
            info!(
                event = "api_success", 
                endpoint = "delete_notification",
                notification_id = %notification_id,
                deleted = deleted
            );
            Ok(deleted)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "delete_notification", error = %e);
            Err(format!("Failed to delete notification: {}", e))
        }
    }
}