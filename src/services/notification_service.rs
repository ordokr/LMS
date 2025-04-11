use crate::api::canvas_api::CanvasApi;
use crate::api::discourse_api::DiscourseApi;
use crate::models::canvas::notification::Notification;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use std::cmp::Ordering;

/// Error types for the notification service
#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Failed to fetch notifications for user {0}: {1}")]
    FetchError(String, String),
    
    #[error("Failed to mark notification {0} as read: {1}")]
    MarkAsReadError(String, String),
    
    #[error("Failed to create notification: {0}")]
    CreateError(String),
    
    #[error("User mapping not found for {0}")]
    UserMappingError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Model conversion error: {0}")]
    ModelConversionError(String),
}

/// User mapping between systems
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMapping {
    pub user_id: String,
    pub canvas_id: String,
    pub discourse_id: String,
}

/// Filter options for notifications
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NotificationFilterOptions {
    pub unread_only: Option<bool>,
    pub since: Option<DateTime<Utc>>,
    pub types: Option<Vec<String>>,
    pub limit: Option<usize>,
}

/// Source system for a notification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NotificationSource {
    Canvas,
    Discourse,
}

impl NotificationSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationSource::Canvas => "canvas",
            NotificationSource::Discourse => "discourse",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "canvas" => NotificationSource::Canvas,
            "discourse" => NotificationSource::Discourse,
            _ => NotificationSource::Canvas, // Default to Canvas
        }
    }
}

/// Service for handling notifications between Canvas and Discourse
pub struct NotificationService {
    canvas_api: Arc<CanvasApi>,
    discourse_api: Arc<DiscourseApi>,
}

impl NotificationService {
    /// Create a new notification service
    pub fn new(canvas_api: Arc<CanvasApi>, discourse_api: Arc<DiscourseApi>) -> Self {
        NotificationService {
            canvas_api,
            discourse_api,
        }
    }
    
    /// Get notifications for a user from both systems
    pub async fn get_user_notifications(
        &self,
        user_id: &str,
        options: Option<NotificationFilterOptions>,
    ) -> Result<Vec<Notification>, NotificationError> {
        // Get user mapping
        let user_mapping = self.get_user_mapping(user_id).await
            .map_err(|e| NotificationError::UserMappingError(user_id.to_string()))?;
        
        // Fetch notifications from both systems in parallel
        let (canvas_results, discourse_results) = tokio::join!(
            self.canvas_api.get_user_notifications(&user_mapping.canvas_id),
            self.discourse_api.get_user_notifications(&user_mapping.discourse_id)
        );
        
        let canvas_notifications = canvas_results
            .map_err(|e| NotificationError::ApiError(e.to_string()))?;
        
        let discourse_notifications = discourse_results
            .map_err(|e| NotificationError::ApiError(e.to_string()))?;
        
        // Convert to unified model
        let mut unified_notifications: Vec<Notification> = Vec::new();
        
        // Add Canvas notifications
        for n in canvas_notifications {
            unified_notifications.push(Notification::from_canvas_notification(&n)
                .map_err(|e| NotificationError::ModelConversionError(e.to_string()))?);
        }
        
        // Add Discourse notifications
        for n in discourse_notifications {
            unified_notifications.push(Notification::from_discourse_notification(&n)
                .map_err(|e| NotificationError::ModelConversionError(e.to_string()))?);
        }
        
        // Sort by date (newest first)
        unified_notifications.sort_by(|a, b| {
            b.created_at.cmp(&a.created_at)
        });
        
        // Apply filters
        let filtered_notifications = self.apply_notification_filters(
            unified_notifications, 
            options.unwrap_or_default()
        );
        
        Ok(filtered_notifications)
    }
    
