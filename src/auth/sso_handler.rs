use axum::{
    extract::{Json, Query, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;
use crate::AppState;

/// Request to generate an SSO URL for Discourse
#[derive(Debug, Deserialize)]
pub struct GenerateSsoRequest {
    pub user_id: String,
    pub return_url: Option<String>,
}

/// Response with the SSO URL
#[derive(Debug, Serialize)]
pub struct SsoUrlResponse {
    pub sso_url: String,
}

/// Query parameters for SSO callback
#[derive(Debug, Deserialize)]
pub struct SsoCallback {
    pub token: String,
    pub return_to: Option<String>,
}

/// Handler to generate an SSO URL for Discourse
pub async fn generate_sso_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GenerateSsoRequest>,
) -> impl IntoResponse {
    // Get user info from database
    match state.db.get_user_by_id(&payload.user_id).await {
        Some(user) => {
            // Generate a special short-lived SSO token            let sso_token = match state.jwt_service.generate_token(
                &user.id,
                &user.role,
                &user.canvas_id,
                user.discourse_id.as_deref(),
                Some(&user.email),
                Some(&user.name),
            ) {
                Ok(token) => token,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": "Failed to generate SSO token"
                    })))
                }
            };
            
            // Construct the SSO URL for Discourse
            let discourse_url = std::env::var("DISCOURSE_URL")
                .unwrap_or_else(|_| "http://localhost:4200".to_string());
                
            let mut sso_url = format!("{}/canvas-sso?token={}", discourse_url, sso_token);
            
            // Add return URL if provided
            if let Some(return_url) = payload.return_url {
                sso_url = format!("{}&return_to={}", sso_url, urlencoding::encode(&return_url));
            }
            
            (StatusCode::OK, Json(SsoUrlResponse { sso_url }))
        },
        None => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "User not found"
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
    match state.jwt_service.validate_token(&params.token) {
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
    match state.jwt_service.validate_token(&token) {
        Ok(claims) => {
            // Return user information for Discourse
            (StatusCode::OK, Json(serde_json::json!({
                "valid": true,
                "user_id": claims.sub,
                "email": claims.email, // Note: You'll need to add email to your Claims struct
                "name": claims.name,   // Note: You'll need to add name to your Claims struct
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
