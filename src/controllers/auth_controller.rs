use crate::auth::jwt_service::{JwtService, User, JwtError};
use crate::services::canvas_auth_service::CanvasAuthService;
use crate::services::discourse_sso_service::DiscourseSsoService;
use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use thiserror::Error;

/// Error types for authentication controller
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("SSO error: {0}")]
    SsoError(String),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
}

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(default = "default_source")]
    pub source: String,
}

fn default_source() -> String {
    "canvas".to_string()
}

/// Login response with JWT token
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// SSO query parameters
#[derive(Debug, Deserialize)]
pub struct SsoParams {
    pub sso: String,
    pub sig: String,
}

/// Authentication Controller for handling user authentication flows
pub struct AuthController {
    jwt_service: Arc<JwtService>,
    canvas_auth_service: Arc<CanvasAuthService>,
    discourse_sso_service: Arc<DiscourseSsoService>,
}

impl AuthController {
    /// Create a new authentication controller
    pub fn new(
        jwt_service: Arc<JwtService>,
        canvas_auth_service: Arc<CanvasAuthService>,
        discourse_sso_service: Arc<DiscourseSsoService>,
    ) -> Self {
        AuthController {
            jwt_service,
            canvas_auth_service,
            discourse_sso_service,
        }
    }
    
    /// Handle user login and issue JWT tokens
    ///
    /// # Route
    /// POST /api/v1/auth/login
    pub async fn login(&self, payload: Json<LoginRequest>) -> Result<impl IntoResponse, AuthError> {
        // Authenticate user based on source
        let user = match self.authenticate_user(&payload.username, &payload.password, &payload.source).await {
            Ok(user) => user,
            Err(e) => {
                return Err(AuthError::AuthenticationFailed(e.to_string()));
            }
        };
        
        // Generate JWT token
        let token = self.jwt_service.generate_jwt_token(&user)?;
        
        // Return token to client
        let response = LoginResponse { token };
        Ok((StatusCode::OK, Json(response)))
    }
    
    /// Handle SSO authentication for Discourse
    /// Creates a bridge between Canvas OAuth tokens and Discourse SSO
    ///
    /// # Route
    /// GET /api/v1/auth/discourse-sso
    pub async fn handle_discourse_sso(
        &self,
        headers: HeaderMap,
        query: Query<SsoParams>,
    ) -> Result<impl IntoResponse, AuthError> {
        // Get authorization header
        let auth_header = headers
            .get("Authorization")
            .ok_or_else(|| AuthError::InvalidRequest("Authorization header is required".to_string()))?
            .to_str()
            .map_err(|_| AuthError::InvalidRequest("Invalid Authorization header".to_string()))?;
        
        // Verify the user is authenticated in Canvas
        let canvas_user = self.canvas_auth_service.authenticate_canvas_user(auth_header)
            .await
            .map_err(|e| AuthError::AuthenticationFailed(format!("Canvas authentication failed: {}", e)))?;
        
        if canvas_user.is_none() {
            return Err(AuthError::AuthenticationFailed("Canvas authentication required".to_string()));
        }
        
        // Unwrap the user since we've checked it's not None
        let canvas_user = canvas_user.unwrap();
        
        // Generate Discourse SSO payload
        let sso_payload = self.discourse_sso_service.generate_discourse_sso_payload(
            &canvas_user,
            &query.sso,
            &query.sig,
        )
        .await
        .map_err(|e| AuthError::SsoError(format!("SSO generation failed: {}", e)))?;
        
        // Get Discourse URL from environment variable
        let discourse_url = env::var("DISCOURSE_URL")
            .unwrap_or_else(|_| "http://discourse.example.com".to_string());
        
        // Redirect to Discourse with SSO payload
        let redirect_url = format!("{}/session/sso_login?{}", discourse_url, sso_payload);
        Ok(Redirect::to(&redirect_url))
    }
    
    /// Convert errors to HTTP responses
    pub fn handle_error(err: AuthError) -> Response {
        let (status, error_message) = match err {
            AuthError::AuthenticationFailed(_) => (StatusCode::UNAUTHORIZED, err.to_string()),
            AuthError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, err.to_string()),
            AuthError::JwtError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication failed".to_string()),
            AuthError::SsoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "SSO authentication failed".to_string()),
            AuthError::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server error".to_string()),
        };
        
        let body = Json(ErrorResponse {
            error: error_message,
        });
        
        (status, body).into_response()
    }
    
    // Private helper methods
    
    /// Authenticate a user against the appropriate service
    async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
        source: &str,
    ) -> Result<User, AuthError> {
        match source {
            "canvas" => {
                // Authenticate against Canvas
                self.canvas_auth_service.authenticate_user(username, password)
                    .await
                    .map_err(|e| AuthError::AuthenticationFailed(e.to_string()))
            },
            "discourse" => {
                // In a real implementation, we would authenticate against Discourse
                // For now, we'll just return an error
                Err(AuthError::AuthenticationFailed("Discourse authentication not implemented".to_string()))
            },
            _ => Err(AuthError::InvalidRequest(format!("Unknown authentication source: {}", source))),
        }
    }
}
