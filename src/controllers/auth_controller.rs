use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{auth::jwt, models::user::User, AppState};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Find user by username
    let user = state.db.find_user_by_username(&payload.username).await
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();
    
    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Generate JWT token
    let token = jwt::generate_token(
        &user.id.to_string(), 
        &user.role, 
        &user.canvas_id.to_string()
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(LoginResponse { token, user }))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    canvas_id: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    // Check if user already exists
    if state.db.find_user_by_username(&payload.username).await.is_some() {
        return Err(StatusCode::CONFLICT);
    }
    
    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    
    // Create new user
    let user = User::new(
        payload.username,
        payload.email,
        password_hash,
        "student".to_string(),
        payload.canvas_id,
    );
    
    // Save user to database
    state.db.create_user(&user).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::CREATED)
}