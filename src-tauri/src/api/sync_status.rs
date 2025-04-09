use serde::{Deserialize, Serialize};
use tauri::{command, State};
use sqlx::SqlitePool;
use crate::error::Error;
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::services::integration::discourse_integration::DiscourseIntegrationService;
use crate::services::integration::sync_service::IntegrationSyncService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub canvas_connected: bool,
    pub discourse_connected: bool,
    pub last_sync: Option<String>,
    pub pending_syncs: i32,
    pub sync_in_progress: bool,
    pub sync_errors: Vec<String>,
}

#[command]
pub async fn get_sync_status(
    db: State<'_, SqlitePool>,
    canvas: State<'_, CanvasIntegrationService>,
    discourse: State<'_, DiscourseIntegrationService>,
    sync_service: State<'_, IntegrationSyncService>,
) -> Result<SyncStatus, Error> {
    // Get current connection status
    let canvas_connected = canvas.check_connectivity().await.is_ok();
    let discourse_connected = discourse.check_connectivity().await.is_ok();
    
    // Get pending syncs count
    let pending_syncs = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM pending_syncs
        WHERE processed = false
        "#
    )
    .fetch_one(&**db)
    .await?
    .count;
    
    // Get sync in progress status
    let sync_in_progress = sync_service.is_sync_in_progress().await;
    
    // Get last sync time
    let last_sync = sqlx::query!(
        r#"
        SELECT MAX(sync_time) as last_sync
        FROM sync_history
        WHERE success = true
        "#
    )
    .fetch_optional(&**db)
    .await?
    .and_then(|row| row.last_sync);
    
    // Get recent sync errors
    let errors = sqlx::query!(
        r#"
        SELECT error_message
        FROM sync_history
        WHERE success = false
        ORDER BY sync_time DESC
        LIMIT 5
        "#
    )
    .fetch_all(&**db)
    .await?
    .into_iter()
    .map(|row| row.error_message.unwrap_or_default())
    .filter(|msg| !msg.is_empty())
    .collect();
    
    Ok(SyncStatus {
        canvas_connected,
        discourse_connected,
        last_sync,
        pending_syncs,
        sync_in_progress,
        sync_errors: errors,
    })
}

#[command]
pub async fn clear_sync_errors(
    db: State<'_, SqlitePool>,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE FROM sync_history
        WHERE success = false
        "#
    )
    .execute(&**db)
    .await?;
    
    Ok(())
}