use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use crate::models::discussion_mapping::{DiscussionMapping, SyncResult};
use crate::db::{DbPool, discussion_mappings};
use crate::services::discussion_sync::DiscussionSyncService;
use crate::api::canvas::CanvasClient;
use crate::api::discourse::DiscourseClient;
use crate::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMappingRequest {
    pub canvas_discussion_id: String,
    pub discourse_topic_id: String,
    pub course_category_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMappingRequest {
    pub title: Option<String>,
    pub sync_enabled: Option<bool>,
    pub sync_posts: Option<bool>,
}

pub fn discussion_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/discussions", get(get_all_mappings))
        .route("/api/discussions", post(create_mapping))
        .route("/api/discussions/:id", get(get_mapping))
        .route("/api/discussions/:id", put(update_mapping))
        .route("/api/discussions/:id", delete(delete_mapping))
        .route("/api/discussions/:id/sync", post(sync_mapping))
        .route("/api/courses/:course_id/discussions", get(get_mappings_by_course))
        .route("/api/courses/:course_id/discussions/sync", post(sync_course_discussions))
}

async fn get_all_mappings(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DiscussionMapping>>, (StatusCode, String)> {
    match discussion_mappings::get_all_discussion_mappings(&state.db_pool).await {
        Ok(mappings) => Ok(Json(mappings)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DiscussionMapping>, (StatusCode, String)> {
    match discussion_mappings::get_discussion_mapping(&state.db_pool, &id).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn create_mapping(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateMappingRequest>,
) -> Result<Json<DiscussionMapping>, (StatusCode, String)> {
    // Verify that Canvas discussion exists
    if let Err(e) = state.canvas_client.get_discussion(&req.canvas_discussion_id).await {
        return Err((StatusCode::BAD_REQUEST, format!("Canvas discussion not found: {}", e)));
    }
    
    // Verify that Discourse topic exists
    if let Err(e) = state.discourse_client.get_topic(&req.discourse_topic_id).await {
        return Err((StatusCode::BAD_REQUEST, format!("Discourse topic not found: {}", e)));
    }
    
    // Create mapping
    let mapping = DiscussionMapping::new(
        &req.canvas_discussion_id,
        &req.discourse_topic_id,
        &req.course_category_id,
        &req.title,
    );
    
    match discussion_mappings::create_discussion_mapping(&state.db_pool, &mapping).await {
        Ok(created) => Ok(Json(created)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateMappingRequest>,
) -> Result<Json<DiscussionMapping>, (StatusCode, String)> {
    // Get current mapping
    let mut mapping = match discussion_mappings::get_discussion_mapping(&state.db_pool, &id).await {
        Ok(m) => m,
        Err(e) => return Err((StatusCode::NOT_FOUND, e.to_string())),
    };
    
    // Update fields
    if let Some(title) = req.title {
        mapping.title = title;
    }
    
    if let Some(sync_enabled) = req.sync_enabled {
        mapping.sync_enabled = sync_enabled;
    }
    
    if let Some(sync_posts) = req.sync_posts {
        mapping.sync_posts = sync_posts;
    }
    
    // Save changes
    match discussion_mappings::update_discussion_mapping(&state.db_pool, &mapping).await {
        Ok(updated) => Ok(Json(updated)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    match discussion_mappings::delete_discussion_mapping(&state.db_pool, &id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn sync_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<SyncResult>, (StatusCode, String)> {
    let sync_service = DiscussionSyncService::new(
        state.db_pool.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match sync_service.sync_discussion(&id).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_mappings_by_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<DiscussionMapping>>, (StatusCode, String)> {
    match discussion_mappings::get_discussion_mappings_by_course(&state.db_pool, &course_id).await {
        Ok(mappings) => Ok(Json(mappings)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn sync_course_discussions(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<SyncResult>>, (StatusCode, String)> {
    let sync_service = DiscussionSyncService::new(
        state.db_pool.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match sync_service.sync_all_for_course(&course_id).await {
        Ok(results) => Ok(Json(results)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}