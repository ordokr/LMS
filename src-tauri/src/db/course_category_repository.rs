use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::integration::{CourseCategory, CourseCategoryCreate, CourseCategoryUpdate};
use crate::models::integration::{CourseCategoryMapping, CourseCategoryCreate};
use sqlx::{Pool, Sqlite};
use async_trait::async_trait;
use tracing::{info, error, instrument};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Data error: {0}")]
    DataError(String),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CourseCategoryRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<CourseCategoryMapping>, DbError>;
    async fn find_by_course_and_category(&self, course_id: &str, category_id: &str) -> Result<Option<CourseCategoryMapping>, DbError>;
    async fn find_by_course(&self, course_id: &str) -> Result<Vec<CourseCategoryMapping>, DbError>;
    async fn find_by_category(&self, category_id: &str) -> Result<Vec<CourseCategoryMapping>, DbError>;
    async fn create_mapping(&self, mapping: CourseCategoryMapping) -> Result<(), DbError>;
    async fn update_mapping(&self, mapping: CourseCategoryMapping) -> Result<(), DbError>;
    async fn delete_mapping(&self, id: &str) -> Result<bool, DbError>;
}

pub struct SqliteCourseCategoryRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCourseCategoryRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CourseCategoryRepository for SqliteCourseCategoryRepository {
    #[instrument(skip(self), err)]
    async fn find_by_id(&self, id: &str) -> Result<Option<CourseCategoryMapping>, DbError> {
        sqlx::query_as::<_, CourseCategoryMapping>(
            "SELECT * FROM course_category_mappings WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch mapping by ID {}: {}", id, e);
            DbError::QueryError(e.to_string())
        })
    }
    
    #[instrument(skip(self), err)]
    async fn find_by_course_and_category(&self, course_id: &str, category_id: &str) -> Result<Option<CourseCategoryMapping>, DbError> {
        sqlx::query_as::<_, CourseCategoryMapping>(
            "SELECT * FROM course_category_mappings WHERE course_id = ? AND category_id = ?"
        )
        .bind(course_id)
        .bind(category_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch mapping for course {} and category {}: {}", 
                course_id, category_id, e);
            DbError::QueryError(e.to_string())
        })
    }
    
    #[instrument(skip(self), err)]
    async fn find_by_course(&self, course_id: &str) -> Result<Vec<CourseCategoryMapping>, DbError> {
        sqlx::query_as::<_, CourseCategoryMapping>(
            "SELECT * FROM course_category_mappings WHERE course_id = ?"
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch mappings for course {}: {}", course_id, e);
            DbError::QueryError(e.to_string())
        })
    }
    
    #[instrument(skip(self), err)]
    async fn find_by_category(&self, category_id: &str) -> Result<Vec<CourseCategoryMapping>, DbError> {
        sqlx::query_as::<_, CourseCategoryMapping>(
            "SELECT * FROM course_category_mappings WHERE category_id = ?"
        )
        .bind(category_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch mappings for category {}: {}", category_id, e);
            DbError::QueryError(e.to_string())
        })
    }
    
    #[instrument(skip(self), fields(mapping_id = %mapping.id), err)]
    async fn create_mapping(&self, mapping: CourseCategoryMapping) -> Result<(), DbError> {
        sqlx::query(
            "INSERT INTO course_category_mappings 
             (id, course_id, category_id, sync_topics, sync_assignments, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&mapping.id)
        .bind(&mapping.course_id)
        .bind(&mapping.category_id)
        .bind(mapping.sync_topics)
        .bind(mapping.sync_assignments)
        .bind(&mapping.created_at)
        .bind(&mapping.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to insert mapping: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        info!("Created new mapping with ID: {}", mapping.id);
        Ok(())
    }
    
    #[instrument(skip(self), fields(mapping_id = %mapping.id), err)]
    async fn update_mapping(&self, mapping: CourseCategoryMapping) -> Result<(), DbError> {
        let result = sqlx::query(
            "UPDATE course_category_mappings 
             SET course_id = ?, category_id = ?, sync_topics = ?, sync_assignments = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(&mapping.course_id)
        .bind(&mapping.category_id)
        .bind(mapping.sync_topics)
        .bind(mapping.sync_assignments)
        .bind(&mapping.updated_at)
        .bind(&mapping.id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to update mapping: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        if result.rows_affected() == 0 {
            return Err(DbError::DataError(format!("Mapping with ID {} not found", mapping.id)));
        }
        
        info!("Updated mapping with ID: {}", mapping.id);
        Ok(())
    }
    
    #[instrument(skip(self), err)]
    async fn delete_mapping(&self, id: &str) -> Result<bool, DbError> {
        let result = sqlx::query("DELETE FROM course_category_mappings WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete mapping {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted mapping with ID: {}", id);
        } else {
            info!("No mapping found to delete with ID: {}", id);
        }
        
        Ok(deleted)
    }
}

pub struct CourseCategoryRepository {
    pool: PgPool,
}

impl CourseCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_mapping: CourseCategoryCreate) -> Result<CourseCategory, sqlx::Error> {
        let now = Utc::now();
        let id = Uuid::new_v4();        let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            INSERT INTO course_categories (
                id, canvas_course_id, discourse_category_id, 
                sync_enabled, sync_direction, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, canvas_course_id, discourse_category_id, sync_enabled, 
                      sync_direction as "sync_direction: _", last_synced_at, created_at, updated_at
            "#,
            id,
            new_mapping.canvas_course_id,
            new_mapping.discourse_category_id,
            new_mapping.sync_enabled,
            crate::models::integration::SyncDirection::Bidirectional as _,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<CourseCategory>, sqlx::Error> {        let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            SELECT id, canvas_course_id, discourse_category_id, sync_enabled, 
                   sync_direction as "sync_direction: _", last_synced_at, created_at, updated_at
            FROM course_categories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn find_by_canvas_course_id(&self, canvas_course_id: &str) -> Result<Option<CourseCategory>, sqlx::Error> {        let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            SELECT id, canvas_course_id, discourse_category_id, sync_enabled, 
                   sync_direction as "sync_direction: _", last_synced_at, created_at, updated_at
            FROM course_categories
            WHERE canvas_course_id = $1
            "#,
            canvas_course_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn find_all(&self) -> Result<Vec<CourseCategory>, sqlx::Error> {        let course_categories = sqlx::query_as!(
            CourseCategory,
            r#"
            SELECT id, canvas_course_id, discourse_category_id, sync_enabled, 
                   sync_direction as "sync_direction: _", last_synced_at, created_at, updated_at
            FROM course_categories
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(course_categories)
    }

    pub async fn update(&self, id: Uuid, update_data: CourseCategoryUpdate) -> Result<Option<CourseCategory>, sqlx::Error> {
        let now = Utc::now();
          let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            UPDATE course_categories
            SET sync_enabled = COALESCE($1, sync_enabled),
                sync_direction = COALESCE($2, sync_direction),
                last_synced_at = COALESCE($3, last_synced_at),
                updated_at = $4
            WHERE id = $5
            RETURNING id, canvas_course_id, discourse_category_id, sync_enabled, 
                      sync_direction as "sync_direction: _", last_synced_at, created_at, updated_at
            "#,
            update_data.sync_enabled,
            update_data.sync_direction as Option<_>,
            update_data.last_synced_at,
            now,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM course_categories
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}