use tauri::{command, State};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::UserSummary;
use crate::error::Error;

/// Follow a user
#[command]
pub async fn follow_user(
    follower_id: String,
    following_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    // Check if already following
    let exists = sqlx::query!(
        "SELECT 1 FROM user_follows WHERE follower_id = ? AND following_id = ?",
        follower_id, following_id
    )
    .fetch_optional(&**db)
    .await?
    .is_some();
    
    if exists {
        return Ok(());  // Already following, just return success
    }
    
    // Create follow record
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO user_follows (id, follower_id, following_id, created_at) VALUES (?, ?, ?, ?)",
        id, follower_id, following_id, now
    )
    .execute(&**db)
    .await?;
    
    // Record activity
    let activity_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        "INSERT INTO user_activities (id, user_id, activity_type, target_id, target_type, created_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        activity_id,
        follower_id,
        "UserFollowed",
        following_id,
        "User",
        now
    )
    .execute(&**db)
    .await?;
    
    // Create notification for the followed user
    let notification_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        "INSERT INTO notifications (id, notification_type, user_id, actor_id, target_id, target_type, read, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        notification_id,
        "UserFollowed",
        following_id,  // Send to the followed user
        follower_id,   // Actor is the follower
        follower_id,   // Target is the follower
        "User",
        0,             // Unread
        now
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}

/// Unfollow a user
#[command]
pub async fn unfollow_user(
    follower_id: String,
    following_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM user_follows WHERE follower_id = ? AND following_id = ?",
        follower_id, following_id
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}

/// Get users followed by a user
#[command]
pub async fn get_following(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<UserSummary>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let users = sqlx::query_as!(
        UserSummary,
        r#"
        SELECT 
            u.id,
            u.username,
            u.display_name,
            u.avatar_url
        FROM user_follows f
        JOIN users u ON f.following_id = u.id
        WHERE f.follower_id = ?
        ORDER BY f.created_at DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
    Ok(users)
}

/// Get users following a user
#[command]
pub async fn get_followers(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<UserSummary>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let users = sqlx::query_as!(
        UserSummary,
        r#"
        SELECT 
            u.id,
            u.username,
            u.display_name,
            u.avatar_url
        FROM user_follows f
        JOIN users u ON f.follower_id = u.id
        WHERE f.following_id = ?
        ORDER BY f.created_at DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
    Ok(users)
}

/// Subscribe to a topic
#[command]
pub async fn subscribe_to_topic(
    user_id: String,
    topic_id: String,
    subscription_level: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Check if already subscribed
    let existing = sqlx::query!(
        "SELECT id FROM topic_subscriptions WHERE user_id = ? AND topic_id = ?",
        user_id, topic_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if let Some(row) = existing {
        // Update existing subscription
        sqlx::query!(
            "UPDATE topic_subscriptions SET subscription_level = ?, updated_at = ? WHERE id = ?",
            subscription_level, now, row.id
        )
        .execute(&**db)
        .await?;
    } else {
        // Create new subscription
        sqlx::query!(
            "INSERT INTO topic_subscriptions (id, user_id, topic_id, subscription_level, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            id, user_id, topic_id, subscription_level, now, now
        )
        .execute(&**db)
        .await?;
    }
    
    Ok(())
}

/// Subscribe to a category
#[command]
pub async fn subscribe_to_category(
    user_id: String,
    category_id: String,
    subscription_level: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Check if already subscribed
    let existing = sqlx::query!(
        "SELECT id FROM category_subscriptions WHERE user_id = ? AND category_id = ?",
        user_id, category_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if let Some(row) = existing {
        // Update existing subscription
        sqlx::query!(
            "UPDATE category_subscriptions SET subscription_level = ?, updated_at = ? WHERE id = ?",
            subscription_level, now, row.id
        )
        .execute(&**db)
        .await?;
    } else {
        // Create new subscription
        sqlx::query!(
            "INSERT INTO category_subscriptions (id, user_id, category_id, subscription_level, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            id, user_id, category_id, subscription_level, now, now
        )
        .execute(&**db)
        .await?;
    }
    
    Ok(())
}

/// Get a user's topic subscription status
#[command]
pub async fn get_topic_subscription(
    user_id: String,
    topic_id: String,
    db: State<'_, SqlitePool>
) -> Result<Option<String>, Error> {
    let subscription = sqlx::query!(
        "SELECT subscription_level FROM topic_subscriptions WHERE user_id = ? AND topic_id = ?",
        user_id, topic_id
    )
    .fetch_optional(&**db)
    .await?
    .map(|row| row.subscription_level);
    
    Ok(subscription)
}

/// Check if a user follows another user
#[command]
pub async fn check_follows_user(
    follower_id: String,
    following_id: String,
    db: State<'_, SqlitePool>
) -> Result<bool, Error> {
    let follows = sqlx::query!(
        "SELECT 1 FROM user_follows WHERE follower_id = ? AND following_id = ?",
        follower_id, following_id
    )
    .fetch_optional(&**db)
    .await?
    .is_some();
    
    Ok(follows)
}