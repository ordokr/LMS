use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub expires_at: usize,
}

// Handler for refreshing access tokens using a refresh token
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    // Extract the refresh token from request
    let refresh_token = &payload.refresh_token;
    
    // Attempt to validate the refresh token and generate a new access token
    match state.jwt_service.validate_refresh_token(refresh_token) {
        Ok(token_data) => {
            // Generate a new access token based on refresh token data            match state.jwt_service.generate_token(
                &token_data.user_id,
                &token_data.role,
                &token_data.canvas_id,
                token_data.discourse_id.as_deref(),
                None, // email is not stored in refresh token
                None, // name is not stored in refresh token
            ) {
                Ok(new_token) => {
                    // Get current time + 24h for expiration timestamp
                    let expires_at = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs() as usize + 24 * 3600;
                
                    // Return the new token
                    (StatusCode::OK, Json(RefreshTokenResponse {
                        token: new_token,
                        expires_at,
                    }))
                },
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": "Failed to generate new token"
                    })))
                }
            }
        },
        Err(_) => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid or expired refresh token"
            })))
        }
    }
}

// Handler for revoking a refresh token (logout)
pub async fn revoke_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    // Extract the refresh token from request
    let refresh_token = &payload.refresh_token;
    
    // Revoke the token
    match state.jwt_service.revoke_refresh_token(refresh_token) {
        Ok(_) => {
            (StatusCode::OK, Json(serde_json::json!({
                "message": "Token revoked successfully"
            })))
        },
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Invalid token or token not found"
            })))
        }
    }
}
