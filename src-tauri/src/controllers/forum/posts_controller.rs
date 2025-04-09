use tauri::{command, State};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::forum::post::{Post, PostRequest};
use crate::error::Error;

/// Get a single post by ID
#[command]
pub async fn get_post(
    id: String,
    db: State<'_, SqlitePool>
) -> Result<Post, Error> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT 
            id, topic_id, user_id, post_number, 
            raw, cooked, reply_to_post_id, deleted_at,
            like_count, created_at, updated_at, 
            NULL as user
        FROM posts
        WHERE id = ? AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Post not found".into()))?;
    
    // Get user data
    let mut post_with_user = post.clone();
    
    let user = sqlx::query!(
        r#"
        SELECT id, username, display_name, email, avatar_url, created_at
        FROM users
        WHERE id = ?
        "#,
        post.user_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if let Some(u) = user {
        post_with_user.user = Some(crate::models::user::User {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            email: u.email,
            avatar_url: u.avatar_url,
            created_at: u.created_at.parse().unwrap(),
            // Other fields would be populated here in a complete implementation
        });
    }
    
    Ok(post_with_user)
}

/// Create a new post in a topic
#[command]
pub async fn create_post(
    user_id: String,
    post_request: PostRequest,
    db: State<'_, SqlitePool>
) -> Result<Post, Error> {
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Check if the topic exists and is not deleted
    let topic = sqlx::query!(
        "SELECT id, highest_post_number, closed FROM topics WHERE id = ? AND deleted_at IS NULL",
        post_request.topic_id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::NotFound("Topic not found".into()))?;
    
    // Check if the topic is closed
    if topic.closed {
        return Err(Error::BadRequest("Topic is closed".into()));
    }
    
    // If replying to a post, make sure it exists and belongs to the topic
    if let Some(reply_id) = &post_request.reply_to_post_id {
        let reply_post = sqlx::query!(
            "SELECT topic_id FROM posts WHERE id = ? AND deleted_at IS NULL",
            reply_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| Error::NotFound("Reply post not found".into()))?;
        
        if reply_post.topic_id != post_request.topic_id {
            return Err(Error::BadRequest("Reply post does not belong to this topic".into()));
        }
    }
    
    // Generate a new UUID for the post
    let post_id = Uuid::new_v4().to_string();
    
    // Calculate the post number (increment from highest)
    let post_number = topic.highest_post_number + 1;
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Insert the post
    sqlx::query!(
        r#"
        INSERT INTO posts (
            id, topic_id, user_id, post_number, 
            raw, cooked, reply_to_post_id,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        post_id,
        post_request.topic_id,
        user_id,
        post_number,
        post_request.raw,
        process_markdown(&post_request.raw),
        post_request.reply_to_post_id,
        now_str,
        now_str
    )
    .execute(&mut *tx)
    .await?;
    
    // Update the topic's metadata
    sqlx::query!(
        r#"
        UPDATE topics SET 
            posts_count = posts_count + 1,
            highest_post_number = ?,
            updated_at = ?,
            bumped_at = ?,
            last_posted_at = ?
        WHERE id = ?
        "#,
        post_number,
        now_str,
        now_str,
        now_str,
        post_request.topic_id
    )
    .execute(&mut *tx)
    .await?;
    
    // Commit the transaction
    tx.commit().await?;
    
    // Return the newly created post
    get_post(post_id, db).await
}

/// Update an existing post
#[command]
pub async fn update_post(
    id: String,
    user_id: String,
    raw: String,
    db: State<'_, SqlitePool>
) -> Result<Post, Error> {
    // Get the post to check permissions and current values
    let post = sqlx::query!(
        "SELECT user_id, topic_id FROM posts WHERE id = ? AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Post not found".into()))?;
    
    // Check if user is the creator of the post
    // In a real implementation, you would also check for moderator/admin permissions
    if post.user_id != user_id {
        return Err(Error::Forbidden("You don't have permission to edit this post".into()));
    }
    
    // Check if the topic is closed
    let topic = sqlx::query!(
        "SELECT closed FROM topics WHERE id = ? AND deleted_at IS NULL",
        post.topic_id
    )
    .fetch_one(&**db)
    .await?;
    
    if topic.closed {
        return Err(Error::BadRequest("Topic is closed".into()));
    }
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Process the markdown
    let cooked = process_markdown(&raw);
    
    // Update the post
    sqlx::query!(
        r#"
        UPDATE posts SET 
            raw = ?,
            cooked = ?,
            updated_at = ?
        WHERE id = ?
        "#,
        raw,
        cooked,
        now_str,
        id
    )
    .execute(&**db)
    .await?;
    
    // Update the topic's updated_at time
    sqlx::query!(
        "UPDATE topics SET updated_at = ? WHERE id = ?",
        now_str,
        post.topic_id
    )
    .execute(&**db)
    .await?;
    
    // Return the updated post
    get_post(id, db).await
}

