use crate::db::DB;
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::services::integration::discourse_integration::DiscourseIntegrationService;
use crate::services::integration::sync_service::IntegrationSyncService;
use crate::models::forum::topic::Topic;
use crate::models::forum::mapping::SyncStatus;
use serde::{Deserialize, Serialize};
use tauri::{State, command};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct TopicMappingResponse {
    pub id: String,
    pub canvas_topic_id: String,
    pub discourse_topic_id: i64,
    pub local_topic_id: Option<String>,
    pub last_sync_at: String,
    pub sync_status: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncTopicRequest {
    pub topic_id: String,
    pub target_system: String, // "canvas" or "discourse"
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStrategy {
    PreferCanvas,
    PreferDiscourse,
    PreferMostRecent,
    MergePreferCanvas,
    MergePreferDiscourse,
}

#[derive(Debug, Deserialize)]
pub struct ResolveConflictRequest {
    pub conflict_id: String,
    pub strategy: ConflictResolutionStrategy,
}

#[command]
pub async fn sync_topic(
    request: SyncTopicRequest,
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
    discourse_service: State<'_, Arc<DiscourseIntegrationService>>,
) -> Result<TopicMappingResponse, String> {
    // Create sync service
    let sync_service = IntegrationSyncService::new(
        db.inner().clone(),
        canvas_service.inner().as_ref().clone(),
        discourse_service.inner().as_ref().clone(),
    );

    // Parse topic ID
    let topic_id = match uuid::Uuid::parse_str(&request.topic_id) {
        Ok(id) => id,
        Err(_) => return Err("Invalid topic ID format".to_string()),
    };

    // Find the topic
    let topic = match Topic::find(db.inner(), topic_id).await {
        Ok(t) => t,
        Err(e) => return Err(format!("Failed to find topic: {}", e)),
    };

    // Perform sync based on target system
    match request.target_system.as_str() {
        "canvas" => {
            if topic.discourse_topic_id.is_some() {
                // If we have a discourse ID, sync to canvas
                let result = sync_service.sync_topic_discourse_to_canvas(
                    topic.discourse_topic_id.unwrap()
                ).await;

                match result {
                    Ok(mapping) => Ok(TopicMappingResponse {
                        id: mapping.id.to_string(),
                        canvas_topic_id: mapping.canvas_topic_id,
                        discourse_topic_id: mapping.discourse_topic_id,
                        local_topic_id: mapping.local_topic_id.map(|id| id.to_string()),
                        last_sync_at: mapping.last_sync_at.to_rfc3339(),
                        sync_status: "Synced".to_string(),
                    }),
                    Err(e) => Err(format!("Failed to sync to Canvas: {}", e)),
                }
            } else {
                Err("Topic doesn't have a Discourse ID, cannot sync to Canvas".to_string())
            }
        },
        "discourse" => {
            if topic.canvas_topic_id.is_some() {
                // If we have a canvas ID, sync to discourse
                let result = sync_service.sync_topic_canvas_to_discourse(
                    &topic.canvas_topic_id.unwrap()
                ).await;

                match result {
                    Ok(mapping) => Ok(TopicMappingResponse {
                        id: mapping.id.to_string(),
                        canvas_topic_id: mapping.canvas_topic_id,
                        discourse_topic_id: mapping.discourse_topic_id,
                        local_topic_id: mapping.local_topic_id.map(|id| id.to_string()),
                        last_sync_at: mapping.last_sync_at.to_rfc3339(),
                        sync_status: "Synced".to_string(),
                    }),
                    Err(e) => Err(format!("Failed to sync to Discourse: {}", e)),
                }
            } else {
                Err("Topic doesn't have a Canvas ID, cannot sync to Discourse".to_string())
            }
        },
        _ => Err("Invalid target_system. Must be 'canvas' or 'discourse'".to_string()),
    }
}

#[command]
pub async fn sync_all_pending(
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
    discourse_service: State<'_, Arc<DiscourseIntegrationService>>,
) -> Result<String, String> {
    // Create sync service
    let sync_service = IntegrationSyncService::new(
        db.inner().clone(),
        canvas_service.inner().as_ref().clone(),
        discourse_service.inner().as_ref().clone(),
    );

    // Sync all pending topics
    match sync_service.sync_all_pending().await {
        Ok(_) => Ok("All pending topics synced successfully".to_string()),
        Err(e) => Err(format!("Failed to sync pending topics: {}", e)),
    }
}

#[command]
pub async fn mark_topic_for_sync(
    topic_id: String,
    db: State<'_, DB>,
) -> Result<String, String> {
    // Parse topic ID
    let topic_id = match uuid::Uuid::parse_str(&topic_id) {
        Ok(id) => id,
        Err(_) => return Err("Invalid topic ID format".to_string()),
    };

    // Find the topic
    let mut topic = match Topic::find(db.inner(), topic_id).await {
        Ok(t) => t,
        Err(e) => return Err(format!("Failed to find topic: {}", e)),
    };

    // Mark for sync
    topic.sync_status = crate::models::forum::topic::SyncStatus::PendingSync;

    // Update in database
    match topic.update(db.inner()).await {
        Ok(_) => Ok(format!("Topic {} marked for sync", topic_id)),
        Err(e) => Err(format!("Failed to mark topic for sync: {}", e)),
    }
}

#[command]
pub async fn test_canvas_connectivity(
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
) -> Result<bool, String> {
    // Try to fetch a known user
    match canvas_service.fetch_canvas_user("1").await {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Canvas connectivity test failed: {}", e)),
    }
}

#[command]
pub async fn test_discourse_connectivity(
    discourse_service: State<'_, Arc<DiscourseIntegrationService>>,
) -> Result<bool, String> {
    // Try to fetch a known category
    match discourse_service.fetch_discourse_category(1).await {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Discourse connectivity test failed: {}", e)),
    }
}

#[command]
pub async fn get_sync_conflicts(
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
    discourse_service: State<'_, Arc<DiscourseIntegrationService>>,
) -> Result<Vec<serde_json::Value>, String> {
    // Create sync service
    let sync_service = IntegrationSyncService::new(
        db.inner().clone(),
        canvas_service.inner().as_ref().clone(),
        discourse_service.inner().as_ref().clone(),
    );

    // Get all conflicts
    match sync_service.get_sync_conflicts().await {
        Ok(conflicts) => Ok(conflicts),
        Err(e) => Err(format!("Failed to get sync conflicts: {}", e)),
    }
}

#[command]
pub async fn resolve_sync_conflict(
    request: ResolveConflictRequest,
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
    discourse_service: State<'_, Arc<DiscourseIntegrationService>>,
) -> Result<String, String> {
    // Create sync service
    let sync_service = IntegrationSyncService::new(
        db.inner().clone(),
        canvas_service.inner().as_ref().clone(),
        discourse_service.inner().as_ref().clone(),
    );

    // Resolve the conflict
    match sync_service.resolve_conflict(&request.conflict_id, request.strategy).await {
        Ok(_) => Ok(format!("Conflict {} resolved successfully", request.conflict_id)),
        Err(e) => Err(format!("Failed to resolve conflict: {}", e)),
    }
}

#[command]
pub async fn get_sync_history(
    db: State<'_, DB>,
) -> Result<Vec<serde_json::Value>, String> {
    // Query the sync history from the database
    let query = "SELECT * FROM sync_history ORDER BY timestamp DESC LIMIT 100";

    match sqlx::query_as::<_, serde_json::Value>(query)
        .fetch_all(db.inner())
        .await
    {
        Ok(history) => Ok(history),
        Err(e) => Err(format!("Failed to get sync history: {}", e)),
    }
}

#[command]
pub async fn get_sync_status(
    db: State<'_, DB>,
    canvas_service: State<'_, Arc<CanvasIntegrationService>>,
    discourse_service: State<'_, Arc<DiscourseIntegrationService>>,
) -> Result<serde_json::Value, String> {
    // Create sync service
    let sync_service = IntegrationSyncService::new(
        db.inner().clone(),
        canvas_service.inner().as_ref().clone(),
        discourse_service.inner().as_ref().clone(),
    );

    // Get sync status
    match sync_service.get_sync_status().await {
        Ok(status) => Ok(status),
        Err(e) => Err(format!("Failed to get sync status: {}", e)),
    }
}