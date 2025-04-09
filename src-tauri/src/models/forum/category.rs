use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::db::DB;
use crate::error::Error;
use crate::models::forum::topic::Topic;
use crate::utils::date_utils::parse_date_string;

/// Category model based on Discourse categories
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub position: i32,
    pub color: String,
    pub text_color: Option<String>,
    pub is_hidden: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Integration-specific fields
    pub discourse_category_id: Option<i64>,
    pub course_id: Option<Uuid>, // If this category is associated with a course
    pub sync_status: SyncStatus,
    
    // Additional properties not stored directly in DB
    #[sqlx(skip)]
    pub topic_count: i32,
    
    #[sqlx(skip)]
    pub subcategories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    LocalOnly,
    SyncedWithDiscourse,
    PendingSync,
    SyncError,
}

impl Category {
    pub fn new(name: String, description: Option<String>) -> Self {
        let slug = name.to_lowercase()
            .replace(" ", "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            
        Category {
            id: Uuid::new_v4(),
            name,
            description,
            slug,
            parent_id: None,
            position: 0,
            color: "#0088CC".to_string(), // Default color
            text_color: Some("#FFFFFF".to_string()), // Default text color
            is_hidden: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            discourse_category_id: None,
            course_id: None,
            sync_status: SyncStatus::LocalOnly,
            topic_count: 0,
            subcategories: Vec::new(),
        }
    }

    /// Validate category data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Name shouldn't be empty
        if self.name.trim().is_empty() {
            errors.push("Category name cannot be empty".to_string());
        }
        
        // Slug shouldn't be empty
        if self.slug.trim().is_empty() {
            errors.push("Category slug cannot be empty".to_string());
        }
        
        // Color should be a valid hex color
        if !self.color.starts_with('#') || (self.color.len() != 7 && self.color.len() != 4) {
            errors.push("Category color should be a valid hex color (e.g., #FF5500)".to_string());
        }
        
        // Text color validation if provided
        if let Some(text_color) = &self.text_color {
            if !text_color.starts_with('#') || (text_color.len() != 7 && text_color.len() != 4) {
                errors.push("Category text color should be a valid hex color (e.g., #FFFFFF)".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let mut category = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load topic count
        category.topic_count = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM topics WHERE category_id = ?"
        )
        .bind(category.id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load subcategories
        category.subcategories = Self::children(&category, db).await?;
        
        Ok(category)
    }
    
    pub async fn find_by_discourse_id(db: &DB, discourse_id: i64) -> Result<Self, Error> {
        let mut category = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE discourse_category_id = ?"
        )
        .bind(discourse_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load topic count
        category.topic_count = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM topics WHERE category_id = ?"
        )
        .bind(category.id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load subcategories
        category.subcategories = Self::children(&category, db).await?;
        
        Ok(category)
    }
    
    pub async fn find_by_slug(db: &DB, slug: &str) -> Result<Self, Error> {
        let mut category = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE slug = ?"
        )
        .bind(slug)
        .fetch_one(&db.pool)
        .await?;
        
        // Load topic count
        category.topic_count = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM topics WHERE category_id = ?"
        )
        .bind(category.id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load subcategories
        category.subcategories = Self::children(&category, db).await?;
        
        Ok(category)
    }
    
    pub async fn find_by_course_id(db: &DB, course_id: Uuid) -> Result<Vec<Self>, Error> {
        let categories = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE course_id = ? ORDER BY position ASC"
        )
        .bind(course_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load topic counts and subcategories for each category
        let mut complete_categories = Vec::with_capacity(categories.len());
        
        for mut category in categories {
            // Load topic count
            category.topic_count = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM topics WHERE category_id = ?"
            )
            .bind(category.id)
            .fetch_one(&db.pool)
            .await?;
            
            // Load subcategories
            category.subcategories = Self::children(&category, db).await?;
            complete_categories.push(category);
        }
        
        Ok(complete_categories)
    }
    
    pub async fn find_root_categories(db: &DB) -> Result<Vec<Self>, Error> {
        let categories = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE parent_id IS NULL ORDER BY position ASC"
        )
        .fetch_all(&db.pool)
        .await?;
        
        // Load topic counts and subcategories for each category
        let mut complete_categories = Vec::with_capacity(categories.len());
        
        for mut category in categories {
            // Load topic count
            category.topic_count = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM topics WHERE category_id = ?"
            )
            .bind(category.id)
            .fetch_one(&db.pool)
            .await?;
            
            // Load subcategories
            category.subcategories = Self::children(&category, db).await?;
            complete_categories.push(category);
        }
        
