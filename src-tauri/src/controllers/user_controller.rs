use tauri::{command, State};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::{User, UserSummary};
use crate::models::user::profile::{UserProfile, UserProfileUpdate};
use crate::models::user::activity::UserActivity;
use crate::models::notifications::Notification;
use crate::models::forum::topic::TopicSummary;
use crate::models::forum::post::PostSummary;
use crate::error::Error;

/// Get a user profile by username
#[command]
pub async fn get_user_profile(
    username: String,
    db: State<'_, SqlitePool>
) -> Result<(User, UserProfile), Error> {
    // Get the user
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, username, display_name, email, 
            avatar_url, created_at
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("User not found".into()))?;
    
    // Get the user's profile
    let profile = sqlx::query_as!(
        UserProfile,
        r#"
        SELECT 
            user_id, bio, website, location, title, tag_line,
            profile_views, trust_level, is_moderator, is_admin,
            last_seen_at, created_topics_count, posts_count,
            likes_given, likes_received, featured_topic_id
        FROM user_profiles
        WHERE user_id = ?
        "#,
        user.id
    )
    .fetch_optional(&**db)
    .await?;
    
    // If profile doesn't exist, create default one
    let profile = match profile {
        Some(p) => p,
        None => {
            // Create a default profile
            let default_profile = UserProfile {
                user_id: user.id.clone(),
                bio: None,
                website: None,
                location: None,
                title: None,
                tag_line: None,
                profile_views: 0,
                trust_level: 0,
                is_moderator: false,
                is_admin: false,
                last_seen_at: Some(Utc::now()),
                created_topics_count: 0,
                posts_count: 0,
                likes_given: 0,
                likes_received: 0,
                featured_topic_id: None,
            };
            
            // Insert the default profile
            sqlx::query!(
                r#"
                INSERT INTO user_profiles (
                    user_id, bio, website, location, title, tag_line,
                    profile_views, trust_level, is_moderator, is_admin,
                    last_seen_at, created_topics_count, posts_count,
                    likes_given, likes_received
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                default_profile.user_id,
                default_profile.bio,
                default_profile.website,
                default_profile.location,
                default_profile.title,
                default_profile.tag_line,
                default_profile.profile_views,
                default_profile.trust_level,
                default_profile.is_moderator,
                default_profile.is_admin,
                Utc::now().to_rfc3339(),
                default_profile.created_topics_count,
                default_profile.posts_count,
                default_profile.likes_given,
                default_profile.likes_received
            )
            .execute(&**db)
            .await?;
            
            default_profile
        }
    };
    
    // Increment profile views
    sqlx::query!(
        "UPDATE user_profiles SET profile_views = profile_views + 1 WHERE user_id = ?",
        user.id
    )
    .execute(&**db)
    .await?;
    
    Ok((user, profile))
}

