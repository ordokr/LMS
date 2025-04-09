use crate::models::user::{UserProfile, UserProfileUpdate};
use crate::db::user_repository::UserRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};

/// Gets the profile for a specific user
///
/// # Arguments
/// * `user_id` - ID of the user to retrieve
///
/// # Returns
/// * `UserProfile` - The user's profile
#[tauri::command]
#[instrument(skip(user_repo), err)]
pub async fn get_user_profile(
    user_id: String,
    user_repo: State<'_, Arc<dyn UserRepository + Send + Sync>>
) -> Result<UserProfile, String> {
    info!(event = "api_call", endpoint = "get_user_profile", user_id = %user_id);
    
    match user_repo.get_user_profile(&user_id).await {
        Ok(Some(profile)) => {
            info!(event = "api_success", endpoint = "get_user_profile", user_id = %user_id);
            Ok(profile)
        },
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "get_user_profile", user_id = %user_id);
            Err(format!("User not found with ID: {}", user_id))
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_user_profile", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Updates a user's profile
///
/// # Arguments
/// * `user_id` - ID of the user to update
/// * `profile_update` - Profile data to update
///
/// # Returns
/// * `UserProfile` - The updated profile
#[tauri::command]
#[instrument(skip(user_repo), err)]
pub async fn update_user_profile(
    user_id: String,
    profile_update: UserProfileUpdate,
    user_repo: State<'_, Arc<dyn UserRepository + Send + Sync>>
) -> Result<UserProfile, String> {
    info!(event = "api_call", endpoint = "update_user_profile", user_id = %user_id);
    
    // First check if the user exists
    let existing_profile = match user_repo.get_user_profile(&user_id).await {
        Ok(Some(profile)) => profile,
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "update_user_profile", user_id = %user_id);
            return Err(format!("User not found with ID: {}", user_id));
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_user_profile", error = %e);
            return Err(format!("Database error: {}", e));
        }
    };
    
    // Update the user profile
    match user_repo.update_user_profile(&user_id, profile_update).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_user_profile", user_id = %user_id);
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_user_profile", error = %e);
            Err(format!("Failed to update user profile: {}", e))
        }
    }
}

/// Gets user preferences
///
/// # Arguments
/// * `user_id` - ID of the user
///
/// # Returns
/// * `UserPreferences` - The user's preferences
#[tauri::command]
#[instrument(skip(user_repo), err)]
pub async fn get_user_preferences(
    user_id: String,
    user_repo: State<'_, Arc<dyn UserRepository + Send + Sync>>
) -> Result<serde_json::Value, String> {
    info!(event = "api_call", endpoint = "get_user_preferences", user_id = %user_id);
    
    match user_repo.get_user_preferences(&user_id).await {
        Ok(prefs) => {
            info!(event = "api_success", endpoint = "get_user_preferences", user_id = %user_id);
            Ok(prefs)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_user_preferences", error = %e);
            Err(format!("Failed to retrieve user preferences: {}", e))
        }
    }
}

/// Updates user preferences
///
/// # Arguments
/// * `user_id` - ID of the user
/// * `preferences` - Updated preferences
///
/// # Returns
/// * `UserPreferences` - The updated preferences
#[tauri::command]
#[instrument(skip(user_repo), err)]
pub async fn update_user_preferences(
    user_id: String,
    preferences: serde_json::Value,
    user_repo: State<'_, Arc<dyn UserRepository + Send + Sync>>
) -> Result<serde_json::Value, String> {
    info!(event = "api_call", endpoint = "update_user_preferences", user_id = %user_id);
    
    match user_repo.update_user_preferences(&user_id, preferences).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_user_preferences", user_id = %user_id);
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_user_preferences", error = %e);
            Err(format!("Failed to update user preferences: {}", e))
        }
    }
}

/// Gets user settings for integrations
///
/// # Arguments
/// * `user_id` - ID of the user
///
/// # Returns
/// * `IntegrationSettings` - The user's integration settings
#[tauri::command]
#[instrument(skip(user_repo), err)]
pub async fn get_user_integration_settings(
    user_id: String,
    user_repo: State<'_, Arc<dyn UserRepository + Send + Sync>>
) -> Result<serde_json::Value, String> {
    info!(event = "api_call", endpoint = "get_user_integration_settings", user_id = %user_id);
    
    match user_repo.get_user_integration_settings(&user_id).await {
        Ok(settings) => {
            info!(event = "api_success", endpoint = "get_user_integration_settings", user_id = %user_id);
            Ok(settings)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_user_integration_settings", error = %e);
            Err(format!("Failed to retrieve user integration settings: {}", e))
        }
    }
}

/// Updates user integration settings
///
/// # Arguments
/// * `user_id` - ID of the user
/// * `settings` - Updated integration settings
///
/// # Returns
/// * `IntegrationSettings` - The updated integration settings
#[tauri::command]
#[instrument(skip(user_repo), err)]
pub async fn update_user_integration_settings(
    user_id: String,
    settings: serde_json::Value,
    user_repo: State<'_, Arc<dyn UserRepository + Send + Sync>>
) -> Result<serde_json::Value, String> {
    info!(event = "api_call", endpoint = "update_user_integration_settings", user_id = %user_id);
    
    match user_repo.update_user_integration_settings(&user_id, settings).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_user_integration_settings", user_id = %user_id);
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_user_integration_settings", error = %e);
            Err(format!("Failed to update user integration settings: {}", e))
        }
    }
}