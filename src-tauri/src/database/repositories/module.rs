use sqlx::{Pool, Sqlite};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::core::errors::AppError;
use crate::lms::models::{Module, ModuleItem, ModuleItemType, CompletionRequirement, CompletionRequirementType};
use crate::sync::operations::{SyncOperation, OperationType};
use crate::sync::engine::SyncEngine;
use std::sync::Arc;

pub struct ModuleRepository {
    db: Pool<Sqlite>,
    sync_engine: Arc<SyncEngine>,
}

impl ModuleRepository {
    pub fn new(db: Pool<Sqlite>, sync_engine: Arc<SyncEngine>) -> Self {
        Self { db, sync_engine }
    }
    
    // Create a new module
    pub async fn create_module(&self, user_id: i64, mut module: Module) -> Result<i64, AppError> {
        // Check if course exists
        let course = sqlx::query!(
            "SELECT id FROM courses WHERE id = ?",
            module.course_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        if course.is_none() {
            return Err(AppError::NotFound(format!("Course with id {} not found", module.course_id)));
        }
        
        // Set position if not provided
        if module.position.is_none() {
            let max_position = sqlx::query!(
                "SELECT MAX(position) as max_pos FROM modules WHERE course_id = ?",
                module.course_id
            )
            .fetch_one(&self.db)
            .await?;
            
            module.position = Some(max_position.max_pos.unwrap_or(0) + 1);
        }
        
        let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
        
        let result = sqlx::query!(
            r#"
            INSERT INTO modules 
                (course_id, name, position, unlock_at, require_sequential_progress, published, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            module.course_id,
            module.name,
            module.position,
            module.unlock_at,
            module.require_sequential_progress,
            module.published,
            now,
            now
        )
        .fetch_one(&self.db)
        .await?;
        
        let module_id = result.id;
        
        // Create sync operation
        let mut module_with_id = module.clone();
        module_with_id.id = Some(module_id);
        
        // Queue sync operation
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Create,
            "module",
            Some(&module_id.to_string()),
            serde_json::to_value(module_with_id).unwrap(),
        ).await?;
        
        Ok(module_id)
    }
    
    // Get modules for a course
    pub async fn get_modules_by_course(&self, course_id: i64) -> Result<Vec<Module>, AppError> {
        let modules = sqlx::query_as!(
            Module,
            r#"
            SELECT 
                id, course_id, name, position, unlock_at, require_sequential_progress, 
                published, items_count, created_at, updated_at
            FROM modules
            WHERE course_id = ?
            ORDER BY position
            "#,
            course_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(modules)
    }
    
    // Get a module by ID
    pub async fn get_module_by_id(&self, module_id: i64) -> Result<Module, AppError> {
        let module = sqlx::query_as!(
            Module,
            r#"
            SELECT 
                id, course_id, name, position, unlock_at, require_sequential_progress, 
                published, items_count, created_at, updated_at
            FROM modules
            WHERE id = ?
            "#,
            module_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Module with id {} not found", module_id)))?;
        
        Ok(module)
    }
    
    // Update a module
    pub async fn update_module(&self, module_id: i64, user_id: i64, module: Module) -> Result<(), AppError> {
        // Check if module exists
        let existing = self.get_module_by_id(module_id).await?;
        
        // Check if user has permission to update the module
        // In a real application, you would check if the user is an instructor for the course
        
        let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
        
        sqlx::query!(
            r#"
            UPDATE modules
            SET name = ?, position = ?, unlock_at = ?, 
                require_sequential_progress = ?, published = ?, updated_at = ?
            WHERE id = ?
            "#,
            module.name,
            module.position,
            module.unlock_at,
            module.require_sequential_progress,
            module.published,
            now,
            module_id
        )
        .execute(&self.db)
        .await?;
        
        // Queue sync operation
        let mut module_with_id = module.clone();
        module_with_id.id = Some(module_id);
        
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Update,
            "module",
            Some(&module_id.to_string()),
            serde_json::to_value(module_with_id).unwrap(),
        ).await?;
        
        Ok(())
    }
    
    // Delete a module
    pub async fn delete_module(&self, module_id: i64, user_id: i64) -> Result<(), AppError> {
        // Check if module exists
        let existing = self.get_module_by_id(module_id).await?;
        
        // Check if user has permission to delete the module
        // In a real application, you would check if the user is an instructor for the course
        
        // Delete all module items first
        sqlx::query!(
            "DELETE FROM module_items WHERE module_id = ?",
            module_id
        )
        .execute(&self.db)
        .await?;
        
        // Delete the module
        sqlx::query!(
            "DELETE FROM modules WHERE id = ?",
            module_id
        )
        .execute(&self.db)
        .await?;
        
        // Queue sync operation
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Delete,
            "module",
            Some(&module_id.to_string()),
            serde_json::json!({}),
        ).await?;
        
        Ok(())
    }
    
    // Get module items
    pub async fn get_module_items(&self, module_id: i64) -> Result<Vec<ModuleItem>, AppError> {
        // Here we would query module items
        // This is a simplified implementation
        
        // We need to join with completion requirements table
        // For now, we'll just fetch items directly
        
        let items = sqlx::query!(
            r#"
            SELECT 
                id, module_id, title, position, indent, item_type, content_id,
                page_url, external_url, published, created_at, updated_at,
                completion_requirement_type, min_score, completed
            FROM module_items
            WHERE module_id = ?
            ORDER BY position
            "#,
            module_id
        )
        .fetch_all(&self.db)
        .await?;
        
        // Convert rows to ModuleItem objects
        let mut module_items = Vec::new();
        
        for item in items {
            let item_type = match item.item_type.as_deref() {
                Some("assignment") => ModuleItemType::Assignment,
                Some("quiz") => ModuleItemType::Quiz,
                Some("file") => ModuleItemType::File,
                Some("page") => ModuleItemType::Page,
                Some("discussion") => ModuleItemType::Discussion,
                Some("external_url") => ModuleItemType::ExternalUrl,
                Some("external_tool") => ModuleItemType::ExternalTool,
                Some("header") => ModuleItemType::Header,
                Some("sub_header") => ModuleItemType::SubHeader,
                _ => ModuleItemType::Page, // Default
            };
            
            // Create completion requirement if available
            let completion_requirement = if let Some(req_type) = item.completion_requirement_type {
                let requirement_type = match req_type.as_str() {
                    "must_view" => CompletionRequirementType::MustView,
                    "must_submit" => CompletionRequirementType::MustSubmit,
                    "must_contribute" => CompletionRequirementType::MustContribute,
                    "min_score" => CompletionRequirementType::MinScore,
                    "mark_done" => CompletionRequirementType::MarkDone,
                    _ => CompletionRequirementType::MustView,
                };
                
                Some(CompletionRequirement {
                    requirement_type,
                    min_score: item.min_score.map(|s| s as f32),
                    completed: item.completed,
                })
            } else {
                None
            };
            
            let module_item = ModuleItem {
                id: Some(item.id),
                module_id: item.module_id,
                title: item.title,
                position: item.position,
                indent: item.indent.unwrap_or(0),
                item_type,
                content_id: item.content_id,
                page_url: item.page_url,
                external_url: item.external_url,
                completion_requirement,
                published: item.published.unwrap_or(false),
                created_at: item.created_at,
                updated_at: item.updated_at,
            };
            
            module_items.push(module_item);
        }
        
        Ok(module_items)
    }
    
    // Create a module item
    pub async fn create_module_item(&self, module_id: i64, user_id: i64, mut item: ModuleItem) -> Result<i64, AppError> {
        // Check if module exists
        let module = self.get_module_by_id(module_id).await?;
        
        // Check if user has permission
        // In a real application, you would check if the user is an instructor for the course
        
        // Set position if not provided
        if item.position.is_none() {
            let max_position = sqlx::query!(
                "SELECT MAX(position) as max_pos FROM module_items WHERE module_id = ?",
                module_id
            )
            .fetch_one(&self.db)
            .await?;
            
            item.position = Some(max_position.max_pos.unwrap_or(0) + 1);
        }
        
        // Convert item_type to string
        let item_type_str = match item.item_type {
            ModuleItemType::Assignment => "assignment",
            ModuleItemType::Quiz => "quiz",
            ModuleItemType::File => "file",
            ModuleItemType::Page => "page",
            ModuleItemType::Discussion => "discussion",
            ModuleItemType::ExternalUrl => "external_url",
            ModuleItemType::ExternalTool => "external_tool",
            ModuleItemType::Header => "header",
            ModuleItemType::SubHeader => "sub_header",
        };
        
        let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
        
        // Extract completion requirement fields
        let (completion_requirement_type, min_score, completed) = if let Some(req) = &item.completion_requirement {
            let req_type = match req.requirement_type {
                CompletionRequirementType::MustView => "must_view",
                CompletionRequirementType::MustSubmit => "must_submit",
                CompletionRequirementType::MustContribute => "must_contribute",
                CompletionRequirementType::MinScore => "min_score",
                CompletionRequirementType::MarkDone => "mark_done",
            };
            
            (Some(req_type), req.min_score.map(|s| s as f64), req.completed)
        } else {
            (None, None, None)
        };
        
        // Insert the module item
        let result = sqlx::query!(
            r#"
            INSERT INTO module_items 
                (module_id, title, position, indent, item_type, content_id, page_url, external_url,
                 completion_requirement_type, min_score, completed, published, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            module_id,
            item.title,
            item.position,
            item.indent,
            item_type_str,
            item.content_id,
            item.page_url,
            item.external_url,
            completion_requirement_type,
            min_score,
            completed,
            item.published,
            now,
            now
        )
        .fetch_one(&self.db)
        .await?;
        
        let item_id = result.id;
        
        // Update module items count
        sqlx::query!(
            r#"
            UPDATE modules
            SET items_count = (SELECT COUNT(*) FROM module_items WHERE module_id = ?),
                updated_at = ?
            WHERE id = ?
            "#,
            module_id,
            now,
            module_id
        )
        .execute(&self.db)
        .await?;
        
        // Queue sync operation
        let mut item_with_id = item.clone();
        item_with_id.id = Some(item_id);
        
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Create,
            "module_item",
            Some(&item_id.to_string()),
            serde_json::to_value(item_with_id).unwrap(),
        ).await?;
        
        Ok(item_id)
    }
    
    // Update a module item
    pub async fn update_module_item(&self, module_id: i64, item_id: i64, user_id: i64, item: ModuleItem) -> Result<(), AppError> {
        // Check if item exists
        let existing_item = sqlx::query!(
            "SELECT id FROM module_items WHERE id = ? AND module_id = ?",
            item_id,
            module_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        if existing_item.is_none() {
            return Err(AppError::NotFound(format!("Module item with id {} not found", item_id)));
        }
        
        // Check if user has permission
        // In a real application, you would check if the user is an instructor for the course
        
        // Convert item_type to string
        let item_type_str = match item.item_type {
            ModuleItemType::Assignment => "assignment",
            ModuleItemType::Quiz => "quiz",
            ModuleItemType::File => "file",
            ModuleItemType::Page => "page",
            ModuleItemType::Discussion => "discussion",
            ModuleItemType::ExternalUrl => "external_url",
            ModuleItemType::ExternalTool => "external_tool",
            ModuleItemType::Header => "header",
            ModuleItemType::SubHeader => "sub_header",
        };
        
        // Extract completion requirement fields
        let (completion_requirement_type, min_score, completed) = if let Some(req) = &item.completion_requirement {
            let req_type = match req.requirement_type {
                CompletionRequirementType::MustView => "must_view",
                CompletionRequirementType::MustSubmit => "must_submit",
                CompletionRequirementType::MustContribute => "must_contribute",
                CompletionRequirementType::MinScore => "min_score",
                CompletionRequirementType::MarkDone => "mark_done",
            };
            
            (Some(req_type), req.min_score.map(|s| s as f64), req.completed)
        } else {
            (None, None, None)
        };
        
        let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
        
        // Update the module item
        sqlx::query!(
            r#"
            UPDATE module_items
            SET title = ?, position = ?, indent = ?, item_type = ?,
                content_id = ?, page_url = ?, external_url = ?,
                completion_requirement_type = ?, min_score = ?, completed = ?,
                published = ?, updated_at = ?
            WHERE id = ? AND module_id = ?
            "#,
            item.title,
            item.position,
            item.indent,
            item_type_str,
            item.content_id,
            item.page_url,
            item.external_url,
            completion_requirement_type,
            min_score,
            completed,
            item.published,
            now,
            item_id,
            module_id
        )
        .execute(&self.db)
        .await?;
        
        // Queue sync operation
        let mut item_with_id = item.clone();
        item_with_id.id = Some(item_id);
        
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Update,
            "module_item",
            Some(&item_id.to_string()),
            serde_json::to_value(item_with_id).unwrap(),
        ).await?;
        
        Ok(())
    }
    
    // Delete a module item
    pub async fn delete_module_item(&self, module_id: i64, item_id: i64, user_id: i64) -> Result<(), AppError> {
        // Check if item exists
        let existing_item = sqlx::query!(
            "SELECT id FROM module_items WHERE id = ? AND module_id = ?",
            item_id,
            module_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        if existing_item.is_none() {
            return Err(AppError::NotFound(format!("Module item with id {} not found", item_id)));
        }
        
        // Check if user has permission
        // In a real application, you would check if the user is an instructor for the course
        
        // Delete the module item
        sqlx::query!(
            "DELETE FROM module_items WHERE id = ? AND module_id = ?",
            item_id,
            module_id
        )
        .execute(&self.db)
        .await?;
        
        let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
        
        // Update module items count
        sqlx::query!(
            r#"
            UPDATE modules
            SET items_count = (SELECT COUNT(*) FROM module_items WHERE module_id = ?),
                updated_at = ?
            WHERE id = ?
            "#,
            module_id,
            now,
            module_id
        )
        .execute(&self.db)
        .await?;
        
        // Queue sync operation
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Delete,
            "module_item",
            Some(&item_id.to_string()),
            serde_json::json!({}),
        ).await?;
        
        Ok(())
    }
    
    // Reorder modules
    pub async fn reorder_modules(&self, course_id: i64, user_id: i64, order: Vec<i64>) -> Result<(), AppError> {
        // Check if course exists
        let course = sqlx::query!(
            "SELECT id FROM courses WHERE id = ?",
            course_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        if course.is_none() {
            return Err(AppError::NotFound(format!("Course with id {} not found", course_id)));
        }
        
        // Check if user has permission
        // In a real application, you would check if the user is an instructor for the course
        
        // Update positions
        for (position, module_id) in order.iter().enumerate() {
            sqlx::query!(
                "UPDATE modules SET position = ? WHERE id = ? AND course_id = ?",
                position as i32 + 1, // 1-based positions
                module_id,
                course_id
            )
            .execute(&self.db)
            .await?;
        }
        
        // Queue sync operation
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Update,
            "course_modules_order",
            Some(&course_id.to_string()),
            serde_json::json!({ "order": order }),
        ).await?;
        
        Ok(())
    }
    
    // Reorder module items
    pub async fn reorder_module_items(&self, module_id: i64, user_id: i64, order: Vec<i64>) -> Result<(), AppError> {
        // Check if module exists
        let module = self.get_module_by_id(module_id).await?;
        
        // Check if user has permission
        // In a real application, you would check if the user is an instructor for the course
        
        // Update positions
        for (position, item_id) in order.iter().enumerate() {
            sqlx::query!(
                "UPDATE module_items SET position = ? WHERE id = ? AND module_id = ?",
                position as i32 + 1, // 1-based positions
                item_id,
                module_id
            )
            .execute(&self.db)
            .await?;
        }
        
        // Queue sync operation
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Update,
            "module_items_order",
            Some(&module_id.to_string()),
            serde_json::json!({ "order": order }),
        ).await?;
        
        Ok(())
    }
}

