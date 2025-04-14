use std::sync::Arc;
use axum::{
    extract::{Path, Multipart, State, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use sqlx::Postgres;

use crate::services::file_storage_service::{FileStorageService, FileInfo};
use crate::auth::auth_middleware::AuthenticatedUser;
use crate::core::errors::ApiError;
use crate::core::pagination::{PaginationParams, PaginatedResponse};

// File upload request
#[derive(Debug, Deserialize)]
pub struct FileUploadRequest {
    #[serde(default)]
    pub context_type: Option<String>,
    #[serde(default)]
    pub context_id: Option<String>,
    #[serde(default)]
    pub is_public: bool,
}

// File upload response
#[derive(Debug, Serialize)]
pub struct FileUploadResponse {
    pub file_id: String,
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub url: Option<String>,
}

// Base64 file upload request
#[derive(Debug, Deserialize)]
pub struct Base64FileUploadRequest {
    pub filename: String,
    pub content: String,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub context_type: Option<String>,
    #[serde(default)]
    pub context_id: Option<String>,
    #[serde(default)]
    pub is_public: bool,
}

// Attachment creation request
#[derive(Debug, Deserialize)]
pub struct CreateAttachmentRequest {
    pub file_id: String,
    pub entity_type: String,
    pub entity_id: String,
}

// Attachment response
#[derive(Debug, Serialize)]
pub struct AttachmentResponse {
    pub id: String,
    pub file_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: FileInfo,
}

// File list response
#[derive(Debug, Serialize)]
pub struct FileListResponse {
    pub files: Vec<FileInfo>,
}

// File filter params for listing and searching
#[derive(Debug, Deserialize)]
pub struct FileFilterParams {
    pub user_id: Option<String>,
    pub context_type: Option<String>,
    pub context_id: Option<String>,
    pub filename: Option<String>,
}

// Create routes for file management
pub fn routes() -> Router {
    Router::new()
        .route("/files/upload", post(upload_file))
        .route("/files/upload/base64", post(upload_file_base64))
        .route("/files", get(list_files))
        .route("/files/:file_id", get(get_file))
        .route("/files/:file_id", delete(delete_file))
        .route("/files/:file_id/content", get(get_file_content))
        .route("/files/:file_id/visibility", post(set_file_visibility))
        .route("/attachments", post(create_attachment))
        .route("/attachments/:entity_type/:entity_id", get(get_attachments))
        .route("/attachments/:entity_type/:entity_id/:file_id", delete(remove_attachment))
}

// Upload a file using multipart form data
async fn upload_file(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    mut multipart: Multipart,
    Query(params): Query<FileUploadRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut file_part = None;
    let mut filename = None;
    let mut content_type = None;

    // Process multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        ApiError::bad_request("Failed to process uploaded file")
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            content_type = field.content_type().map(|s| s.to_string());
            let data = field.bytes().await.map_err(|e| {
                error!("Failed to read file data: {}", e);
                ApiError::bad_request("Failed to read file data")
            })?;
            file_part = Some(data);
        }
    }

    // Ensure we got a file
    let file_data = file_part.ok_or_else(|| ApiError::bad_request("No file provided"))?;
    let file_name = filename.ok_or_else(|| ApiError::bad_request("No filename provided"))?;

    // Upload file
    let result = file_service
        .upload_file(
            &file_data,
            &file_name,
            content_type.as_deref(),
            &auth_user.user_id,
            params.context_type.as_deref(),
            params.context_id.as_deref(),
            params.is_public,
        )
        .await
        .map_err(|e| {
            error!("File upload error: {}", e);
            ApiError::internal_error("Failed to upload file")
        })?;

    // Return response
    Ok(Json(FileUploadResponse {
        file_id: result.file_id,
        filename: result.filename,
        content_type: result.content_type,
        size: result.size,
        url: result.url,
    }))
}

// Upload a file using base64 encoding
async fn upload_file_base64(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    Json(request): Json<Base64FileUploadRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Upload file
    let result = file_service
        .upload_from_base64(
            &request.content,
            &request.filename,
            request.content_type.as_deref(),
            &auth_user.user_id,
            request.context_type.as_deref(),
            request.context_id.as_deref(),
            request.is_public,
        )
        .await
        .map_err(|e| {
            error!("Base64 file upload error: {}", e);
            ApiError::internal_error("Failed to upload file")
        })?;

    // Return response
    Ok(Json(FileUploadResponse {
        file_id: result.file_id,
        filename: result.filename,
        content_type: result.content_type,
        size: result.size,
        url: result.url,
    }))
}

// Get file metadata
async fn get_file(
    State(file_service): State<Arc<FileStorageService>>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let file = file_service.get_file(&file_id).await.map_err(|e| {
        error!("Error retrieving file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve file metadata")
    })?;

    match file {
        Some(file_info) => Ok(Json(file_info)),
        None => Err(ApiError::not_found("File not found")),
    }
}

// Get file content - returns the actual file binary
async fn get_file_content(
    State(file_service): State<Arc<FileStorageService>>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // Get file metadata
    let file = file_service.get_file(&file_id).await.map_err(|e| {
        error!("Error retrieving file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve file metadata")
    })?;

    let file_info = file.ok_or_else(|| ApiError::not_found("File not found"))?;

    // Get file content
    let content = file_service.get_file_content(&file_id).await.map_err(|e| {
        error!("Error retrieving file content: {}", e);
        ApiError::internal_error("Failed to retrieve file content")
    })?;

    // Build response with content type
    let mut response = Response::new(content.into());
    let headers = response.headers_mut();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_str(&file_info.content_type).unwrap_or_else(|_| {
            axum::http::HeaderValue::from_static("application/octet-stream")
        }),
    );

    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_str(&format!(
            "attachment; filename=\"{}\"",
            file_info.filename
        ))
        .unwrap_or_else(|_| {
            axum::http::HeaderValue::from_static("attachment")
        }),
    );

    Ok(response)
}

