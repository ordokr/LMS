#[cfg(test)]
mod tests {
    use crate::api::auth::register_user;
    use crate::api::notifications::{
        create_notification, get_notifications, get_unread_notification_count,
        mark_notifications_as_read, mark_all_notifications_as_read, delete_notification
    };
    use crate::models::user::{RegisterRequest, UserRole};
    use crate::models::notification::{NotificationCreate, NotificationType, NotificationStatus};
    use crate::db::test_utils::{create_test_db_pool, clean_test_db};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_notification_flow() {
        // Set up test DB
        let pool = create_test_db_pool().await;
        clean_test_db(&pool).await;
        
        // Create repositories
        let user_repo = Arc::new(crate::db::user_repository::SqliteUserRepository::new(pool.clone()));
        let notification_repo = Arc::new(crate::db::notification_repository::SqliteNotificationRepository::new(pool.clone()));
        
        // 1. Register a user
        let register_req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            role: Some(UserRole::Student),
        };
        
        let user = register_user(
            register_req, 
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to register user");
        
        // 2. Create a notification
        let notification_create = NotificationCreate {
            user_id: user.id.clone(),
            title: "Test Notification".to_string(),
            message: "This is a test notification".to_string(),
            notification_type: NotificationType::SystemMessage,
            reference_id: None,
            reference_type: None,
        };
        
        let notification = create_notification(
            notification_create,
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to create notification");
        
        // 3. Create a second notification
        let notification_create2 = NotificationCreate {
            user_id: user.id.clone(),
            title: "Another Notification".to_string(),
            message: "This is another test notification".to_string(),
            notification_type: NotificationType::Assignment,
            reference_id: Some("test-assignment-123".to_string()),
            reference_type: Some("assignment".to_string()),
        };
        
        let notification2 = create_notification(
            notification_create2,
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to create second notification");
        
        // 4. Get unread notification count
        let unread_count = get_unread_notification_count(
            user.id.clone(),
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to get unread count");
        
        assert_eq!(unread_count, 2);
        
        // 5. Get notifications for user
        let notifications = get_notifications(
            user.id.clone(),
            None,
            None,
            None,
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to get notifications");
        
        assert_eq!(notifications.len(), 2);
        assert_eq!(notifications[0].title, "Another Notification");
        assert_eq!(notifications[1].title, "Test Notification");
        
        // 6. Mark one notification as read
        let read_count = mark_notifications_as_read(
            vec![notification.id.clone()],
            user.id.clone(),
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to mark notification as read");
        
        assert_eq!(read_count, 1);
        
        // 7. Get unread count again
        let unread_count = get_unread_notification_count(
            user.id.clone(),
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to get unread count");
        
        assert_eq!(unread_count, 1);
        
        // 8. Get only unread notifications
        let unread_notifications = get_notifications(
            user.id.clone(),
            Some(NotificationStatus::Unread),
            None,
            None,
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to get unread notifications");
        
        assert_eq!(unread_notifications.len(), 1);
        assert_eq!(unread_notifications[0].id, notification2.id);
        
        // 9. Mark all as read
        let read_all_count = mark_all_notifications_as_read(
            user.id.clone(),
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to mark all as read");
        
        assert_eq!(read_all_count, 1);
        
        // 10. Get unread count one more time
        let unread_count = get_unread_notification_count(
            user.id.clone(),
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to get unread count");
        
        assert_eq!(unread_count, 0);
        
        // 11. Delete a notification
        let deleted = delete_notification(
            notification2.id.clone(),
            user.id.clone(),
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to delete notification");
        
        assert!(deleted);
        
        // 12. Get all notifications one final time
        let notifications = get_notifications(
            user.id.clone(),
            None,
            None,
            None,
            tauri::State::new(notification_repo.clone())
        ).await.expect("Failed to get notifications");
        
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].id, notification.id);
    }
}