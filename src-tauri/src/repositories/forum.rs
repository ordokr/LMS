use sqlx::SqlitePool;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// Error handling
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Entity not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(String),
}

// Models - Adjust these according to your actual model definitions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub category_id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTopic {
    pub title: String,
    pub slug: String,
    pub category_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub per_page: i64,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

// Category Repository Implementation
pub struct ForumCategoryRepository {
    pool: Arc<SqlitePool>,
}

impl ForumCategoryRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self, page: i64, per_page: i64) -> Result<Vec<Category>, DbError> {
        let offset = (page - 1) * per_page;
        
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT 
                id, name, slug, description, 
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM categories
            ORDER BY name
            LIMIT ? OFFSET ?
            "#,
            per_page, offset
        )
        .fetch_all(&*self.pool)
        .await?;
        
        Ok(categories)
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Category, DbError> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT 
                id, name, slug, description, 
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM categories
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?
        .ok_or(DbError::NotFound)?;
        
        Ok(category)
    }
    
    pub async fn create(&self, new_category: NewCategory) -> Result<Category, DbError> {
        // Validate slug uniqueness
        let existing = sqlx::query!(
            "SELECT COUNT(*) as count FROM categories WHERE slug = ?",
            new_category.slug
        )
        .fetch_one(&*self.pool)
        .await?;
        
        if existing.count > 0 {
            return Err(DbError::Validation("Category slug must be unique".into()));
        }
        
        let now = Utc::now();
        
        // Use a transaction to ensure atomicity
        let mut tx = self.pool.begin().await?;
        
        let id = sqlx::query!(
            r#"
            INSERT INTO categories (name, slug, description, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            new_category.name,
            new_category.slug,
            new_category.description,
            now,
            now
        )
        .execute(&mut tx)
        .await?
        .last_insert_rowid();
        
        tx.commit().await?;
        
        Ok(Category {
            id,
            name: new_category.name,
            slug: new_category.slug,
            description: new_category.description,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub async fn bulk_insert(&self, categories: &[NewCategory]) -> Result<Vec<i64>, DbError> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();
        let mut ids = Vec::with_capacity(categories.len());
        
        for category in categories {
            // Using prepared statement for better performance
            let id = sqlx::query!(
                r#"
                INSERT INTO categories (name, slug, description, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?)
                "#,
                category.name,
                category.slug,
                category.description,
                now,
                now
            )
            .execute(&mut tx)
            .await?
            .last_insert_rowid();
            
            ids.push(id);
        }
        
        tx.commit().await?;
        Ok(ids)
    }
    
    // Additional methods like update, delete, etc.
}

// Topic Repository Implementation
pub struct ForumTopicRepository {
    pool: Arc<SqlitePool>,
}

impl ForumTopicRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self, page: i64, per_page: i64) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        
        let topics = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                id, title, slug, category_id, user_id,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM topics
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            per_page, offset
        )
        .fetch_all(&*self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_by_category(&self, category_id: i64, page: i64, per_page: i64) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        
        let topics = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                id, title, slug, category_id, user_id,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM topics
            WHERE category_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            category_id, per_page, offset
        )
        .fetch_all(&*self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Topic, DbError> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                id, title, slug, category_id, user_id,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
            FROM topics
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?
        .ok_or(DbError::NotFound)?;
        
        Ok(topic)
    }
    
    pub async fn create(&self, new_topic: NewTopic) -> Result<Topic, DbError> {
        // Validate slug uniqueness
        let existing = sqlx::query!(
            "SELECT COUNT(*) as count FROM topics WHERE slug = ?",
            new_topic.slug
        )
        .fetch_one(&*self.pool)
        .await?;
        
        if existing.count > 0 {
            return Err(DbError::Validation("Topic slug must be unique".into()));
        }
        
        let now = Utc::now();
        
        // Use a transaction to ensure atomicity
        let mut tx = self.pool.begin().await?;
        
        let id = sqlx::query!(
            r#"
            INSERT INTO topics (title, slug, category_id, user_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            new_topic.title,
            new_topic.slug,
            new_topic.category_id,
            new_topic.user_id,
            now,
            now
        )
        .execute(&mut tx)
        .await?
        .last_insert_rowid();
        
        tx.commit().await?;
        
        Ok(Topic {
            id,
            title: new_topic.title,
            slug: new_topic.slug,
            category_id: new_topic.category_id,
            user_id: new_topic.user_id,
            created_at: now,
            updated_at: now,
        })
    }
    
    // Optimized bulk insert for topics
    pub async fn bulk_insert(&self, topics: &[NewTopic]) -> Result<Vec<i64>, DbError> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();
        let mut ids = Vec::with_capacity(topics.len());
        
        for topic in topics {
            // Using prepared statement for better performance
            let id = sqlx::query!(
                r#"
                INSERT INTO topics (title, slug, category_id, user_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                topic.title,
                topic.slug,
                topic.category_id,
                topic.user_id,
                now,
                now
            )
            .execute(&mut tx)
            .await?
            .last_insert_rowid();
            
            ids.push(id);
        }
        
        tx.commit().await?;
        Ok(ids)
    }
    
    // Additional methods like update, delete, etc.
}