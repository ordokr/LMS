use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use log::{error, info};

use crate::AppState;
use crate::auth::jwt_service::Claims;
use crate::services::file_storage_service::{FileMetadata, FileUploadRequest, FileListParams};

// Canvas file adapter routes
pub fn canvas_file_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/canvas/files", get(list_canvas_files))
        .route("/api/canvas/files/:id", get(get_canvas_file))
        .route("/api/canvas/files", post(upload_canvas_file))
        .route("/api/canvas/courses/:course_id/files", get(get_course_files))
        .route("/api/canvas/courses/:course_id/assignments/:assignment_id/submissions/:user_id/files", 
               get(get_submission_files))
        .route("/api/canvas/users/:user_id/files", get(get_user_files))
}

// Request models
#[derive(Debug, Deserialize)]
pub struct CanvasFileUploadRequest {
    pub name: String,
    pub content_type: String,
    pub content: String, // Base64 encoded content
    pub parent_folder_id: Option<String>,
    pub on_duplicate: Option<String>, // overwrite, rename
}

// List Canvas files with Canvas-compatible query parameters
#[derive(Debug, Deserialize)]
pub struct CanvasFileListParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub search_term: Option<String>,
    pub include: Option<String>,
}

// View model for Canvas file format
#[derive(Debug, Serialize)]
pub struct CanvasFileViewModel {
    pub id: String,
    pub uuid: String,
    pub folder_id: Option<String>,
    pub display_name: String,
    pub filename: String,
    pub content_type: String,
    pub url: String,
    pub size: i64,
    pub created_at: String,
    pub updated_at: String,
    pub unlock_at: Option<String>,
    pub locked: bool,
    pub hidden: bool,
    pub thumbnail_url: Option<String>,
    pub modified_at: String,
    pub mime_class: String,
    pub media_entry_id: Option<String>,
    pub locked_for_user: bool,
}

// Convert from our storage model to Canvas format
fn convert_to_canvas_format(file: &FileMetadata, base_url: &str) -> CanvasFileViewModel {
    CanvasFileViewModel {
        id: file.id.clone(),
        uuid: file.id.clone(),
        folder_id: None,
        display_name: file.name.clone(),
        filename: file.name.clone(),
        content_type: file.content_type.clone(),
        url: format!("{}/api/files/{}/content", base_url, file.id),
        size: file.size,
        created_at: file.created_at.to_rfc3339(),
        updated_at: file.updated_at.to_rfc3339(),
        unlock_at: None,
        locked: !file.is_public,
        hidden: !file.is_public,
        thumbnail_url: None,
        modified_at: file.updated_at.to_rfc3339(),
        mime_class: get_mime_class(&file.content_type),
        media_entry_id: None,
        locked_for_user: false,
    }
}

// Get a simple mime class for thumbnail display
fn get_mime_class(content_type: &str) -> String {
    if content_type.starts_with("image/") {
        "image".to_string()
    } else if content_type.starts_with("video/") {
        "video".to_string()
    } else if content_type.starts_with("audio/") {
        "audio".to_string()
    } else if content_type == "application/pdf" {
        "pdf".to_string()
    } else if content_type.contains("document") || content_type.contains("msword") {
        "document".to_string()
    } else if content_type.contains("spreadsheet") || content_type.contains("excel") {
        "spreadsheet".to_string()
    } else {
        "file".to_string()
    }
}

// Handler implementations
async fn list_canvas_files(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CanvasFileListParams>,
) -> impl IntoResponse {
    // Convert Canvas parameters to our format
    let page = params.page.unwrap_or(1);
    let limit = params.per_page.unwrap_or(10);
    
    let list_params = FileListParams {
        page: Some(page),
        limit: Some(limit),
        entity_type: None,
        entity_id: None,
        search: params.search_term,
    };
    
    match state.file_storage.list_files(&list_params).await {
        Ok(files) => {
            // Get base URL for file URLs
            let base_url = std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
            // Convert to Canvas format
            let canvas_files: Vec<CanvasFileViewModel> = files.iter()
                .map(|file| convert_to_canvas_format(file, &base_url))
                .collect();
            
            (StatusCode::OK, Json(canvas_files)).into_response()
        },
        Err(e) => {
            error!("Error listing files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "errors": [{
                    "message": format!("Failed to list files: {}", e)
                }]
            }))).into_response()
        }
    }
}