/// Update a user profile
#[command]
pub async fn update_user_profile(
    user_id: String,
    profile_update: UserProfileUpdate,
    db: State<'_, SqlitePool>
) -> Result<UserProfile, Error> {
    // Check if profile exists
    let profile_exists = sqlx::query!(
        "SELECT user_id FROM user_profiles WHERE user_id = ?",
        user_id
    )
    .fetch_optional(&**db)
    .await?
    .is_some();
    
    if !profile_exists {
        // Create default profile first
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (
                user_id, bio, website, location, title, tag_line,
                profile_views, trust_level, is_moderator, is_admin,
                last_seen_at, created_topics_count, posts_count,
                likes_given, likes_received
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            user_id,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<String>,
            0,
            0,
            false,
            false,
            Utc::now().to_rfc3339(),
            0,
            0,
            0,
            0
        )
        .execute(&**db)
        .await?;
    }
    
    // Update the profile
    sqlx::query!(
        r#"
        UPDATE user_profiles SET
            bio = COALESCE(?, bio),
            website = COALESCE(?, website),
            location = COALESCE(?, location),
            title = COALESCE(?, title),
            tag_line = COALESCE(?, tag_line)
        WHERE user_id = ?
        "#,
        profile_update.bio,
        profile_update.website,
        profile_update.location,
        profile_update.title,
        profile_update.tag_line,
        user_id
    )
    .execute(&**db)
    .await?;
    
    // Return the updated profile
    let updated_profile = sqlx::query_as!(
        UserProfile,
        r#"
        SELECT 
            user_id, bio, website, location, title, tag_line,
            profile_views, trust_level, is_moderator, is_admin,
            last_seen_at, created_topics_count, posts_count,
            likes_given, likes_received, featured_topic_id
        FROM user_profiles
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_one(&**db)
    .await?;
    
    Ok(updated_profile)
}

/// Get a user's activity feed
#[command]
pub async fn get_user_activities(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<UserActivity>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let activities = sqlx::query_as!(
        UserActivity,
        r#"
        SELECT 
            id, user_id, activity_type as "activity_type: _", 
            target_id, target_type as "target_type: _", 
            data, created_at
        FROM user_activities
        WHERE user_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
    Ok(activities)
}

/// Record a user activity
#[command]
pub async fn record_user_activity(
    activity: UserActivity,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_activities (
            id, user_id, activity_type, target_id, 
            target_type, data, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        activity.id,
        activity.user_id,
        activity.activity_type as _,
        activity.target_id,
        activity.target_type as _,
        activity.data,
        activity.created_at.to_rfc3339()
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}

/// Get user's topics (created by the user)
#[command]
pub async fn get_user_topics(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<TopicSummary>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let topics = sqlx::query_as!(
        TopicSummary,
        r#"
        SELECT 
            t.id, t.title, t.slug, t.category_id, t.user_id, 
            t.closed, t.pinned, t.visible, t.created_at, 
            t.posts_count, t.views, t.last_posted_at, t.excerpt,
            u.display_name as user_display_name,
            c.name as category_name
        FROM topics t
        JOIN users u ON t.user_id = u.id
        JOIN categories c ON t.category_id = c.id
        WHERE t.user_id = ? AND t.deleted_at IS NULL
        ORDER BY t.created_at DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
    Ok(topics)
}

/// Get user's posts (created by the user)
#[command]
pub async fn get_user_posts(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<PostSummary>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let posts = sqlx::query_as!(
        PostSummary,
        r#"
        SELECT 
            p.id, p.topic_id, p.user_id, p.post_number, 
            p.raw, p.created_at, p.like_count,
            t.title as topic_title, t.slug as topic_slug,
            u.display_name as user_display_name,
            u.avatar_url as user_avatar_url
        FROM posts p
        JOIN topics t ON p.topic_id = t.id
        JOIN users u ON p.user_id = u.id
        WHERE p.user_id = ? AND p.deleted_at IS NULL AND t.deleted_at IS NULL
        ORDER BY p.created_at DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
    Ok(posts)
}

/// Get user's notifications
#[command]
pub async fn get_user_notifications(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    unread_only: Option<bool>,
    db: State<'_, SqlitePool>
) -> Result<Vec<Notification>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let mut query = "
        SELECT 
            id, notification_type as \"notification_type: _\", 
            user_id, actor_id, target_id, 
            target_type as \"target_type: _\", 
            data, read, created_at
        FROM notifications
        WHERE user_id = ?
    ".to_string();
    
    let mut args = vec![user_id];
    
    if unread_only.unwrap_or(false) {
        query.push_str(" AND read = 0");
    }
    
    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    args.push(per_page.to_string());
    args.push(offset.to_string());
    
    // Build the query with dynamic arguments
    let mut sql_query = sqlx::query_as::<_, Notification>(&query);
    
    for arg in args {
        sql_query = sql_query.bind(arg);
    }
    
    let notifications = sql_query.fetch_all(&**db).await?;
    
    Ok(notifications)
}

/// Mark notification as read
#[command]
pub async fn mark_notification_read(
    notification_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE notifications SET read = 1 WHERE id = ?",
        notification_id
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}

/// Mark all notifications as read
#[command]
pub async fn mark_all_notifications_read(
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE notifications SET read = 1 WHERE user_id = ?",
        user_id
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}

/// Get unread notification count
#[command]
pub async fn get_unread_notification_count(
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<i64, Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM notifications WHERE user_id = ? AND read = 0",
        user_id
    )
    .fetch_one(&**db)
    .await?;
    
    Ok(result.count)
}

/// Create a notification
#[command]
pub async fn create_notification(
    notification: Notification,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO notifications (
            id, notification_type, user_id, actor_id, 
            target_id, target_type, data, read, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        notification.id,
        notification.notification_type as _,
        notification.user_id,
        notification.actor_id,
        notification.target_id,
        notification.target_type as _,
        notification.data,
        notification.read,
        notification.created_at.to_rfc3339()
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}