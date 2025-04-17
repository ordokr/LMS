use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteIntegrationRepository, IntegrationRepository, IntegrationStatusRow};

#[derive(Deserialize)]
pub struct SyncRequest {
    pub resource: Option<String>,
}

#[derive(Serialize)]
pub struct SyncStatusResponse {
    pub id: i64,
    pub status: String,
    pub last_synced: Option<String>,
}

async fn trigger_sync(
    Json(_req): Json<SyncRequest>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<SyncStatusResponse>, StatusCode> {
    let repo = SqliteIntegrationRepository { pool: &pool };
    // For demo, just insert a pending status
    match repo.create_sync_status("pending").await {
        Ok(id) => Ok(Json(SyncStatusResponse { id, status: "pending".to_string(), last_synced: None })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_sync_status(
    Path(id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<SyncStatusResponse>, StatusCode> {
    let repo = SqliteIntegrationRepository { pool: &pool };
    match repo.get_sync_status(id).await {
        Ok(Some(row)) => Ok(Json(SyncStatusResponse { id: row.id, status: row.status, last_synced: row.last_synced })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn integration_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/integration/sync", post(trigger_sync))
        .route("/integration/status/:id", get(get_sync_status))
        .with_state(pool)
}
