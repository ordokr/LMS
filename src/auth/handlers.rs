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
    pub user_id: String,
    pub role: String,
    pub canvas_id: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Authenticate user against database
    match state.db.authenticate_user(&payload.username, &payload.password).await {
        Some(user) => {
            // Generate JWT token
            match jwt::generate_token(&user.id, &user.role, &user.canvas_id) {
                Ok(token) => {
                    let response = LoginResponse {
                        token,
                        user_id: user.id,
                        role: user.role,
                        canvas_id: user.canvas_id,
                    };
                    
                    (StatusCode::OK, Json(response))
                },
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": "Failed to generate token"
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
            match jwt::generate_token(&claims.sub, &claims.role, &claims.canvas_id) {
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