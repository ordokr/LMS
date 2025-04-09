use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;
use chrono::Utc;
use crate::error::Error;
use crate::models::forum::category::{Category, CategoryRequest};
use crate::utils::slugify::slugify;

pub struct CategoryRepository {
    pool: SqlitePool,
}

impl CategoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create(&self, request: CategoryRequest) -> Result<Category, Error> {
        let category_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let slug = slugify(&request.name);
        
        // Check if slug already exists
        let existing = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM forum_categories WHERE slug = ?",
            slug
        )
        .fetch_one(&self.pool)
        .await?;
        
        if existing > 0 {
            return Err(Error::Validation("Category slug already exists".into()));
        }
        
        // If position is not specified, find the next available position
        let position = match request.position {
            Some(pos) => pos,
            None => {
                let max_position = sqlx::query_scalar!(
                    "SELECT COALESCE(MAX(position), 0) FROM forum_categories"
                )
                .fetch_one(&self.pool)
                .await?;
                
                max_position + 1
            }
        };
        
        let category = sqlx::query_as!(
            Category,
            r#"
            INSERT INTO forum_categories (
                id, name, slug, description, color, text_color,
                position, parent_category_id, course_id, module_id,
                created_at, updated_at, topic_count, post_count
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
            category_id,
            request.name,
            slug,
            request.description,
            request.color,
            request.text_color,
            position,
            request.parent_category_id,
            request.course_id,
            request.module_id,
            now.to_rfc3339(),
            now.to_rfc3339(),
            0, // topic_count
            0  // post_count
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(category)
    }
    
    pub async fn get_by_id(&self, id: &str) -> Result<Category, Error> {
        sqlx::query_as!(
            Category,
            "SELECT * FROM forum_categories WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Category not found: {}", id)))
    }
    
    pub async fn list_by_course(&self, course_id: &str) -> Result<Vec<Category>, Error> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT * FROM forum_categories
            WHERE course_id = ?
            ORDER BY position
            "#,
            course_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)
    }
    
    pub async fn list_root_categories(&self) -> Result<Vec<Category>, Error> {
        sqlx::query_as!(
            Category,
            r#"
            SELECT * FROM forum_categories
            WHERE parent_category_id IS NULL
            ORDER BY position
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)
    }
}