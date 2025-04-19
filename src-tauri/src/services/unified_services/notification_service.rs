use async_trait::async_trait;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use uuid::Uuid;
use crate::errors::error::{Error, Result};
use super::base_service::{Service, ServiceConfig, BaseService};

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Success notification
    Success,
    
    /// Error notification
    Error,
    
    /// Warning notification
    Warning,
    
    /// Info notification
    Info,
}

impl NotificationType {
    /// Convert a string to a NotificationType
    pub fn from_str(s: &str) -> Self {
        match s {
            "success" => Self::Success,
            "error" => Self::Error,
            "warning" => Self::Warning,
            "info" => Self::Info,
            _ => Self::Info,
        }
    }
    
    /// Convert a NotificationType to a string
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

/// Notification model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    
    /// Notification title
    pub title: String,
    
    /// Notification message
    pub message: String,
    
    /// Notification type
    pub notification_type: NotificationType,
    
    /// Creation timestamp
    pub created_at: chrono::DateTime<Utc>,
    
    /// Whether the notification has been read
    pub read: bool,
    
    /// User ID (optional)
    pub user_id: Option<String>,
    
    /// Entity type (optional)
    pub entity_type: Option<String>,
    
    /// Entity ID (optional)
    pub entity_id: Option<String>,
    
    /// Action URL (optional)
    pub action_url: Option<String>,
    
    /// Action text (optional)
    pub action_text: Option<String>,
}

/// Notification service for managing notifications
#[derive(Debug)]
pub struct NotificationService {
    /// Base service
    base: BaseService,
}

