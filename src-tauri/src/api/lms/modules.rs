use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::core::auth::Claims;
use crate::core::errors::AppError;
use crate::database::repositories::ModuleRepository;
use crate::lms::models::{Module, ModuleItem};

// Create a new module
pub async fn create_module(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Json(module): Json<Module>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Create the module
    let module_id = module_repo.create_module(user_id, module).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "module_id": module_id,
        "message": "Module created successfully"
    }))))
}

// Get modules for a course
pub async fn get_course_modules(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(course_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Get the modules
    let modules = module_repo.get_modules_by_course(course_id).await?;
    
    Ok((StatusCode::OK, Json(modules)))
}

// Get a module by ID
pub async fn get_module(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(module_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Get the module
    let module = module_repo.get_module_by_id(module_id).await?;
    
    // Get module items
    let items = module_repo.get_module_items(module_id).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "module": module,
        "items": items,
    }))))
}

// Update a module
pub async fn update_module(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(module_id): Path<i64>,
    Json(module): Json<Module>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Update the module
    module_repo.update_module(module_id, user_id, module).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Module updated successfully"
    }))))
}

// Delete a module
pub async fn delete_module(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(module_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Delete the module
    module_repo.delete_module(module_id, user_id).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Module deleted successfully"
    }))))
}

// Create a module item
pub async fn create_module_item(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(module_id): Path<i64>,
    Json(item): Json<ModuleItem>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Create the module item
    let item_id = module_repo.create_module_item(module_id, user_id, item).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "item_id": item_id,
        "message": "Module item created successfully"
    }))))
}

// Update module item
pub async fn update_module_item(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path((module_id, item_id)): Path<(i64, i64)>,
    Json(item): Json<ModuleItem>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Update the module item
    module_repo.update_module_item(module_id, item_id, user_id, item).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Module item updated successfully"
    }))))
}

// Delete module item
pub async fn delete_module_item(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path((module_id, item_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Delete the module item
    module_repo.delete_module_item(module_id, item_id, user_id).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Module item deleted successfully"
    }))))
}

// Reorder modules
pub async fn reorder_modules(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(course_id): Path<i64>,
    Json(order): Json<Vec<i64>>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Reorder the modules
    module_repo.reorder_modules(course_id, user_id, order).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Modules reordered successfully"
    }))))
}

// Reorder module items
pub async fn reorder_module_items(
    claims: Claims,
    State(module_repo): State<Arc<ModuleRepository>>,
    Path(module_id): Path<i64>,
    Json(order): Json<Vec<i64>>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Reorder the module items
    module_repo.reorder_module_items(module_id, user_id, order).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Module items reordered successfully"
    }))))
}