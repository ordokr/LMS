use tauri::{command, State};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::{User, UserSummary};
use crate::models::user::profile::{UserProfile, UserProfileUpdate};
use crate::models::user::activity::UserActivity;
use crate::models::notifications::{Notification, NotificationSummary, NotificationType};
use crate::models::forum::topic::TopicSummary;
use crate::models::forum::post::PostSummary;
use crate::error::Error;

// Update your get_user_profile function to include follower counts

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
            id, username, email, display_name, avatar_url, created_at, updated_at
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("User not found".to_string()))?;
    
    // Get the profile
    let mut profile = sqlx::query_as!(
        UserProfile,
        r#"
        SELECT 
            user_id, bio, website, location, title, tag_line, profile_views,
            trust_level, is_moderator, is_admin, last_seen_at,
            created_topics_count, posts_count, likes_given, likes_received,
            featured_topic_id
        FROM user_profiles
        WHERE user_id = ?
        "#,
        user.id
    )
    .fetch_optional(&**db)
    .await?
    .unwrap_or_else(|| UserProfile {
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
        last_seen_at: None,
        created_topics_count: 0,
        posts_count: 0,
        likes_given: 0,
        likes_received: 0,
        featured_topic_id: None,
        followers_count: 0,
        following_count: 0,
    });
    
    // Get followers count
    profile.followers_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM user_follows WHERE following_id = ?",
        user.id
    )
    .fetch_one(&**db)
    .await?
    .count;
    
    // Get following count
    profile.following_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM user_follows WHERE follower_id = ?",
        user.id
    )
    .fetch_one(&**db)
    .await?
    .count;
    
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
        SELECT id, notification_type, user_id, actor_id, target_id, 
               target_type, data, read, created_at
        FROM notifications
        WHERE user_id = ?
    ".to_string();
    
    if unread_only.unwrap_or(false) {
        query.push_str(" AND read = 0");
    }
    
    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    
    let notifications = sqlx::query_as!(
        Notification,
        &query,
        user_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
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

/// Mark all notifications as read for a user
#[command]
pub async fn mark_all_notifications_read(
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE notifications SET read = 1 WHERE user_id = ? AND read = 0",
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

// Add this function to load notification summaries

/// Get user's notification summaries (more user-friendly format)
#[command]
pub async fn get_user_notification_summaries(
    user_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    unread_only: Option<bool>,
    db: State<'_, SqlitePool>
) -> Result<Vec<NotificationSummary>, Error> {
    // First get the raw notifications
    let notifications = get_user_notifications(
        user_id, page, per_page, unread_only, db.clone()
    ).await?;
    
    // Convert to summaries with additional info
    let mut summaries = Vec::with_capacity(notifications.len());
    
    for notification in notifications {
        let mut title = String::new();
        let mut body = String::new();
        let mut url = None;
        let mut actor_name = None;
        let mut actor_avatar = None;
        
        // Get actor info if present
        if let Some(actor_id) = &notification.actor_id {
            if let Ok(actor) = sqlx::query!(
                "SELECT display_name, avatar_url FROM users WHERE id = ?",
                actor_id
            )
            .fetch_optional(&**db)
            .await? {
                actor_name = Some(actor.display_name);
                actor_avatar = actor.avatar_url;
            }
        }
        
        // Set title, body and URL based on notification type
        match notification.notification_type {
            NotificationType::Mentioned => {
                title = "You were mentioned".into();
                
                if let Ok(post) = sqlx::query!(
                    r#"
                    SELECT t.title as topic_title, p.topic_id
                    FROM posts p
                    JOIN topics t ON p.topic_id = t.id
                    WHERE p.id = ?
                    "#,
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("You were mentioned in {}", post.topic_title);
                    url = Some(format!("/topics/{}?post={}", post.topic_id, notification.target_id));
                }
            },
            NotificationType::Replied => {
                title = "New reply".into();
                
                if let Ok(post) = sqlx::query!(
                    r#"
                    SELECT t.title as topic_title, p.topic_id
                    FROM posts p
                    JOIN topics t ON p.topic_id = t.id
                    WHERE p.id = ?
                    "#,
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("New reply in {}", post.topic_title);
                    url = Some(format!("/topics/{}?post={}", post.topic_id, notification.target_id));
                }
            },
            NotificationType::Liked => {
                title = "Your post was liked".into();
                
                if let Ok(post) = sqlx::query!(
                    r#"
                    SELECT t.title as topic_title, p.topic_id
                    FROM posts p
                    JOIN topics t ON p.topic_id = t.id
                    WHERE p.id = ?
                    "#,
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("Your post in {} was liked", post.topic_title);
                    url = Some(format!("/topics/{}?post={}", post.topic_id, notification.target_id));
                }
            },
            NotificationType::PrivateMessage => {
                title = "New private message".into();
                body = "You received a new private message".into();
                url = Some(format!("/messages/{}", notification.target_id));
            },
            NotificationType::BadgeAwarded => {
                title = "Badge awarded".into();
                
                if let Ok(badge) = sqlx::query!(
                    "SELECT name FROM badges WHERE id = ?",
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("You earned the {} badge", badge.name);
                    url = Some("/badges".into());
                }
            },
            NotificationType::TopicUpdated => {
                title = "Topic updated".into();
                
                if let Ok(topic) = sqlx::query!(
                    "SELECT title FROM topics WHERE id = ?",
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("Topic {} was updated", topic.title);
                    url = Some(format!("/topics/{}", notification.target_id));
                }
            },
            NotificationType::Announcement => {
                title = "New announcement".into();
                body = if let Some(data) = notification.data {
                    if let Ok(parsed) = serde_json::from_value::<serde_json::Value>(data) {
                        parsed.get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or("New system announcement")
                            .to_string()
                    } else {
                        "New system announcement".into()
                    }
                } else {
                    "New system announcement".into()
                };
            },
            NotificationType::AssignmentGraded => {
                title = "Assignment graded".into();
                
                if let Ok(assignment) = sqlx::query!(
                    "SELECT title FROM assignments WHERE id = ?",
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("Your assignment {} has been graded", assignment.title);
                    url = Some(format!("/assignments/{}", notification.target_id));
                }
            },
            NotificationType::DueDateReminder => {
                title = "Due date reminder".into();
                
                if let Ok(assignment) = sqlx::query!(
                    "SELECT title, due_date FROM assignments WHERE id = ?",
                    notification.target_id
                )
                .fetch_optional(&**db)
                .await? {
                    body = format!("Assignment {} is due soon", assignment.title);
                    url = Some(format!("/assignments/{}", notification.target_id));
                }
            },
        }
        
        summaries.push(NotificationSummary {
            id: notification.id,
            notification_type: notification.notification_type,
            title,
            body,
            url,
            actor_name,
            actor_avatar,
            read: notification.read,
            created_at: notification.created_at,
        });
    }
    
    Ok(summaries)
}

// Add this helper function to look up a user ID by username

/// Get user ID by username
#[command]
pub async fn get_user_id_by_username(
    username: String,
    db: State<'_, SqlitePool>
) -> Result<String, Error> {
    let user_id = sqlx::query!(
        "SELECT id FROM users WHERE username = ?",
        username
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("User not found".to_string()))?
    .id;
    
    Ok(user_id)
}