/// Delete a post (soft delete)
#[command]
pub async fn delete_post(
    id: String,
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    // Get the post to check permissions
    let post = sqlx::query!(
        "SELECT user_id, topic_id, post_number FROM posts WHERE id = ? AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Post not found".into()))?;
    
    // Check if user is the creator of the post
    // In a real implementation, you would also check for moderator/admin permissions
    if post.user_id != user_id {
        return Err(Error::Forbidden("You don't have permission to delete this post".into()));
    }
    
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Soft delete the post
    sqlx::query!(
        "UPDATE posts SET deleted_at = ? WHERE id = ?",
        now_str,
        id
    )
    .execute(&mut *tx)
    .await?;
    
    // Update the topic's post count
    sqlx::query!(
        "UPDATE topics SET posts_count = posts_count - 1 WHERE id = ?",
        post.topic_id
    )
    .execute(&mut *tx)
    .await?;
    
    // If this was the first post, soft delete the entire topic
    if post.post_number == 1 {
        sqlx::query!(
            "UPDATE topics SET deleted_at = ? WHERE id = ?",
            now_str,
            post.topic_id
        )
        .execute(&mut *tx)
        .await?;
    }
    
    // Commit the transaction
    tx.commit().await?;
    
    Ok(())
}

/// Like a post
#[command]
pub async fn like_post(
    id: String,
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    // Check if the post exists
    let post = sqlx::query!(
        "SELECT id, topic_id FROM posts WHERE id = ? AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Post not found".into()))?;
    
    // Check if the user has already liked this post
    let existing_like = sqlx::query!(
        "SELECT id FROM post_likes WHERE post_id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if existing_like.is_some() {
        return Err(Error::BadRequest("You have already liked this post".into()));
    }
    
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Add the like
    let like_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        "INSERT INTO post_likes (id, post_id, user_id, created_at) VALUES (?, ?, ?, ?)",
        like_id,
        id,
        user_id,
        now_str
    )
    .execute(&mut *tx)
    .await?;
    
    // Update the post's like count
    sqlx::query!(
        "UPDATE posts SET like_count = like_count + 1 WHERE id = ?",
        id
    )
    .execute(&mut *tx)
    .await?;
    
    // Update the topic's like count
    sqlx::query!(
        "UPDATE topics SET like_count = like_count + 1 WHERE id = ?",
        post.topic_id
    )
    .execute(&mut *tx)
    .await?;
    
    // Commit the transaction
    tx.commit().await?;
    
    Ok(())
}

/// Unlike a post
#[command]
pub async fn unlike_post(
    id: String,
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    // Check if the post exists
    let post = sqlx::query!(
        "SELECT id, topic_id FROM posts WHERE id = ? AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Post not found".into()))?;
    
    // Check if the user has liked this post
    let existing_like = sqlx::query!(
        "SELECT id FROM post_likes WHERE post_id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if existing_like.is_none() {
        return Err(Error::BadRequest("You haven't liked this post".into()));
    }
    
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Remove the like
    sqlx::query!(
        "DELETE FROM post_likes WHERE post_id = ? AND user_id = ?",
        id,
        user_id
    )
    .execute(&mut *tx)
    .await?;
    
    // Update the post's like count
    sqlx::query!(
        "UPDATE posts SET like_count = like_count - 1 WHERE id = ?",
        id
    )
    .execute(&mut *tx)
    .await?;
    
    // Update the topic's like count
    sqlx::query!(
        "UPDATE topics SET like_count = like_count - 1 WHERE id = ?",
        post.topic_id
    )
    .execute(&mut *tx)
    .await?;
    
    // Commit the transaction
    tx.commit().await?;
    
    Ok(())
}

// Helper function to process markdown to HTML
fn process_markdown(markdown: &str) -> String {
    // In a real implementation, use a proper markdown processor
    // For this example, we'll just do a simple conversion
    let html = markdown
        .replace("# ", "<h1>")
        .replace("\n# ", "\n<h1>")
        .replace("## ", "<h2>")
        .replace("\n## ", "\n<h2>")
        .replace("### ", "<h3>")
        .replace("\n### ", "\n<h3>")
        .replace("**", "<strong>")
        .replace("*", "<em>")
        .replace("```", "<pre><code>")
        .replace("\n\n", "<br><br>");
    
    html
}