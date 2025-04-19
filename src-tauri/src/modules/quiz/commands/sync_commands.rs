use tauri::{State, command};
use std::sync::Arc;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

use crate::app_state::AppState;
use super::super::services::{SyncOperation, SyncPriority, SyncStatus, SyncItem};

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub items_processed: Option<usize>,
}

/// Track quiz activity
#[command]
pub async fn track_quiz_activity(
    user_id: String,
    quiz_id: String,
    activity_type: String,
    data: serde_json::Value,
    app_state: State<'_, Arc<AppState>>
) -> Result<SyncResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    match quiz_service.track_activity(&user_id, &quiz_id, &activity_type, data).await {
        Ok(_) => Ok(SyncResponse {
            success: true,
            message: format!("Activity tracked: {} - {}", quiz_id, activity_type),
            items_processed: None,
        }),
        Err(e) => Ok(SyncResponse {
            success: false,
            message: format!("Failed to track activity: {}", e),
            items_processed: None,
        }),
    }
}

/// Sync with main app
#[command]
pub async fn sync_with_main_app(
    sync_path: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<SyncResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    let path = PathBuf::from(sync_path);
    
    match quiz_service.sync_with_main_app(&path).await {
        Ok(_) => Ok(SyncResponse {
            success: true,
            message: "Sync completed successfully".to_string(),
            items_processed: None,
        }),
        Err(e) => Ok(SyncResponse {
            success: false,
            message: format!("Sync failed: {}", e),
            items_processed: None,
        }),
    }
}

/// Process pending sync items
#[command]
pub async fn process_sync_items(
    app_state: State<'_, Arc<AppState>>
) -> Result<SyncResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    if let Ok(sync_service) = quiz_service.get_sync_service() {
        match sync_service.process_sync_items().await {
            Ok(count) => Ok(SyncResponse {
                success: true,
                message: format!("Processed {} sync items", count),
                items_processed: Some(count),
            }),
            Err(e) => Ok(SyncResponse {
                success: false,
                message: format!("Failed to process sync items: {}", e),
                items_processed: None,
            }),
        }
    } else {
        Ok(SyncResponse {
            success: false,
            message: "Sync service not initialized".to_string(),
            items_processed: None,
        })
    }
}

/// Export sync data
#[command]
pub async fn export_sync_data(
    export_path: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<SyncResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    let path = PathBuf::from(export_path);
    
    if let Ok(sync_service) = quiz_service.get_sync_service() {
        match sync_service.export_sync_data(&path).await {
            Ok(_) => Ok(SyncResponse {
                success: true,
                message: format!("Sync data exported to {}", export_path),
                items_processed: None,
            }),
            Err(e) => Ok(SyncResponse {
                success: false,
                message: format!("Failed to export sync data: {}", e),
                items_processed: None,
            }),
        }
    } else {
        Ok(SyncResponse {
            success: false,
            message: "Sync service not initialized".to_string(),
            items_processed: None,
        })
    }
}

/// Import sync data
#[command]
pub async fn import_sync_data(
    import_path: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<SyncResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    let path = PathBuf::from(import_path);
    
    if let Ok(sync_service) = quiz_service.get_sync_service() {
        match sync_service.import_sync_data(&path).await {
            Ok(count) => Ok(SyncResponse {
                success: true,
                message: format!("Imported {} sync items", count),
                items_processed: Some(count),
            }),
            Err(e) => Ok(SyncResponse {
                success: false,
                message: format!("Failed to import sync data: {}", e),
                items_processed: None,
            }),
        }
    } else {
        Ok(SyncResponse {
            success: false,
            message: "Sync service not initialized".to_string(),
            items_processed: None,
        })
    }
}
