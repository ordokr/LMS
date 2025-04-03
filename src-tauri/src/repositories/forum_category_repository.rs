use sqlx::{Pool, Sqlite, Transaction, query, query_as};
use chrono::{DateTime, Utc};

use crate::db::DbError;
use crate::models::forum::Category;

pub struct ForumCategoryRepository {
    pool: Pool<Sqlite>,
}

impl ForumCategoryRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self) -> Result<Vec<Category>, DbError> {
        let categories = query_as!(
            Category,
            r#"
            SELECT 
                id, name, slug, description, parent_id, course_id,
                topic_count, post_count, color, text_color,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM forum_categories
            ORDER BY parent_id NULLS FIRST, position
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(categories)
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Option<Category>, DbError> {
        let category = query_as!(
            Category,
            r#"
            SELECT 
                id, name, slug, description, parent_id, course_id,
                topic_count, post_count, color, text_color,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM forum_categories
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(category)
    }
    
    pub async fn get_by_course_id(&self, course_id: i64) -> Result<Vec<Category>, DbError> {
        let categories = query_as!(
            Category,
            r#"
            SELECT 
                id, name, slug, description, parent_id, course_id,
                topic_count, post_count, color, text_color,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM forum_categories
            WHERE course_id = ?
            ORDER BY position
            "#,
            course_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(categories)
    }
    
    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
        parent_id: Option<i64>,
        course_id: Option<i64>,
        color: Option<&str>,
        text_color: Option<&str>,
    ) -> Result<Category, DbError> {
        let now = Utc::now();
        
        // Get the next position
        let position: i32 = if let Some(parent_id) = parent_id {
            // Get max position within parent
            query!(
                "SELECT COALESCE(MAX(position), 0) + 1 as position FROM forum_categories WHERE parent_id = ?",
                parent_id
            )
            .fetch_one(&self.pool)
            .await?
            .position
        } else {
            // Get max position at root level
            query!(
                "SELECT COALESCE(MAX(position), 0) + 1 as position FROM forum_categories WHERE parent_id IS NULL"
            )
            .fetch_one(&self.pool)
            .await?
            .position
        };
        
        let id = query!(
            r#"
            INSERT INTO forum_categories (
                name, slug, description, parent_id, course_id,
                color, text_color, position, topic_count, post_count,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?)
            RETURNING id
            "#,
            name, slug, description, parent_id, course_id,
            color, text_color, position, now, now
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        self.get_by_id(id)
            .await?
            .ok_or_else(|| DbError::NotFound("Category not found after creation".into()))
    }
    
    pub async fn update(
        &self,
        id: i64,
        name: &str,
        slug: &str,
        description: Option<&str>,
        parent_id: Option<i64>,
        course_id: Option<i64>,
        color: Option<&str>,
        text_color: Option<&str>,
    ) -> Result<Category, DbError> {
        let now = Utc::now();
        
        query!(
            r#"
            UPDATE forum_categories
            SET name = ?, slug = ?, description = ?,
                parent_id = ?, course_id = ?,
                color = ?, text_color = ?, updated_at = ?
            WHERE id = ?
            "#,
            name, slug, description,
            parent_id, course_id,
            color, text_color, now, id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id)
            .await?
            .ok_or_else(|| DbError::NotFound("Category not found after update".into()))
    }
    
    pub async fn delete(&self, id: i64) -> Result<(), DbError> {
        // Delete all topics in this category
        query!(
            "DELETE FROM forum_topics WHERE category_id = ?",
            id
        )
        .execute(&self.pool)
        .await?;
        
        // Delete the category
        query!(
            "DELETE FROM forum_categories WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn increment_counts(&self, id: i64, topics: i32, posts: i32) -> Result<(), DbError> {
        query!(
            r#"
            UPDATE forum_categories
            SET topic_count = topic_count + ?,
                post_count = post_count + ?,
                updated_at = ?
            WHERE id = ?
            "#,
            topics, posts, Utc::now(), id
        )
        .execute(&self.pool)
        .await?;
        
        // Also update parent categories if any
        let category = self.get_by_id(id).await?
            .ok_or_else(|| DbError::NotFound("Category not found".into()))?;
        
        if let Some(parent_id) = category.parent_id {
            self.increment_counts(parent_id, topics, posts).await?;
        }
        
        Ok(())
    }
    
    pub async fn get_updated_since(&self, timestamp: i64) -> Result<Vec<Category>, DbError> {
        let since = DateTime::<Utc>::from_timestamp(timestamp, 0)
            .ok_or_else(|| DbError::InvalidInput("Invalid timestamp".into()))?;
        
        let categories = query_as!(
            Category,
            r#"
            SELECT 
                id, name, slug, description, parent_id, course_id,
                topic_count, post_count, color, text_color,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM forum_categories
            WHERE updated_at > ?
            ORDER BY updated_at DESC
            "#,
            since
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(categories)
    }
}