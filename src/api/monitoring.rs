use axum::{
    extract::State,
    routing::get,
    Json,
    Router,
};
use crate::AppState;
use crate::monitoring::sync_metrics::SyncMetrics;

async fn get_sync_metrics(State(state): State<AppState>) -> Json<SyncMetrics> {
    let metrics = state.sync_monitor.get_metrics().await;
    Json(metrics)
}

async fn health_check(State(state): State<AppState>) -> &'static str {
    if state.sync_monitor.check_system_health().await {
        "OK"
    } else {
        "ERROR"
    }
}

pub fn monitoring_routes() -> Router<AppState> {
    Router::new()
        .route("/api/monitoring/metrics", get(get_sync_metrics))
        .route("/health", get(health_check))
}