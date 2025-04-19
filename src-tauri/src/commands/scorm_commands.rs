use crate::AppState;
use tauri::State;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScormPackageInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub version: String,
    pub launch_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScormSessionInfo {
    pub id: String,
    pub package_id: String,
    pub user_id: String,
    pub state: String,
    pub score: Option<f32>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// Import a SCORM package
#[tauri::command]
pub async fn import_scorm_package(
    state: State<'_, AppState>,
    package_path: String,
) -> Result<String, String> {
    let scorm_service = state.scorm_service.clone();
    let path = PathBuf::from(package_path);
    
    let mut service = scorm_service.lock().await;
    
    let package_id = service.import_package(&path)
        .map_err(|e| e.to_string())?;
    
    Ok(package_id.to_string())
}

/// Get all SCORM packages
#[tauri::command]
pub async fn get_scorm_packages(
    state: State<'_, AppState>,
) -> Result<Vec<ScormPackageInfo>, String> {
    let scorm_service = state.scorm_service.clone();
    let service = scorm_service.lock().await;
    
    let packages = service.get_packages();
    
    let package_infos = packages.into_iter()
        .map(|package| {
            ScormPackageInfo {
                id: package.id.to_string(),
                title: package.title.clone(),
                description: package.description.clone(),
                version: format!("{:?}", package.version),
                launch_url: package.launch_url.clone(),
            }
        })
        .collect();
    
    Ok(package_infos)
}

/// Launch a SCORM package
#[tauri::command]
pub async fn launch_scorm_package(
    state: State<'_, AppState>,
    package_id: String,
    user_id: String,
) -> Result<String, String> {
    let scorm_service = state.scorm_service.clone();
    let mut service = scorm_service.lock().await;
    
    let package_uuid = Uuid::parse_str(&package_id)
        .map_err(|e| format!("Invalid package ID: {}", e))?;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    let session_id = service.create_session(&package_uuid, &user_uuid)
        .map_err(|e| e.to_string())?;
    
    let launch_url = service.get_launch_url(&package_uuid)
        .map_err(|e| e.to_string())?;
    
    Ok(launch_url)
}

/// Get all SCORM sessions for a user
#[tauri::command]
pub async fn get_scorm_user_sessions(
    state: State<'_, AppState>,
    user_id: String,
) -> Result<Vec<ScormSessionInfo>, String> {
    let scorm_service = state.scorm_service.clone();
    let service = scorm_service.lock().await;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    let sessions = service.get_sessions_for_user(&user_uuid);
    
    let session_infos = sessions.into_iter()
        .map(|session| {
            ScormSessionInfo {
                id: session.id.to_string(),
                package_id: session.package_id.to_string(),
                user_id: session.user_id.to_string(),
                state: format!("{:?}", session.state),
                score: session.score,
                created_at: session.created_at.to_rfc3339(),
                updated_at: session.updated_at.to_rfc3339(),
                completed_at: session.completed_at.map(|t| t.to_rfc3339()),
            }
        })
        .collect();
    
    Ok(session_infos)
}

/// Handle a SCORM API call
#[tauri::command]
pub async fn handle_scorm_api_call(
    state: State<'_, AppState>,
    session_id: String,
    function: String,
    args: Vec<String>,
) -> Result<String, String> {
    let scorm_service = state.scorm_service.clone();
    let mut service = scorm_service.lock().await;
    
    let session_uuid = Uuid::parse_str(&session_id)
        .map_err(|e| format!("Invalid session ID: {}", e))?;
    
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    
    let result = service.handle_api_call(&session_uuid, &function, &args_str)
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}
