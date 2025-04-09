use sqlx::{Pool, Postgres};
use anyhow::{Result, anyhow};
use chrono::Utc;
use crate::models::mapping::CourseCategoryMapping;

pub struct CourseCategoryRepository {
    pool: Pool<Postgres>,
}

impl CourseCategoryRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn create(
        &self, 
        course_id: i64, 
        category_id: i64
    ) -> Result<CourseCategoryMapping> {
        let now = Utc::now();
        
        let id = sqlx::query!(
            r#"
            INSERT INTO course_category_mappings 
                (course_id, category_id, sync_enabled, sync_topics, sync_users, created_at, updated_at) 
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            course_id,
            category_id,
            true, // sync_enabled
            true, // sync_topics
            true, // sync_users
            now,
            now,
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        self.get_by_id(id).await
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<CourseCategoryMapping> {
        sqlx::query_as!(
            CourseCategoryMapping,
            r#"
            SELECT * FROM course_category_mappings 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get mapping by ID: {}", e))
    }
    
    pub async fn get_by_course_id(&self, course_id: i64) -> Result<CourseCategoryMapping> {
        sqlx::query_as!(
            CourseCategoryMapping,
            r#"
            SELECT * FROM course_category_mappings 
            WHERE course_id = $1
            "#,
            course_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get mapping for course ID {}: {}", course_id, e))
    }

    pub async fn get_by_category_id(&self, category_id: i64) -> Result<CourseCategoryMapping> {
        sqlx::query_as!(
            CourseCategoryMapping,
            r#"
            SELECT * FROM course_category_mappings 
            WHERE category_id = $1
            "#,
            category_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get mapping for category ID {}: {}", category_id, e))
    }
    
    pub async fn update_sync_time(&self, id: i64) -> Result<CourseCategoryMapping> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE course_category_mappings
            SET last_synced_at = $1, updated_at = $2
            WHERE id = $3
            "#,
            now,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id).await
    }
    
    pub async fn list_all(&self) -> Result<Vec<CourseCategoryMapping>> {
        sqlx::query_as!(
            CourseCategoryMapping,
            r#"
            SELECT * FROM course_category_mappings 
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to list all mappings: {}", e))
    }
    
    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM course_category_mappings
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn update(
        &self, 
        id: i64, 
        sync_enabled: bool,
        sync_topics: bool,
        sync_users: bool
    ) -> Result<CourseCategoryMapping> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE course_category_mappings
            SET sync_enabled = $1, sync_topics = $2, sync_users = $3, updated_at = $4
            WHERE id = $5
            "#,
            sync_enabled,
            sync_topics,
            sync_users,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id).await
    }
}