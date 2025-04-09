use crate::models::notification::{Notification, NotificationStatus};
use sqlx::{Pool, Sqlite};
use async_trait::async_trait;
use tracing::{info, error, instrument};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Data error: {0}")]
    DataError(String),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn get_notifications_for_user(
        &self,
        user_id: &str,
        status: Option<NotificationStatus>,
        limit: Option<u32>,
        offset: Option<u32>
    ) -> Result<Vec<Notification>, DbError>;
    
    async fn get_unread_count(&self, user_id: &str) -> Result<u32, DbError>;
    async fn create_notification(&self, notification: Notification) -> Result<Notification, DbError>;
    async fn mark_as_read(&self, notification_ids: &[String], user_id: &str) -> Result<u32, DbError>;
    async fn mark_all_as_read(&self, user_id: &str) -> Result<u32, DbError>;
    async fn delete_notification(&self, notification_id: &str, user_id: &str) -> Result<bool, DbError>;
}

pub struct SqliteNotificationRepository {
    pool: Pool<Sqlite>,
}

impl SqliteNotificationRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for SqliteNotificationRepository {
    #[instrument(skip(self), err)]
    async fn get_notifications_for_user(
        &self,
        user_id: &str,
        status: Option<NotificationStatus>,
        limit: Option<u32>,
        offset: Option<u32>
    ) -> Result<Vec<Notification>, DbError> {
        // Base query
        let mut query_string = String::from(
            "SELECT * FROM notifications WHERE user_id = ? "
        );
        
        // Add status filter if provided
        if let Some(status_filter) = &status {
            query_string.push_str("AND status = ? ");
        }
        
        // Add sorting
        query_string.push_str("ORDER BY created_at DESC ");
        
        // Add limit and offset if provided
        if let Some(limit_val) = limit {
            query_string.push_str(&format!("LIMIT {} ", limit_val));
            
            if let Some(offset_val) = offset {
                query_string.push_str(&format!("OFFSET {} ", offset_val));
            }
        }
        
        // Build and execute the query
        let mut query = sqlx::query_as::<_, Notification>(&query_string);
        
        // Bind parameters
        query = query.bind(user_id);
        if let Some(status_filter) = &status {
            query = query.bind(status_filter.to_string());
        }
        
        // Execute query
        let notifications = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch notifications for user {}: {}", user_id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        Ok(notifications)
    }
    
    #[instrument(skip(self), err)]
    async fn get_unread_count(&self, user_id: &str) -> Result<u32, DbError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM notifications WHERE user_id = ? AND status = ?",
            user_id,
            NotificationStatus::Unread.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get unread notification count for user {}: {}", user_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        Ok(result.count as u32)
    }
    
    #[instrument(skip(self), fields(notification_id = %notification.id), err)]
    async fn create_notification(&self, notification: Notification) -> Result<Notification, DbError> {
        sqlx::query(
            "INSERT INTO notifications 
             (id, user_id, title, message, notification_type, status, reference_id, reference_type, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&notification.id)
        .bind(&notification.user_id)
        .bind(&notification.title)
        .bind(&notification.message)
        .bind(notification.notification_type.to_string())
        .bind(notification.status.to_string())
        .bind(&notification.reference_id)
        .bind(&notification.reference_type)
        .bind(&notification.created_at)
        .bind(&notification.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to create notification: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        info!("Created new notification with ID: {}", notification.id);
        
        // Return the created notification
        let created = sqlx::query_as::<_, Notification>("SELECT * FROM notifications WHERE id = ?")
            .bind(&notification.id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to retrieve created notification: {}", e);
                DbError::QueryError(e.to_string())
            })?;
        
        Ok(created)
    }
    
    #[instrument(skip(self), err)]
    async fn mark_as_read(&self, notification_ids: &[String], user_id: &str) -> Result<u32, DbError> {
        if notification_ids.is_empty() {
            return Ok(0);
        }
        
        let now = chrono::Utc::now().to_rfc3339();
        
        // Build placeholders for the IN clause
        let placeholders: Vec<String> = (0..notification_ids.len()).map(|_| "?".to_string()).collect();
        let in_clause = placeholders.join(", ");
        
        let query_string = format!(
            "UPDATE notifications SET status = ?, updated_at = ? WHERE id IN ({}) AND user_id = ?",
            in_clause
        );
        
        // Create a query builder
        let mut query = sqlx::query(&query_string);
        
        // Bind the fixed parameters
        query = query.bind(NotificationStatus::Read.to_string());
        query = query.bind(&now);
        
        // Bind each notification ID
        for id in notification_ids {
            query = query.bind(id);
        }
        
        // Bind the user ID for security check
        query = query.bind(user_id);
        
        // Execute the query
        let result = query
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(
                    "Failed to mark notifications as read for user {}: {}", 
                    user_id, e
                );
                DbError::QueryError(e.to_string())
            })?;
        
        info!(
            "Marked {} notifications as read for user {}",
            result.rows_affected(), user_id
        );
        
        Ok(result.rows_affected() as u32)
    }
    
    #[instrument(skip(self), err)]
    async fn mark_all_as_read(&self, user_id: &str) -> Result<u32, DbError> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let result = sqlx::query(
            "UPDATE notifications SET status = ?, updated_at = ? WHERE user_id = ? AND status = ?"
        )
        .bind(NotificationStatus::Read.to_string())
        .bind(&now)
        .bind(user_id)
        .bind(NotificationStatus::Unread.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(
                "Failed to mark all notifications as read for user {}: {}", 
                user_id, e
            );
            DbError::QueryError(e.to_string())
        })?;
        
        info!(
            "Marked {} notifications as read for user {}",
            result.rows_affected(), user_id
        );
        
        Ok(result.rows_affected() as u32)
    }
    
    #[instrument(skip(self), err)]
    async fn delete_notification(&self, notification_id: &str, user_id: &str) -> Result<bool, DbError> {
        let result = sqlx::query(
            "DELETE FROM notifications WHERE id = ? AND user_id = ?"
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(
                "Failed to delete notification {} for user {}: {}", 
                notification_id, user_id, e
            );
            DbError::QueryError(e.to_string())
        })?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted notification with ID: {}", notification_id);
        } else {
            info!(
                "No notification found to delete with ID: {} for user: {}", 
                notification_id, user_id
            );
        }
        
        Ok(deleted)
    }
}