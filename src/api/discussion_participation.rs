use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::models::lms::{Topic, Post};
use crate::services::user_participation_service::UserParticipationService;
use crate::errors::AppError;

// Request and response types
#[derive(Deserialize)]
pub struct RecordViewRequest {
    user_id: String,
    topic_id: String,
}

#[derive(Deserialize)]
pub struct RecordPostRequest {
    user_id: String,
    topic_id: String,
    post_id: String,
    parent_post_id: Option<String>,
}

#[derive(Deserialize)]
pub struct RecordSolutionRequest {
    user_id: String,
    topic_id: String,
    post_id: String,
}

/// Create a router for discussion participation endpoints with proper namespacing
pub fn participation_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/discussion/participation/view", post(record_topic_view))
        .route("/api/discussion/participation/user/:user_id", get(get_user_participation))
        .route("/api/discussion/participation/course/:course_id", get(get_course_participation))
        .route("/api/discussion/participation/solution", post(record_solution))
}

/// API handlers
async fn record_topic_view(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RecordViewRequest>,
) -> Result<StatusCode, AppError> {
    let participation_service = UserParticipationService::new(
        state.db_pool.clone(),
        state.notification_service.clone(),
    );
    
    participation_service.record_topic_view(&payload.user_id, &payload.topic_id)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    
    Ok(StatusCode::OK)
}

async fn get_user_participation(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let participation_service = UserParticipationService::new(
        state.db_pool.clone(),
        state.notification_service.clone(),
    );
    
    let report = participation_service.get_user_participation_report(&user_id)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    
    Ok(Json(serde_json::to_value(report).unwrap()))
}

async fn get_course_participation(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let participation_service = UserParticipationService::new(
        state.db_pool.clone(),
        state.notification_service.clone(),
    );
    
    let summary = participation_service.get_course_participation_summary(&course_id)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    
    Ok(Json(serde_json::to_value(summary).unwrap()))
}

async fn record_solution(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RecordSolutionRequest>,
) -> Result<StatusCode, AppError> {
    let participation_service = UserParticipationService::new(
        state.db_pool.clone(),
        state.notification_service.clone(),
    );
    
    participation_service.record_solution(&payload.user_id, &payload.topic_id, &payload.post_id)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    
    Ok(StatusCode::OK)
}