// Helper functions for enum conversions
fn module_item_type_to_string(item_type: ModuleItemType) -> String {
    match item_type {
        ModuleItemType::Assignment => "assignment".to_string(),
        ModuleItemType::Quiz => "quiz".to_string(),
        ModuleItemType::File => "file".to_string(),
        ModuleItemType::Page => "page".to_string(),
        ModuleItemType::Discussion => "discussion".to_string(),
        ModuleItemType::ExternalUrl => "external_url".to_string(),
        ModuleItemType::ExternalTool => "external_tool".to_string(),
        ModuleItemType::Header => "header".to_string(),
    }
}

fn string_to_module_item_type(item_type: &str) -> ModuleItemType {
    match item_type {
        "assignment" => ModuleItemType::Assignment,
        "quiz" => ModuleItemType::Quiz,
        "file" => ModuleItemType::File,
        "page" => ModuleItemType::Page,
        "discussion" => ModuleItemType::Discussion,
        "external_url" => ModuleItemType::ExternalUrl,
        "external_tool" => ModuleItemType::ExternalTool,
        "header" => ModuleItemType::Header,
        _ => ModuleItemType::Header, // Default
    }
}

fn completion_requirement_type_to_string(req_type: CompletionRequirementType) -> String {
    match req_type {
        CompletionRequirementType::MustView => "must_view".to_string(),
        CompletionRequirementType::MustSubmit => "must_submit".to_string(),
        CompletionRequirementType::MustContribute => "must_contribute".to_string(),
        CompletionRequirementType::MinScore => "min_score".to_string(),
        CompletionRequirementType::MarkDone => "mark_done".to_string(),
    }
}

fn string_to_completion_requirement_type(req_type: &str) -> CompletionRequirementType {
    match req_type {
        "must_view" => CompletionRequirementType::MustView,
        "must_submit" => CompletionRequirementType::MustSubmit,
        "must_contribute" => CompletionRequirementType::MustContribute,
        "min_score" => CompletionRequirementType::MinScore,
        "mark_done" => CompletionRequirementType::MarkDone,
        _ => CompletionRequirementType::MustView, // Default
    }
}