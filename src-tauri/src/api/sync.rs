use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::core::auth::Claims;
use crate::core::errors::AppError;
use crate::sync::engine::SyncEngine;
use crate::sync::operations::SyncBatch;

// Receive sync batch from client
pub async fn receive_sync_batch(
    claims: Claims,
    State(engine): State<Arc<SyncEngine>>,
    Json(batch): Json<SyncBatch>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user_id from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
        
    // Validate user_id in batch matches authenticated user
    if batch.user_id != user_id {
        return Err(AppError::AuthorizationError("User ID in batch does not match authenticated user".to_string()));
    }
    
    // Apply batch operations
    engine.apply_sync_batch(batch).await?;
    
    // Create response batch with server operations for the client
    let response_batch = engine.create_sync_batch(user_id, 100).await?;
    
    // Return the response batch or empty success response
    match response_batch {
        Some(batch) => Ok((StatusCode::OK, Json(batch))),
        None => Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Sync successful" })))),
    }
}