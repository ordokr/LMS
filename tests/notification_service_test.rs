#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;
    use chrono::{DateTime, Utc};
    use serde_json::json;
    
    use crate::models::canvas::notification::Notification;
    use crate::services::notification_service::{NotificationService, NotificationSource, NotificationFilterOptions};
    
    // Mock Canvas API
    mock! {
        CanvasApi {}
        
        impl CanvasApi {
            async fn get_user_notifications(&self, user_id: &str) -> Result<Vec<serde_json::Value>, anyhow::Error>;
            async fn mark_notification_as_read(&self, notification_id: &str) -> Result<serde_json::Value, anyhow::Error>;
            async fn create_notification(&self, notification_data: serde_json::Value) -> Result<serde_json::Value, anyhow::Error>;
        }
    }
    
    // Mock Discourse API
    mock! {
        DiscourseApi {}
        
        impl DiscourseApi {
            async fn get_user_notifications(&self, user_id: &str) -> Result<Vec<serde_json::Value>, anyhow::Error>;
            async fn mark_notification_as_read(&self, notification_id: &str) -> Result<serde_json::Value, anyhow::Error>;
            async fn create_notification(&self, notification_data: serde_json::Value) -> Result<serde_json::Value, anyhow::Error>;
        }
    }
    
    #[tokio::test]
    async fn test_fetch_and_combine_notifications() {
        // Sample data
        let user_id = "123";
        
        // Canvas notifications
        let canvas_notifications = vec![
            json!({
                "id": "c1", 
                "user_id": "123", 
                "subject": "Canvas Note 1", 
                "message": "Hello Canvas", 
                "read": false, 
                "created_at": "2025-04-06T10:00:00Z"
            }),
            json!({
                "id": "c2", 
                "user_id": "123", 
                "subject": "Canvas Note 2", 
                "message": "Hello Again", 
                "read": true, 
                "created_at": "2025-04-05T10:00:00Z"
            })
        ];
        
        // Discourse notifications
        let discourse_notifications = vec![
            json!({
                "id": "d1", 
                "user_id": "123", 
                "data": { 
                    "excerpt": "New reply" 
                }, 
                "read": false, 
                "created_at": "2025-04-07T10:00:00Z"
            }),
            json!({
                "id": "d2", 
                "user_id": "123", 
                "data": { 
                    "excerpt": "Topic mentioned" 
                }, 
                "read": false, 
                "created_at": "2025-04-04T10:00:00Z"
            })
        ];
        
        // Create mocks
        let mut mock_canvas_api = MockCanvasApi::new();
        mock_canvas_api
            .expect_get_user_notifications()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(canvas_notifications.clone()));
            
        let mut mock_discourse_api = MockDiscourseApi::new();
        mock_discourse_api
            .expect_get_user_notifications()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(discourse_notifications.clone()));
        
        // Create service with mocked dependencies
        let service = NotificationService::new(
            Arc::new(mock_canvas_api),
            Arc::new(mock_discourse_api)
        );
        
        // Mock the user mapping method
        service.set_user_mapping_for_test(user_id.to_string(), user_id.to_string(), user_id.to_string());
          // Call the method being tested
        let filter_options = None;
        let notifications = service.get_user_notifications(user_id, filter_options).await.unwrap();
        // Verify results
        assert_eq!(notifications.len(), 4);
        
        // Should be sorted by date (newest first)
        assert_eq!(notifications[0].discourse_id.as_ref().unwrap(), "d1");  // April 7
        assert_eq!(notifications[1].canvas_id.as_ref().unwrap(), "c1");     // April 6
        assert_eq!(notifications[2].canvas_id.as_ref().unwrap(), "c2");     // April 5
        assert_eq!(notifications[3].discourse_id.as_ref().unwrap(), "d2");  // April 4
    }
    
    #[tokio::test]
    async fn test_filter_notifications_by_read_status() {
        // Sample data
        let user_id = "123";
        
        // Canvas notifications
        let canvas_notifications = vec![
            json!({
                "id": "c1", 
                "user_id": "123", 
                "subject": "Canvas Note 1", 
                "message": "Hello Canvas", 
                "read": false, 
                "created_at": "2025-04-06T10:00:00Z"
            }),
            json!({
                "id": "c2", 
                "user_id": "123", 
                "subject": "Canvas Note 2", 
                "message": "Hello Again", 
                "read": true, 
                "created_at": "2025-04-05T10:00:00Z"
            })
        ];
        
        // Discourse notifications
        let discourse_notifications = vec![
            json!({
                "id": "d1", 
                "user_id": "123", 
                "data": { 
                    "excerpt": "New reply" 
                }, 
                "read": false, 
                "created_at": "2025-04-07T10:00:00Z"
            }),
            json!({
                "id": "d2", 
                "user_id": "123", 
                "data": { 
                    "excerpt": "Topic mentioned" 
                }, 
                "read": false, 
                "created_at": "2025-04-04T10:00:00Z"
            })
        ];
        
        // Create mocks
        let mut mock_canvas_api = MockCanvasApi::new();
        mock_canvas_api
            .expect_get_user_notifications()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(canvas_notifications.clone()));
            
        let mut mock_discourse_api = MockDiscourseApi::new();
        mock_discourse_api
            .expect_get_user_notifications()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(discourse_notifications.clone()));
        
        // Create service with mocked dependencies
        let service = NotificationService::new(
            Arc::new(mock_canvas_api),
            Arc::new(mock_discourse_api)
        );
        
        // Mock the user mapping method
        service.set_user_mapping_for_test(user_id.to_string(), user_id.to_string(), user_id.to_string());
        
        // Call the method being tested with read:false filter
        let unread_notifications = service.get_user_notifications(user_id, Some(false)).await.unwrap();
        
        // Verify results
        assert_eq!(unread_notifications.len(), 3);
        for notification in unread_notifications {
            assert_eq!(notification.read, false);
        }
    }
    
    #[tokio::test]
    async fn test_mark_notification_as_read_in_canvas() {
        // Sample data
        let notification_id = "c1";
        
        // Create mock response
        let updated_notification = json!({
            "id": "c1", 
            "user_id": "123", 
            "subject": "Canvas Note 1", 
            "message": "Hello Canvas", 
            "read": true, 
            "created_at": "2025-04-06T10:00:00Z"
        });
        
        // Create mocks
        let mut mock_canvas_api = MockCanvasApi::new();
        mock_canvas_api
            .expect_mark_notification_as_read()
            .with(eq(notification_id))
            .times(1)
            .returning(move |_| Ok(updated_notification.clone()));
            
        let mock_discourse_api = MockDiscourseApi::new();
        
        // Create service with mocked dependencies
        let service = NotificationService::new(
            Arc::new(mock_canvas_api),
            Arc::new(mock_discourse_api)
        );
        
        // Call the method being tested
        let result = service.mark_as_read(notification_id, "canvas").await.unwrap();
        
        // Verify results
        assert_eq!(result.read, true);
    }
    
    #[tokio::test]
    async fn test_mark_notification_as_read_in_discourse() {
        // Sample data
        let notification_id = "d1";
        
        // Create mock response
        let updated_notification = json!({
            "id": "d1", 
            "user_id": "123", 
            "data": { 
                "excerpt": "New reply" 
            }, 
            "read": true, 
            "created_at": "2025-04-07T10:00:00Z"
        });
        
        // Create mocks
        let mock_canvas_api = MockCanvasApi::new();
            
        let mut mock_discourse_api = MockDiscourseApi::new();
        mock_discourse_api
            .expect_mark_notification_as_read()
            .with(eq(notification_id))
            .times(1)
            .returning(move |_| Ok(updated_notification.clone()));
        
        // Create service with mocked dependencies
        let service = NotificationService::new(
            Arc::new(mock_canvas_api),
            Arc::new(mock_discourse_api)
        );
        
        // Call the method being tested
        let result = service.mark_as_read(notification_id, "discourse").await.unwrap();
        
        // Verify results
        assert_eq!(result.read, true);
    }
    
    #[tokio::test]
    async fn test_create_notification_in_both_systems() {
        // Sample data
        let notification_data = json!({
            "userId": "123",
            "subject": "Test Notification",
            "message": "This is a test notification"
        });
        
        // Create mock responses
        let canvas_response = json!({ 
            "id": "cn1", 
            "userId": "123",
            "subject": "Test Notification",
            "message": "This is a test notification"
        });
        
        let discourse_response = json!({ 
            "id": "dn1", 
            "userId": "123",
            "subject": "Test Notification",
            "message": "This is a test notification"
        });
        
        // Create mocks
        let mut mock_canvas_api = MockCanvasApi::new();
        mock_canvas_api
            .expect_create_notification()
            .times(1)
            .returning(move |_| Ok(canvas_response.clone()));
            
        let mut mock_discourse_api = MockDiscourseApi::new();
        mock_discourse_api
            .expect_create_notification()
            .times(1)
            .returning(move |_| Ok(discourse_response.clone()));
        
        // Create service with mocked dependencies
        let service = NotificationService::new(
            Arc::new(mock_canvas_api),
            Arc::new(mock_discourse_api)
        );
        
        // Call the method being tested
        let result = service.create_notification(&notification_data).await.unwrap();
        
        // Verify result contains expected IDs
        assert!(result.canvas_id.is_some());
        assert!(result.discourse_id.is_some());
        assert_eq!(result.canvas_id.unwrap(), "cn1");
        assert_eq!(result.discourse_id.unwrap(), "dn1");
    }
}
