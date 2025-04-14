use crate::db::DB;
use crate::services::notification::notification_service::NotificationService;
use serde::{Deserialize, Serialize};
use tauri::{State, command};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct NotificationIdRequest {
    pub notification_id: String,
}

#[command]
pub async fn get_notifications(
    db: State<'_, DB>,
) -> Result<Vec<serde_json::Value>, String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.get_notifications(None).await {
        Ok(notifications) => {
            let json_notifications = notifications
                .iter()
                .map(|n| notification_service.notification_to_json(n))
                .collect();
            
            Ok(json_notifications)
        },
        Err(e) => Err(format!("Failed to get notifications: {}", e)),
    }
}

#[command]
pub async fn get_unread_notifications(
    db: State<'_, DB>,
) -> Result<Vec<serde_json::Value>, String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.get_unread_notifications(None).await {
        Ok(notifications) => {
            let json_notifications = notifications
                .iter()
                .map(|n| notification_service.notification_to_json(n))
                .collect();
            
            Ok(json_notifications)
        },
        Err(e) => Err(format!("Failed to get unread notifications: {}", e)),
    }
}

#[command]
pub async fn get_unread_notification_count(
    db: State<'_, DB>,
) -> Result<u32, String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.get_unread_count(None).await {
        Ok(count) => Ok(count),
        Err(e) => Err(format!("Failed to get unread notification count: {}", e)),
    }
}

#[command]
pub async fn mark_notification_read(
    request: NotificationIdRequest,
    db: State<'_, DB>,
) -> Result<(), String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.mark_notification_read(&request.notification_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to mark notification as read: {}", e)),
    }
}

#[command]
pub async fn mark_all_notifications_read(
    db: State<'_, DB>,
) -> Result<(), String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.mark_all_notifications_read(None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to mark all notifications as read: {}", e)),
    }
}

#[command]
pub async fn dismiss_notification(
    request: NotificationIdRequest,
    db: State<'_, DB>,
) -> Result<(), String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.delete_notification(&request.notification_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to dismiss notification: {}", e)),
    }
}

#[command]
pub async fn dismiss_all_notifications(
    db: State<'_, DB>,
) -> Result<(), String> {
    let notification_service = NotificationService::new(db.inner().clone());
    
    match notification_service.delete_all_notifications(None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to dismiss all notifications: {}", e)),
    }
}
