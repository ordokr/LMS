use crate::models::module::{Module, ModuleCreate, ModuleUpdate, ModuleItem, ModuleItemCreate, ModuleItemUpdate, ModuleStatus};
use crate::db::module_repository::ModuleRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

/// Gets modules for a course
///
/// # Arguments
/// * `course_id` - ID of the course
///
/// # Returns
/// * `Vec<Module>` - List of modules in the course
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn get_modules(
    course_id: String,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Vec<Module>, String> {
    info!(event = "api_call", endpoint = "get_modules", course_id = %course_id);
    
    match module_repo.get_modules_for_course(&course_id).await {
        Ok(modules) => {
            info!(
                event = "api_success", 
                endpoint = "get_modules",
                course_id = %course_id,
                module_count = modules.len()
            );
            Ok(modules)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_modules", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets a specific module by ID
///
/// # Arguments
/// * `module_id` - ID of the module
///
/// # Returns
/// * `Module` - The requested module
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn get_module(
    module_id: String,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Module, String> {
    info!(event = "api_call", endpoint = "get_module", module_id = %module_id);
    
    match module_repo.get_module(&module_id).await {
        Ok(Some(module)) => {
            info!(event = "api_success", endpoint = "get_module", module_id = %module_id);
            Ok(module)
        },
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "get_module", module_id = %module_id);
            Err(format!("Module not found with ID: {}", module_id))
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_module", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new module
///
/// # Arguments
/// * `module_create` - Module creation data
///
/// # Returns
/// * `Module` - The created module
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn create_module(
    module_create: ModuleCreate,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Module, String> {
    info!(
        event = "api_call", 
        endpoint = "create_module",
        course_id = %module_create.course_id,
        title = %module_create.title
    );
    
    // Generate a unique ID
    let module_id = Uuid::new_v4().to_string();
    
    // Set default values
    let published = module_create.published.unwrap_or(true);
    let publish_final_grade = module_create.publish_final_grade.unwrap_or(false);
    
    // Determine position if not provided
    let position = match module_create.position {
        Some(pos) => pos,
        None => {
            // Get count of existing modules to set position to the end
            match module_repo.get_modules_count(&module_create.course_id).await {
                Ok(count) => count + 1,
                Err(e) => {
                    error!(
                        event = "api_error", 
                        endpoint = "create_module", 
                        error = %e,
                        message = "Failed to get modules count, defaulting to position 1"
                    );
                    1
                }
            }
        }
    };
    
    let now = chrono::Utc::now().to_rfc3339();
    
    // Create the module
    let module = Module {
        id: module_id,
        course_id: module_create.course_id,
        title: module_create.title,
        position,
        items_count: 0,
        publish_final_grade,
        published,
        status: if published { ModuleStatus::Active } else { ModuleStatus::Unpublished },
        created_at: now.clone(),
        updated_at: now,
    };
    
    match module_repo.create_module(module).await {
        Ok(created) => {
            info!(
                event = "api_success", 
                endpoint = "create_module", 
                module_id = %created.id,
                course_id = %created.course_id
            );
            Ok(created)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "create_module", error = %e);
            Err(format!("Failed to create module: {}", e))
        }
    }
}

/// Updates a module
///
/// # Arguments
/// * `module_id` - ID of the module to update
/// * `module_update` - Module update data
///
/// # Returns
/// * `Module` - The updated module
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn update_module(
    module_id: String,
    module_update: ModuleUpdate,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Module, String> {
    info!(
        event = "api_call", 
        endpoint = "update_module",
        module_id = %module_id
    );
    
    // First check if module exists
    let existing = match module_repo.get_module(&module_id).await {
        Ok(Some(module)) => module,
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "update_module", module_id = %module_id);
            return Err(format!("Module not found with ID: {}", module_id));
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_module", error = %e);
            return Err(format!("Database error: {}", e));
        }
    };
    
    // Update the module
    match module_repo.update_module(&module_id, module_update).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_module", module_id = %module_id);
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_module", error = %e);
            Err(format!("Failed to update module: {}", e))
        }
    }
}

/// Deletes a module
///
/// # Arguments
/// * `module_id` - ID of the module to delete
///
/// # Returns
/// * `bool` - Whether the deletion was successful
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn delete_module(
    module_id: String,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<bool, String> {
    info!(event = "api_call", endpoint = "delete_module", module_id = %module_id);
    
    match module_repo.delete_module(&module_id).await {
        Ok(deleted) => {
            info!(
                event = "api_success", 
                endpoint = "delete_module",
                module_id = %module_id,
                deleted = deleted
            );
            Ok(deleted)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "delete_module", error = %e);
            Err(format!("Failed to delete module: {}", e))
        }
    }
}

/// Reorders modules within a course
///
/// # Arguments
/// * `course_id` - ID of the course
/// * `module_ids` - Ordered list of module IDs
///
/// # Returns
/// * `Vec<Module>` - The updated modules
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn reorder_modules(
    course_id: String,
    module_ids: Vec<String>,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Vec<Module>, String> {
    info!(
        event = "api_call", 
        endpoint = "reorder_modules",
        course_id = %course_id,
        module_count = module_ids.len()
    );
    
    if module_ids.is_empty() {
        return Err("Module IDs list cannot be empty".to_string());
    }
    
    match module_repo.reorder_modules(&course_id, &module_ids).await {
        Ok(modules) => {
            info!(
                event = "api_success", 
                endpoint = "reorder_modules",
                course_id = %course_id,
                module_count = modules.len()
            );
            Ok(modules)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "reorder_modules", error = %e);
            Err(format!("Failed to reorder modules: {}", e))
        }
    }
}

/// Gets items for a module
///
/// # Arguments
/// * `module_id` - ID of the module
///
/// # Returns
/// * `Vec<ModuleItem>` - List of items in the module
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn get_module_items(
    module_id: String,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Vec<ModuleItem>, String> {
    info!(event = "api_call", endpoint = "get_module_items", module_id = %module_id);
    
    match module_repo.get_module_items(&module_id).await {
        Ok(items) => {
            info!(
                event = "api_success", 
                endpoint = "get_module_items",
                module_id = %module_id,
                item_count = items.len()
            );
            Ok(items)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_module_items", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets a specific module item
///
/// # Arguments
/// * `item_id` - ID of the module item
///
/// # Returns
/// * `ModuleItem` - The requested module item
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn get_module_item(
    item_id: String,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<ModuleItem, String> {
    info!(event = "api_call", endpoint = "get_module_item", item_id = %item_id);
    
    match module_repo.get_module_item(&item_id).await {
        Ok(Some(item)) => {
            info!(event = "api_success", endpoint = "get_module_item", item_id = %item_id);
            Ok(item)
        },
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "get_module_item", item_id = %item_id);
            Err(format!("Module item not found with ID: {}", item_id))
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_module_item", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new module item
///
/// # Arguments
/// * `item_create` - Module item creation data
///
/// # Returns
/// * `ModuleItem` - The created module item
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn create_module_item(
    item_create: ModuleItemCreate,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<ModuleItem, String> {
    info!(
        event = "api_call", 
        endpoint = "create_module_item",
        module_id = %item_create.module_id,
        title = %item_create.title
    );
    
    // Generate a unique ID
    let item_id = Uuid::new_v4().to_string();
    
    // Set default values
    let published = item_create.published.unwrap_or(true);
    
    // Determine position if not provided
    let position = match item_create.position {
        Some(pos) => pos,
        None => {
            // Get count of existing items to set position to the end
            match module_repo.get_module_items_count(&item_create.module_id).await {
                Ok(count) => count + 1,
                Err(e) => {
                    error!(
                        event = "api_error", 
                        endpoint = "create_module_item", 
                        error = %e,
                        message = "Failed to get items count, defaulting to position 1"
                    );
                    1
                }
            }
        }
    };
    
    let now = chrono::Utc::now().to_rfc3339();
    
    // Create the module item
    let item = ModuleItem {
        id: item_id,
        module_id: item_create.module_id,
        title: item_create.title,
        position,
        item_type: item_create.item_type,
        content_id: item_create.content_id,
        content_type: item_create.content_type,
        url: item_create.url,
        page_url: item_create.page_url,
        published,
        created_at: now.clone(),
        updated_at: now,
    };
    
    match module_repo.create_module_item(item).await {
        Ok(created) => {
            info!(
                event = "api_success", 
                endpoint = "create_module_item", 
                item_id = %created.id,
                module_id = %created.module_id
            );
            Ok(created)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "create_module_item", error = %e);
            Err(format!("Failed to create module item: {}", e))
        }
    }
}

/// Updates a module item
///
/// # Arguments
/// * `item_id` - ID of the module item to update
/// * `item_update` - Module item update data
///
/// # Returns
/// * `ModuleItem` - The updated module item
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn update_module_item(
    item_id: String,
    item_update: ModuleItemUpdate,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<ModuleItem, String> {
    info!(
        event = "api_call", 
        endpoint = "update_module_item",
        item_id = %item_id
    );
    
    // First check if item exists
    let existing = match module_repo.get_module_item(&item_id).await {
        Ok(Some(item)) => item,
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "update_module_item", item_id = %item_id);
            return Err(format!("Module item not found with ID: {}", item_id));
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_module_item", error = %e);
            return Err(format!("Database error: {}", e));
        }
    };
    
    // Update the module item
    match module_repo.update_module_item(&item_id, item_update).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_module_item", item_id = %item_id);
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_module_item", error = %e);
            Err(format!("Failed to update module item: {}", e))
        }
    }
}

/// Deletes a module item
///
/// # Arguments
/// * `item_id` - ID of the module item to delete
///
/// # Returns
/// * `bool` - Whether the deletion was successful
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn delete_module_item(
    item_id: String,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<bool, String> {
    info!(event = "api_call", endpoint = "delete_module_item", item_id = %item_id);
    
    match module_repo.delete_module_item(&item_id).await {
        Ok(deleted) => {
            info!(
                event = "api_success", 
                endpoint = "delete_module_item",
                item_id = %item_id,
                deleted = deleted
            );
            Ok(deleted)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "delete_module_item", error = %e);
            Err(format!("Failed to delete module item: {}", e))
        }
    }
}

/// Reorders items within a module
///
/// # Arguments
/// * `module_id` - ID of the module
/// * `item_ids` - Ordered list of item IDs
///
/// # Returns
/// * `Vec<ModuleItem>` - The updated module items
#[tauri::command]
#[instrument(skip(module_repo), err)]
pub async fn reorder_module_items(
    module_id: String,
    item_ids: Vec<String>,
    module_repo: State<'_, Arc<dyn ModuleRepository + Send + Sync>>
) -> Result<Vec<ModuleItem>, String> {
    info!(
        event = "api_call", 
        endpoint = "reorder_module_items",
        module_id = %module_id,
        item_count = item_ids.len()
    );
    
    if item_ids.is_empty() {
        return Err("Item IDs list cannot be empty".to_string());
    }
    
    match module_repo.reorder_module_items(&module_id, &item_ids).await {
        Ok(items) => {
            info!(
                event = "api_success", 
                endpoint = "reorder_module_items",
                module_id = %module_id,
                item_count = items.len()
            );
            Ok(items)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "reorder_module_items", error = %e);
            Err(format!("Failed to reorder module items: {}", e))
        }
    }
}