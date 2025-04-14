use axum::{
    extract::{Json, State, Query},
    http::StatusCode,
    response::IntoResponse,
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use http::Uri;
use oauth2::{PkceCodeVerifier, CsrfToken};

use crate::AppState;
use crate::models::user::User;
use crate::services::unified_auth_service::UnifiedAuthService;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub source: Option<String>, // "canvas" or "local"
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

#[derive(Debug, Deserialize)]
pub struct OAuth2Callback {
    pub code: String,
    pub state: String,
}

// Session storage for PKCE and CSRF tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Session {
    pub csrf_token: String,
    pub pkce_verifier: String,
    pub return_url: Option<String>,
}

/// Handler for user login via credentials
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let source = payload.source.unwrap_or_else(|| "local".to_string());
    
    match source.as_str() {
        "canvas" => {
            // Redirect to Canvas OAuth flow
            let (auth_url, csrf_token, pkce_verifier) = state.auth_service.start_oauth_flow();
            
            // Store CSRF token and PKCE verifier in session
            let session = OAuth2Session {
                csrf_token: csrf_token.secret().clone(),
                pkce_verifier: serde_json::to_string(&pkce_verifier).unwrap(),
                return_url: None,
            };
            
            // In a real implementation, you would store the session in Redis or a database
            // For now, we'll just return the auth URL
            (StatusCode::OK, Json(serde_json::json!({
                "auth_url": auth_url,
                "message": "Please complete OAuth2 authentication flow"
            })))
        },
        _ => {
            // Authenticate with username/password
            match state.auth_service.authenticate_with_credentials(
                &payload.username, 
                &payload.password,
                &state.db
            ).await {
                Ok((token, refresh_token, user)) => {
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
                Err(error) => {
                    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                        "error": error
                    })))
                }
            }
        }
    }
}

/// Handler for OAuth2 callback
pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<OAuth2Callback>,
) -> impl IntoResponse {
    // In a real implementation, you would retrieve the session from Redis or a database
    // For this example, we'll simulate session retrieval
    let pkce_verifier = PkceCodeVerifier::new("simulated_pkce_verifier".to_string());
    
    // Complete OAuth flow
    match state.auth_service.complete_oauth_flow(&params.code, pkce_verifier).await {
        Ok(oauth_token) => {
            // Authenticate with Canvas using OAuth token
            match state.auth_service.authenticate_with_canvas(&oauth_token, &state.canvas_client).await {
                Ok((token, refresh_token, user)) => {
                    // In a real implementation, you would redirect to a frontend page with the token
                    // For this example, we'll just return the token
                    (StatusCode::OK, Json(serde_json::json!({
                        "token": token,
                        "refresh_token": refresh_token,
                        "user": user
                    })))
                },
                Err(error) => {
                    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                        "error": error
                    })))
                }
            }
        },
        Err(error) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": error
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
    
    // Validate the existing token and refresh
    match state.auth_service.refresh_token(token).await {
        Ok((new_token, new_refresh_token)) => {
            (StatusCode::OK, Json(serde_json::json!({
                "token": new_token,
                "refresh_token": new_refresh_token
            })))
        },
        Err(error) => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": error
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
    
    // Validate the token
    match state.auth_service.validate_token(token) {
        Ok(claims) => {
            // Check if user exists
            match state.db.check_user_exists(&claims.sub).await {
                true => (StatusCode::OK, Json(serde_json::json!({
                    "valid": true,
                    "user_id": claims.sub,
                    "role": claims.role,
                    "canvas_id": claims.canvas_id,
                    "discourse_id": claims.discourse_id
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