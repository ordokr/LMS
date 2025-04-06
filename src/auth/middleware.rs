use super::jwt;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::AppState;

pub async fn auth_middleware<B>(
    State(state): State<Arc<AppState>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Get token from the Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate the token
    match jwt::validate_token(token) {
        Ok(claims) => {
            // Check if user exists in database
            match state.db.check_user_exists(&claims.sub).await {
                true => Ok(next.run(req).await),
                false => Err(StatusCode::UNAUTHORIZED),
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

// Role-based authorization middleware
pub async fn require_role<B>(
    role: String,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match jwt::get_user_role_from_token(token) {
        Some(user_role) if user_role == role => Ok(next.run(req).await),
        _ => Err(StatusCode::FORBIDDEN),
    }
}