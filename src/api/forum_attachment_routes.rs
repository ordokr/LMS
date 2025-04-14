use axum::{
    Router,
    routing::{get, post, delete},
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::file_storage_service::FileMetadata;
use crate::models::forum::post::Post;
use crate::auth::jwt_service::Claims;
use crate::AppState;

// Create forum post attachment routes
pub fn forum_attachment_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/forum/posts/:id/attachments", get(get_post_attachments))
        .route("/api/forum/posts/:id/attachments", post(add_post_attachments))
        .route("/api/forum/posts/:id/attachments/:file_id", delete(remove_post_attachment))
}

// Request model for adding attachments
#[derive(Debug, Deserialize)]
pub struct AddPostAttachmentsRequest {
    pub file_ids: Vec<String>,
}

// Response models
#[derive(Debug, Serialize)]
pub struct PostAttachmentsResponse {
    pub success: bool,
    pub attachments: Vec<FileMetadata>,
}

// Get attachments for a forum post
async fn get_post_attachments(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> impl IntoResponse {
    let storage_service = &state.file_storage;
    
    // Get attachments
    match storage_service.get_files_for_entity("forum_post", &post_id).await {
        Ok(attachments) => {
            (StatusCode::OK, Json(PostAttachmentsResponse {
                success: true,
                attachments,
            })).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to get attachments: {}", e)
            }))).into_response()
        }
    }
}

// Add attachments to a forum post
async fn add_post_attachments(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(post_id): Path<String>,
    Json(payload): Json<AddPostAttachmentsRequest>,
) -> impl IntoResponse {
    let storage_service = &state.file_storage;
    let db = &state.db;
    
    // Verify post exists and user has permission
    match Post::find_by_id(db, &post_id).await {
        Ok(post) => {
            // Check permission - user must be the author or have moderator/admin role
            if post.user_id != claims.sub && 
               !claims.roles.contains(&"admin".to_string()) && 
               !claims.roles.contains(&"moderator".to_string()) {
                return (StatusCode::FORBIDDEN, Json(serde_json::json!({
                    "success": false,
                    "error": "You don't have permission to modify this post's attachments"
                }))).into_response();
            }
            
            // Attach files
            match storage_service.attach_files_batch(payload.file_ids, "forum_post", &post_id).await {
                Ok(_) => {
                    // Get the updated list of attachments
                    match storage_service.get_files_for_entity("forum_post", &post_id).await {
                        Ok(attachments) => {
                            // Update post with attachment IDs
                            let attachment_ids: Vec<String> = attachments.iter()
                                .map(|a| a.id.clone())
                                .collect();
                                
                            let mut updated_post = post.clone();
                            updated_post.attachment_ids = Some(attachment_ids);
                            
                            // Update post in database
                            if let Err(e) = updated_post.update(db).await {
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                    "success": false,
                                    "error": format!("Failed to update post: {}", e)
                                }))).into_response();
                            }
                            
                            (StatusCode::OK, Json(PostAttachmentsResponse {
                                success: true,
                                attachments,
                            })).into_response()
                        },
                        Err(e) => {
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                "success": false,
                                "error": format!("Failed to get updated attachments: {}", e)
                            }))).into_response()
                        }
                    }
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "success": false,
                        "error": format!("Failed to attach files: {}", e)
                    }))).into_response()
                }
            }
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "success": false,
                "error": format!("Post not found: {}", e)
            }))).into_response()
        }
    }
}

// Remove an attachment from a forum post
async fn remove_post_attachment(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path((post_id, file_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let storage_service = &state.file_storage;
    let db = &state.db;
    
    // Verify post exists and user has permission
    match Post::find_by_id(db, &post_id).await {
        Ok(post) => {
            // Check permission - user must be the author or have moderator/admin role
            if post.user_id != claims.sub && 
               !claims.roles.contains(&"admin".to_string()) && 
               !claims.roles.contains(&"moderator".to_string()) {
                return (StatusCode::FORBIDDEN, Json(serde_json::json!({
                    "success": false,
                    "error": "You don't have permission to modify this post's attachments"
                }))).into_response();
            }
            
            // Detach file
            match storage_service.detach_file(&file_id, "forum_post", &post_id).await {
                Ok(_) => {
                    // Get the updated list of attachments
                    match storage_service.get_files_for_entity("forum_post", &post_id).await {
                        Ok(attachments) => {
                            // Update post with attachment IDs
                            let attachment_ids: Vec<String> = attachments.iter()
                                .map(|a| a.id.clone())
                                .collect();
                                
                            let mut updated_post = post.clone();
                            updated_post.attachment_ids = Some(attachment_ids);
                            
                            // Update post in database
                            if let Err(e) = updated_post.update(db).await {
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                    "success": false,
                                    "error": format!("Failed to update post: {}", e)
                                }))).into_response();
                            }
                            
                            (StatusCode::OK, Json(PostAttachmentsResponse {
                                success: true,
                                attachments,
                            })).into_response()
                        },
                        Err(e) => {
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                "success": false,
                                "error": format!("Failed to get updated attachments: {}", e)
                            }))).into_response()
                        }
                    }
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "success": false,
                        "error": format!("Failed to remove attachment: {}", e)
                    }))).into_response()
                }
            }
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "success": false,
                "error": format!("Post not found: {}", e)
            }))).into_response()
        }
    }
}
