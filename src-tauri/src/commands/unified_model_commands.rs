use crate::models::unified::{User, Course, Assignment, Discussion, Notification};
use crate::repositories::unified::UserRepository;
use crate::services::sync::UserSyncService;
use crate::AppState;
use tauri::{AppHandle, State, Manager};
use log::{info, error};
use std::sync::Arc;
use serde_json::Value as JsonValue;

#[tauri::command]
pub async fn get_user(
    state: State<'_, AppState>,
    id: String,
) -> Result<User, String> {
    info!("Getting user with ID: {}", id);
    
    let user_repo = state.user_repository.clone();
    
    match user_repo.find_by_id(&id).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(format!("User with ID {} not found", id)),
        Err(e) => {
            error!("Error fetching user: {}", e);
            Err(format!("Failed to fetch user: {}", e))
        }
    }
}

#[tauri::command]
pub async fn sync_user(
    state: State<'_, AppState>,
    id: String,
) -> Result<User, String> {
    info!("Syncing user with ID: {}", id);
    
    let user_sync_service = state.user_sync_service.clone();
    
    match user_sync_service.sync_user(&id).await {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Error syncing user: {}", e);
            Err(format!("Failed to sync user: {}", e))
        }
    }
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    user_data: JsonValue,
) -> Result<User, String> {
    info!("Creating new user");
    
    // Convert JSON to User model
    let user: User = match serde_json::from_value(user_data) {
        Ok(user) => user,
        Err(e) => {
            error!("Error deserializing user: {}", e);
            return Err(format!("Invalid user data: {}", e));
        }
    };
    
    let user_repo = state.user_repository.clone();
    
    match user_repo.create(&user).await {
        Ok(created_user) => Ok(created_user),
        Err(e) => {
            error!("Error creating user: {}", e);
            Err(format!("Failed to create user: {}", e))
        }
    }
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, AppState>,
    user_data: JsonValue,
) -> Result<User, String> {
    // Convert JSON to User model
    let user: User = match serde_json::from_value(user_data) {
        Ok(user) => user,
        Err(e) => return Err(format!("Invalid user data: {}", e)),
    };
    
    info!("Updating user with ID: {}", user.id);
    
    let user_repo = state.user_repository.clone();
    
    match user_repo.update(&user).await {
        Ok(updated_user) => Ok(updated_user),
        Err(e) => {
            error!("Error updating user: {}", e);
            Err(format!("Failed to update user: {}", e))
        }
    }
}

#[tauri::command]
pub async fn delete_user(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    info!("Deleting user with ID: {}", id);
    
    let user_repo = state.user_repository.clone();
    
    match user_repo.delete(&id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error deleting user: {}", e);
            Err(format!("Failed to delete user: {}", e))
        }
    }
}

// Similar commands for other model types would follow the same pattern