        Ok(complete_categories)
    }
    
    pub async fn find_all(db: &DB) -> Result<Vec<Self>, Error> {
        let categories = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories ORDER BY position ASC"
        )
        .fetch_all(&db.pool)
        .await?;
        
        // Load topic counts for each category
        let mut complete_categories = Vec::with_capacity(categories.len());
        
        for mut category in categories {
            // Load topic count
            category.topic_count = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM topics WHERE category_id = ?"
            )
            .bind(category.id)
            .fetch_one(&db.pool)
            .await?;
            
            complete_categories.push(category);
        }
        
        Ok(complete_categories)
    }
    
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        // Validate the category first
        if let Err(errors) = self.validate() {
            return Err(Error::ValidationError(errors));
        }
        
        sqlx::query(
            "INSERT INTO categories 
            (id, name, description, slug, parent_id, position, color, text_color,
            is_hidden, created_at, updated_at, discourse_category_id, course_id, sync_status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.slug)
        .bind(self.parent_id)
        .bind(self.position)
        .bind(&self.color)
        .bind(&self.text_color)
        .bind(self.is_hidden)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.discourse_category_id)
        .bind(self.course_id)
        .bind(self.sync_status as i32)
        .execute(&db.pool)
        .await?;
        
        Ok(self.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        // Validate the category first
        if let Err(errors) = self.validate() {
            return Err(Error::ValidationError(errors));
        }
        
        sqlx::query(
            "UPDATE categories SET
            name = ?, description = ?, slug = ?, parent_id = ?, position = ?, 
            color = ?, text_color = ?, is_hidden = ?, updated_at = ?, 
            discourse_category_id = ?, course_id = ?, sync_status = ?
            WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.slug)
        .bind(self.parent_id)
        .bind(self.position)
        .bind(&self.color)
        .bind(&self.text_color)
        .bind(self.is_hidden)
        .bind(Utc::now())
        .bind(self.discourse_category_id)
        .bind(self.course_id)
        .bind(self.sync_status as i32)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // First check if there are any topics in this category
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM topics WHERE category_id = ?"
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
        
        if count > 0 {
            return Err(Error::ValidationError(vec![
                "Cannot delete category with existing topics".to_string()
            ]));
        }
        
        // Check if there are any child categories
        let child_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM categories WHERE parent_id = ?"
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
        
        if child_count > 0 {
            return Err(Error::ValidationError(vec![
                "Cannot delete category with child categories".to_string()
            ]));
        }
        
        // Delete the category
        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Commit transaction
        tx.commit().await?;
            
        Ok(())
    }
    
    // Get all topics in this category
    pub async fn topics(&self, db: &DB) -> Result<Vec<Topic>, Error> {
        Topic::find_by_category(db, self.id).await
    }
    
    // Get child categories
    pub async fn children(&self, db: &DB) -> Result<Vec<Self>, Error> {
        let subcategories = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE parent_id = ? ORDER BY position ASC"
        )
        .bind(self.id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load topic counts for each subcategory
        let mut complete_subcategories = Vec::with_capacity(subcategories.len());
        
        for mut subcategory in subcategories {
            // Load topic count
            subcategory.topic_count = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM topics WHERE category_id = ?"
            )
            .bind(subcategory.id)
            .fetch_one(&db.pool)
            .await?;
            
            complete_subcategories.push(subcategory);
        }
        
        Ok(complete_subcategories)
    }
    
    // Set visibility
    pub async fn set_hidden(&mut self, db: &DB, hidden: bool) -> Result<(), Error> {
        self.is_hidden = hidden;
        self.updated_at = Utc::now();
        
        sqlx::query(
            "UPDATE categories SET is_hidden = ?, updated_at = ? WHERE id = ?"
        )
        .bind(self.is_hidden)
        .bind(self.updated_at)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    // Update position
    pub async fn set_position(&mut self, db: &DB, position: i32) -> Result<(), Error> {
        self.position = position;
        self.updated_at = Utc::now();
        
        sqlx::query(
            "UPDATE categories SET position = ?, updated_at = ? WHERE id = ?"
        )
        .bind(self.position)
        .bind(self.updated_at)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    // Discourse API integration method
    pub fn from_discourse_api(
        discourse_category: &serde_json::Value,
        parent_map: &std::collections::HashMap<i64, Uuid>
    ) -> Result<Self, String> {
        // Extract and validate required fields
        let category_id = discourse_category["id"]
            .as_i64()
            .ok_or("Missing or invalid category ID")?;
            
        let name = discourse_category["name"]
            .as_str()
            .ok_or("Missing or invalid category name")?
            .to_string();
            
        let slug = discourse_category["slug"]
            .as_str()
            .ok_or("Missing or invalid category slug")?
            .to_string();
            
        // Extract optional fields
        let description = discourse_category["description"]
            .as_str()
            .map(String::from);
            
        let color = discourse_category["color"]
            .as_str()
            .map(|c| format!("#{}", c))
            .unwrap_or_else(|| "#0088CC".to_string());
            
        let text_color = discourse_category["text_color"]
            .as_str()
            .map(|c| format!("#{}", c));
            
        let position = discourse_category["position"]
            .as_i64()
            .unwrap_or(0) as i32;
            
        // Handle parent category mapping
        let parent_id = if let Some(parent_discourse_id) = discourse_category["parent_category_id"].as_i64() {
            parent_map.get(&parent_discourse_id).cloned()
        } else {
            None
        };
        
        // Parse dates if available (they might not be in the basic category response)
        let created_at = parse_date_string(discourse_category["created_at"].as_str())
            .unwrap_or_else(Utc::now);
        
        let updated_at = parse_date_string(discourse_category["updated_at"].as_str())
            .unwrap_or_else(|| created_at);
            
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            slug,
            parent_id,
            position,
            color,
            text_color,
            is_hidden: false, // Default to visible
            created_at,
            updated_at,
            discourse_category_id: Some(category_id),
            course_id: None, // Discourse doesn't have courses
            sync_status: SyncStatus::SyncedWithDiscourse,
            topic_count: 0,
            subcategories: Vec::new(),
        })
    }
    
    /// Regenerate the slug for this category based on its name
    pub fn regenerate_slug(&mut self) {
        self.slug = self.name.to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();
    }
}