async fn get_canvas_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.file_storage.get_file(&id).await {
        Ok(file) => {
            // Get base URL for file URLs
            let base_url = std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
            // Convert to Canvas format
            let canvas_file = convert_to_canvas_format(&file, &base_url);
            
            (StatusCode::OK, Json(canvas_file)).into_response()
        },
        Err(e) => {
            error!("Error getting file {}: {}", id, e);
            (StatusCode::NOT_FOUND, Json(json!({
                "errors": [{
                    "message": format!("File not found: {}", e)
                }]
            }))).into_response()
        }
    }
}

async fn upload_canvas_file(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(payload): Json<CanvasFileUploadRequest>,
) -> impl IntoResponse {
    // Convert to our upload format
    let upload_request = FileUploadRequest {
        name: payload.name,
        content_type: payload.content_type,
        content: payload.content,
        is_public: Some(true), // Canvas files are typically public within the context
    };
    
    let user_id = Some(claims.sub);
    
    match state.file_storage.store_file_from_base64(
        &upload_request.name,
        &upload_request.content_type,
        &upload_request.content,
        upload_request.is_public.unwrap_or(true),
        user_id,
    ).await {
        Ok(file) => {
            // Get base URL for file URLs
            let base_url = std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
            // Convert to Canvas format
            let canvas_file = convert_to_canvas_format(&file, &base_url);
            
            (StatusCode::CREATED, Json(canvas_file)).into_response()
        },
        Err(e) => {
            error!("Error uploading file: {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "errors": [{
                    "message": format!("Failed to upload file: {}", e)
                }]
            }))).into_response()
        }
    }
}

// Get files for a specific course
async fn get_course_files(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
    Query(params): Query<CanvasFileListParams>,
) -> impl IntoResponse {
    // Convert Canvas parameters to our format
    let page = params.page.unwrap_or(1);
    let limit = params.per_page.unwrap_or(10);
    
    // Get files attached to the course
    match state.file_storage.get_files_for_entity("course", &course_id).await {
        Ok(files) => {
            // Get base URL for file URLs
            let base_url = std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
            // Convert to Canvas format
            let canvas_files: Vec<CanvasFileViewModel> = files.iter()
                .map(|file| convert_to_canvas_format(file, &base_url))
                .collect();
            
            (StatusCode::OK, Json(canvas_files)).into_response()
        },
        Err(e) => {
            error!("Error getting course files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "errors": [{
                    "message": format!("Failed to get course files: {}", e)
                }]
            }))).into_response()
        }
    }
}

// Get files for a specific submission
async fn get_submission_files(
    State(state): State<Arc<AppState>>,
    Path((course_id, assignment_id, user_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // Construct submission ID from components (format may vary based on your system)
    let submission_id = format!("{}_{}_{}",
        assignment_id, 
        user_id,
        course_id
    );
    
    // Get files attached to the submission
    match state.file_storage.get_files_for_entity("submission", &submission_id).await {
        Ok(files) => {
            // Get base URL for file URLs
            let base_url = std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
            // Convert to Canvas format
            let canvas_files: Vec<CanvasFileViewModel> = files.iter()
                .map(|file| convert_to_canvas_format(file, &base_url))
                .collect();
            
            (StatusCode::OK, Json(canvas_files)).into_response()
        },
        Err(e) => {
            error!("Error getting submission files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "errors": [{
                    "message": format!("Failed to get submission files: {}", e)
                }]
            }))).into_response()
        }
    }
}

// Get files for a specific user
async fn get_user_files(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    Query(params): Query<CanvasFileListParams>,
) -> impl IntoResponse {
    // Convert Canvas parameters to our format
    let page = params.page.unwrap_or(1);
    let limit = params.per_page.unwrap_or(10);
    
    // Build query to find files by user ID
    let list_params = FileListParams {
        page: Some(page),
        limit: Some(limit),
        entity_type: Some("user".to_string()),
        entity_id: Some(user_id.clone()),
        search: params.search_term,
    };
    
    match state.file_storage.list_files(&list_params).await {
        Ok(files) => {
            // Get base URL for file URLs
            let base_url = std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
            // Convert to Canvas format
            let canvas_files: Vec<CanvasFileViewModel> = files.iter()
                .map(|file| convert_to_canvas_format(file, &base_url))
                .collect();
            
            (StatusCode::OK, Json(canvas_files)).into_response()
        },
        Err(e) => {
            error!("Error getting user files: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "errors": [{
                    "message": format!("Failed to get user files: {}", e)
                }]
            }))).into_response()
        }
    }
}
