use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::integration::{CourseCategory, CourseCategoryCreate, CourseCategoryUpdate};

pub struct CourseCategoryRepository {
    pool: PgPool,
}

impl CourseCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_mapping: CourseCategoryCreate) -> Result<CourseCategory, sqlx::Error> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            INSERT INTO course_categories (
                id, canvas_course_id, discourse_category_id, 
                sync_enabled, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, canvas_course_id, discourse_category_id, sync_enabled, 
                      last_synced_at, created_at, updated_at
            "#,
            id,
            new_mapping.canvas_course_id,
            new_mapping.discourse_category_id,
            new_mapping.sync_enabled,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<CourseCategory>, sqlx::Error> {
        let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            SELECT id, canvas_course_id, discourse_category_id, sync_enabled, 
                   last_synced_at, created_at, updated_at
            FROM course_categories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn find_by_canvas_course_id(&self, canvas_course_id: &str) -> Result<Option<CourseCategory>, sqlx::Error> {
        let course_category = sqlx::query_as!(
            CourseCategory,
            r#"
            SELECT id, canvas_course_id, discourse_category_id, sync_enabled, 
                   last_synced_at, created_at, updated_at
            FROM course_categories
            WHERE canvas_course_id = $1
            "#,
            canvas_course_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course_category)
    }

    pub async fn find_all(&self) -> Result<Vec<CourseCategory>, sqlx::Error> {
        let course_categories = sqlx::query_as!(
            CourseCategory,
            r#"
            SELECT id, canvas_course_id, discourse_category_id, sync_enabled, 
                   last_synced_at, created_at, updated_at
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
                last_synced_at = COALESCE($2, last_synced_at),
                updated_at = $3
            WHERE id = $4
            RETURNING id, canvas_course_id, discourse_category_id, sync_enabled, 
                      last_synced_at, created_at, updated_at
            "#,
            update_data.sync_enabled,
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