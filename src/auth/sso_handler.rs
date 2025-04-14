use axum::{
    extract::{Json, State, Query},
    http::StatusCode,
    response::IntoResponse,
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use http::Uri;

use crate::AppState;
use crate::models::user::User;
use crate::services::unified_auth_service::UnifiedAuthService;

#[derive(Debug, Deserialize)]
pub struct SsoRequest {
    pub return_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SsoCallback {
    pub token: String,
    pub return_to: Option<String>,
}

/// Handler for initiating SSO flow to Discourse
pub async fn sso_init(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SsoRequest>,
    auth_header: Option<String>,
) -> impl IntoResponse {
    // Extract token from authorization header
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Authentication required"
        }))),
    };
    
    // Validate the token
    match state.auth_service.validate_token(token) {
        Ok(claims) => {
            // Get user from database
            match state.db.get_user_by_id(&claims.sub).await {
                Some(user) => {
                    // Generate SSO URL with token
                    match state.auth_service.generate_sso_token(&user, params.return_url.as_deref()) {
                        Ok(sso_token) => {
                            // Construct SSO URL
                            let discourse_url = std::env::var("DISCOURSE_URL")
                                .unwrap_or_else(|_| "http://localhost:3000".to_string());
                            
                            let sso_path = "/auth/canvas/sso";
                            
                            let mut url = format!("{}{}", discourse_url, sso_path);
                            url.push_str(&format!("?token={}", sso_token));
                            
                            if let Some(return_url) = &params.return_url {
                                url.push_str(&format!("&return_url={}", urlencoding::encode(return_url)));
                            }
                            
                            (StatusCode::OK, Json(serde_json::json!({ "sso_url": url })))
                        },
                        Err(e) => {
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                "error": format!("Failed to generate SSO token: {}", e)
                            })))
                        }
                    }
                },
                None => {
                    (StatusCode::NOT_FOUND, Json(serde_json::json!({
                        "error": "User not found"
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

/// Handler for SSO callback from Discourse
pub async fn sso_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SsoCallback>,
) -> impl IntoResponse {
    // Validate the token
    match state.auth_service.validate_token(&params.token) {
        Ok(claims) => {
            // Check if the user exists
            match state.db.get_user_by_id(&claims.sub).await {
                Some(user) => {
                    // Process SSO authentication
                    // If the user doesn't have a Discourse ID yet, we'd handle that here
                    
                    // For a real implementation, you might:
                    // 1. Create the user in Discourse if they don't exist
                    // 2. Update mapping tables
                    // 3. Log the SSO event
                    
                    // Redirect back to Canvas or the specified return URL
                    let return_url = params.return_to
                        .unwrap_or_else(|| "/dashboard".to_string());
                        
                    let uri = match Uri::try_from(&return_url) {
                        Ok(uri) => uri,
                        Err(_) => {
                            return (
                                StatusCode::BAD_REQUEST,
                                Json(serde_json::json!({"error": "Invalid return URL"})),
                            );
                        }
                    };
                    
                    Redirect::to(uri)
                },
                None => {
                    (StatusCode::NOT_FOUND, Json(serde_json::json!({
                        "error": "User not found"
                    })))
                }
            }
        },
        Err(_) => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid SSO token"
            })))
        }
    }
}

/// Handler for Discourse SSO verification
/// This is called by Discourse to verify a user
pub async fn verify_discourse_sso(
    State(state): State<Arc<AppState>>,
    Json(token): Json<String>,
) -> impl IntoResponse {
    // Validate the token
    match state.auth_service.validate_token(&token) {
        Ok(claims) => {
            // Return user information for Discourse
            (StatusCode::OK, Json(serde_json::json!({
                "valid": true,
                "user_id": claims.sub,
                "email": claims.email,
                "name": claims.name,
                "external_id": claims.canvas_id,
                "admin": claims.role == "admin",
                "moderator": claims.role == "teacher" || claims.role == "admin"
            })))
        },
        Err(_) => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "valid": false,
                "error": "Invalid token"
            })))
        }
    }
}
