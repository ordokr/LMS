use crate::db::DB;
use crate::error::Error;
use crate::models::notification::{Notification, NotificationType};
use chrono::Utc;
use serde_json::Value as JsonValue;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;

pub struct NotificationService {
    db: DB,
}

impl NotificationService {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
    
    // Create a new notification
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
    ) -> Result<Notification, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        
        // Convert notification type to string
        let type_str = match notification_type {
            NotificationType::Success => "success",
            NotificationType::Error => "error",
            NotificationType::Warning => "warning",
            NotificationType::Info => "info",
        };
        
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
        .bind(id.to_string())
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
        .execute(&self.db)
        .await?;
        
        // Return the created notification
        Ok(Notification {
            id: id.to_string(),
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
    
    // Get all notifications
    pub async fn get_notifications(&self, user_id: Option<&str>) -> Result<Vec<Notification>, Error> {
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
        
        let rows = query.fetch_all(&self.db).await?;
        
        let notifications = rows
            .iter()
            .map(|row| self.row_to_notification(row))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(notifications)
    }
    
    // Get unread notifications
    pub async fn get_unread_notifications(&self, user_id: Option<&str>) -> Result<Vec<Notification>, Error> {
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
        
        let rows = query.fetch_all(&self.db).await?;
        
        let notifications = rows
            .iter()
            .map(|row| self.row_to_notification(row))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(notifications)
    }
    
    // Get unread notification count
    pub async fn get_unread_count(&self, user_id: Option<&str>) -> Result<u32, Error> {
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
        
        let row = query.fetch_one(&self.db).await?;
        let count: i64 = row.get("count");
        
        Ok(count as u32)
    }
    
    // Mark notification as read
    pub async fn mark_notification_read(&self, notification_id: &str) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET read = 1
            WHERE id = ?
            "#,
        )
        .bind(notification_id)
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    // Mark all notifications as read
    pub async fn mark_all_notifications_read(&self, user_id: Option<&str>) -> Result<(), Error> {
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
        
        query.execute(&self.db).await?;
        
        Ok(())
    }
    
    // Delete notification
    pub async fn delete_notification(&self, notification_id: &str) -> Result<(), Error> {
        sqlx::query(
            r#"
            DELETE FROM notifications
            WHERE id = ?
            "#,
        )
        .bind(notification_id)
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    // Delete all notifications
    pub async fn delete_all_notifications(&self, user_id: Option<&str>) -> Result<(), Error> {
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
        
        query.execute(&self.db).await?;
        
        Ok(())
    }
    
    // Create a notification for a sync event
    pub async fn create_sync_notification(
        &self,
        success: bool,
        entity_type: &str,
        entity_id: &str,
        entity_name: &str,
    ) -> Result<Notification, Error> {
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
        
        // Create action URL
        let action_url = Some(format!("/integrations"));
        let action_text = Some("View Sync Status".to_string());
        
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
    
    // Create a notification for a conflict
    pub async fn create_conflict_notification(
        &self,
        entity_type: &str,
        entity_id: &str,
        entity_name: &str,
    ) -> Result<Notification, Error> {
        let title = format!("{} Sync Conflict", entity_type);
        let message = format!(
            "A conflict was detected while synchronizing {} '{}'. Please resolve the conflict manually.",
            entity_type, entity_name
        );
        
        // Create action URL
        let action_url = Some(format!("/integrations"));
        let action_text = Some("Resolve Conflict".to_string());
        
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
    
    // Helper to convert a database row to a Notification
    fn row_to_notification(&self, row: &SqliteRow) -> Result<Notification, Error> {
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
        let notification_type = match type_str.as_str() {
            "success" => NotificationType::Success,
            "error" => NotificationType::Error,
            "warning" => NotificationType::Warning,
            "info" => NotificationType::Info,
            _ => NotificationType::Info,
        };
        
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
    
    // Convert Notification to JSON
    pub fn notification_to_json(&self, notification: &Notification) -> JsonValue {
        // Convert notification type to string
        let type_str = match notification.notification_type {
            NotificationType::Success => "success",
            NotificationType::Error => "error",
            NotificationType::Warning => "warning",
            NotificationType::Info => "info",
        };
        
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
