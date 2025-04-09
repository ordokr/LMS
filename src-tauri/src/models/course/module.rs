use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;
use crate::db::DB;
use crate::error::Error;
use crate::models::content::resource::Resource;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Module {
    pub id: Uuid,
    pub course_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub prerequisite_module_id: Option<Uuid>,
    pub unlock_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    pub canvas_module_id: Option<String>,
    
    #[sqlx(skip)]
    pub items: Vec<ModuleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModuleItem {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub item_type: String, // "Assignment", "Resource", "Quiz", "Discussion", "ExternalUrl", "Page"
    pub content_id: Option<Uuid>,
    pub external_url: Option<String>,
    pub position: i32,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canvas_module_item_id: Option<String>,
}

impl Module {
    pub fn new(course_id: Uuid, name: String) -> Self {
        Module {
            id: Uuid::new_v4(),
            course_id,
            name,
            description: None,
            position: 0,
            prerequisite_module_id: None,
            unlock_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            published: false,
            canvas_module_id: None,
            items: Vec::new(),
        }
    }
    
    // Database operations
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let mut module = sqlx::query_as::<_, Self>(
            "SELECT * FROM modules WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load module items
        module.items = ModuleItem::find_by_module_id(db, id).await?;
        
        Ok(module)
    }
    
    pub async fn find_all(db: &DB) -> Result<Vec<Self>, Error> {
        let modules = sqlx::query_as::<_, Self>(
            "SELECT * FROM modules ORDER BY position ASC"
        )
        .fetch_all(&db.pool)
        .await?;
        
        Ok(modules)
    }
    
    pub async fn find_by_course_id(db: &DB, course_id: Uuid) -> Result<Vec<Self>, Error> {
        let mut modules = sqlx::query_as::<_, Self>(
            "SELECT * FROM modules WHERE course_id = ? ORDER BY position ASC"
        )
        .bind(course_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load items for each module
        for module in &mut modules {
            module.items = ModuleItem::find_by_module_id(db, module.id).await?;
        }
        
        Ok(modules)
    }
    
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let mut module = sqlx::query_as::<_, Self>(
            "SELECT * FROM modules WHERE canvas_module_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load module items
        module.items = ModuleItem::find_by_module_id(db, module.id).await?;
        
        Ok(module)
    }
    
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO modules 
            (id, course_id, name, description, position, prerequisite_module_id, 
            unlock_at, created_at, updated_at, published, canvas_module_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.course_id)
        .bind(&self.name)
        .bind(&self.description)
        .bind(self.position)
        .bind(self.prerequisite_module_id)
        .bind(self.unlock_at)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.published)
        .bind(&self.canvas_module_id)
        .execute(&db.pool)
        .await?;
        
        // Create module items
        for item in &self.items {
            item.create(db).await?;
        }
        
        Ok(self.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE modules SET
            course_id = ?, name = ?, description = ?, position = ?, 
            prerequisite_module_id = ?, unlock_at = ?, updated_at = ?, 
            published = ?, canvas_module_id = ?
            WHERE id = ?"
        )
        .bind(self.course_id)
        .bind(&self.name)
        .bind(&self.description)
        .bind(self.position)
        .bind(self.prerequisite_module_id)
        .bind(self.unlock_at)
        .bind(Utc::now())
        .bind(self.published)
        .bind(&self.canvas_module_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        // Update module items (clear and recreate for simplicity)
        ModuleItem::delete_all_for_module(db, self.id).await?;
        
        for item in &self.items {
            item.create(db).await?;
        }
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        // First delete all module items
        ModuleItem::delete_all_for_module(db, id).await?;
        
        // Then delete the module
        sqlx::query("DELETE FROM modules WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    // Add item to this module
    pub fn add_item(&mut self, title: String, item_type: String, content_id: Option<Uuid>, external_url: Option<String>) -> ModuleItem {
        let new_item = ModuleItem {
            id: Uuid::new_v4(),
            module_id: self.id,
            title,
            item_type,
            content_id,
            external_url,
            position: self.items.len() as i32,
            published: self.published,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canvas_module_item_id: None,
        };
        
        self.items.push(new_item.clone());
        new_item
    }
    
    // Helper method to remove an item by ID
    pub fn remove_item(&mut self, item_id: Uuid) -> bool {
        let initial_len = self.items.len();
        self.items.retain(|item| item.id != item_id);
        
        // Update positions after removal
        for (index, item) in self.items.iter_mut().enumerate() {
            item.position = index as i32;
        }
        
        self.items.len() != initial_len
    }
}

// Module item implementations
impl ModuleItem {
    pub async fn find_by_module_id(db: &DB, module_id: Uuid) -> Result<Vec<Self>, Error> {
        let items = sqlx::query_as::<_, Self>(
            "SELECT * FROM module_items WHERE module_id = ? ORDER BY position ASC"
        )
        .bind(module_id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(items)
    }
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let item = sqlx::query_as::<_, Self>(
            "SELECT * FROM module_items WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(item)
    }
    
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO module_items 
            (id, module_id, title, item_type, content_id, external_url, 
            position, published, created_at, updated_at, canvas_module_item_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.module_id)
        .bind(&self.title)
        .bind(&self.item_type)
        .bind(self.content_id)
        .bind(&self.external_url)
        .bind(self.position)
        .bind(self.published)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.canvas_module_item_id)
        .execute(&db.pool)
        .await?;
        
        Ok(self.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE module_items SET
            module_id = ?, title = ?, item_type = ?, content_id = ?, 
            external_url = ?, position = ?, published = ?, updated_at = ?,
            canvas_module_item_id = ?
            WHERE id = ?"
        )
        .bind(self.module_id)
        .bind(&self.title)
        .bind(&self.item_type)
        .bind(self.content_id)
        .bind(&self.external_url)
        .bind(self.position)
        .bind(self.published)
        .bind(Utc::now())
        .bind(&self.canvas_module_item_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM module_items WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    pub async fn delete_all_for_module(db: &DB, module_id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM module_items WHERE module_id = ?")
            .bind(module_id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    // Get the resource associated with this module item (if it's a resource type)
    pub async fn get_resource(&self, db: &DB) -> Result<Option<Resource>, Error> {
        if self.item_type != "Resource" || self.content_id.is_none() {
            return Ok(None);
        }
        
        let resource = Resource::find(db, self.content_id.unwrap()).await?;
        Ok(Some(resource))
    }
}