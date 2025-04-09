use tauri::{command, State};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::forum::topic::{Topic, TopicRequest, TopicSummary};
use crate::models::forum::post::Post;
use crate::error::Error;
use crate::services::slugify::slugify;

/// Get all topics with optional filtering
#[command]
pub async fn list_topics(
    category_id: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<TopicSummary>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let mut query = "
        SELECT 
            t.id, t.title, t.slug, t.category_id, t.user_id, 
            t.closed, t.pinned, t.visible, t.created_at, 
            t.posts_count, t.views, t.last_posted_at, t.excerpt,
            u.display_name as user_display_name,
            c.name as category_name
        FROM topics t
        JOIN users u ON t.user_id = u.id
        JOIN categories c ON t.category_id = c.id
        WHERE t.deleted_at IS NULL
    ".to_string();
    
    let mut args = Vec::<String>::new();
    
    // Add category filter if provided
    if let Some(cat_id) = category_id {
        query.push_str(" AND t.category_id = ?");
        args.push(cat_id);
    }
    
    // Add order by and pagination
    query.push_str(" ORDER BY t.pinned_globally DESC, t.pinned DESC, t.bumped_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));
    
    // Build the query with dynamic arguments
    let mut sql_query = sqlx::query_as::<_, TopicSummary>(&query);
    
    for arg in args {
        sql_query = sql_query.bind(arg);
    }
    
    let topics = sql_query.fetch_all(&**db).await?;
    
    Ok(topics)
}

/// Get a single topic by id or slug with its posts
#[command]
pub async fn get_topic(
    id_or_slug: String,
    db: State<'_, SqlitePool>
) -> Result<Topic, Error> {
    // First try to get the topic by either ID or slug
    let topic = sqlx::query_as!(
        Topic,
        r#"
        SELECT 
            id, title, slug, category_id, user_id, closed, pinned, 
            pinned_globally, visible, deleted_at, views, posts_count, 
            like_count, created_at, updated_at, bumped_at, 
            last_posted_at, highest_post_number, excerpt,
            NULL as posts,
            NULL as category,
            NULL as user
        FROM topics
        WHERE (id = ? OR slug = ?) AND deleted_at IS NULL
        "#,
        id_or_slug, id_or_slug
    )
    .fetch_optional(&**db)
    .await?;
    
    let mut topic = topic.ok_or_else(|| Error::NotFound("Topic not found".into()))?;
    
    // Increment view count
    sqlx::query!(
        "UPDATE topics SET views = views + 1 WHERE id = ?",
        topic.id
    )
    .execute(&**db)
    .await?;
    
    // Get all posts for this topic
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT 
            id, topic_id, user_id, post_number, 
            raw, cooked, reply_to_post_id, deleted_at,
            like_count, created_at, updated_at, 
            NULL as user
        FROM posts
        WHERE topic_id = ? AND deleted_at IS NULL
        ORDER BY post_number ASC
        "#,
        topic.id
    )
    .fetch_all(&**db)
    .await?;
    
    // For each post, get the user data
    let mut posts_with_users = Vec::new();
    for mut post in posts {
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
            post.user = Some(crate::models::user::User {
                id: u.id,
                username: u.username,
                display_name: u.display_name,
                email: u.email,
                avatar_url: u.avatar_url,
                created_at: u.created_at.parse().unwrap(),
                // Other fields would be populated here in a complete implementation
            });
        }
        
        posts_with_users.push(post);
    }
    
    // Get category data
    let category = sqlx::query!(
        r#"
        SELECT id, name, slug, description, parent_id, created_at
        FROM categories
        WHERE id = ?
        "#,
        topic.category_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if let Some(c) = category {
        topic.category = Some(crate::models::forum::category::Category {
            id: c.id,
            name: c.name,
            slug: c.slug,
            description: c.description,
            parent_id: c.parent_id,
            created_at: c.created_at.parse().unwrap(),
            // Other fields would be populated here in a complete implementation
        });
    }
    
    // Get user data for topic creator
    let user = sqlx::query!(
        r#"
        SELECT id, username, display_name, email, avatar_url, created_at
        FROM users
        WHERE id = ?
        "#,
        topic.user_id
    )
    .fetch_optional(&**db)
    .await?;
    
    if let Some(u) = user {
        topic.user = Some(crate::models::user::User {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            email: u.email,
            avatar_url: u.avatar_url,
            created_at: u.created_at.parse().unwrap(),
            // Other fields would be populated here in a complete implementation
        });
    }
    
    // Set the posts in the topic
    topic.posts = Some(posts_with_users);
    
    Ok(topic)
}

