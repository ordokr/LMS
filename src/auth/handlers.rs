use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::jwt;
use crate::AppState;
use crate::models::user::User;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_at: usize,
    pub user_id: String,
    pub role: String,
    pub canvas_id: String,
    pub discourse_id: Option<String>,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Authenticate user against database
    match state.db.authenticate_user(&payload.username, &payload.password).await {
        Some(user) => {
            // Generate both access token and refresh token            match state.jwt_service.generate_auth_tokens(
                &user.id, 
                &user.role, 
                &user.canvas_id,
                user.discourse_id.as_deref(),
                Some(&user.email),
                Some(&user.name)
            ) {
                Ok((token, refresh_token)) => {
                    // Calculate token expiration (24 hours from now)
                    let expires_at = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs() as usize + 24 * 3600;
                    
                    let response = LoginResponse {
                        token,
                        refresh_token,
                        expires_at,
                        user_id: user.id,
                        role: user.role,
                        canvas_id: user.canvas_id,
                        discourse_id: user.discourse_id,
                    };
                    
                    (StatusCode::OK, Json(response))
                },
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": "Failed to generate authentication tokens"
                    })))
                }
            }
        },
        None => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid credentials"
            })))
        }
    }
}

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    auth_header: Option<String>,
) -> impl IntoResponse {
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid authorization header"
        }))),
    };
    
    // Validate the existing token
    match jwt::validate_token(token) {
        Ok(claims) => {
            // Check if user still exists
            if !state.db.check_user_exists(&claims.sub).await {
                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                    "error": "User not found"
                })));
            }
              // Generate a new token
            match jwt::generate_token(
                &claims.sub,
                &claims.role,
                &claims.canvas_id,
                claims.discourse_id.as_deref(),
                claims.email.as_deref(),
                claims.name.as_deref()
            ) {
                Ok(new_token) => {
                    (StatusCode::OK, Json(serde_json::json!({
                        "token": new_token
                    })))
                },
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": "Failed to refresh token"
                    })))
                }
            }
        },
        Err(_) => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid token"
            })))
        }
    }
}

pub async fn verify(
    State(state): State<Arc<AppState>>,
    auth_header: Option<String>,
) -> impl IntoResponse {
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "valid": false,
            "error": "Invalid authorization header"
        }))),
    };
    
    match jwt::validate_token(token) {
        Ok(claims) => {
            // Check if user exists
            match state.db.check_user_exists(&claims.sub).await {
                true => (StatusCode::OK, Json(serde_json::json!({
                    "valid": true,
                    "user_id": claims.sub,
                    "role": claims.role
                }))),
                false => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                    "valid": false,
                    "error": "User not found"
                }))),
            }
        },
        Err(_) => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "valid": false,
                "error": "Invalid token"
            })))
        }
    }
}