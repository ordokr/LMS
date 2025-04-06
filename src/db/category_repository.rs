use sqlx::PgPool;
use uuid::Uuid;
use crate::models::category::Category;

pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_category(&self, category: &Category) -> Result<Category, sqlx::Error> {
        let created_category = sqlx::query_as!(
            Category,
            r#"
            INSERT INTO categories (
                id, name, slug, description, parent_id, 
                created_at, updated_at, course_id, position
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            category.id,
            category.name,
            category.slug,
            category.description,
            category.parent_id,
            category.created_at,
            category.updated_at,
            category.course_id,
            category.position
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_category)
    }

    pub async fn find_category_by_id(&self, id: &Uuid) -> Result<Option<Category>, sqlx::Error> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT * FROM categories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    pub async fn find_category_by_course_id(&self, course_id: &Uuid) -> Result<Option<Category>, sqlx::Error> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT * FROM categories
            WHERE course_id = $1
            "#,
            course_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(category)
    }

    pub async fn update_category(&self, category: &Category) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE categories
            SET name = $1, slug = $2, description = $3, parent_id = $4,
                updated_at = $5, course_id = $6, position = $7
            WHERE id = $8
            "#,
            category.name,
            category.slug,
            category.description,
            category.parent_id,
            category.updated_at,
            category.course_id,
            category.position,
            category.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT * FROM categories
            ORDER BY position ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    pub async fn list_subcategories(&self, parent_id: &Uuid) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT * FROM categories
            WHERE parent_id = $1
            ORDER BY position ASC
            "#,
            parent_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }
}