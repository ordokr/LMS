use crate::db::DB;
use crate::models::course::module::{Module, ModuleRequest};
use crate::models::course::module_item::{ModuleItem, ModuleItemRequest};
use crate::services::integration::canvas_module_sync::CanvasModuleSync;
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::error::Error;

use tauri::{command, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct CanvasSyncResponse {
    message: String,
    created_modules: usize,
    updated_modules: usize,
    created_items: usize,
    updated_items: usize,
}

#[command]
pub async fn get_course_modules(
    course_id: String,
    db: State<'_, DB>,
) -> Result<Vec<Module>, String> {
    Module::find_all_by_course(&db, &course_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_module(
    module_id: String,
    db: State<'_, DB>,
) -> Result<Module, String> {
    Module::find(&db, Uuid::parse_str(&module_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_module(
    request: ModuleRequest,
    db: State<'_, DB>,
) -> Result<Module, String> {
    let module_id = Uuid::new_v4();
    let mut module = Module::new(
        module_id,
        request.course_id,
        request.name,
        request.description,
        request.position.unwrap_or(0),
    );
    
    module.published = request.published.unwrap_or(false);
    module.prerequisite_module_id = request.prerequisite_module_id;
    module.unlock_at = request.unlock_at;
    
    module.create(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(module)
}

#[command]
pub async fn update_module(
    module_id: String,
    request: ModuleRequest,
    db: State<'_, DB>,
) -> Result<Module, String> {
    let mut module = Module::find(&db, Uuid::parse_str(&module_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    
    module.name = request.name;
    module.description = request.description;
    if let Some(position) = request.position {
        module.position = position;
    }
    module.prerequisite_module_id = request.prerequisite_module_id;
    module.unlock_at = request.unlock_at;
    if let Some(published) = request.published {
        module.published = published;
    }
    
    module.update(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(module)
}

#[command]
pub async fn delete_module(
    module_id: String,
    db: State<'_, DB>,
) -> Result<(), String> {
    let module = Module::find(&db, Uuid::parse_str(&module_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    
    module.delete(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub async fn reorder_modules(
    course_id: String,
    module_ids: Vec<String>,
    db: State<'_, DB>,
) -> Result<(), String> {
    for (position, module_id) in module_ids.iter().enumerate() {
        let mut module = Module::find(&db, Uuid::parse_str(module_id).map_err(|e| e.to_string())?)
            .await
            .map_err(|e| e.to_string())?;
        
        // Only update if position actually changed
        if module.position != position as i32 {
            module.position = position as i32;
            module.update(&db).await.map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

#[command]
pub async fn get_module_items(
    module_id: String,
    db: State<'_, DB>,
) -> Result<Vec<ModuleItem>, String> {
    ModuleItem::find_all_by_module(&db, &module_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_module_item(
    item_id: String,
    db: State<'_, DB>,
) -> Result<ModuleItem, String> {
    ModuleItem::find(&db, Uuid::parse_str(&item_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_module_item(
    request: ModuleItemRequest,
    db: State<'_, DB>,
) -> Result<ModuleItem, String> {
    // Get the highest position number for the module to append this item
    let module_items = ModuleItem::find_all_by_module(&db, &request.module_id)
        .await
        .map_err(|e| e.to_string())?;
    
    let position = request.position.unwrap_or_else(|| {
        if let Some(max_pos) = module_items.iter().map(|item| item.position).max() {
            max_pos + 1
        } else {
            0
        }
    });
    
    let item_id = Uuid::new_v4();
    let mut item = ModuleItem::new(
        item_id,
        request.module_id,
        request.title,
        request.item_type,
        position,
    );
    
    item.content_id = request.content_id;
    item.external_url = request.external_url;
    item.published = request.published.unwrap_or(true);
    
    item.create(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(item)
}

#[command]
pub async fn update_module_item(
    item_id: String,
    request: ModuleItemRequest,
    db: State<'_, DB>,
) -> Result<ModuleItem, String> {
    let mut item = ModuleItem::find(&db, Uuid::parse_str(&item_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    
    item.title = request.title;
    item.item_type = request.item_type;
    item.content_id = request.content_id;
    item.external_url = request.external_url;
    
    if let Some(position) = request.position {
        item.position = position;
    }
    
    if let Some(published) = request.published {
        item.published = published;
    }
    
    item.update(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(item)
}

#[command]
pub async fn delete_module_item(
    item_id: String,
    db: State<'_, DB>,
) -> Result<(), String> {
    let item = ModuleItem::find(&db, Uuid::parse_str(&item_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    
    item.delete(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub async fn reorder_module_items(
    module_id: String,
    item_ids: Vec<String>,
    db: State<'_, DB>,
) -> Result<(), String> {
    for (position, item_id) in item_ids.iter().enumerate() {
        let mut item = ModuleItem::find(&db, Uuid::parse_str(item_id).map_err(|e| e.to_string())?)
            .await
            .map_err(|e| e.to_string())?;
        
        // Only update if position actually changed
        if item.position != position as i32 {
            item.position = position as i32;
            item.update(&db).await.map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

#[command]
pub async fn sync_canvas_modules(
    course_id: String,
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
) -> Result<String, String> {
    // Get course to find Canvas course ID
    let course = crate::models::course::course::Course::find(&db, Uuid::parse_str(&course_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    
    // Check if course has Canvas ID
    let canvas_course_id = match course.canvas_course_id {
        Some(id) => id,
        None => return Err("Course is not connected to Canvas. Please connect it first.".to_string()),
    };
    
    // Create sync service and run sync
    let sync = CanvasModuleSync::new(db.inner().clone(), canvas_service.inner().as_ref().clone());
    
    match sync.sync_modules_from_canvas(&course_id, &canvas_course_id).await {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("Failed to sync modules: {}", e)),
    }
}

#[command]
pub async fn push_module_to_canvas(
    module_id: String,
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
) -> Result<String, String> {
    // Create sync service
    let sync = CanvasModuleSync::new(db.inner().clone(), canvas_service.inner().as_ref().clone());
    
    match sync.push_module_to_canvas(&module_id).await {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("Failed to push module to Canvas: {}", e)),
    }
}

#[command]
pub async fn push_module_item_to_canvas(
    item_id: String,
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
) -> Result<String, String> {
    // Load the item
    let item = ModuleItem::find(&db, Uuid::parse_str(&item_id).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;
    
    // Create sync service
    let sync = CanvasModuleSync::new(db.inner().clone(), canvas_service.inner().as_ref().clone());
    
    match sync.push_module_item_to_canvas(&item).await {
        Ok(_) => Ok(format!("Item '{}' pushed to Canvas successfully", item.title)),
        Err(e) => Err(format!("Failed to push item to Canvas: {}", e)),
    }
}

#[command]
pub async fn get_available_content(
    course_id: String,
    content_type: String,
    db: State<'_, DB>,
) -> Result<Vec<serde_json::Value>, String> {
    // Fetch available content of the requested type
    // This would need to be implemented based on your data model
    // For example, fetch assignments, quizzes, etc.
    
    match content_type.as_str() {
        "Assignment" => {
            // Fetch assignments for the course
            let assignments = crate::models::content::assignment::Assignment::find_by_course(&db, &course_id)
                .await
                .map_err(|e| e.to_string())?;
            
            Ok(assignments.into_iter().map(|a| {
                let mut obj = serde_json::Map::new();
                obj.insert("id".to_string(), serde_json::Value::String(a.id.to_string()));
                obj.insert("title".to_string(), serde_json::Value::String(a.title));
                obj.insert("content_type".to_string(), serde_json::Value::String("Assignment".to_string()));
                serde_json::Value::Object(obj)
            }).collect())
        },
        "Quiz" => {
            // Fetch quizzes for the course
            let quizzes = crate::models::content::quiz::Quiz::find_by_course(&db, &course_id)
                .await
                .map_err(|e| e.to_string())?;
            
            Ok(quizzes.into_iter().map(|q| {
                let mut obj = serde_json::Map::new();
                obj.insert("id".to_string(), serde_json::Value::String(q.id.to_string()));
                obj.insert("title".to_string(), serde_json::Value::String(q.title));
                obj.insert("content_type".to_string(), serde_json::Value::String("Quiz".to_string()));
                serde_json::Value::Object(obj)
            }).collect())
        },
        "Discussion" => {
            // Fetch discussions for the course
            let discussions = crate::models::forum::topic::Topic::find_by_course(&db, &course_id)
                .await
                .map_err(|e| e.to_string())?;
            
            Ok(discussions.into_iter().map(|d| {
                let mut obj = serde_json::Map::new();
                obj.insert("id".to_string(), serde_json::Value::String(d.id.to_string()));
                obj.insert("title".to_string(), serde_json::Value::String(d.title));
                obj.insert("content_type".to_string(), serde_json::Value::String("Discussion".to_string()));
                serde_json::Value::Object(obj)
            }).collect())
        },
        "Page" => {
            // Fetch pages for the course
            let pages = crate::models::content::page::Page::find_by_course(&db, &course_id)
                .await
                .map_err(|e| e.to_string())?;
            
            Ok(pages.into_iter().map(|p| {
                let mut obj = serde_json::Map::new();
                obj.insert("id".to_string(), serde_json::Value::String(p.id.to_string()));
                obj.insert("title".to_string(), serde_json::Value::String(p.title));
                obj.insert("content_type".to_string(), serde_json::Value::String("Page".to_string()));
                serde_json::Value::Object(obj)
            }).collect())
        },
        "Resource" => {
            // Fetch resources/files for the course
            let resources = crate::models::content::resource::Resource::find_by_course(&db, &course_id)
                .await
                .map_err(|e| e.to_string())?;
            
            Ok(resources.into_iter().map(|r| {
                let mut obj = serde_json::Map::new();
                obj.insert("id".to_string(), serde_json::Value::String(r.id.to_string()));
                obj.insert("title".to_string(), serde_json::Value::String(r.filename));
                obj.insert("content_type".to_string(), serde_json::Value::String("Resource".to_string()));
                if let Some(url) = r.url {
                    obj.insert("url".to_string(), serde_json::Value::String(url));
                }
                serde_json::Value::Object(obj)
            }).collect())
        },
        _ => Err(format!("Unsupported content type: {}", content_type)),
    }
}

// Update our main.rs to include this module and register the commands
// Inside main.rs, add to Tauri's invoke_handler:
// .invoke_handler(tauri::generate_handler![
//     // Existing commands...
//     module_commands::get_course_modules,
//     module_commands::get_module,
//     module_commands::create_module,
//     module_commands::update_module,
//     module_commands::delete_module,
//     module_commands::reorder_modules,
//     module_commands::get_module_items,
//     module_commands::get_module_item,
//     module_commands::create_module_item,
//     module_commands::update_module_item,
//     module_commands::delete_module_item,
//     module_commands::reorder_module_items,
//     module_commands::sync_canvas_modules,
//     module_commands::push_module_to_canvas,
//     module_commands::push_module_item_to_canvas,
//     module_commands::get_available_content,
// ])