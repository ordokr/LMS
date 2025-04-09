use super::common::TestContext;
use crate::controllers::{user_controller, follow_controller};
use crate::models::user::{User, UserSummary};
use crate::models::user::profile::UserProfile;
use crate::models::user::activity::UserActivity;
use crate::models::notifications::{Notification, NotificationSummary, NotificationType};
use sqlx::SqlitePool;
use uuid::Uuid;

#[tokio::test]
async fn test_user_profile_system() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test context
    let ctx = TestContext::new().await?;
    let pool = ctx.db_pool.clone();
    
    // Create test users
    let user1_id = create_test_user(&pool, "testuser1", "Test User 1").await?;
    let user2_id = create_test_user(&pool, "testuser2", "Test User 2").await?;
    let user3_id = create_test_user(&pool, "testuser3", "Test User 3").await?;
    
    // Test: Get user profile
    let (user1, profile1) = user_controller::get_user_profile(
        "testuser1".to_string(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(user1.username, "testuser1");
    assert_eq!(user1.id, user1_id);
    assert_eq!(profile1.followers_count, 0);
    assert_eq!(profile1.following_count, 0);
    
    // Test: Follow a user
    follow_controller::follow_user(
        user2_id.clone(), 
        user1_id.clone(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    // Test: User 3 also follows user 1
    follow_controller::follow_user(
        user3_id.clone(), 
        user1_id.clone(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    // Test: User 1 follows user 3
    follow_controller::follow_user(
        user1_id.clone(), 
        user3_id.clone(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    // Verify follow relationships
    let (user1_after, profile1_after) = user_controller::get_user_profile(
        "testuser1".to_string(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(profile1_after.followers_count, 2); // User 2 and 3 follow User 1
    assert_eq!(profile1_after.following_count, 1); // User 1 follows User 3
    
    // Test: check_follows_user
    let user2_follows_user1 = follow_controller::check_follows_user(
        user2_id.clone(), 
        user1_id.clone(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    assert!(user2_follows_user1);
    
    let user1_follows_user2 = follow_controller::check_follows_user(
        user1_id.clone(), 
        user2_id.clone(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    assert!(!user1_follows_user2);
    
    // Test: Get followers
    let followers = follow_controller::get_followers(
        user1_id.clone(),
        Some(1),
        Some(10),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(followers.len(), 2);
    assert!(followers.iter().any(|f| f.id == user2_id));
    assert!(followers.iter().any(|f| f.id == user3_id));
    
    // Test: Get following
    let following = follow_controller::get_following(
        user1_id.clone(),
        Some(1),
        Some(10),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(following.len(), 1);
    assert_eq!(following[0].id, user3_id);
    
    // Test: Create notification
    let notification = user_controller::create_notification(
        NotificationType::Mentioned,
        user1_id.clone(),
        Some(user2_id.clone()),
        "post-123".to_string(),
        "Post",
        None,
        tauri::State::new(pool.clone())
    ).await?;
    
    // Test: Get notifications
    let notifications = user_controller::get_user_notifications(
        user1_id.clone(),
        Some(1),
        Some(10),
        Some(false),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].notification_type, NotificationType::Mentioned);
    assert_eq!(notifications[0].actor_id, Some(user2_id.clone()));
    
    // Test: Get notification summaries
    let summaries = user_controller::get_user_notification_summaries(
        user1_id.clone(),
        Some(1),
        Some(10),
        Some(false),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].notification_type, NotificationType::Mentioned);
    assert!(summaries[0].title.contains("mentioned"));
    
    // Test: Get unread count
    let unread_count = user_controller::get_unread_notification_count(
        user1_id.clone(),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(unread_count, 1);
    
    // Test: Mark notification as read
    user_controller::mark_notification_read(
        notifications[0].id.clone(),
        tauri::State::new(pool.clone())
    ).await?;
    
    // Verify it's marked as read
    let unread_count_after = user_controller::get_unread_notification_count(
        user1_id.clone(),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert_eq!(unread_count_after, 0);
    
    // Test: Unfollow user
    follow_controller::unfollow_user(
        user2_id.clone(),
        user1_id.clone(),
        tauri::State::new(pool.clone())
    ).await?;
    
    // Verify unfollow
    let user2_follows_user1_after = follow_controller::check_follows_user(
        user2_id.clone(), 
        user1_id.clone(), 
        tauri::State::new(pool.clone())
    ).await?;
    
    assert!(!user2_follows_user1_after);
    
    Ok(())
}

async fn create_test_user(pool: &SqlitePool, username: &str, display_name: &str) -> Result<String, sqlx::Error> {
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO users (id, username, email, display_name, avatar_url, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        user_id,
        username,
        format!("{}@example.com", username),
        display_name,
        None::<String>,
        now,
        now
    )
    .execute(pool)
    .await?;
    
    // Create user profile
    sqlx::query!(
        "INSERT INTO user_profiles (user_id, profile_views, trust_level, is_moderator, is_admin)
         VALUES (?, ?, ?, ?, ?)",
        user_id,
        0,
        0,
        false,
        false
    )
    .execute(pool)
    .await?;
    
    Ok(user_id)
}