    /// Mark a notification as read
    pub async fn mark_as_read(
        &self,
        notification_id: &str,
        source: NotificationSource,
    ) -> Result<Notification, NotificationError> {
        match source {
            NotificationSource::Canvas => {
                let result = self.canvas_api.mark_notification_as_read(notification_id).await
                    .map_err(|e| NotificationError::MarkAsReadError(
                        notification_id.to_string(), 
                        e.to_string()
                    ))?;
                
                Notification::from_canvas_notification(&result)
                    .map_err(|e| NotificationError::ModelConversionError(e.to_string()))
            },
            NotificationSource::Discourse => {
                let result = self.discourse_api.mark_notification_as_read(notification_id).await
                    .map_err(|e| NotificationError::MarkAsReadError(
                        notification_id.to_string(), 
                        e.to_string()
                    ))?;
                
                Notification::from_discourse_notification(&result)
                    .map_err(|e| NotificationError::ModelConversionError(e.to_string()))
            }
        }
    }
    
    /// Creates a notification in both systems
    pub async fn create_notification(
        &self,
        notification_data: &serde_json::Value,
    ) -> Result<Notification, NotificationError> {
        // Create unified notification model
        let mut notification = Notification::new(notification_data)
            .map_err(|e| NotificationError::ModelConversionError(e.to_string()))?;
        
        // Get user mapping
        let user_mapping = self.get_user_mapping(&notification.user_id).await
            .map_err(|e| NotificationError::UserMappingError(notification.user_id.clone()))?;
        
        // Send to Canvas
        let mut canvas_notification = notification.to_canvas_notification()
            .map_err(|e| NotificationError::ModelConversionError(e.to_string()))?;
        
        canvas_notification["user_id"] = serde_json::Value::String(user_mapping.canvas_id.clone());
        
        let canvas_result = self.canvas_api.create_notification(&canvas_notification).await
            .map_err(|e| NotificationError::CreateError(e.to_string()))?;
        
        // Send to Discourse
        let mut discourse_notification = notification.to_discourse_notification()
            .map_err(|e| NotificationError::ModelConversionError(e.to_string()))?;
        
        discourse_notification["user_id"] = serde_json::Value::String(user_mapping.discourse_id.clone());
        
        let discourse_result = self.discourse_api.create_notification(&discourse_notification).await
            .map_err(|e| NotificationError::CreateError(e.to_string()))?;
        
        // Update the notification with IDs from both systems
        notification.canvas_id = Some(canvas_result["id"].as_str()
            .ok_or_else(|| NotificationError::ModelConversionError("Missing canvas_id".to_string()))?
            .to_string());
        
        notification.discourse_id = Some(discourse_result["id"].as_str()
            .ok_or_else(|| NotificationError::ModelConversionError("Missing discourse_id".to_string()))?
            .to_string());
        
        Ok(notification)
    }
    
    /// Get user mapping between Canvas and Discourse
    async fn get_user_mapping(&self, user_id: &str) -> Result<UserMapping, NotificationError> {
        // In a real implementation, this would look up the mapping in a database
        // For now, just return a mock mapping
        
        // TODO: Replace with actual implementation
        Ok(UserMapping {
            user_id: user_id.to_string(),
            canvas_id: format!("canvas-{}", user_id),
            discourse_id: format!("discourse-{}", user_id),
        })
    }
    
    /// Apply filters to notifications
    fn apply_notification_filters(
        &self,
        notifications: Vec<Notification>,
        options: NotificationFilterOptions,
    ) -> Vec<Notification> {
        let mut filtered = notifications;
        
        // Filter by read status
        if let Some(true) = options.unread_only {
            filtered = filtered.into_iter()
                .filter(|n| !n.read)
                .collect();
        }
        
        // Filter by date
        if let Some(since) = options.since {
            filtered = filtered.into_iter()
                .filter(|n| n.created_at >= since)
                .collect();
        }
        
        // Filter by type
        if let Some(types) = options.types {
            filtered = filtered.into_iter()
                .filter(|n| types.contains(&n.notification_type))
                .collect();
        }
        
        // Apply limit
        if let Some(limit) = options.limit {
            filtered.truncate(limit);
        }
        
        filtered
    }
}
