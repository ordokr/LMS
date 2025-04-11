use crate::models::canvas::notification::Notification;
use crate::services::notification_service::NotificationService;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::sync::Arc;
use log::{info, error};

/// Error types for the webhook service
#[derive(Error, Debug)]
pub enum WebhookError {
    #[error("Failed to process Canvas webhook: {0}")]
    CanvasWebhookError(String),
    
    #[error("Failed to process Discourse webhook: {0}")]
    DiscourseWebhookError(String),
    
    #[error("Notification error: {0}")]
    NotificationError(String),
    
    #[error("Unknown event type: {0}")]
    UnknownEventType(String),
}

/// Webhook processing result
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookResult {
    pub status: String,
    pub event_type: String,
    pub notification_id: Option<String>,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}

/// Service for handling webhooks between Canvas and Discourse
pub struct WebhookService {
    notification_service: Arc<NotificationService>,
}

impl WebhookService {
    /// Create a new webhook service
    pub fn new(notification_service: Arc<NotificationService>) -> Self {
        WebhookService {
            notification_service,
        }
    }
    
    /// Process incoming webhook from Canvas
    pub async fn handle_canvas_webhook(&self, payload: serde_json::Value) -> Result<WebhookResult, WebhookError> {
        let event_type = payload["event_type"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing event_type".to_string()))?;
        
        // Process based on event type
        let result = match event_type {
            "submission_created" => {
                self.process_submission_webhook(&payload, "created").await?
            },
            "submission_updated" => {
                self.process_submission_webhook(&payload, "updated").await?
            },
            "discussion_entry_created" => {
                self.process_discussion_webhook(&payload, "created").await?
            },
            "course_created" => {
                self.process_course_webhook(&payload, "created").await?
            },
            "user_created" => {
                self.process_user_webhook(&payload, "created").await?
            },
            _ => {
                info!("Unhandled Canvas webhook event: {}", event_type);
                WebhookResult {
                    status: "ignored".to_string(),
                    event_type: event_type.to_string(),
                    notification_id: None,
                    message: None,
                    data: None,
                }
            }
        };
        
        Ok(result)
    }
    
    /// Process incoming webhook from Discourse
    pub async fn handle_discourse_webhook(&self, payload: serde_json::Value) -> Result<WebhookResult, WebhookError> {
        let event_type = payload["event_name"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing event_name".to_string()))?;
        
        // Process based on event type
        let result = match event_type {
            "post_created" => {
                self.process_post_webhook(&payload, "created").await?
            },
            "post_edited" => {
                self.process_post_webhook(&payload, "updated").await?
            },
            "topic_created" => {
                self.process_topic_webhook(&payload, "created").await?
            },
            "user_created" => {
                self.process_user_webhook(&payload, "created").await?
            },
            "category_created" => {
                self.process_category_webhook(&payload, "created").await?
            },
            _ => {
                info!("Unhandled Discourse webhook event: {}", event_type);
                WebhookResult {
                    status: "ignored".to_string(),
                    event_type: event_type.to_string(),
                    notification_id: None,
                    message: None,
                    data: None,
                }
            }
        };
        
        Ok(result)
    }
    
    /// Process submission-related webhooks
    async fn process_submission_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let submission = &payload["submission"];
        let user = &payload["user"];
        let course = &payload["course"];
        
        // Extract required fields
        let user_id = user["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing user.id".to_string()))?;
        
        let assignment_name = submission["assignment"]["name"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing assignment name".to_string()))?;
        
        let assignment_id = submission["assignment"]["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing assignment id".to_string()))?;
        
        let submission_id = submission["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing submission id".to_string()))?;
        
