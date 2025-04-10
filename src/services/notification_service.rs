// src/services/notification_service.rs
use crate::api::{canvas_api, discourse_api};
use crate::models::unified_models::Notification;
use crate::error::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NotificationOptions {
    pub limit: Option<usize>,
    pub only_unread: Option<bool>,
    pub category: Option<String>,
}

#[async_trait]
pub trait NotificationService {
    async fn get_user_notifications(&self, user_id: &str, options: Option<NotificationOptions>) -> Result<Vec<Notification>, AppError>;
    async fn mark_as_read(&self, notification_id: &str, source: &str) -> Result<Notification, AppError>;
}

pub struct DefaultNotificationService {
    canvas_api: Box<dyn canvas_api::CanvasApi>,
    discourse_api: Box<dyn discourse_api::DiscourseApi>,
}

impl DefaultNotificationService {
    pub fn new(
        canvas_api: Box<dyn canvas_api::CanvasApi>,
        discourse_api: Box<dyn discourse_api::DiscourseApi>,
    ) -> Self {
        Self {
            canvas_api,
            discourse_api,
        }
    }
    
    async fn get_user_mapping(&self, user_id: &str) -> Result<UserMapping, AppError> {
        // Implementation to get user mapping between systems
        // This would typically call a repository or service
        todo!("Implement user mapping retrieval")
    }
    
    fn apply_notification_filters(&self, notifications: Vec<Notification>, options: &NotificationOptions) -> Vec<Notification> {
        let mut filtered = notifications;
        
        // Apply filters based on options
        if let Some(true) = options.only_unread {
            filtered = filtered.into_iter().filter(|n| !n.read).collect();
        }
        
        if let Some(category) = &options.category {
            filtered = filtered.into_iter().filter(|n| n.category == *category).collect();
        }
        
        // Apply limit if specified
        if let Some(limit) = options.limit {
            filtered.truncate(limit);
        }
        
        filtered
    }
}

#[async_trait]
impl NotificationService for DefaultNotificationService {
    async fn get_user_notifications(&self, user_id: &str, options: Option<NotificationOptions>) -> Result<Vec<Notification>, AppError> {
        // Get user mapping
        let user_mapping = self.get_user_mapping(user_id).await?;
        
        // Fetch notifications from both systems concurrently
        let canvas_future = self.canvas_api.get_user_notifications(&user_mapping.canvas_id);
        let discourse_future = self.discourse_api.get_user_notifications(&user_mapping.discourse_id);
        
        let (canvas_result, discourse_result) = futures::join!(canvas_future, discourse_future);
        
        let canvas_notifications = canvas_result?;
        let discourse_notifications = discourse_result?;
        
        // Convert to unified model
        let mut unified_notifications: Vec<Notification> = Vec::new();
        
        for notification in canvas_notifications {
            unified_notifications.push(Notification::from_canvas_notification(&notification));
        }
        
        for notification in discourse_notifications {
            unified_notifications.push(Notification::from_discourse_notification(&notification));
        }
        
        // Sort by date (newest first)
        unified_notifications.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply filters
        let options = options.unwrap_or_default();
        let filtered = self.apply_notification_filters(unified_notifications, &options);
        
        Ok(filtered)
    }
    
    async fn mark_as_read(&self, notification_id: &str, source: &str) -> Result<Notification, AppError> {
        match source {
            "canvas" => {
                let notification = self.canvas_api.mark_notification_as_read(notification_id).await?;
                Ok(Notification::from_canvas_notification(&notification))
            },
            "discourse" => {
                let notification = self.discourse_api.mark_notification_as_read(notification_id).await?;
                Ok(Notification::from_discourse_notification(&notification))
            },
            _ => Err(AppError::InvalidSource(source.to_string())),
        }
    }
}

// Helper types
#[derive(Debug)]
struct UserMapping {
    canvas_id: String,
    discourse_id: String,
}
