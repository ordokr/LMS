use sqlx::{Pool, Sqlite};
use crate::models::mapping::CourseCategoryMapping;
use chrono::Utc;

pub struct CourseCategoryRepository {
    pool: Pool<Sqlite>,
}

impl CourseCategoryRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn create(
        &self, 
        course_id: i64, 
        category_id: i64
    ) -> Result<CourseCategoryMapping, sqlx::Error> {
        let mapping = CourseCategoryMapping::new(course_id, category_id);
        
        let id = sqlx::query!(
            r#"
            INSERT INTO course_category_mappings 
                (course_id, category_id, sync_enabled, created_at, updated_at) 
            VALUES 
                (?, ?, ?, ?, ?)
            "#,
            mapping.course_id,
            mapping.category_id,
            mapping.sync_enabled,
            mapping.created_at,
            mapping.updated_at,
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        self.get_by_id(id).await
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<CourseCategoryMapping, sqlx::Error> {
        sqlx::query_as!(
            CourseCategoryMapping,
            r#"
            SELECT * FROM course_category_mappings 
            WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn get_by_course_id(&self, course_id: i64) -> Result<CourseCategoryMapping, sqlx::Error> {
        sqlx::query_as!(
            CourseCategoryMapping,
            r#"
            SELECT * FROM course_category_mappings 
            WHERE course_id = ?
            "#,
            course_id
        )
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn update_sync_time(&self, id: i64) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE course_category_mappings
            SET last_synced_at = ?, updated_at = ?
            WHERE id = ?
            "#,
            now,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}