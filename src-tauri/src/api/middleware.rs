use crate::api::auth::{verify_token, Claims};
use std::sync::Arc;

pub async fn auth_middleware<B>(
    req: axum::http::Request<B>, 
    next: axum::middleware::Next<B>
) -> Result<axum::response::Response, axum::http::StatusCode> {
    // Get token from Authorization header
    let auth_header = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header[7..].to_string()) // Skip "Bearer " prefix
            } else {
                None
            }
        });

    // If no Authorization header or invalid format, return 401
    let token = match auth_header {
        Some(token) => token,
        None => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    // Verify token
    let claims = match verify_token(&token) {
        Ok(claims) => claims,
        Err(_) => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    // Add claims to request extensions for route handlers to access
    let mut req_with_ext = req;
    req_with_ext.extensions_mut().insert(claims);
    
    // Continue to the route handler
    let response = next.run(req_with_ext).await;
    Ok(response)
}