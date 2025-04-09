use crate::db::DB;
use crate::models::course::course::Course;
use crate::models::course::module::Module;
use crate::models::course::module_item::ModuleItem;
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasModule {
    pub id: i64,
    pub name: String,
    pub position: i32,
    pub published: bool,
    pub items_count: i32,
    pub items_url: String,
    pub state: String,
    pub unlock_at: Option<DateTime<Utc>>,
    pub prerequisite_module_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasModuleItem {
    pub id: i64,
    pub module_id: i64,
    pub position: i32,
    pub title: String,
    pub content_id: Option<i64>,
    pub content_type: Option<String>,
    pub published: bool,
    pub external_url: Option<String>,
}

pub struct CanvasModuleSync {
    db: DB,
    canvas_service: CanvasIntegrationService,
}

impl CanvasModuleSync {
    pub fn new(db: DB, canvas_service: CanvasIntegrationService) -> Self {
        CanvasModuleSync {
            db,
            canvas_service,
        }
    }

    /// Fetch all modules from Canvas for a course
    pub async fn fetch_canvas_modules(&self, canvas_course_id: &str) -> Result<Vec<CanvasModule>, Error> {
        let url = format!("/api/v1/courses/{}/modules?include[]=items", canvas_course_id);
        self.canvas_service.get::<Vec<CanvasModule>>(&url).await
    }

    /// Fetch module items from Canvas for a specific module
    pub async fn fetch_canvas_module_items(&self, canvas_course_id: &str, canvas_module_id: i64) -> Result<Vec<CanvasModuleItem>, Error> {
        let url = format!("/api/v1/courses/{}/modules/{}/items", canvas_course_id, canvas_module_id);
        self.canvas_service.get::<Vec<CanvasModuleItem>>(&url).await
    }

    /// Fetch a specific module from Canvas
    pub async fn fetch_canvas_module(&self, canvas_course_id: &str, canvas_module_id: i64) -> Result<CanvasModule, Error> {
        let url = format!("/api/v1/courses/{}/modules/{}", canvas_course_id, canvas_module_id);
        self.canvas_service.get::<CanvasModule>(&url).await
    }

    /// Synchronize modules from Canvas to local LMS for a given course
    pub async fn sync_modules_from_canvas(&self, course_id: &str, canvas_course_id: &str) -> Result<String, Error> {
        // Fetch existing modules to handle updates properly
        let existing_modules = Module::find_all_by_course(&self.db, course_id).await?;
        let mut existing_module_map = HashMap::new();
        
        // Create mapping by canvas_module_id
        for module in &existing_modules {
            if let Some(canvas_id) = &module.canvas_module_id {
                existing_module_map.insert(canvas_id.clone(), module.clone());
            }
        }
        
        // Fetch Canvas modules
        let canvas_modules = self.fetch_canvas_modules(canvas_course_id).await?;
        
        // Process Canvas modules
        let mut created_modules = 0;
        let mut updated_modules = 0;
        let mut created_items = 0;
        let mut updated_items = 0;
        
        // Map Canvas module IDs to local module IDs (needed for prerequisites)
        let mut canvas_to_local_id = HashMap::new();
        
        // Track existing items by their Canvas IDs for updates
        let mut existing_items_map = HashMap::new();
        for module in &existing_modules {
            let items = ModuleItem::find_all_by_module(&self.db, &module.id.to_string()).await?;
            for item in items {
                if let Some(canvas_id) = &item.canvas_item_id {
                    existing_items_map.insert(canvas_id.clone(), item);
                }
            }
        }
        
        // First pass: Create or update modules
        for canvas_module in &canvas_modules {
            let canvas_module_id = canvas_module.id.to_string();
            
            if let Some(existing_module) = existing_module_map.get(&canvas_module_id) {
                // Update existing module
                let mut module = existing_module.clone();
                module.name = canvas_module.name.clone();
                module.position = canvas_module.position;
                module.published = canvas_module.published;
                module.unlock_at = canvas_module.unlock_at.map(|dt| dt.to_rfc3339());
                // We'll update prerequisites in second pass
                
                module.update(&self.db).await?;
                updated_modules += 1;
                
                canvas_to_local_id.insert(canvas_module.id, module.id);
            } else {
                // Create a new module
                let module_id = Uuid::new_v4();
                let mut module = Module::new(
                    module_id,
                    course_id.to_string(),
                    canvas_module.name.clone(),
                    None, // No description from Canvas API
                    canvas_module.position,
                );
                
                module.published = canvas_module.published;
                module.unlock_at = canvas_module.unlock_at.map(|dt| dt.to_rfc3339());
                module.canvas_module_id = Some(canvas_module_id);
                
                module.create(&self.db).await?;
                created_modules += 1;
                
                canvas_to_local_id.insert(canvas_module.id, module_id);
            }
        }
        
        // Second pass: Set prerequisites and sync items
        for canvas_module in &canvas_modules {
            let module_id = canvas_to_local_id.get(&canvas_module.id).unwrap();
            let mut module = Module::find(&self.db, *module_id).await?;
            
            // Handle prerequisites
            if let Some(prereq_id) = canvas_module.prerequisite_module_ids.first() {
                if let Some(local_prereq_id) = canvas_to_local_id.get(prereq_id) {
                    module.prerequisite_module_id = Some(local_prereq_id.to_string());
                    module.update(&self.db).await?;
                }
            }
            
            // Now sync module items
            let canvas_items = self.fetch_canvas_module_items(canvas_course_id, canvas_module.id).await?;
            
            for canvas_item in canvas_items {
                let canvas_item_id = canvas_item.id.to_string();
                
                if let Some(existing_item) = existing_items_map.get(&canvas_item_id) {
                    // Update existing item
                    let mut item = existing_item.clone();
                    item.title = canvas_item.title.clone();
                    item.position = canvas_item.position;
                    item.published = canvas_item.published;
                    item.external_url = canvas_item.external_url.clone();
                    
                    // Map Canvas content types to our internal types
                    if let Some(content_type) = &canvas_item.content_type {
                        item.item_type = match content_type.as_str() {
                            "Assignment" => "Assignment".to_string(),
                            "Quiz" => "Quiz".to_string(),
                            "Discussion" => "Discussion".to_string(),
                            "ExternalUrl" => "ExternalUrl".to_string(),
                            "Page" => "Page".to_string(),
                            "File" => "Resource".to_string(),
                            _ => "Resource".to_string(),
                        };
                    }
                    
                    item.update(&self.db).await?;
                    updated_items += 1;
                } else {
                    // Create new item
                    let item_type = if let Some(content_type) = &canvas_item.content_type {
                        match content_type.as_str() {
                            "Assignment" => "Assignment",
                            "Quiz" => "Quiz",
                            "Discussion" => "Discussion",
                            "ExternalUrl" => "ExternalUrl",
                            "Page" => "Page",
                            "File" => "Resource",
                            _ => "Resource",
                        }
                    } else if canvas_item.external_url.is_some() {
                        "ExternalUrl"
                    } else {
                        "Resource"
                    };
                    
                    let mut item = ModuleItem::new(
                        Uuid::new_v4(),
                        module_id.to_string(),
                        canvas_item.title.clone(),
                        item_type.to_string(),
                        canvas_item.position,
                    );
                    
                    item.published = canvas_item.published;
                    item.external_url = canvas_item.external_url.clone();
                    item.canvas_item_id = Some(canvas_item_id);
                    
                    // If we have a content ID from Canvas, store it
                    if let Some(content_id) = canvas_item.content_id {
                        item.canvas_content_id = Some(content_id.to_string());
                    }
                    
                    item.create(&self.db).await?;
                    created_items += 1;
                }
            }
        }
        
        Ok(format!(
            "Sync completed: {} modules created, {} modules updated, {} items created, {} items updated",
            created_modules, updated_modules, created_items, updated_items
        ))
    }
    
    /// Push a module to Canvas
    pub async fn push_module_to_canvas(&self, module_id: &str) -> Result<String, Error> {
        // Load module and course
        let module = Module::find(&self.db, Uuid::parse_str(module_id)?).await?;
        let course = Course::find(&self.db, Uuid::parse_str(&module.course_id)?).await?;
        
        // Make sure we have a Canvas course ID
        let canvas_course_id = match &course.canvas_course_id {
            Some(id) => id,
            None => return Err(Error::Integration("Course is not linked to Canvas".to_string())),
        };
        
        // Check if module already exists in Canvas
        if let Some(canvas_module_id) = &module.canvas_module_id {
            // Update existing module
            let url = format!("/api/v1/courses/{}/modules/{}", canvas_course_id, canvas_module_id);
            
            let mut update_data = serde_json::Map::new();
            update_data.insert("name".to_string(), serde_json::Value::String(module.name.clone()));
            update_data.insert("position".to_string(), serde_json::Value::Number(serde_json::Number::from(module.position)));
            update_data.insert("published".to_string(), serde_json::Value::Bool(module.published));
            
            if let Some(unlock_at) = &module.unlock_at {
                update_data.insert("unlock_at".to_string(), serde_json::Value::String(unlock_at.clone()));
            }
            
            self.canvas_service.put::<_, CanvasModule>(&url, &update_data).await?;
            
            // Now sync items
            let items = ModuleItem::find_all_by_module(&self.db, module_id).await?;
            
            for item in &items {
                await self.push_module_item_to_canvas(item).await?;
            }
            
            Ok(format!("Module '{}' updated in Canvas with {} items", module.name, items.len()))
        } else {
            // Create new module in Canvas
            let url = format!("/api/v1/courses/{}/modules", canvas_course_id);
            
            let mut create_data = serde_json::Map::new();
            create_data.insert("name".to_string(), serde_json::Value::String(module.name.clone()));
            create_data.insert("position".to_string(), serde_json::Value::Number(serde_json::Number::from(module.position)));
            create_data.insert("published".to_string(), serde_json::Value::Bool(module.published));
            
            if let Some(unlock_at) = &module.unlock_at {
                create_data.insert("unlock_at".to_string(), serde_json::Value::String(unlock_at.clone()));
            }
            
            // If we have a prerequisite module that has a Canvas ID, set it
            if let Some(prereq_id) = &module.prerequisite_module_id {
                if let Ok(prereq_module) = Module::find(&self.db, Uuid::parse_str(prereq_id)?).await {
                    if let Some(canvas_prereq_id) = &prereq_module.canvas_module_id {
                        let prereq_array = vec![serde_json::Value::String(canvas_prereq_id.clone())];
                        create_data.insert("prerequisite_module_ids".to_string(), serde_json::Value::Array(prereq_array));
                    }
                }
            }
            
            let canvas_module = self.canvas_service.post::<_, CanvasModule>(&url, &create_data).await?;
            
            // Update our module with the Canvas ID
            let mut updated_module = module.clone();
            updated_module.canvas_module_id = Some(canvas_module.id.to_string());
            updated_module.update(&self.db).await?;
            
            // Now create all the items
            let items = ModuleItem::find_all_by_module(&self.db, module_id).await?;
            let mut created_items = 0;
            
            for item in &items {
                await self.push_module_item_to_canvas(item).await?;
                created_items += 1;
            }
            
            Ok(format!("Module '{}' created in Canvas with {} items", module.name, created_items))
        }
    }
    
    /// Push a module item to Canvas
    pub async fn push_module_item_to_canvas(&self, item: &ModuleItem) -> Result<(), Error> {
        // Get the module for this item to find the course
        let module = Module::find(&self.db, Uuid::parse_str(&item.module_id)?).await?;
        let course = Course::find(&self.db, Uuid::parse_str(&module.course_id)?).await?;
        
        // Make sure we have Canvas IDs for both course and module
        let canvas_course_id = match &course.canvas_course_id {
            Some(id) => id,
            None => return Err(Error::Integration("Course is not linked to Canvas".to_string())),
        };
        
        let canvas_module_id = match &module.canvas_module_id {
            Some(id) => id,
            None => return Err(Error::Integration("Module is not linked to Canvas".to_string())),
        };
        
        if let Some(canvas_item_id) = &item.canvas_item_id {
            // Update existing item
            let url = format!(
                "/api/v1/courses/{}/modules/{}/items/{}", 
                canvas_course_id, canvas_module_id, canvas_item_id
            );
            
            let mut update_data = serde_json::Map::new();
            update_data.insert("title".to_string(), serde_json::Value::String(item.title.clone()));
            update_data.insert("position".to_string(), serde_json::Value::Number(serde_json::Number::from(item.position)));
            update_data.insert("published".to_string(), serde_json::Value::Bool(item.published));
            
            if let Some(url) = &item.external_url {
                update_data.insert("external_url".to_string(), serde_json::Value::String(url.clone()));
            }
            
            self.canvas_service.put::<_, CanvasModuleItem>(&url, &update_data).await?;
            Ok(())
        } else {
            // Create new item
            let url = format!(
                "/api/v1/courses/{}/modules/{}/items", 
                canvas_course_id, canvas_module_id
            );
            
            let mut create_data = serde_json::Map::new();
            create_data.insert("title".to_string(), serde_json::Value::String(item.title.clone()));
            create_data.insert("position".to_string(), serde_json::Value::Number(serde_json::Number::from(item.position)));
            
            // Handle different content types
            match item.item_type.as_str() {
                "ExternalUrl" => {
                    if let Some(ext_url) = &item.external_url {
                        create_data.insert("type".to_string(), serde_json::Value::String("ExternalUrl".to_string()));
                        create_data.insert("external_url".to_string(), serde_json::Value::String(ext_url.clone()));
                    } else {
                        return Err(Error::Integration("External URL item is missing URL".to_string()));
                    }
                },
                "Assignment" => {
                    if let Some(content_id) = &item.content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Assignment".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(content_id.clone()));
                    } else if let Some(canvas_content_id) = &item.canvas_content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Assignment".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(canvas_content_id.clone()));
                    } else {
                        return Err(Error::Integration("Assignment item is missing content ID".to_string()));
                    }
                },
                "Quiz" => {
                    if let Some(content_id) = &item.content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Quiz".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(content_id.clone()));
                    } else if let Some(canvas_content_id) = &item.canvas_content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Quiz".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(canvas_content_id.clone()));
                    } else {
                        return Err(Error::Integration("Quiz item is missing content ID".to_string()));
                    }
                },
                "Discussion" => {
                    if let Some(content_id) = &item.content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Discussion".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(content_id.clone()));
                    } else if let Some(canvas_content_id) = &item.canvas_content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Discussion".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(canvas_content_id.clone()));
                    } else {
                        return Err(Error::Integration("Discussion item is missing content ID".to_string()));
                    }
                },
                "Page" => {
                    if let Some(content_id) = &item.content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Page".to_string()));
                        create_data.insert("page_url".to_string(), serde_json::Value::String(content_id.clone()));
                    } else if let Some(canvas_content_id) = &item.canvas_content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("Page".to_string()));
                        create_data.insert("page_url".to_string(), serde_json::Value::String(canvas_content_id.clone()));
                    } else {
                        return Err(Error::Integration("Page item is missing content ID".to_string()));
                    }
                },
                "Resource" => {
                    if let Some(content_id) = &item.content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("File".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(content_id.clone()));
                    } else if let Some(canvas_content_id) = &item.canvas_content_id {
                        create_data.insert("type".to_string(), serde_json::Value::String("File".to_string()));
                        create_data.insert("content_id".to_string(), serde_json::Value::String(canvas_content_id.clone()));
                    } else {
                        return Err(Error::Integration("Resource item is missing content ID".to_string()));
                    }
                },
                _ => {
                    return Err(Error::Integration(format!("Unsupported item type: {}", item.item_type)));
                }
            }
            
            let canvas_item = self.canvas_service.post::<_, CanvasModuleItem>(&url, &create_data).await?;
            
            // Update our item with the Canvas ID
            let mut updated_item = item.clone();
            updated_item.canvas_item_id = Some(canvas_item.id.to_string());
            updated_item.update(&self.db).await?;
            
            Ok(())
        }
    }
}