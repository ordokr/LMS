use axum::{
    Router,
    routing::{get, post, delete},
    extract::{Path, State, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use log::{error, info};

use crate::AppState;
use crate::auth::jwt_service::Claims;
use crate::services::file_storage_service::{FileMetadata, FileUploadRequest};

// Discourse file routes
pub fn discourse_file_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/discourse/posts/:post_id/files", get(list_post_files))
        .route("/api/discourse/posts/:post_id/files", post(attach_file_to_post))
        .route("/api/discourse/posts/:post_id/files/:file_id", delete(remove_file_from_post))
        .route("/api/discourse/uploads", post(upload_discourse_file))
        .route("/api/discourse/files/:file_id", get(get_discourse_file))
}

#[derive(Debug, Deserialize)]
pub struct DiscourseFileAttachRequest {
    pub file_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscourseUploadRequest {
    pub filename: String,
    pub content_type: String,
    pub for_message: Option<bool>,
    pub for_forum: Option<bool>,
}

// List files attached to a Discourse post
async fn list_post_files(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> impl IntoResponse {
    match state.file_storage.get_files_for_entity("discourse_post", &post_id).await {
        Ok(files) => {
            (StatusCode::OK, Json(files)).into_response()
        },
        Err(e) => {
            error!("Error listing post files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to list post files: {}", e)
            }))).into_response()
        }
    }
}

// Attach a file to a Discourse post
async fn attach_file_to_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Json(payload): Json<DiscourseFileAttachRequest>,
) -> impl IntoResponse {
    match state.file_storage.attach_file(
        &payload.file_id,
        "discourse_post",
        &post_id,
    ).await {
        Ok(attachment) => {
            (StatusCode::CREATED, Json(attachment)).into_response()
        },
        Err(e) => {
            error!("Error attaching file to post: {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to attach file to post: {}", e)
            }))).into_response()
        }
    }
}

// Remove a file from a Discourse post
async fn remove_file_from_post(
    State(state): State<Arc<AppState>>,
    Path((post_id, file_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.file_storage.detach_file(
        &file_id,
        "discourse_post",
        &post_id,
    ).await {
    
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => {
            error!("Error removing file from post: {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to remove file from post: {}", e)
            }))).into_response()
        }
    }
}

// Upload a file for Discourse
async fn upload_discourse_file(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DiscourseUploadRequest>,
) -> impl IntoResponse {
    // Create a file upload request
    let upload_request = FileUploadRequest {
        filename: payload.filename,
        content_type: payload.content_type,
        entity_type: "discourse".to_string(),
        entity_id: None,
        metadata: Some(json!({
            "for_message": payload.for_message.unwrap_or(false),
            "for_forum": payload.for_forum.unwrap_or(true),
            "platform": "discourse"
        })),
    };

    // Register the upload request and get a presigned URL
    match state.file_storage.create_upload_request(upload_request).await {
        Ok(upload_info) => {
            info!("Created Discourse file upload request");
            (StatusCode::OK, Json(upload_info)).into_response()
        },
        Err(e) => {
            error!("Error creating Discourse file upload request: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to create upload request: {}", e)
            }))).into_response()
        }
    }
}
