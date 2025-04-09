use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
    routing::{get, post, delete, put},
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::AppState;
use crate::models::forum::mapping::TopicMapping;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct CreateTopicMappingRequest {
    pub canvas_topic_id: String,
    pub discourse_topic_id: String,
    pub course_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostMappingRequest {
    pub canvas_entry_id: String,
    pub discourse_post_id: String,
    pub canvas_topic_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ToggleSyncRequest {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTimeRequest {
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TopicMappingResponse {
    pub id: Uuid,
    pub canvas_topic_id: String,
    pub discourse_topic_id: String,
    pub mapping_id: Uuid,
    pub sync_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_synced_at: Option<String>,
    pub canvas_updated_at: Option<String>,
    pub discourse_updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostMappingResponse {
    pub id: Uuid,
    pub canvas_entry_id: String,
    pub discourse_post_id: String,
    pub topic_mapping_id: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SyncStatusResponse {
    pub topic_mapping_id: Uuid,
    pub needs_sync: bool,
    pub sync_direction: Option<String>,
}

// Helper function to handle errors
fn handle_error<E: std::fmt::Display>(err: E) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", err),
    )
        .into_response()
}

// Convert TopicMapping to TopicMappingResponse
fn to_topic_response(mapping: TopicMapping) -> TopicMappingResponse {
    TopicMappingResponse {
        id: mapping.id,
        canvas_topic_id: mapping.canvas_topic_id,
        discourse_topic_id: mapping.discourse_topic_id,
        mapping_id: mapping.mapping_id,
        sync_enabled: mapping.sync_enabled,
        created_at: mapping.created_at.to_rfc3339(),
        updated_at: mapping.updated_at.to_rfc3339(),
        last_synced_at: mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
        canvas_updated_at: mapping.canvas_updated_at.map(|dt| dt.to_rfc3339()),
        discourse_updated_at: mapping.discourse_updated_at.map(|dt| dt.to_rfc3339()),
    }
}

async fn create_topic_mapping(
    State(state): State<AppState>,
    Json(req): Json<CreateTopicMappingRequest>,
) -> Result<impl IntoResponse, Response> {
    let sync_service = &state.sync_service;
    
    let mapping = sync_service
        .create_topic_mapping(&req.canvas_topic_id, &req.discourse_topic_id, &req.course_id)
        .await
        .map_err(handle_error)?;
    
    Ok((StatusCode::CREATED, Json(to_topic_response(mapping))))
}

async fn get_topic_mapping_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.topic_mapping_repo
        .get_topic_mapping_by_id(id)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Topic mapping with ID {} not found", id)).into_response()
        })?;
    
    Ok((StatusCode::OK, Json(to_topic_response(mapping))))
}

async fn get_topic_mapping_by_canvas_id(
    State(state): State<AppState>,
    Path(canvas_id): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.topic_mapping_repo
        .get_topic_mapping_by_canvas_id(&canvas_id)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Topic mapping for Canvas topic ID {} not found", canvas_id)).into_response()
        })?;
    
    Ok((StatusCode::OK, Json(to_topic_response(mapping))))
}

async fn get_topic_mappings_for_course(
    State(state): State<AppState>,
    Path(course_id): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let mappings = state.sync_service
        .get_topic_mappings_for_course(&course_id)
        .await
        .map_err(handle_error)?;
    
    let response: Vec<TopicMappingResponse> = mappings
        .into_iter()
        .map(to_topic_response)
        .collect();
    
    Ok((StatusCode::OK, Json(response)))
}

async fn toggle_topic_sync(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ToggleSyncRequest>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.sync_service
        .toggle_topic_sync(id, req.enabled)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Topic mapping with ID {} not found", id)).into_response()
        })?;
    
    Ok((StatusCode::OK, Json(to_topic_response(mapping))))
}

async fn record_canvas_update(
    State(state): State<AppState>,
    Path(canvas_id): Path<String>,
    Json(req): Json<UpdateTimeRequest>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.sync_service
        .record_canvas_topic_update(&canvas_id, req.timestamp)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Topic mapping for Canvas topic ID {} not found", canvas_id)).into_response()
        })?;
    
    Ok((StatusCode::OK, Json(to_topic_response(mapping))))
}

async fn record_discourse_update(
    State(state): State<AppState>,
    Path(discourse_id): Path<String>,
    Json(req): Json<UpdateTimeRequest>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.sync_service
        .record_discourse_topic_update(&discourse_id, req.timestamp)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Topic mapping for Discourse topic ID {} not found", discourse_id)).into_response()
        })?;
    
    Ok((StatusCode::OK, Json(to_topic_response(mapping))))
}

async fn get_sync_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Response> {
    let sync_direction = state.sync_service
        .needs_sync(id)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Topic mapping with ID {} not found", id)).into_response()
        })?;
    
    let response = SyncStatusResponse {
        topic_mapping_id: id,
        needs_sync: sync_direction.is_some(),
        sync_direction,
    };
    
    Ok((StatusCode::OK, Json(response)))
}

async fn create_post_mapping(
    State(state): State<AppState>,
    Json(req): Json<CreatePostMappingRequest>,
) -> Result<impl IntoResponse, Response> {
    let post_mapping = state.sync_service
        .create_post_mapping(&req.canvas_entry_id, &req.discourse_post_id, &req.canvas_topic_id)
        .await
        .map_err(handle_error)?;
    
    let response = PostMappingResponse {
        id: post_mapping.id,
        canvas_entry_id: post_mapping.canvas_entry_id,
        discourse_post_id: post_mapping.discourse_post_id,
        topic_mapping_id: post_mapping.topic_mapping_id,
        created_at: post_mapping.created_at.to_rfc3339(),
        updated_at: post_mapping.updated_at.to_rfc3339(),
        last_synced_at: post_mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

pub fn topic_mapping_routes() -> Router<AppState> {
    Router::new()
        .route("/api/integration/topic-mappings", post(create_topic_mapping))
        .route("/api/integration/topic-mappings/:id", get(get_topic_mapping_by_id))
        .route("/api/integration/topic-mappings/canvas/:canvas_id", get(get_topic_mapping_by_canvas_id))
        .route("/api/integration/topic-mappings/course/:course_id", get(get_topic_mappings_for_course))
        .route("/api/integration/topic-mappings/:id/sync", put(toggle_topic_sync))
        .route("/api/integration/topic-mappings/:id/status", get(get_sync_status))
        .route("/api/integration/topic-mappings/canvas/:canvas_id/update", post(record_canvas_update))
        .route("/api/integration/topic-mappings/discourse/:discourse_id/update", post(record_discourse_update))
        .route("/api/integration/post-mappings", post(create_post_mapping))
}