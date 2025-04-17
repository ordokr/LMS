use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncStatus {
    pub id: String,
    pub status: String, // e.g., "pending", "in_progress", "completed", "failed"
    pub last_updated: String,
    pub error: Option<String>,
}

static SYNC_STATUS: once_cell::sync::Lazy<std::sync::Mutex<Vec<SyncStatus>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

// POST /api/sync/status
async fn update_sync_status(
    Json(status): Json<SyncStatus>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<SyncStatus>) {
    let mut statuses = SYNC_STATUS.lock().unwrap();
    if let Some(existing) = statuses.iter_mut().find(|s| s.id == status.id) {
        *existing = status.clone();
    } else {
        statuses.push(status.clone());
    }
    (StatusCode::OK, Json(status))
}

// GET /api/sync/status/:id
async fn get_sync_status(
    Path(id): Path<String>,
    _state: Arc<AppState>,
) -> Json<Option<SyncStatus>> {
    let statuses = SYNC_STATUS.lock().unwrap();
    let found = statuses.iter().find(|s| s.id == id).cloned();
    Json(found)
}

// GET /api/sync/status
async fn list_sync_status(_state: Arc<AppState>) -> Json<Vec<SyncStatus>> {
    let statuses = SYNC_STATUS.lock().unwrap();
    Json(statuses.clone())
}

pub fn sync_status_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/sync/status", post(update_sync_status).get(list_sync_status))
        .route("/sync/status/:id", get(get_sync_status))
        .with_state(state)
}
