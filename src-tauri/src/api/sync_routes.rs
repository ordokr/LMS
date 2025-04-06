use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use crate::AppState;
use crate::services::sync_manager::SyncManager;
use crate::models::sync_config::SyncConfig;
use crate::auth::middleware::auth_middleware;
use crate::auth::middleware::require_role;

pub fn sync_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/sync/status", get(get_sync_status))
        .route("/api/sync/run", post(run_sync))
        .route("/api/sync/config", get(get_sync_config))
        .route("/api/sync/config", put(update_sync_config))
        .route("/api/sync/discussions", get(get_discussion_sync_status))
        .route("/api/sync/discussions/run", post(run_discussion_sync))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(|state, req, next| 
                require_role("admin".to_string(), req, next)
            ),
        ))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(auth_middleware),
        ))
}

async fn get_sync_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let sync_manager = state.sync_manager.read().await;
    let status = sync_manager.get_status();
    
    Ok(Json(serde_json::json!({
        "status": status
    })))
}

async fn run_sync(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let sync_manager = state.sync_manager.read().await;
    
    match sync_manager.run_manual_sync().await {
        Ok(_) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "Synchronization started"
        }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_sync_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SyncConfig>, (StatusCode, String)> {
    let sync_manager = state.sync_manager.read().await;
    
    match sync_manager.get_sync_config().await {
        Ok(config) => Ok(Json(config)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_sync_config(
    State(state): State<Arc<AppState>>,
    Json(config): Json<SyncConfig>,
) -> Result<Json<SyncConfig>, (StatusCode, String)> {
    let sync_manager = state.sync_manager.read().await;
    
    match sync_manager.update_sync_config(config).await {
        Ok(updated) => Ok(Json(updated)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_discussion_sync_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let db = &state.db_pool;
    
    match db.get_discussion_sync_status().await {
        Ok(status) => Ok(Json(serde_json::json!({
            "status": status
        }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn run_discussion_sync(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let sync_manager = state.sync_manager.read().await;
    
    match sync_manager.sync_all_discussions().await {
        Ok(results) => {
            // Calculate summary statistics
            let total = results.len();
            let successful = results.iter().filter(|r| r.status == "success").count();
            let failed = results.iter().filter(|r| r.status == "failed").count();
            let partial = results.iter().filter(|r| r.status == "partial").count();
            
            Ok(Json(serde_json::json!({
                "success": true,
                "total": total,
                "successful": successful,
                "failed": failed,
                "partial": partial,
                "results": results
            })))
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}