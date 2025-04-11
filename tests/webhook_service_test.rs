#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;
    use serde_json::{json, Value};
    
    // Import the webhook service
    use crate::services::webhook_service::{self, handle_canvas_webhook, handle_discourse_webhook};
    
    // Create mock for notification service
    mock! {
        NotificationService {}
        
        impl NotificationService {
            fn create_notification(&self, notification_data: Value) -> Result<Value, anyhow::Error>;
        }
    }
    
    #[tokio::test]
    async fn test_canvas_submission_created_webhook() {
        // Set up mock notification service
        let mut mock_notification_service = MockNotificationService::new();
        mock_notification_service
            .expect_create_notification()
            .with(function(|notification_data: &Value| {
                notification_data["notificationType"] == "submission_created"
            }))
            .times(1)
            .returning(|_| Ok(json!({"id": "test-notification"})));
        
        // Override the notification service with our mock
        crate::services::notification_service::set_test_notification_service(Arc::new(mock_notification_service));
        
        // Create test payload
        let payload = json!({
            "event_type": "submission_created",
            "submission": {
                "id": "s123",
                "assignment": {
                    "id": "a123",
                    "name": "Test Assignment"
                }
            },
            "user": { "id": "u123" },
            "course": { "id": "c123" }
        });
        
        // Call the webhook handler
        let result = handle_canvas_webhook(&payload).await.unwrap();
        
        // Verify the result
        assert_eq!(result["status"], "processed");
    }
    
    #[tokio::test]
    async fn test_canvas_discussion_entry_created_webhook() {
        // Set up mock notification service
        let mut mock_notification_service = MockNotificationService::new();
        mock_notification_service
            .expect_create_notification()
            .with(function(|notification_data: &Value| {
                notification_data["notificationType"] == "discussion_entry_created"
            }))
            .times(1)
            .returning(|_| Ok(json!({"id": "test-notification"})));
        
        // Override the notification service with our mock
        crate::services::notification_service::set_test_notification_service(Arc::new(mock_notification_service));
        
        // Create test payload
        let payload = json!({
            "event_type": "discussion_entry_created",
            "discussion_entry": {
                "id": "de123",
                "message": "Test reply"
            },
            "discussion_topic": {
                "id": "dt123",
                "user_id": "u456"
            },
            "user": { "id": "u123" },
            "course": { "id": "c123" }
        });
        
        // Call the webhook handler
        let result = handle_canvas_webhook(&payload).await.unwrap();
        
        // Verify the result
        assert_eq!(result["userId"], "u456");
    }
    
    #[tokio::test]
    async fn test_canvas_ignores_unhandled_events() {
        // Set up mock notification service
        let mock_notification_service = MockNotificationService::new();
        // No expectations set because it shouldn't be called
        
        // Override the notification service with our mock
        crate::services::notification_service::set_test_notification_service(Arc::new(mock_notification_service));
        
        // Create test payload
        let payload = json!({
            "event_type": "unknown_event",
            "data": {}
        });
        
        // Call the webhook handler
        let result = handle_canvas_webhook(&payload).await.unwrap();
        
        // Verify the result
        assert_eq!(result["status"], "ignored");
    }
    
    #[tokio::test]
    async fn test_discourse_post_created_webhook() {
        // Set up mock notification service if needed for this test
        let mock_notification_service = MockNotificationService::new();
        crate::services::notification_service::set_test_notification_service(Arc::new(mock_notification_service));
        
        // Create test payload
        let payload = json!({
            "event_name": "post_created",
            "post": {
                "id": "p123",
                "raw": "Test post content",
                "topic_id": "t123"
            },
            "topic": {
                "id": "t123",
                "title": "Test Topic"
            },
            "user": { "id": "u123" }
        });
        
        // Call the webhook handler
        let result = handle_discourse_webhook(&payload).await.unwrap();
        
        // Verify the result
        assert_eq!(result["status"], "processed");
    }
}
