use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;
use chrono::Utc;
use crate::error::Error;
use crate::models::forum::post::{Post, PostRequest};

pub struct PostRepository {
    pool: SqlitePool,
}

impl PostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create(&self, user_id: &str, request: PostRequest) -> Result<Post, Error> {
        let post_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Get topic to check if closed
        let topic = sqlx::query!(
            "SELECT closed FROM forum_topics WHERE id = ?",
            request.topic_id
        )
        .fetch_optional(&mut tx)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Topic not found: {}", request.topic_id)))?;
        
        if topic.closed {
            return Err(Error::Forbidden("Topic is closed".into()));
        }
        
        // Get next post number
        let next_post_number = sqlx::query_scalar!(
            "SELECT highest_post_number + 1 FROM forum_topics WHERE id = ?",
            request.topic_id
        )
        .fetch_one(&mut tx)
        .await?;
        
        // Insert the post
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO forum_posts (
                id, topic_id, user_id, post_number,
                raw, html, cooked, reply_to_post_id,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
            post_id,
            request.topic_id,
            user_id,
            next_post_number,
            request.raw,
            request.raw, // Simple conversion for now
            request.raw, // Cooked version
            request.reply_to_post_id,
            now.to_rfc3339(),
            now.to_rfc3339()
        )
        .fetch_one(&mut tx)
        .await?;
        
        // Update topic
        sqlx::query!(
            r#"
            UPDATE forum_topics
            SET 
                posts_count = posts_count + 1,
                highest_post_number = ?,
                last_posted_at = ?,
                bumped_at = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            next_post_number,
            now.to_rfc3339(),
            now.to_rfc3339(),
            now.to_rfc3339(),
            request.topic_id
        )
        .execute(&mut tx)
        .await?;
        
        // Update category post count
        sqlx::query!(
            r#"
            UPDATE forum_categories c
            SET 
                post_count = post_count + 1,
                updated_at = ?
            FROM forum_topics t
            WHERE t.id = ? AND t.category_id = c.id
            "#,
            now.to_rfc3339(),
            request.topic_id
        )
        .execute(&mut tx)
        .await?;
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(post)
    }
    
    pub async fn get_by_id(&self, id: &str) -> Result<Post, Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM forum_posts
            WHERE id = ? AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Post not found: {}", id)))?;
        
        // Load user details
        let user = sqlx::query_as!(
            crate::models::user::User,
            "SELECT * FROM users WHERE id = ?",
            post.user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Return post with user
        let mut post = post;
        post.user = Some(user);
        
        Ok(post)
    }
    
    pub async fn list_by_topic(&self, topic_id: &str, page: i64, per_page: i64) -> Result<Vec<Post>, Error> {
        let offset = (page - 1) * per_page;
        
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM forum_posts
            WHERE topic_id = ? AND deleted_at IS NULL
            ORDER BY post_number
            LIMIT ? OFFSET ?
            "#,
            topic_id,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Load user details for each post
        let mut posts_with_users = Vec::new();
        for post in posts {
            let user = sqlx::query_as!(
                crate::models::user::User,
                "SELECT * FROM users WHERE id = ?",
                post.user_id
            )
            .fetch_one(&self.pool)
            .await?;
            
            let mut post = post;
            post.user = Some(user);
            posts_with_users.push(post);
        }
        
        Ok(posts_with_users)
    }
}