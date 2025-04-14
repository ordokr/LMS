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
use crate::services::file_storage_service::{FileMetadata, FileUploadRequest, FileAttachment};

// Submission attachment routes
pub fn submission_attachment_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/submissions/:submission_id/attachments", get(list_submission_attachments))
        .route("/api/submissions/:submission_id/attachments", post(add_submission_attachment))
        .route("/api/submissions/:submission_id/attachments/:file_id", delete(remove_submission_attachment))
}

// Models
#[derive(Debug, Deserialize)]
pub struct SubmissionAttachmentRequest {
    pub file_id: String,
}

// List attachments for a submission
async fn list_submission_attachments(
    State(state): State<Arc<AppState>>,
    Path(submission_id): Path<String>,
) -> impl IntoResponse {
    match state.file_storage.get_files_for_entity("submission", &submission_id).await {
        Ok(files) => {
            (StatusCode::OK, Json(files)).into_response()
        },
        Err(e) => {
            error!("Error listing submission attachments: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Failed to list submission attachments: {}", e)
            }))).into_response()
        }
    }
}

// Add an attachment to a submission
async fn add_submission_attachment(
    State(state): State<Arc<AppState>>,
    Path(submission_id): Path<String>,
    Json(payload): Json<SubmissionAttachmentRequest>,
) -> impl IntoResponse {
    match state.file_storage.attach_file(
        &payload.file_id,
        "submission",
        &submission_id,
    ).await {
        Ok(attachment) => {
            (StatusCode::CREATED, Json(attachment)).into_response()
        },
        Err(e) => {
            error!("Error adding submission attachment: {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to add submission attachment: {}", e)
            }))).into_response()
        }
    }
}

// Remove an attachment from a submission
async fn remove_submission_attachment(
    State(state): State<Arc<AppState>>,
    Path((submission_id, file_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.file_storage.detach_file(
        &file_id,
        "submission",
        &submission_id,
    ).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => {
            error!("Error removing submission attachment: {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Failed to remove submission attachment: {}", e)
            }))).into_response()
        }
    }
}
