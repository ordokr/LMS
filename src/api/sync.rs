use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncOperation {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String, // e.g., "create", "update", "delete"
    pub payload: Option<String>,
    pub status: String, // e.g., "pending", "synced", "failed"
}

static SYNC_QUEUE: once_cell::sync::Lazy<std::sync::Mutex<Vec<SyncOperation>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

// POST /api/sync/queue
async fn queue_sync_operation(
    Json(op): Json<SyncOperation>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<SyncOperation>) {
    let mut queue = SYNC_QUEUE.lock().unwrap();
    queue.push(op.clone());
    (StatusCode::CREATED, Json(op))
}

// GET /api/sync/queue
async fn list_sync_queue(_state: Arc<AppState>) -> Json<Vec<SyncOperation>> {
    let queue = SYNC_QUEUE.lock().unwrap();
    Json(queue.clone())
}

// POST /api/sync/queue/:id/mark_synced
async fn mark_synced(
    Path(id): Path<String>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<String>) {
    let mut queue = SYNC_QUEUE.lock().unwrap();
    if let Some(op) = queue.iter_mut().find(|op| op.id == id) {
        op.status = "synced".to_string();
        (StatusCode::OK, Json("marked as synced".to_string()))
    } else {
        (StatusCode::NOT_FOUND, Json("not found".to_string()))
    }
}

pub fn sync_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/sync/queue", post(queue_sync_operation).get(list_sync_queue))
        .route("/sync/queue/:id/mark_synced", post(mark_synced))
        .with_state(state)
}