impl NotificationService {
    /// Create a new notification service
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            base: BaseService::new(config),
        }
    }
    
    /// Create a new notification
    pub async fn create_notification(
        &self,
        title: &str,
        message: &str,
        notification_type: NotificationType,
        user_id: Option<&str>,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        action_url: Option<&str>,
        action_text: Option<&str>,
    ) -> Result<Notification> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Generate notification ID
        let id = Uuid::new_v4().to_string();
        
        // Get current timestamp
        let created_at = Utc::now();
        
        // Convert notification type to string
        let type_str = notification_type.to_str();
        
        // Insert notification into database
        sqlx::query(
            r#"
            INSERT INTO notifications (
                id, title, message, notification_type, created_at, read,
                user_id, entity_type, entity_id, action_url, action_text
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.clone())
        .bind(title)
        .bind(message)
        .bind(type_str)
        .bind(created_at)
        .bind(false) // Not read yet
        .bind(user_id)
        .bind(entity_type)
        .bind(entity_id)
        .bind(action_url)
        .bind(action_text)
        .execute(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to create notification: {}", e)))?;
        
        // Return the created notification
        Ok(Notification {
            id,
            title: title.to_string(),
            message: message.to_string(),
            notification_type,
            created_at,
            read: false,
            user_id: user_id.map(|s| s.to_string()),
            entity_type: entity_type.map(|s| s.to_string()),
            entity_id: entity_id.map(|s| s.to_string()),
            action_url: action_url.map(|s| s.to_string()),
            action_text: action_text.map(|s| s.to_string()),
        })
    }
    
    /// Get all notifications
    pub async fn get_notifications(&self, user_id: Option<&str>) -> Result<Vec<Notification>> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Build query based on user ID
        let query = if let Some(user_id) = user_id {
            sqlx::query(
                r#"
                SELECT * FROM notifications
                WHERE user_id = ? OR user_id IS NULL
                ORDER BY created_at DESC
                "#,
            )
            .bind(user_id)
        } else {
            sqlx::query(
                r#"
                SELECT * FROM notifications
                ORDER BY created_at DESC
                "#,
            )
        };
        
        // Execute query
        let rows = query
            .fetch_all(pool)
            .await
            .map_err(|e| Error::database(format!("Failed to get notifications: {}", e)))?;
        
        // Convert rows to notifications
        let notifications = rows
            .iter()
            .map(|row| self.row_to_notification(row))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(notifications)
    }
    
    /// Get unread notifications
    pub async fn get_unread_notifications(&self, user_id: Option<&str>) -> Result<Vec<Notification>> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Build query based on user ID
        let query = if let Some(user_id) = user_id {
            sqlx::query(
                r#"
                SELECT * FROM notifications
                WHERE (user_id = ? OR user_id IS NULL) AND read = 0
                ORDER BY created_at DESC
                "#,
            )
            .bind(user_id)
        } else {
            sqlx::query(
                r#"
                SELECT * FROM notifications
                WHERE read = 0
                ORDER BY created_at DESC
                "#,
            )
        };
        
        // Execute query
        let rows = query
            .fetch_all(pool)
            .await
            .map_err(|e| Error::database(format!("Failed to get unread notifications: {}", e)))?;
        
        // Convert rows to notifications
        let notifications = rows
            .iter()
            .map(|row| self.row_to_notification(row))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(notifications)
    }
    
    /// Get unread notification count
    pub async fn get_unread_count(&self, user_id: Option<&str>) -> Result<u32> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Build query based on user ID
        let query = if let Some(user_id) = user_id {
            sqlx::query(
                r#"
                SELECT COUNT(*) as count FROM notifications
                WHERE (user_id = ? OR user_id IS NULL) AND read = 0
                "#,
            )
            .bind(user_id)
        } else {
            sqlx::query(
                r#"
                SELECT COUNT(*) as count FROM notifications
                WHERE read = 0
                "#,
            )
        };
        
        // Execute query
        let row = query
            .fetch_one(pool)
            .await
            .map_err(|e| Error::database(format!("Failed to get unread notification count: {}", e)))?;
        
        // Get count
        let count: i64 = row.get("count");
        
        Ok(count as u32)
    }
    
    /// Mark notification as read
    pub async fn mark_notification_read(&self, notification_id: &str) -> Result<()> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Update notification
        sqlx::query(
            r#"
            UPDATE notifications
            SET read = 1
            WHERE id = ?
            "#,
        )
        .bind(notification_id)
        .execute(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to mark notification as read: {}", e)))?;
        
        Ok(())
    }
    
    /// Mark all notifications as read
    pub async fn mark_all_notifications_read(&self, user_id: Option<&str>) -> Result<()> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Build query based on user ID
        let query = if let Some(user_id) = user_id {
            sqlx::query(
                r#"
                UPDATE notifications
                SET read = 1
                WHERE user_id = ? OR user_id IS NULL
                "#,
            )
            .bind(user_id)
        } else {
            sqlx::query(
                r#"
                UPDATE notifications
                SET read = 1
                "#,
            )
        };
        
        // Execute query
        query
            .execute(pool)
            .await
            .map_err(|e| Error::database(format!("Failed to mark all notifications as read: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete notification
    pub async fn delete_notification(&self, notification_id: &str) -> Result<()> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Delete notification
        sqlx::query(
            r#"
            DELETE FROM notifications
            WHERE id = ?
            "#,
        )
        .bind(notification_id)
        .execute(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to delete notification: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete all notifications
    pub async fn delete_all_notifications(&self, user_id: Option<&str>) -> Result<()> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Build query based on user ID
        let query = if let Some(user_id) = user_id {
            sqlx::query(
                r#"
                DELETE FROM notifications
                WHERE user_id = ? OR user_id IS NULL
                "#,
            )
            .bind(user_id)
        } else {
            sqlx::query(
                r#"
                DELETE FROM notifications
                "#,
            )
        };
        
        // Execute query
        query
            .execute(pool)
            .await
            .map_err(|e| Error::database(format!("Failed to delete all notifications: {}", e)))?;
        
        Ok(())
    }
    
    /// Create a notification for a sync event
    pub async fn create_sync_notification(
        &self,
        success: bool,
        entity_type: &str,
        entity_id: &str,
        entity_name: &str,
    ) -> Result<Notification> {
        // Create notification title and message
        let (title, message, notification_type) = if success {
            (
                format!("{} Synced", entity_type),
                format!("{} '{}' was successfully synchronized.", entity_type, entity_name),
                NotificationType::Success,
            )
        } else {
            (
                format!("{} Sync Failed", entity_type),
                format!("Failed to synchronize {} '{}'. Please check the sync history for details.", entity_type, entity_name),
                NotificationType::Error,
            )
        };
        
        // Create action URL and text
        let action_url = Some(format!("/integrations"));
        let action_text = Some("View Sync Status".to_string());
        
        // Create notification
        self.create_notification(
            &title,
            &message,
            notification_type,
            None,
            Some(entity_type),
            Some(entity_id),
            action_url.as_deref(),
            action_text.as_deref(),
        )
        .await
    }
    
    /// Create a notification for a conflict
    pub async fn create_conflict_notification(
        &self,
        entity_type: &str,
        entity_id: &str,
        entity_name: &str,
    ) -> Result<Notification> {
        // Create notification title and message
        let title = format!("{} Sync Conflict", entity_type);
        let message = format!(
            "A conflict was detected while synchronizing {} '{}'. Please resolve the conflict manually.",
            entity_type, entity_name
        );
        
        // Create action URL and text
        let action_url = Some(format!("/integrations"));
        let action_text = Some("Resolve Conflict".to_string());
        
        // Create notification
        self.create_notification(
            &title,
            &message,
            NotificationType::Warning,
            None,
            Some(entity_type),
            Some(entity_id),
            action_url.as_deref(),
            action_text.as_deref(),
        )
        .await
    }
    
    /// Helper to convert a database row to a Notification
    fn row_to_notification(&self, row: &SqliteRow) -> Result<Notification> {
        // Extract fields from row
        let id: String = row.get("id");
        let title: String = row.get("title");
        let message: String = row.get("message");
        let type_str: String = row.get("notification_type");
        let created_at: chrono::DateTime<Utc> = row.get("created_at");
        let read: bool = row.get("read");
        let user_id: Option<String> = row.get("user_id");
        let entity_type: Option<String> = row.get("entity_type");
        let entity_id: Option<String> = row.get("entity_id");
        let action_url: Option<String> = row.get("action_url");
        let action_text: Option<String> = row.get("action_text");
        
        // Convert string to NotificationType
        let notification_type = NotificationType::from_str(&type_str);
        
        // Create notification
        Ok(Notification {
            id,
            title,
            message,
            notification_type,
            created_at,
            read,
            user_id,
            entity_type,
            entity_id,
            action_url,
            action_text,
        })
    }
    
    /// Convert Notification to JSON
    pub fn notification_to_json(&self, notification: &Notification) -> JsonValue {
        // Convert notification type to string
        let type_str = notification.notification_type.to_str();
        
        // Create JSON
        serde_json::json!({
            "id": notification.id,
            "title": notification.title,
            "message": notification.message,
            "notification_type": type_str,
            "created_at": notification.created_at.to_rfc3339(),
            "read": notification.read,
            "user_id": notification.user_id,
            "entity_type": notification.entity_type,
            "entity_id": notification.entity_id,
            "action_url": notification.action_url,
            "action_text": notification.action_text,
        })
    }
}

#[async_trait]
impl Service for NotificationService {
    fn name(&self) -> &str {
        self.base.name()
    }
    
    async fn init(&self) -> Result<()> {
        // Initialize the service
        self.base.init().await
    }
    
    async fn shutdown(&self) -> Result<()> {
        // Shutdown the service
        self.base.shutdown().await
    }
    
    async fn health_check(&self) -> Result<bool> {
        // Check if the service is healthy
        self.base.health_check().await
    }
}
