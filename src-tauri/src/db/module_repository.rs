use crate::models::module::{Module, ModuleUpdate, ModuleItem, ModuleItemUpdate, ModuleStatus, ModuleItemType};
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
pub trait ModuleRepository: Send + Sync {
    async fn get_modules_for_course(&self, course_id: &str) -> Result<Vec<Module>, DbError>;
    async fn get_module(&self, module_id: &str) -> Result<Option<Module>, DbError>;
    async fn get_modules_count(&self, course_id: &str) -> Result<i32, DbError>;
    async fn create_module(&self, module: Module) -> Result<Module, DbError>;
    async fn update_module(&self, module_id: &str, update: ModuleUpdate) -> Result<Module, DbError>;
    async fn delete_module(&self, module_id: &str) -> Result<bool, DbError>;
    async fn reorder_modules(&self, course_id: &str, module_ids: &[String]) -> Result<Vec<Module>, DbError>;
    
    async fn get_module_items(&self, module_id: &str) -> Result<Vec<ModuleItem>, DbError>;
    async fn get_module_item(&self, item_id: &str) -> Result<Option<ModuleItem>, DbError>;
    async fn get_module_items_count(&self, module_id: &str) -> Result<i32, DbError>;
    async fn create_module_item(&self, item: ModuleItem) -> Result<ModuleItem, DbError>;
    async fn update_module_item(&self, item_id: &str, update: ModuleItemUpdate) -> Result<ModuleItem, DbError>;
    async fn delete_module_item(&self, item_id: &str) -> Result<bool, DbError>;
    async fn reorder_module_items(&self, module_id: &str, item_ids: &[String]) -> Result<Vec<ModuleItem>, DbError>;
}

pub struct SqliteModuleRepository {
    pool: Pool<Sqlite>,
}

