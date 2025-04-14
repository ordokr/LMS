use axum::{
    extract::{Path, State, Multipart},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tracing::{info, error, warn};
use uuid::Uuid;

use crate::core::errors::AppError;
use crate::core::auth::CurrentUserId;
use crate::services::file_storage_service::{FileStorageService, FileInfo, FileUploadResult};

// Request structures
#[derive(Debug, Deserialize)]
pub struct Base64FileUploadRequest {
    pub filename: String,
    pub content_type: Option<String>,
    pub base64_data: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct FileMetadataUpdateRequest {
    pub filename: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

// Response structures
#[derive(Debug, Serialize)]
pub struct FileListResponse {
    pub files: Vec<FileInfo>,
    pub total: usize,
}

// Handler for uploading files using multipart/form-data
pub async fn upload_file(
    State(file_service): State<Arc<FileStorageService>>,
    current_user: CurrentUserId,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut results = Vec::new();
    
    // Process each part in the multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Error reading multipart field: {}", e);
        AppError::internal_server_error("Failed to process uploaded file")
    })? {
        // Get field name, filename, and content-type
        let name = field.name().unwrap_or("file").to_string();
        let file_name = match field.file_name() {
            Some(filename) => filename.to_string(),
            None => {
                warn!("Field {} has no filename", name);
                continue;
            }
        };
        
        let content_type = field.content_type().map(|ct| ct.to_string());
        
        // Read the file data
        let data = field.bytes().await.map_err(|e| {
            error!("Error reading field data: {}", e);
            AppError::internal_server_error("Failed to read file data")
        })?;
        
        // Extract entity information from field name if available
        // Expected format: "file:entityType:entityId"
        let mut entity_type = None;
        let mut entity_id = None;
        
        if name.contains(':') {
            let parts: Vec<&str> = name.split(':').collect();
            if parts.len() >= 3 {
                entity_type = Some(parts[1].to_string());
                entity_id = Some(parts[2].to_string());
            }
        }
        
        // Store the file
        match file_service.store_file(
            file_name,
            content_type,
            data.to_vec(),
            entity_type,
            entity_id,
            Some(current_user.0.to_string()),
            None,
        ).await {
            Ok(file_info) => {
                results.push(FileUploadResult {
                    file_info,
                    success: true,
                    error: None,
                });
            },
            Err(e) => {
                error!("Failed to store file: {}", e);
                results.push(FileUploadResult {
                    file_info: FileInfo {
                        id: Uuid::new_v4().to_string(),
                        filename: file_name,
                        content_type: content_type.unwrap_or_else(|| "application/octet-stream".to_string()),
                        size: 0,
                        upload_date: chrono::Utc::now(),
                        entity_type,
                        entity_id,
                        user_id: Some(current_user.0.to_string()),
                        metadata: None,
                        url: "".to_string(),
                    },
                    success: false,
                    error: Some(e),
                });
            },
        }
    }
    
    Ok((StatusCode::OK, Json(results)))
}

// Handler for uploading base64 encoded files
pub async fn upload_base64_file(
    State(file_service): State<Arc<FileStorageService>>,
    current_user: CurrentUserId,
    Json(request): Json<Base64FileUploadRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Store the base64 encoded file
    match file_service.store_base64_file(
        request.filename,
        request.content_type,
        request.base64_data,
        request.entity_type,
        request.entity_id,
        Some(current_user.0.to_string()),
        request.metadata,
    ).await {
        Ok(file_info) => {
            Ok((StatusCode::OK, Json(FileUploadResult {
                file_info,
                success: true,
                error: None,
            })))
        },
        Err(e) => {
            error!("Failed to store base64 file: {}", e);
            Err(AppError::internal_server_error(&e))
        },
    }
}

// Handler for retrieving file info by ID
pub async fn get_file_info(
    State(file_service): State<Arc<FileStorageService>>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match file_service.get_file_info(&file_id).await {
        Ok(file_info) => Ok((StatusCode::OK, Json(file_info))),
        Err(e) => {
            if e.contains("not found") {
                Err(AppError::not_found(&e))
            } else {
                error!("Failed to get file info: {}", e);
                Err(AppError::internal_server_error(&e))
            }
        },
    }
}

// Handler for downloading a file by ID
pub async fn download_file(
    State(file_service): State<Arc<FileStorageService>>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match file_service.read_file(&file_id).await {
        Ok((data, file_info)) => {
            // Create response with appropriate headers
            let mut response = Response::new(data.into());
            
            // Set Content-Type header
            let headers = response.headers_mut();
            headers.insert(
                axum::http::header::CONTENT_TYPE, 
                axum::http::HeaderValue::from_str(&file_info.content_type).unwrap_or_else(|_| {
                    axum::http::HeaderValue::from_static("application/octet-stream")
                })
            );
            
            // Set Content-Disposition header (for download)
            let disposition = format!("attachment; filename=\"{}\"", file_info.filename);
            headers.insert(
                axum::http::header::CONTENT_DISPOSITION,
                axum::http::HeaderValue::from_str(&disposition).unwrap_or_else(|_| {
                    axum::http::HeaderValue::from_static("attachment")
                })
            );
            
            Ok(response)
        },
        Err(e) => {
            if e.contains("not found") {
                Err(AppError::not_found(&e))
            } else {
                error!("Failed to read file: {}", e);
                Err(AppError::internal_server_error(&e))
            }
        },
    }
}

// Handler for deleting a file by ID
pub async fn delete_file(
    State(file_service): State<Arc<FileStorageService>>,
    current_user: CurrentUserId,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Check if the file exists and belongs to the current user
    match file_service.get_file_info(&file_id).await {
        Ok(file_info) => {
            // Check if the file belongs to the user (unless they're an admin)
            // TODO: Add admin check here
            if let Some(user_id) = &file_info.user_id {
                if user_id != &current_user.0.to_string() {
                    return Err(AppError::forbidden("You don't have permission to delete this file"));
                }
            }
            
            // Delete the file
            match file_service.delete_file(&file_id).await {
                Ok(_) => Ok(StatusCode::NO_CONTENT),
                Err(e) => {
                    error!("Failed to delete file: {}", e);
                    Err(AppError::internal_server_error(&e))
                },
            }
        },
        Err(e) => {
            if e.contains("not found") {
                Err(AppError::not_found(&e))
            } else {
                error!("Failed to get file info: {}", e);
                Err(AppError::internal_server_error(&e))
            }
        },
    }
}

// Handler for listing files for an entity
pub async fn list_files_for_entity(
    State(file_service): State<Arc<FileStorageService>>,
    Path((entity_type, entity_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    match file_service.list_files_for_entity(&entity_type, &entity_id).await {
        Ok(files) => {
            Ok((StatusCode::OK, Json(FileListResponse {
                files,
                total: files.len(),
            })))
        },
        Err(e) => {
            error!("Failed to list files for entity: {}", e);
            Err(AppError::internal_server_error(&e))
        },
    }
}

// Handler for listing files for a user
pub async fn list_files_for_user(
    State(file_service): State<Arc<FileStorageService>>,
    current_user: CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    match file_service.list_files_for_user(&current_user.0.to_string()).await {
        Ok(files) => {
            Ok((StatusCode::OK, Json(FileListResponse {
                files,
                total: files.len(),
            })))
        },
        Err(e) => {
            error!("Failed to list files for user: {}", e);
            Err(AppError::internal_server_error(&e))
        },
    }
}
