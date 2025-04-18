use sqlx::{Pool, Sqlite, SqlitePool};
use uuid::Uuid;

// Fixed unresolved imports
use crate::shared::models::{forum::{ForumCategory, ForumPost, ForumTopic}, Category, Topic, Post};
use crate::api::forum::AppError;

// Added instructions for `sqlx` query macros
// Ensure `DATABASE_URL` is set or run `cargo sqlx prepare` to update the query cache.

// Repository for forum categories
pub struct ForumCategoryRepository {
    db: Pool<Sqlite>,
}

impl ForumCategoryRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
    
    pub async fn create_category(
        &self,
        name: &str,
        description: Option<&str>,
        course_id: Option<i64>,
        parent_id: Option<i64>,
        color: Option<&str>,
    ) -> Result<i64, AppError> {
        let slug = create_slug(name);
        
        let result = sqlx::query!(
            r#"
            INSERT INTO forum_categories (name, slug, description, course_id, parent_id, color)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            name,
            slug,
            description,
            course_id,
            parent_id,
            color
        )
        .fetch_one(&self.db)
        .await?;
        
        Ok(result.id)
    }
    
    pub async fn get_all_categories(&self) -> Result<Vec<ForumCategory>, AppError> {
        let categories = sqlx::query_as!(
            ForumCategory,
            r#"
            SELECT 
                id, name, slug, description, course_id, parent_id, 
                color, text_color, created_at, updated_at
            FROM forum_categories
            ORDER BY name
            "#
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(categories)
    }
    
    pub async fn get_categories_by_course(&self, course_id: i64) -> Result<Vec<ForumCategory>, AppError> {
        let categories = sqlx::query_as!(
            ForumCategory,
            r#"
            SELECT 
                id, name, slug, description, course_id, parent_id, 
                color, text_color, created_at, updated_at
            FROM forum_categories
            WHERE course_id = ?
            ORDER BY name
            "#,
            course_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(categories)
    }
}

// Repository for forum topics and posts
pub struct ForumTopicRepository {
    db: Pool<Sqlite>,
}

impl ForumTopicRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
    
    pub async fn create_topic(
        &self,
        category_id: i64,
        title: &str,
        user_id: i64,
    ) -> Result<i64, AppError> {
        let slug = create_slug(title);
        let now = chrono::Utc::now().to_rfc3339();
        
        let result = sqlx::query!(
            r#"
            INSERT INTO forum_topics 
                (category_id, title, slug, user_id, last_post_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id
            "#,
            category_id,
            title,
            slug,
            user_id,
            now
        )
        .fetch_one(&self.db)
        .await?;
        
        Ok(result.id)
    }
    
    pub async fn create_post(
        &self,
        topic_id: i64,
        user_id: i64,
        content: &str,
    ) -> Result<i64, AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO forum_posts 
                (topic_id, user_id, content)
            VALUES (?, ?, ?)
            RETURNING id
            "#,
            topic_id,
            user_id,
            content
        )
        .fetch_one(&self.db)
        .await?;
        
        // Update the last_post_at timestamp for the topic
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query!(
            "UPDATE forum_topics SET last_post_at = ? WHERE id = ?",
            now,
            topic_id
        )
        .execute(&self.db)
        .await?;
        
        Ok(result.id)
    }
    
    pub async fn get_topics_by_category(&self, category_id: i64) -> Result<Vec<ForumTopic>, AppError> {
        let topics = sqlx::query_as!(
            ForumTopic,
            r#"
            SELECT 
                id, category_id, title, slug, user_id, pinned, locked,
                created_at, updated_at, last_post_at, view_count
            FROM forum_topics
            WHERE category_id = ?
            ORDER BY pinned DESC, last_post_at DESC
            "#,
            category_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_topic_by_id(&self, topic_id: i64) -> Result<ForumTopic, AppError> {
        let topic = sqlx::query_as!(
            ForumTopic,
            r#"
            SELECT 
                id, category_id, title, slug, user_id, pinned, locked,
                created_at, updated_at, last_post_at, view_count
            FROM forum_topics
            WHERE id = ?
            "#,
            topic_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Topic with id {} not found", topic_id)))?;
        
        // Increment view count
        sqlx::query!(
            "UPDATE forum_topics SET view_count = view_count + 1 WHERE id = ?",
            topic_id
        )
        .execute(&self.db)
        .await?;
        
        Ok(topic)
    }
    
    pub async fn get_posts_by_topic_id(&self, topic_id: i64) -> Result<Vec<ForumPost>, AppError> {
        let posts = sqlx::query_as!(
            ForumPost,
            r#"
            SELECT 
                id, topic_id, user_id, content, is_solution, parent_id,
                created_at, updated_at
            FROM forum_posts
            WHERE topic_id = ?
            ORDER BY created_at
            "#,
            topic_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(posts)
    }
}

// Helper function to create URL-friendly slugs
fn create_slug(text: &str) -> String {
    let slug = text
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .replace(' ', "-");
    
    if slug.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        slug
    }
}

// Forum Post Repository
pub struct ForumPostRepository {
    db: SqlitePool
}

impl ForumPostRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
    
    pub async fn get_by_topic_id(&self, topic_id: i64) -> Result<Vec<Post>, AppError> {
        let posts = sqlx::query_as!(
            Post,
            "SELECT * FROM posts 
             WHERE topic_id = ? AND deleted_at IS NULL
             ORDER BY created_at",
            topic_id
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;
        
        Ok(posts)
    }
    
    pub async fn create(&self, content: String, topic_id: i64, parent_id: Option<i64>) -> Result<Post, AppError> {
        // Verify topic exists
        sqlx::query!("SELECT id FROM topics WHERE id = ? AND deleted_at IS NULL", topic_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .ok_or_else(|| AppError::BadRequest(format!("Topic with id {} not found", topic_id)))?;
        
        // If parent_id is provided, verify parent post exists
        if let Some(parent) = parent_id {
            sqlx::query!("SELECT id FROM posts WHERE id = ? AND deleted_at IS NULL", parent)
                .fetch_optional(&self.db)
                .await
                .map_err(|e| AppError::InternalError(e.to_string()))?
                .ok_or_else(|| AppError::BadRequest(format!("Parent post with id {} not found", parent)))?;
        }
        
        let post_id = sqlx::query!(
            "INSERT INTO posts (content, topic_id, parent_id, created_at, updated_at)
             VALUES (?, ?, ?, datetime('now'), datetime('now'))
             RETURNING id",
            content,
            topic_id,
            parent_id
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .id;
        
        let post = sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE id = ?",
            post_id
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;
        
        Ok(post)
    }
}