impl SqliteModuleRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ModuleRepository for SqliteModuleRepository {
    #[instrument(skip(self), err)]
    async fn get_modules_for_course(&self, course_id: &str) -> Result<Vec<Module>, DbError> {
        sqlx::query_as!(
            Module,
            r#"
            SELECT 
                id, course_id, title, position, items_count, 
                publish_final_grade, published, 
                status as "status: ModuleStatus", 
                created_at, updated_at
            FROM modules
            WHERE course_id = ?
            ORDER BY position ASC
            "#,
            course_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get modules for course {}: {}", course_id, e);
            DbError::QueryError(e.to_string())
        })
    }
    
    #[instrument(skip(self), err)]
    async fn get_module(&self, module_id: &str) -> Result<Option<Module>, DbError> {
        sqlx::query_as!(
            Module,
            r#"
            SELECT 
                id, course_id, title, position, items_count, 
                publish_final_grade, published, 
                status as "status: ModuleStatus", 
                created_at, updated_at
            FROM modules
            WHERE id = ?
            "#,
            module_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get module {}: {}", module_id, e);
            DbError::QueryError(e.to_string())
        })
    }
    
    #[instrument(skip(self), err)]
    async fn get_modules_count(&self, course_id: &str) -> Result<i32, DbError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM modules WHERE course_id = ?",
            course_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get modules count for course {}: {}", course_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        Ok(result.count as i32)
    }
    
    #[instrument(skip(self), fields(module_id = %module.id), err)]
    async fn create_module(&self, module: Module) -> Result<Module, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        sqlx::query!(
            r#"
            INSERT INTO modules (
                id, course_id, title, position, items_count,
                publish_final_grade, published, status, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            module.id,
            module.course_id,
            module.title,
            module.position,
            module.items_count,
            module.publish_final_grade,
            module.published,
            module.status.to_string(),
            module.created_at,
            module.updated_at
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to create module: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        info!("Created new module with ID: {}", module.id);
        Ok(module)
    }
    
    #[instrument(skip(self), err)]
    async fn update_module(&self, module_id: &str, update: ModuleUpdate) -> Result<Module, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Get existing module
        let module = self.get_module(module_id).await?
            .ok_or_else(|| DbError::DataError(format!("Module not found with ID: {}", module_id)))?;
        
        // Build update query
        let mut query_parts = Vec::new();
        let mut query_params = Vec::new();
        
        if let Some(title) = &update.title {
            query_parts.push("title = ?");
            query_params.push(title.clone());
        }
        
        if let Some(position) = update.position {
            query_parts.push("position = ?");
            query_params.push(position.to_string());
        }
        
        if let Some(publish_final_grade) = update.publish_final_grade {
            query_parts.push("publish_final_grade = ?");
            query_params.push(publish_final_grade.to_string());
        }
        
        if let Some(published) = update.published {
            query_parts.push("published = ?");
            query_params.push(published.to_string());
        }
        
        if let Some(status) = &update.status {
            query_parts.push("status = ?");
            query_params.push(status.to_string());
        }
        
        // Add updated_at
        let now = chrono::Utc::now().to_rfc3339();
        query_parts.push("updated_at = ?");
        query_params.push(now.clone());
        
        // If no fields to update, return the original module
        if query_parts.is_empty() {
            return Ok(module);
        }
        
        // Build and execute the query
        let query = format!(
            "UPDATE modules SET {} WHERE id = ?",
            query_parts.join(", ")
        );
        
        // Build dynamic query with parameters
        let mut query_builder = sqlx::QueryBuilder::new(query);
        for param in &query_params {
            query_builder.push_bind(param);
        }
        query_builder.push_bind(module_id);
        
        query_builder
            .build()
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to update module {}: {}", module_id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Fetch the updated module
        self.get_module(module_id).await?
            .ok_or_else(|| DbError::DataError(format!("Module not found after update with ID: {}", module_id)))
    }
    
    #[instrument(skip(self), err)]
    async fn delete_module(&self, module_id: &str) -> Result<bool, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // First delete all module items
        sqlx::query!("DELETE FROM module_items WHERE module_id = ?", module_id)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to delete module items for module {}: {}", module_id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        // Then delete the module
        let result = sqlx::query!("DELETE FROM modules WHERE id = ?", module_id)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to delete module {}: {}", module_id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted module with ID: {}", module_id);
        } else {
            info!("No module found to delete with ID: {}", module_id);
        }
        
        Ok(deleted)
    }
    
    #[instrument(skip(self), err)]
    async fn reorder_modules(&self, course_id: &str, module_ids: &[String]) -> Result<Vec<Module>, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Verify all module IDs belong to the course
        let existing_modules = sqlx::query!(
            "SELECT id FROM modules WHERE course_id = ?",
            course_id
        )
        .fetch_all(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to get existing modules for course {}: {}", course_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        let existing_ids: Vec<String> = existing_modules.iter().map(|m| m.id.clone()).collect();
        
        // Check if all provided IDs exist in the course
        for id in module_ids {
            if !existing_ids.contains(id) {
                return Err(DbError::DataError(format!("Module ID {} does not belong to course {}", id, course_id)));
            }
        }
        
        // Update positions
        for (idx, id) in module_ids.iter().enumerate() {
            sqlx::query!(
                "UPDATE modules SET position = ?, updated_at = ? WHERE id = ?",
                idx + 1,
                chrono::Utc::now().to_rfc3339(),
                id
            )
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to update position for module {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        }
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
            "UPDATE modules SET items_count = items_count + 1, updated_at = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(&item_create.module_id)
        .execute(&mut tx)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        info!("Created new module item with ID: {}", id);
        
        // Return the created item
        self.get_module_item(&id)
            .await?
            .ok_or_else(|| DbError::DataError(format!("Failed to retrieve created module item with ID: {}", id)))
    }
    
    #[instrument(skip(self), err)]
    async fn get_module_items(&self, module_id: &str) -> Result<Vec<ModuleItem>, DbError> {
        sqlx::query_as!(
            ModuleItem,
            r#"
            SELECT 
                id, module_id, title, position, 
                item_type as "item_type: ModuleItemType", 
                content_id, content_type, url, page_url,
                published, created_at, updated_at
            FROM module_items
            WHERE module_id = ?
            ORDER BY position ASC
            "#,
            module_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get items for module {}: {}", module_id, e);
            DbError::QueryError(e.to_string())
        })
    }

    #[instrument(skip(self), err)]
    async fn get_module_item(&self, item_id: &str) -> Result<Option<ModuleItem>, DbError> {
        sqlx::query_as!(
            ModuleItem,
            r#"
            SELECT 
                id, module_id, title, position, 
                item_type as "item_type: ModuleItemType", 
                content_id, content_type, url, page_url,
                published, created_at, updated_at
            FROM module_items
            WHERE id = ?
            "#,
            item_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get module item {}: {}", item_id, e);
            DbError::QueryError(e.to_string())
        })
    }

    #[instrument(skip(self), err)]
    async fn get_module_items_count(&self, module_id: &str) -> Result<i32, DbError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM module_items WHERE module_id = ?",
            module_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get items count for module {}: {}", module_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        Ok(result.count as i32)
    }

    #[instrument(skip(self), fields(item_id = %item.id), err)]
    async fn create_module_item(&self, item: ModuleItem) -> Result<ModuleItem, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Insert item
        sqlx::query!(
            r#"
            INSERT INTO module_items (
                id, module_id, title, position, item_type,
                content_id, content_type, url, page_url,
                published, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            item.id,
            item.module_id,
            item.title,
            item.position,
            item.item_type.to_string(),
            item.content_id,
            item.content_type,
            item.url,
            item.page_url,
            item.published,
            item.created_at,
            item.updated_at
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to create module item: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        // Update items_count in the module
        sqlx::query!(
            "UPDATE modules SET items_count = items_count + 1, updated_at = ? WHERE id = ?",
            chrono::Utc::now().to_rfc3339(),
            item.module_id
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to update items count for module {}: {}", item.module_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        info!("Created new module item with ID: {}", item.id);
        Ok(item)
    }

    #[instrument(skip(self), err)]
    async fn update_module_item(&self, item_id: &str, update: ModuleItemUpdate) -> Result<ModuleItem, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Get existing item
        let item = self.get_module_item(item_id).await?
            .ok_or_else(|| DbError::DataError(format!("Module item not found with ID: {}", item_id)))?;
        
        // Build update query
        let mut query_parts = Vec::new();
        let mut query_params = Vec::new();
        
        if let Some(title) = &update.title {
            query_parts.push("title = ?");
            query_params.push(title.clone());
        }
        
        if let Some(position) = update.position {
            query_parts.push("position = ?");
            query_params.push(position.to_string());
        }
        
        if let Some(published) = update.published {
            query_parts.push("published = ?");
            query_params.push(published.to_string());
        }
        
        if let Some(url) = &update.url {
            query_parts.push("url = ?");
            query_params.push(url.clone());
        }
        
        if let Some(page_url) = &update.page_url {
            query_parts.push("page_url = ?");
            query_params.push(page_url.clone());
        }
        
        // Add updated_at
        let now = chrono::Utc::now().to_rfc3339();
        query_parts.push("updated_at = ?");
        query_params.push(now.clone());
        
        // If no fields to update, return the original item
        if query_parts.len() == 1 {  // Only updated_at
            return Ok(item);
        }
        
        // Build and execute the query
        let query = format!(
            "UPDATE module_items SET {} WHERE id = ?",
            query_parts.join(", ")
        );
        
        // Build dynamic query with parameters
        let mut query_builder = sqlx::QueryBuilder::new(query);
        for param in &query_params {
            query_builder.push_bind(param);
        }
        query_builder.push_bind(item_id);
        
        query_builder
            .build()
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to update module item {}: {}", item_id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Fetch the updated item
        self.get_module_item(item_id).await?
            .ok_or_else(|| DbError::DataError(format!("Module item not found after update with ID: {}", item_id)))
    }

    #[instrument(skip(self), err)]
    async fn delete_module_item(&self, item_id: &str) -> Result<bool, DbError> {
        // Get the item to get its module_id
        let item = match self.get_module_item(item_id).await? {
            Some(i) => i,
            None => {
                // Item not found, nothing to delete
                return Ok(false);
            }
        };
        
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Delete the item
        let result = sqlx::query!("DELETE FROM module_items WHERE id = ?", item_id)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to delete module item {}: {}", item_id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        if result.rows_affected() > 0 {
            // Update items_count in the module
            sqlx::query!(
                "UPDATE modules SET items_count = items_count - 1, updated_at = ? WHERE id = ?",
                chrono::Utc::now().to_rfc3339(),
                item.module_id
            )
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to update items count for module {}: {}", item.module_id, e);
                DbError::QueryError(e.to_string())
            })?;
        }
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted module item with ID: {}", item_id);
        } else {
            info!("No module item found to delete with ID: {}", item_id);
        }
        
        Ok(deleted)
    }

    #[instrument(skip(self), err)]
    async fn reorder_module_items(&self, module_id: &str, item_ids: &[String]) -> Result<Vec<ModuleItem>, DbError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Verify all item IDs belong to the module
        let existing_items = sqlx::query!(
            "SELECT id FROM module_items WHERE module_id = ?",
            module_id
        )
        .fetch_all(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to get existing items for module {}: {}", module_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        let existing_ids: Vec<String> = existing_items.iter().map(|i| i.id.clone()).collect();
        
        // Check if all provided IDs exist in the module
        for id in item_ids {
            if !existing_ids.contains(id) {
                return Err(DbError::DataError(format!("Item ID {} does not belong to module {}", id, module_id)));
            }
        }
        
        // Update positions
        for (idx, id) in item_ids.iter().enumerate() {
            sqlx::query!(
                "UPDATE module_items SET position = ?, updated_at = ? WHERE id = ?",
                idx + 1,
                chrono::Utc::now().to_rfc3339(),
                id
            )
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to update position for item {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        }
        
        // Update module's updated_at
        sqlx::query!(
            "UPDATE modules SET updated_at = ? WHERE id = ?",
            chrono::Utc::now().to_rfc3339(),
            module_id
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to update module timestamp {}: {}", module_id, e);
            DbError::QueryError(e.to_string())
        })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Fetch updated items
        self.get_module_items(module_id).await
    }
}