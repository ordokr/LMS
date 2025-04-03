use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::core::errors::AppError;
use crate::core::auth::AuthService;
use crate::database::repositories::user::UserRepository;
use crate::shared::models::user::{RegisterRequest, LoginRequest, AuthResponse};

pub async fn register(
    State(user_repo): State<Arc<UserRepository>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    if request.name.trim().is_empty() {
        return Err(AppError::ValidationError("Name cannot be empty".to_string()));
    }
    
    if request.email.trim().is_empty() || !request.email.contains('@') {
        return Err(AppError::ValidationError("Valid email is required".to_string()));
    }
    
    if request.password.len() < 8 {
        return Err(AppError::ValidationError("Password must be at least 8 characters".to_string()));
    }
    
    // Create the user
    let user_id = user_repo.create_user(request).await?;
    
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id).await?;
    
    // Generate auth token
    let token = auth_service.generate_token(&user_profile)?;
    
    // Return the response
    let response = AuthResponse {
        token,
        user: user_profile.user,
        roles: user_profile.roles,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(user_repo): State<Arc<UserRepository>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Authenticate the user
    let user_profile = user_repo.authenticate_user(request).await?;
    
    // Generate auth token
    let token = auth_service.generate_token(&user_profile)?;
    
    // Return the response
    let response = AuthResponse {
        token,
        user: user_profile.user,
        roles: user_profile.roles,
    };
    
    Ok((StatusCode::OK, Json(response)))
}

pub async fn me(
    State(user_repo): State<Arc<UserRepository>>,
    user_id: crate::core::auth::CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id.0).await?;
    
    Ok((StatusCode::OK, Json(user_profile)))
}

pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    State(user_repo): State<Arc<UserRepository>>,
    user_id: crate::core::auth::CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id.0).await?;
    
    // Generate new token
    let token = auth_service.generate_token(&user_profile)?;
    
    // Return the response
    let response = AuthResponse {
        token,
        user: user_profile.user,
        roles: user_profile.roles,
    };
    
    Ok((StatusCode::OK, Json(response)))
}