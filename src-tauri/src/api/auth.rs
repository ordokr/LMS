use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use tauri::State;
use crate::models::auth::User;
use crate::db::user_repository::UserRepository;

use crate::core::errors::AppError;
use crate::core::auth::AuthService;
use crate::database::repositories::user::UserRepository as AxumUserRepository;
use crate::shared::models::user::{RegisterRequest, LoginRequest as AxumLoginRequest, AuthResponse as AxumAuthResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // Subject (user ID)
    pub name: String,        // User's name
    pub email: String,       // User's email
    pub role: String,        // User's role (admin, teacher, student)
    pub exp: i64,            // Expiration time
    pub iat: i64,            // Issued at time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[tauri::command]
pub async fn login(
    login_req: LoginRequest, 
    user_repo: State<'_, UserRepository>
) -> Result<AuthResponse, String> {
    // Find user by email
    let user = user_repo.find_by_email(&login_req.email).await
        .map_err(|e| format!("Authentication error: {}", e))?;
    
    // Verify password (assuming password is hashed in the database)
    if !verify_password(&login_req.password, &user.password_hash) {
        return Err("Invalid email or password".into());
    }
    
    // Generate JWT token
    let token = generate_token(&user)
        .map_err(|e| format!("Failed to generate token: {}", e))?;
    
    Ok(AuthResponse {
        token,
        user,
    })
}

fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    
    let claims = Claims {
        sub: user.id.to_string(),
        name: user.display_name.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
        iat: Utc::now().timestamp(),
    };
    
    // In a real app, this key would be loaded from a secure environment variable
    let secret = "your_jwt_secret_key_here"; // Replace with actual secret from config
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
}

#[tauri::command]
pub fn verify_token(token: &str) -> Result<Claims, String> {
    let secret = "your_jwt_secret_key_here"; // Replace with actual secret from config
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256)
    ).map_err(|e| format!("Token verification failed: {}", e))?;
    
    Ok(token_data.claims)
}

fn verify_password(provided_password: &str, stored_hash: &str) -> bool {
    // In a real implementation, you would use bcrypt or argon2 to verify
    // For simplicity, we'll assume the stored_hash is a bcrypt hash
    match bcrypt::verify(provided_password, stored_hash) {
        Ok(result) => result,
        Err(_) => false
    }
}

pub async fn register(
    State(user_repo): State<Arc<AxumUserRepository>>,
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
    let response = AxumAuthResponse {
        token,
        user: user_profile.user,
        roles: user_profile.roles,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(user_repo): State<Arc<AxumUserRepository>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<AxumLoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Authenticate the user
    let user_profile = user_repo.authenticate_user(request).await?;
    
    // Generate auth token
    let token = auth_service.generate_token(&user_profile)?;
    
    // Return the response
    let response = AxumAuthResponse {
        token,
        user: user_profile.user,
        roles: user_profile.roles,
    };
    
    Ok((StatusCode::OK, Json(response)))
}

pub async fn me(
    State(user_repo): State<Arc<AxumUserRepository>>,
    user_id: crate::core::auth::CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id.0).await?;
    
    Ok((StatusCode::OK, Json(user_profile)))
}

pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    State(user_repo): State<Arc<AxumUserRepository>>,
    user_id: crate::core::auth::CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id.0).await?;
    
    // Generate new token
    let token = auth_service.generate_token(&user_profile)?;
    
    // Return the response
    let response = AxumAuthResponse {
        token,
        user: user_profile.user,
        roles: user_profile.roles,
    };
    
    Ok((StatusCode::OK, Json(response)))
}