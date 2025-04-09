
## 37. Create a Notification Service

This service will help with creating notifications in response to events:

```rust
use crate::models::notification::{NotificationCreate, NotificationType};
use crate::db::notification_repository::NotificationRepository;
use std::sync::Arc;
use tracing::{info, error, instrument};

#[derive(Debug, thiserror::Error)]
pub enum NotificationServiceError {
    #[error("Database error: {0}")]
    DbError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub struct NotificationService {
    notification_repo: Arc<dyn NotificationRepository + Send + Sync>,
}

impl NotificationService {
    pub fn new(notification_repo: Arc<dyn NotificationRepository + Send + Sync>) -> Self {
        Self { notification_repo }
    }
    
    #[instrument(skip(self), err)]
    pub async fn notify_discussion_created(
        &self,
        user_id: &str,
        discussion_title: &str,
        discussion_id: &str,
        course_id: &str
    ) -> Result<(), NotificationServiceError> {
        let notification = NotificationCreate {
            user_id: user_id.to_string(),
            title: format!("New Discussion: {}", discussion_title),
            message: format!("A new discussion '{}' has been created in your course.", discussion_title),
            notification_type: NotificationType::Discussion,
            reference_id: Some(discussion_id.to_string()),
            reference_type: Some("discussion".to_string()),
        };
        
        self.create_notification(notification).await?;
        
        // In a real system, we'd find all users enrolled in the course and notify them
        
        Ok(())
    }
    
    #[instrument(skip(self), err)]
    pub async fn notify_assignment_created(
        &self,
        user_id: &str,
        assignment_title: &str,
        assignment_id: &str,
        course_id: &str
    ) -> Result<(), NotificationServiceError> {
        let notification = NotificationCreate {
            user_id: user_id.to_string(),
            title: format!("New Assignment: {}", assignment_title),
            message: format!("A new assignment '{}' has been posted.", assignment_title),
            notification_type: NotificationType::Assignment,
            reference_id: Some(assignment_id.to_string()),
            reference_type: Some("assignment".to_string()),
        };
        
        self.create_notification(notification).await?;
        
        // In a real system, we'd find all users enrolled in the course and notify them
        
        Ok(())
    }
    
    #[instrument(skip(self), err)]
    pub async fn notify_submission_graded(
        &self,
        user_id: &str,
        assignment_title: &str,
        assignment_id: &str,
        score: f64
    ) -> Result<(), NotificationServiceError> {
        let notification = NotificationCreate {
            user_id: user_id.to_string(),
            title: format!("Assignment Graded: {}", assignment_title),
            message: format!("Your submission for '{}' has been graded. Score: {}", assignment_title, score),
            notification_type: NotificationType::Grade,
            reference_id: Some(assignment_id.to_string()),
            reference_type: Some("assignment".to_string()),
        };
        
        self.create_notification(notification).await?;
        
        Ok(())
    }
    
    #[instrument(skip(self), err)]
    pub async fn notify_discourse_reply(
        &self,
        user_id: &str,
        topic_title: &str,
        topic_id: &str,
        reply_author: &str
    ) -> Result<(), NotificationServiceError> {
        let notification = NotificationCreate {
            user_id: user_id.to_string(),
            title: format!("New Reply in '{}'", topic_title),
            message: format!("{} replied to a topic you're following: '{}'", reply_author, topic_title),
            notification_type: NotificationType::DiscourseReply,
            reference_id: Some(topic_id.to_string()),
            reference_type: Some("discourse_topic".to_string()),
        };
        
        self.create_notification(notification).await?;
        
        Ok(())
    }
    
    #[instrument(skip(self), err)]
    pub async fn create_notification(
        &self,
        notification_create: NotificationCreate
    ) -> Result<(), NotificationServiceError> {
        // Validate the notification
        if notification_create.user_id.is_empty() {
            return Err(NotificationServiceError::ValidationError("User ID is required".to_string()));
        }
        if notification_create.title.is_empty() {
            return Err(NotificationServiceError::ValidationError("Title is required".to_string()));
        }
        if notification_create.message.is_empty() {
            return Err(NotificationServiceError::ValidationError("Message is required".to_string()));
        }
        
        // Create the notification
        self.notification_repo.create_notification(crate::models::notification::Notification {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: notification_create.user_id,
            title: notification_create.title,
            message: notification_create.message,
            notification_type: notification_create.notification_type,
            status: crate::models::notification::NotificationStatus::Unread,
            reference_id: notification_create.reference_id,
            reference_type: notification_create.reference_type,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
        .await
        .map_err(|e| {
            error!("Failed to create notification: {}", e);
            NotificationServiceError::DbError(e.to_string())
        })?;
        
        Ok(())
    }
}