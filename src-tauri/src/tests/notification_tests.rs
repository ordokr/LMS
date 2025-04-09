use super::common::TestContext;
use crate::controllers::user_controller;
use crate::models::notifications::{NotificationType, Notification, NotificationSummary};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_notification_summaries() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test context
    let ctx = TestContext::new().await?;
    let pool = ctx.db_pool.clone();
    
    // Create test users
    let user1_id = create_test_user(&pool, "notifyuser1").await?;
    let user2_id = create_test_user(&pool, "notifyuser2").await?;
    
    // Create test topic
    let topic_id = create_test_topic(&pool, &user2_id, "Test Topic").await?;
    
    // Create test post
    let post_id = create_test_post(&pool, &user2_id, &topic_id, "Hello @notifyuser1!").await?;
    
    // Create test badge
    let badge_id = create_test_badge(&pool, "First Post").await?;
    
    // Create various notification types
    let notifications = vec![
        // Mentioned notification
        create_notification(&pool, NotificationType::Mentioned, &user1_id, Some(&user2_id), &post_id, "Post").await?,
        
        // Replied notification
        create_notification(&pool, NotificationType::Replied, &user1_id, Some(&user2_id), &post_id, "Post").await?,
        
        // Liked notification
        create_notification(&pool, NotificationType::Liked, &user1_id, Some(&user2_id), &post_id, "Post").await?,
        
        // Badge awarded notification
        create_notification(&pool, NotificationType::BadgeAwarded, &user1_id, None, &badge_id, "Badge").await?,
        
        // Topic updated notification
        create_notification(&pool, NotificationType::TopicUpdated, &user1_id, Some(&user2_id), &topic_id, "Topic").await?,
        
        // Announcement notification
        create_notification_with_data(&pool, NotificationType::Announcement, &user1_id, None, "system", "Announcement", 
            r#"{"message":"Important system announcement!"}"#).await?,
    ];
    
    // Get notification summaries
    let summaries = user_controller::get_user_notification_summaries(
        user1_id.clone(),
        Some(1),
        Some(20),
        Some(false),
        tauri::State::new(pool.clone())
    ).await?;
    
    // Test that we have all notifications
    assert_eq!(summaries.len(), notifications.len());
    
    // Test each notification type has been properly summarized
    for summary in &summaries {
        match summary.notification_type {
            NotificationType::Mentioned => {
                assert!(summary.title.contains("mentioned"));
                assert!(summary.body.contains("Test Topic"));
                assert!(summary.url.is_some());
            },
            NotificationType::Replied => {
                assert!(summary.title.contains("reply"));
                assert!(summary.body.contains("Test Topic"));
                assert!(summary.url.is_some());
            },
            NotificationType::Liked => {
                assert!(summary.title.contains("liked"));
                assert!(summary.body.contains("Test Topic"));
                assert!(summary.url.is_some());
            },
            NotificationType::BadgeAwarded => {
                assert!(summary.title.contains("Badge"));
                assert!(summary.body.contains("First Post"));
                assert_eq!(summary.url, Some("/badges".to_string()));
            },
            NotificationType::TopicUpdated => {
                assert!(summary.title.contains("updated"));
                assert!(summary.body.contains("Test Topic"));
                assert!(summary.url.is_some());
            },
            NotificationType::Announcement => {
                assert!(summary.title.contains("announcement"));
                assert!(summary.body.contains("Important system announcement!"));
                assert!(summary.url.is_none());
            },
            _ => {}
        }
    }
    
    Ok(())
}

async fn create_test_user(pool: &SqlitePool, username: &str) -> Result<String, sqlx::Error> {
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO users (id, username, email, display_name, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        user_id,
        username,
        format!("{}@example.com", username),
        username,
        now,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(user_id)
}

async fn create_test_topic(pool: &SqlitePool, user_id: &str, title: &str) -> Result<String, sqlx::Error> {
    let topic_id = Uuid::new_v4().to_string();
    let category_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Create category first
    sqlx::query!(
        "INSERT INTO categories (id, name, created_at) VALUES (?, ?, ?)",
        category_id,
        "Test Category",
        now
    )
    .execute(pool)
    .await?;
    
    // Create topic
    sqlx::query!(
        "INSERT INTO topics (id, title, user_id, category_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        topic_id,
        title,
        user_id,
        category_id,
        now,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(topic_id)
}

async fn create_test_post(pool: &SqlitePool, user_id: &str, topic_id: &str, content: &str) -> Result<String, sqlx::Error> {
    let post_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO posts (id, topic_id, user_id, raw, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        post_id,
        topic_id,
        user_id,
        content,
        now,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(post_id)
}

async fn create_test_badge(pool: &SqlitePool, name: &str) -> Result<String, sqlx::Error> {
    let badge_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        "INSERT INTO badges (id, name, description) VALUES (?, ?, ?)",
        badge_id,
        name,
        format!("Description of {}", name)
    )
    .execute(pool)
    .await?;
    
    Ok(badge_id)
}

async fn create_notification(
    pool: &SqlitePool, 
    notification_type: NotificationType,
    user_id: &str,
    actor_id: Option<&str>,
    target_id: &str,
    target_type: &str
) -> Result<String, sqlx::Error> {
    let notification_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO notifications (id, notification_type, user_id, actor_id, target_id, target_type, read, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        notification_id,
        notification_type.as_ref(),
        user_id,
        actor_id,
        target_id,
        target_type,
        false,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(notification_id)
}

async fn create_notification_with_data(
    pool: &SqlitePool, 
    notification_type: NotificationType,
    user_id: &str,
    actor_id: Option<&str>,
    target_id: &str,
    target_type: &str,
    data: &str
) -> Result<String, sqlx::Error> {
    let notification_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO notifications (id, notification_type, user_id, actor_id, target_id, target_type, data, read, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        notification_id,
        notification_type.as_ref(),
        user_id,
        actor_id,
        target_id,
        target_type,
        data,
        false,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(notification_id)
}