// List files with filtering and pagination
async fn list_files(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<FileFilterParams>,
) -> Result<impl IntoResponse, ApiError> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = pagination.offset.unwrap_or(0) as i64;

    let files = if let Some(filename) = filters.filename {
        // Search by filename pattern
        file_service.search_files(&filename, limit).await
    } else if let (Some(context_type), Some(context_id)) = (filters.context_type, filters.context_id) {
        // List by context
        file_service
            .list_files_by_context(&context_type, &context_id, limit, offset)
            .await
    } else {
        // Default to user's files
        let user_id = filters.user_id.unwrap_or_else(|| auth_user.user_id.clone());
        file_service.list_files_by_user(&user_id, limit, offset).await
    };

    let files = files.map_err(|e| {
        error!("Error listing files: {}", e);
        ApiError::internal_error("Failed to list files")
    })?;

    Ok(Json(PaginatedResponse {
        data: files,
        total: files.len() as u64, // This is not accurate for total count, should be improved
        limit: limit as u64,
        offset: offset as u64,
    }))
}

// Delete a file
async fn delete_file(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // Check if file exists and belongs to user
    let file = file_service.get_file(&file_id).await.map_err(|e| {
        error!("Error retrieving file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve file metadata")
    })?;

    let file_info = file.ok_or_else(|| ApiError::not_found("File not found"))?;

    // Only allow the owner or an admin to delete the file
    if file_info.user_id != auth_user.user_id && !auth_user.is_admin {
        return Err(ApiError::forbidden("You are not authorized to delete this file"));
    }

    // Delete the file
    file_service.delete_file(&file_id).await.map_err(|e| {
        error!("Error deleting file: {}", e);
        ApiError::internal_error("Failed to delete file")
    })?;

    Ok(StatusCode::NO_CONTENT)
}

// Set file visibility (public/private)
async fn set_file_visibility(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    Path(file_id): Path<String>,
    Json(params): Json<FileUploadRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Check if file exists and belongs to user
    let file = file_service.get_file(&file_id).await.map_err(|e| {
        error!("Error retrieving file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve file metadata")
    })?;

    let file_info = file.ok_or_else(|| ApiError::not_found("File not found"))?;

    // Only allow the owner or an admin to change visibility
    if file_info.user_id != auth_user.user_id && !auth_user.is_admin {
        return Err(ApiError::forbidden("You are not authorized to modify this file"));
    }

    // Set visibility
    file_service
        .set_file_visibility(&file_id, params.is_public)
        .await
        .map_err(|e| {
            error!("Error setting file visibility: {}", e);
            ApiError::internal_error("Failed to update file visibility")
        })?;

    // Get updated file info
    let updated_file = file_service.get_file(&file_id).await.map_err(|e| {
        error!("Error retrieving updated file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve updated file metadata")
    })?;

    match updated_file {
        Some(file_info) => Ok(Json(file_info)),
        None => Err(ApiError::not_found("File not found")),
    }
}

// Create an attachment linking a file to an entity
async fn create_attachment(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    Json(request): Json<CreateAttachmentRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Check if file exists
    let file = file_service.get_file(&request.file_id).await.map_err(|e| {
        error!("Error retrieving file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve file metadata")
    })?;

    let file_info = file.ok_or_else(|| ApiError::not_found("File not found"))?;

    // Create attachment
    let attachment = file_service
        .create_attachment(&request.file_id, &request.entity_type, &request.entity_id)
        .await
        .map_err(|e| {
            error!("Error creating attachment: {}", e);
            ApiError::internal_error("Failed to create attachment")
        })?;

    Ok(Json(AttachmentResponse {
        id: attachment.id,
        file_id: attachment.file_id,
        entity_type: attachment.entity_type,
        entity_id: attachment.entity_id,
        created_at: attachment.created_at,
        file: file_info,
    }))
}

// Get attachments for an entity
async fn get_attachments(
    State(file_service): State<Arc<FileStorageService>>,
    Path((entity_type, entity_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let files = file_service
        .get_attachments(&entity_type, &entity_id)
        .await
        .map_err(|e| {
            error!("Error retrieving attachments: {}", e);
            ApiError::internal_error("Failed to retrieve attachments")
        })?;

    Ok(Json(FileListResponse { files }))
}

// Remove an attachment
async fn remove_attachment(
    State(file_service): State<Arc<FileStorageService>>,
    auth_user: AuthenticatedUser,
    Path((entity_type, entity_id, file_id)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    // Check if file exists
    let file = file_service.get_file(&file_id).await.map_err(|e| {
        error!("Error retrieving file metadata: {}", e);
        ApiError::internal_error("Failed to retrieve file metadata")
    })?;

    let file_info = file.ok_or_else(|| ApiError::not_found("File not found"))?;

    // Only allow the owner or an admin to remove the attachment
    if file_info.user_id != auth_user.user_id && !auth_user.is_admin {
        return Err(ApiError::forbidden("You are not authorized to remove this attachment"));
    }

    // Remove attachment
    file_service
        .remove_attachment(&entity_type, &entity_id, &file_id)
        .await
        .map_err(|e| {
            error!("Error removing attachment: {}", e);
            ApiError::internal_error("Failed to remove attachment")
        })?;

    Ok(StatusCode::NO_CONTENT)
}
