use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use axum::body::Body;
use axum::extract::FromRef;
use axum::http::header;
use std::sync::Arc;
use serde_json::json;
use anyhow::{Result, anyhow};
use crate::app_state::AppState;
use crate::auth::jwt::{JwtService, Claims};

/// Extract JWT token from Authorization header
fn extract_token(req: &Request) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_string())
            } else {
                None
            }
        })
}

/// Authentication middleware for Axum
///
/// This middleware extracts the JWT token from the Authorization header,
/// validates it, and adds the user claims to the request extensions.
///
/// # Arguments
/// * `req` - The incoming request
/// * `next` - The next middleware in the chain
///
/// # Returns
/// A Response or an error
pub async fn verify_auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let token = match extract_token(&req) {
        Some(token) => token,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    // Create JWT service
    let jwt_service = JwtService::new(state.jwt_secret.clone(), None);
    
    // Validate token
    let claims = match jwt_service.validate_token(&token) {
        Ok(claims) => claims,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    // Add claims to request extensions
    req.extensions_mut().insert(claims);
    
    // Continue with the request
    Ok(next.run(req).await)
}

/// Require a specific role for a route
///
/// This middleware checks if the user has the required role.
/// It should be used after the verify_auth middleware.
///
/// # Arguments
/// * `req` - The incoming request
/// * `next` - The next middleware in the chain
/// * `required_role` - The role required for this route
///
/// # Returns
/// A Response or an error
pub async fn require_role(
    mut req: Request,
    next: Next,
    required_role: &'static str,
) -> Result<Response, StatusCode> {
    // Get claims from request extensions
    let claims = req.extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Check if user has the required role
    if claims.role != required_role {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Continue with the request
    Ok(next.run(req).await)
}

/// Extract user ID from request extensions
///
/// This function extracts the user ID from the JWT claims in the request extensions.
/// It should be used after the verify_auth middleware.
///
/// # Arguments
/// * `req` - The incoming request
///
/// # Returns
/// A Result containing the user ID or an error
pub fn extract_user_id(req: &Request) -> Result<String, StatusCode> {
    // Get claims from request extensions
    let claims = req.extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Return user ID
    Ok(claims.sub.clone())
}

/// Create an unauthorized response
///
/// # Returns
/// A Response with a 401 status code and a JSON error message
pub fn unauthorized_response() -> Response {
    let body = Json(json!({
        "error": "Unauthorized",
        "message": "You must be logged in to access this resource"
    }));
    
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&body.0).unwrap()))
        .unwrap()
}

/// Create a forbidden response
///
/// # Returns
/// A Response with a 403 status code and a JSON error message
pub fn forbidden_response() -> Response {
    let body = Json(json!({
        "error": "Forbidden",
        "message": "You do not have permission to access this resource"
    }));
    
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&body.0).unwrap()))
        .unwrap()
}
