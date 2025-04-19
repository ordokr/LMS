use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::{SyncManager, SyncState};
use crate::services::sync_service::{SyncDirection, SyncResult};
use crate::AppState;

/// Start sync request
#[derive(Debug, Deserialize)]
pub struct StartSyncRequest {
    pub direction: SyncDirectionDto,
    pub strategy: Option<String>,
}

/// Sync direction DTO
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SyncDirectionDto {
    CanvasToDiscourse,
    DiscourseToCanvas,
    Bidirectional,
}

impl From<SyncDirectionDto> for SyncDirection {
    fn from(dto: SyncDirectionDto) -> Self {
        match dto {
            SyncDirectionDto::CanvasToDiscourse => SyncDirection::CanvasToDiscourse,
            SyncDirectionDto::DiscourseToCanvas => SyncDirection::DiscourseToCanvas,
            SyncDirectionDto::Bidirectional => SyncDirection::Bidirectional,
        }
    }
}

impl From<SyncDirection> for SyncDirectionDto {
    fn from(direction: SyncDirection) -> Self {
        match direction {
            SyncDirection::CanvasToDiscourse => SyncDirectionDto::CanvasToDiscourse,
            SyncDirection::DiscourseToCanvas => SyncDirectionDto::DiscourseToCanvas,
            SyncDirection::Bidirectional => SyncDirectionDto::Bidirectional,
        }
    }
}

/// Sync entity request
#[derive(Debug, Deserialize)]
pub struct SyncEntityRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub direction: SyncDirectionDto,
    pub strategy: Option<String>,
}

/// Sync state response
#[derive(Debug, Serialize)]
pub struct SyncStateResponse {
    pub is_syncing: bool,
    pub last_sync: Option<String>,
    pub current_sync_started: Option<String>,
    pub current_sync_progress: f32,
    pub current_sync_stage: String,
    pub current_sync_entity_type: Option<String>,
    pub current_sync_entity_id: Option<String>,
    pub current_sync_direction: Option<SyncDirectionDto>,
    pub error_count: usize,
    pub success_count: usize,
}

impl From<SyncState> for SyncStateResponse {
    fn from(state: SyncState) -> Self {
        Self {
            is_syncing: state.is_syncing,
            last_sync: state.last_sync.map(|dt| dt.to_rfc3339()),
            current_sync_started: state.current_sync_started.map(|dt| dt.to_rfc3339()),
            current_sync_progress: state.current_sync_progress,
            current_sync_stage: state.current_sync_stage,
            current_sync_entity_type: state.current_sync_entity_type,
            current_sync_entity_id: state.current_sync_entity_id,
            current_sync_direction: state.current_sync_direction.map(|d| d.into()),
            error_count: state.error_count,
            success_count: state.success_count,
        }
    }
}

/// Get the sync state
async fn get_sync_state(
    State(state): State<Arc<AppState>>,
) -> Json<SyncStateResponse> {
    let sync_state = state.sync_manager.get_sync_state().await;
    Json(sync_state.into())
}

/// Start a full sync
async fn start_full_sync(
    State(state): State<Arc<AppState>>,
    Json(request): Json<StartSyncRequest>,
) -> Json<SyncStateResponse> {
    let direction = request.direction.into();
    let strategy = request.strategy.as_deref();

    if let Err(e) = state.sync_manager.start_full_sync(direction, strategy).await {
        log::error!("Failed to start sync: {}", e);
    }

    let sync_state = state.sync_manager.get_sync_state().await;
    Json(sync_state.into())
}

/// Sync a specific entity
async fn sync_entity(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SyncEntityRequest>,
) -> Json<SyncResult> {
    let direction = request.direction.into();
    let strategy = request.strategy.as_deref();

    match state.sync_manager.sync_entity(&request.entity_type, &request.entity_id, direction, strategy).await {
        Ok(result) => Json(result),
        Err(e) => {
            log::error!("Failed to sync entity: {}", e);
            Json(SyncResult {
                id: Uuid::new_v4(),
                entity_type: request.entity_type,
                entity_id: Uuid::nil(),
                canvas_updates: 0,
                discourse_updates: 0,
                errors: vec![e.to_string()],
                status: crate::services::model_mapper::SyncStatus::Error,
                started_at: chrono::Utc::now(),
                completed_at: chrono::Utc::now(),
            })
        }
    }
}

/// Cancel the current sync
async fn cancel_sync(
    State(state): State<Arc<AppState>>,
) -> Json<SyncStateResponse> {
    if let Err(e) = state.sync_manager.cancel_sync().await {
        log::error!("Failed to cancel sync: {}", e);
    }

    let sync_state = state.sync_manager.get_sync_state().await;
    Json(sync_state.into())
}

/// Get available synchronization strategies
async fn get_available_strategies(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<String>> {
    Json(state.sync_manager.get_available_strategies())
}

/// Create the sync API routes
pub fn sync_api_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/sync/state", get(get_sync_state))
        .route("/sync/start", post(start_full_sync))
        .route("/sync/entity", post(sync_entity))
        .route("/sync/cancel", post(cancel_sync))
        .route("/sync/strategies", get(get_available_strategies))
        .with_state(state)
}