/// Create a new topic
#[command]
pub async fn create_topic(
    user_id: String,
    topic_request: TopicRequest,
    db: State<'_, SqlitePool>
) -> Result<Topic, Error> {
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Generate a new UUID for the topic
    let topic_id = Uuid::new_v4().to_string();
    
    // Generate a slug from the title
    let slug = slugify(&topic_request.title);
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Insert the topic
    sqlx::query!(
        r#"
        INSERT INTO topics (
            id, title, slug, category_id, user_id, 
            closed, pinned, pinned_globally, visible, 
            views, posts_count, like_count, 
            created_at, updated_at, bumped_at, 
            highest_post_number, excerpt
        ) VALUES (
            ?, ?, ?, ?, ?, 
            ?, ?, ?, ?, 
            ?, ?, ?, 
            ?, ?, ?, 
            ?, ?
        )
        "#,
        topic_id,
        topic_request.title,
        slug,
        topic_request.category_id,
        user_id,
        false, // closed
        false, // pinned
        false, // pinned_globally
        true,  // visible
        0,     // views
        1,     // posts_count (will have one post initially)
        0,     // like_count
        now_str,
        now_str,
        now_str,
        1,     // highest_post_number
        generate_excerpt(&topic_request.raw)
    )
    .execute(&mut *tx)
    .await?;
    
    // Create the first post
    let post_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO posts (
            id, topic_id, user_id, post_number, 
            raw, cooked, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        post_id,
        topic_id,
        user_id,
        1, // First post has number 1
        topic_request.raw,
        process_markdown(&topic_request.raw), // Process markdown to HTML
        now_str,
        now_str
    )
    .execute(&mut *tx)
    .await?;
    
    // Add tags if provided
    if let Some(tags) = &topic_request.tags {
        for tag in tags {
            // Check if tag exists or create it
            let tag_id = ensure_tag_exists(tag, &mut tx).await?;
            
            // Associate tag with the topic
            sqlx::query!(
                "INSERT INTO topic_tags (topic_id, tag_id) VALUES (?, ?)",
                topic_id,
                tag_id
            )
            .execute(&mut *tx)
            .await?;
        }
    }
    
    // Commit the transaction
    tx.commit().await?;
    
    // Return the newly created topic
    get_topic(topic_id, db).await
}

/// Update an existing topic
#[command]
pub async fn update_topic(
    id: String,
    user_id: String,
    title: Option<String>,
    category_id: Option<String>,
    closed: Option<bool>,
    pinned: Option<bool>,
    tags: Option<Vec<String>>,
    db: State<'_, SqlitePool>
) -> Result<Topic, Error> {
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Get the topic to check permissions and current values
    let topic = sqlx::query!(
        r#"
        SELECT user_id, category_id, title, closed, pinned
        FROM topics
        WHERE id = ? AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::NotFound("Topic not found".into()))?;
    
    // Check if user is the creator of the topic
    // In a real implementation, you would also check for moderator/admin permissions
    if topic.user_id != user_id {
        return Err(Error::Forbidden("You don't have permission to edit this topic".into()));
    }
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Update the fields that were provided
    let new_title = title.unwrap_or(topic.title);
    let new_category_id = category_id.unwrap_or(topic.category_id);
    let new_closed = closed.unwrap_or(topic.closed);
    let new_pinned = pinned.unwrap_or(topic.pinned);
    
    // Generate new slug if title changed
    let slug = if title.is_some() {
        slugify(&new_title)
    } else {
        // Query for current slug
        sqlx::query!(
            "SELECT slug FROM topics WHERE id = ?",
            id
        )
        .fetch_one(&mut *tx)
        .await?
        .slug
    };
    
    // Update the topic
    sqlx::query!(
        r#"
        UPDATE topics SET 
            title = ?,
            slug = ?,
            category_id = ?,
            closed = ?,
            pinned = ?,
            updated_at = ?
        WHERE id = ?
        "#,
        new_title,
        slug,
        new_category_id,
        new_closed,
        new_pinned,
        now_str,
        id
    )
    .execute(&mut *tx)
    .await?;
    
    // Update tags if provided
    if let Some(new_tags) = tags {
        // First remove existing tags
        sqlx::query!(
            "DELETE FROM topic_tags WHERE topic_id = ?",
            id
        )
        .execute(&mut *tx)
        .await?;
        
        // Add new tags
        for tag in new_tags {
            let tag_id = ensure_tag_exists(&tag, &mut tx).await?;
            
            sqlx::query!(
                "INSERT INTO topic_tags (topic_id, tag_id) VALUES (?, ?)",
                id,
                tag_id
            )
            .execute(&mut *tx)
            .await?;
        }
    }
    
    // Commit the transaction
    tx.commit().await?;
    
    // Return the updated topic
    get_topic(id, db).await
}

/// Delete a topic (soft delete)
#[command]
pub async fn delete_topic(
    id: String,
    user_id: String,
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    // Get the topic to check permissions
    let topic = sqlx::query!(
        "SELECT user_id FROM topics WHERE id = ? AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Topic not found".into()))?;
    
    // Check if user is the creator of the topic
    // In a real implementation, you would also check for moderator/admin permissions
    if topic.user_id != user_id {
        return Err(Error::Forbidden("You don't have permission to delete this topic".into()));
    }
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Soft delete the topic
    sqlx::query!(
        "UPDATE topics SET deleted_at = ? WHERE id = ?",
        now_str,
        id
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}

// Helper function to generate an excerpt from content
fn generate_excerpt(content: &str) -> Option<String> {
    let plain_text = content
        .replace("#", "")
        .replace("*", "")
        .replace("_", "")
        .replace("`", "");
        
    let excerpt = plain_text
        .split_whitespace()
        .take(50)
        .collect::<Vec<_>>()
        .join(" ");
        
    if excerpt.is_empty() {
        None
    } else {
        Some(format!("{}...", excerpt))
    }
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

// Helper function to ensure a tag exists and return its ID
async fn ensure_tag_exists(tag_name: &str, tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>) -> Result<String, Error> {
    // Check if tag exists
    let existing_tag = sqlx::query!(
        "SELECT id FROM tags WHERE name = ?",
        tag_name
    )
    .fetch_optional(&mut **tx)
    .await?;
    
    // If tag exists, return its ID
    if let Some(tag) = existing_tag {
        return Ok(tag.id);
    }
    
    // Otherwise, create a new tag
    let tag_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query!(
        "INSERT INTO tags (id, name, created_at) VALUES (?, ?, ?)",
        tag_id,
        tag_name,
        now.to_rfc3339()
    )
    .execute(&mut **tx)
    .await?;
    
    Ok(tag_id)
}