        let course_id = course["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing course id".to_string()))?;
        
        // Create notification data
        let notification_data = serde_json::json!({
            "userId": user_id,
            "subject": format!("Assignment submission {}", action),
            "message": format!("Your submission for {} has been {}", assignment_name, action),
            "contextType": "Assignment",
            "contextId": assignment_id,
            "notificationType": format!("submission_{}", action),
            "sourceSystem": "canvas",
            "data": {
                "submissionId": submission_id,
                "assignmentId": assignment_id,
                "courseId": course_id
            }
        });
        
        // Create the notification
        let notification = self.notification_service.create_notification(&notification_data).await
            .map_err(|e| WebhookError::NotificationError(e.to_string()))?;
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("submission_{}", action),
            notification_id: Some(notification.id.clone()),
            message: Some(format!("Processed submission {} event", action)),
            data: Some(serde_json::json!({
                "submissionId": submission_id,
                "notificationId": notification.id
            })),
        })
    }
    
    /// Process discussion-related webhooks
    async fn process_discussion_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let discussion = &payload["discussion_entry"];
        let user = &payload["user"];
        let topic = &payload["discussion_topic"];
        
        // Extract required fields
        let user_id = user["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing user.id".to_string()))?;
        
        let topic_title = topic["title"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing topic title".to_string()))?;
        
        let topic_id = topic["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing topic id".to_string()))?;
        
        let entry_id = discussion["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing discussion entry id".to_string()))?;
        
        // Create notification data
        let notification_data = serde_json::json!({
            "userId": user_id,
            "subject": format!("Discussion reply {}", action),
            "message": format!("A new reply has been {} in discussion: {}", action, topic_title),
            "contextType": "DiscussionTopic",
            "contextId": topic_id,
            "notificationType": format!("discussion_entry_{}", action),
            "sourceSystem": "canvas",
            "data": {
                "entryId": entry_id,
                "topicId": topic_id
            }
        });
        
        // Create the notification
        let notification = self.notification_service.create_notification(&notification_data).await
            .map_err(|e| WebhookError::NotificationError(e.to_string()))?;
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("discussion_entry_{}", action),
            notification_id: Some(notification.id.clone()),
            message: Some(format!("Processed discussion entry {} event", action)),
            data: Some(serde_json::json!({
                "entryId": entry_id,
                "notificationId": notification.id
            })),
        })
    }
    
    /// Process course-related webhooks
    async fn process_course_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let course = &payload["course"];
        
        // Extract required fields
        let course_id = course["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing course.id".to_string()))?;
        
        let course_name = course["name"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing course name".to_string()))?;
        
        // For course creation, we don't have a specific user to notify
        // In a real implementation, we might notify admin users or create system records
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("course_{}", action),
            notification_id: None,
            message: Some(format!("Processed course {} event for: {}", action, course_name)),
            data: Some(serde_json::json!({
                "courseId": course_id,
                "courseName": course_name
            })),
        })
    }
    
    /// Process user-related webhooks
    async fn process_user_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let user = &payload["user"];
        
        // Extract required fields
        let user_id = user["id"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing user.id".to_string()))?;
        
        let user_name = user["name"].as_str()
            .ok_or_else(|| WebhookError::CanvasWebhookError("Missing user name".to_string()))?;
        
        // For user creation, we don't typically create notifications
        // Instead, we might sync the user to the other system
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("user_{}", action),
            notification_id: None,
            message: Some(format!("Processed user {} event for: {}", action, user_name)),
            data: Some(serde_json::json!({
                "userId": user_id,
                "userName": user_name
            })),
        })
    }
    
    /// Process post-related webhooks from Discourse
    async fn process_post_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let post = &payload["post"];
        let topic = &payload["topic"];
        let user = &payload["user"];
        
        // Extract required fields
        let user_id = user["id"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing user.id".to_string()))?;
        
        let topic_title = topic["title"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing topic title".to_string()))?;
        
        let topic_id = topic["id"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing topic id".to_string()))?;
        
        let post_id = post["id"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing post id".to_string()))?;
        
        // Create notification data
        let notification_data = serde_json::json!({
            "userId": user_id,
            "subject": format!("Forum post {}", action),
            "message": format!("A post has been {} in topic: {}", action, topic_title),
            "contextType": "DiscussionTopic",
            "contextId": topic_id,
            "notificationType": format!("post_{}", action),
            "sourceSystem": "discourse",
            "data": {
                "postId": post_id,
                "topicId": topic_id
            }
        });
        
        // Create the notification
        let notification = self.notification_service.create_notification(&notification_data).await
            .map_err(|e| WebhookError::NotificationError(e.to_string()))?;
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("post_{}", action),
            notification_id: Some(notification.id.clone()),
            message: Some(format!("Processed post {} event", action)),
            data: Some(serde_json::json!({
                "postId": post_id,
                "notificationId": notification.id
            })),
        })
    }
    
    /// Process topic-related webhooks from Discourse
    async fn process_topic_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let topic = &payload["topic"];
        let user = &payload["user"];
        
        // Extract required fields
        let user_id = user["id"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing user.id".to_string()))?;
        
        let topic_title = topic["title"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing topic title".to_string()))?;
        
        let topic_id = topic["id"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing topic id".to_string()))?;
        
        // Create notification data
        let notification_data = serde_json::json!({
            "userId": user_id,
            "subject": format!("Forum topic {}", action),
            "message": format!("A new topic has been {}: {}", action, topic_title),
            "contextType": "Topic",
            "contextId": topic_id,
            "notificationType": format!("topic_{}", action),
            "sourceSystem": "discourse",
            "data": {
                "topicId": topic_id
            }
        });
        
        // Create the notification
        let notification = self.notification_service.create_notification(&notification_data).await
            .map_err(|e| WebhookError::NotificationError(e.to_string()))?;
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("topic_{}", action),
            notification_id: Some(notification.id.clone()),
            message: Some(format!("Processed topic {} event", action)),
            data: Some(serde_json::json!({
                "topicId": topic_id,
                "notificationId": notification.id
            })),
        })
    }
    
    /// Process category-related webhooks from Discourse
    async fn process_category_webhook(&self, payload: &serde_json::Value, action: &str) -> Result<WebhookResult, WebhookError> {
        let category = &payload["category"];
        
        // Extract required fields
        let category_id = category["id"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing category.id".to_string()))?;
        
        let category_name = category["name"].as_str()
            .ok_or_else(|| WebhookError::DiscourseWebhookError("Missing category name".to_string()))?;
        
        // For category creation, we don't have a specific user to notify
        // In a real implementation, we might notify admin users or create system records
        
        Ok(WebhookResult {
            status: "processed".to_string(),
            event_type: format!("category_{}", action),
            notification_id: None,
            message: Some(format!("Processed category {} event for: {}", action, category_name)),
            data: Some(serde_json::json!({
                "categoryId": category_id,
                "categoryName": category_name
            })),
        })
    }
}
