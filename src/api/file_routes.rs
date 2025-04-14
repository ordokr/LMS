use axum::{
    Router,
    routing::{get, post, delete},
    extract::{Path, State, Query, Json, Multipart},
    http::{StatusCode, HeaderMap, HeaderValue, header},
    response::{IntoResponse, Response},
    body::Bytes,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::task;
use log::{error, info};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::AppState;
use crate::auth::jwt_service::Claims;
use crate::services::file_storage_service::{
    FileMetadata, FileUploadRequest, FileListParams, FileAttachment
};

// File routes
pub fn file_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/files", get(list_files))
        .route("/api/files/:id", get(get_file))
        .route("/api/files/:id/content", get(get_file_content))
        .route("/api/files", post(upload_file))
        .route("/api/files/:id", delete(delete_file))
        .route("/api/files/:id/attach", post(attach_file))
        .route("/api/files/:id/detach", post(detach_file))
        .route("/api/entities/:entity_type/:entity_id/files", get(get_entity_files))
}

// List files with pagination
async fn list_files(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FileListParams>,
) -> impl IntoResponse {
    match state.file_storage.list_files(&params).await {
        Ok(files) => {
            (StatusCode::OK, Json(files)).into_response()
        },
        Err(e) => {
            error!("Error listing files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to list files: {}", e)
            }))).into_response()
        }
    }
}

// Get file metadata
async fn get_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.file_storage.get_file(&id).await {
        Ok(file) => {
            (StatusCode::OK, Json(file)).into_response()
        },
        Err(e) => {
            error!("Error getting file {}: {}", id, e);
            (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("File not found: {}", e)
            }))).into_response()
        }
    }
}

// Get file content
async fn get_file_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.file_storage.get_file_content(&id).await {
        Ok((content, metadata)) => {
            let mut headers = HeaderMap::new();
            
            // Set content type
            headers.insert(
                header::CONTENT_TYPE, 
                HeaderValue::from_str(&metadata.content_type).unwrap_or_else(|_| {
                    HeaderValue::from_static("application/octet-stream")
                })
            );
            
            // Set content disposition (for downloads)
            let disposition = format!("inline; filename=\"{}\"", metadata.name);
            headers.insert(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&disposition).unwrap_or_else(|_| {
                    HeaderValue::from_static("inline")
                })
            );
            
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, metadata.content_type)
                .header(header::CONTENT_DISPOSITION, disposition)
                .body(Bytes::from(content).into())
                .unwrap_or_else(|_| Response::new(Bytes::new().into()))
        },
        Err(e) => {
            error!("Error getting file content {}: {}", id, e);
            (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("File content not found: {}", e)
            }))).into_response()
        }
    }
}

// Upload file
async fn upload_file(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(payload): Json<FileUploadRequest>,
) -> impl IntoResponse {
    let user_id = Some(claims.sub);
    
    match state.file_storage.store_file_from_base64(
        &payload.name,
        &payload.content_type,
        &payload.content,
        payload.is_public.unwrap_or(false),
        user_id,
    ).await {
        Ok(file) => {
            (StatusCode::CREATED, Json(file)).into_response()
        },
        Err(e) => {
            error!("Error uploading file: {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to upload file: {}", e)
            }))).into_response()
        }
    }
}

// Delete file
async fn delete_file(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permissions
    match state.file_storage.get_file(&id).await {
        Ok(file) => {
            if let Some(user_id) = &file.user_id {
                if user_id != &claims.sub && !claims.roles.contains(&"admin".to_string()) {
                    return (StatusCode::FORBIDDEN, Json(json!({
                        "error": "You don't have permission to delete this file"
                    }))).into_response();
                }
            }
            
            // Delete the file
            match state.file_storage.delete_file(&id).await {
                Ok(_) => (StatusCode::NO_CONTENT).into_response(),
                Err(e) => {
                    error!("Error deleting file {}: {}", id, e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to delete file: {}", e)
                    }))).into_response()
                }
            }
        },
        Err(e) => {
            error!("Error getting file {}: {}", id, e);
            (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("File not found: {}", e)
            }))).into_response()
        }
    }
}

// Attach file to an entity
#[derive(Debug, Deserialize)]
pub struct AttachmentRequest {
    pub entity_type: String,
    pub entity_id: String,
}

async fn attach_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<AttachmentRequest>,
) -> impl IntoResponse {
    match state.file_storage.attach_file(
        &id,
        &payload.entity_type,
        &payload.entity_id,
    ).await {
        Ok(attachment) => {
            (StatusCode::CREATED, Json(attachment)).into_response()
        },
        Err(e) => {
            error!("Error attaching file {}: {}", id, e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to attach file: {}", e)
            }))).into_response()
        }
    }
}

// Detach file from an entity
async fn detach_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<AttachmentRequest>,
) -> impl IntoResponse {
    match state.file_storage.detach_file(
        &id,
        &payload.entity_type,
        &payload.entity_id,
    ).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => {
            error!("Error detaching file {}: {}", id, e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to detach file: {}", e)
            }))).into_response()
        }
    }
}

// Get files for a specific entity
async fn get_entity_files(
    State(state): State<Arc<AppState>>,
    Path((entity_type, entity_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.file_storage.get_files_for_entity(&entity_type, &entity_id).await {
        Ok(files) => {
            (StatusCode::OK, Json(files)).into_response()
        },
        Err(e) => {
            error!("Error getting entity files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to get entity files: {}", e)
            }))).into_response()
        }
    }
}
