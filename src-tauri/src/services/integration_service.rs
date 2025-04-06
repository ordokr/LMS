// Add these imports
use crate::api::auth::Claims;
use axum::extract::Extension;

// Then update your integration endpoints to extract and use claims
pub async fn some_protected_endpoint(
    Extension(claims): Extension<Claims>,
    // other parameters
) -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    // Now you can use claims.sub (user ID), claims.role, etc.
    // For example, only allow admins or teachers
    if claims.role != "admin" && claims.role != "teacher" {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    
    // Continue with your endpoint logic
    // ...
}