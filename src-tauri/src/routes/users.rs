use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    models::User,
    database::UserRepository,
    AppState,
    auth,
};

#[derive(Debug, Deserialize)]
pub struct RegisterUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub trust_level: i32,
}

// Register a new user
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = UserRepository::new(&conn);
    
    // Check if username already exists
    if let Ok(Some(_)) = repo.find_by_username(&payload.username) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Username already taken"})),
        )
            .into_response();
    }
    
    // Hash the password
    let password_hash = match auth::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password hashing error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to process password"})),
            )
                .into_response();
        }
    };
    
    // Create new user
    let mut user = User::new(
        payload.username,
        payload.email,
        password_hash,
    );
    
    // Set display name if provided
    if let Some(display_name) = payload.display_name {
        user.display_name = Some(display_name);
    }
    
    // Save user to database
    match repo.create(&user) {
        Ok(id) => {
            // Get the created user with ID
            match repo.find_by_id(id) {
                Ok(Some(created_user)) => {
                    // Generate JWT token
                    match auth::generate_token(&created_user) {
                        Ok(token) => {
                            let user_response = UserResponse {
                                id: created_user.id.unwrap(),
                                username: created_user.username,
                                email: created_user.email,
                                display_name: created_user.display_name().to_string(),
                                avatar_url: created_user.avatar_url,
                                is_admin: created_user.is_admin,
                                trust_level: created_user.trust_level,
                            };
                            
                            (
                                StatusCode::CREATED,
                                Json(AuthResponse {
                                    token,
                                    user: user_response,
                                }),
                            )
                                .into_response()
                        },
                        Err(e) => {
                            eprintln!("Token generation error: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(serde_json::json!({"error": "Failed to generate authentication token"})),
                            )
                                .into_response()
                        }
                    }
                },
                _ => {
                    (
                        StatusCode::CREATED,
                        Json(serde_json::json!({"id": id, "message": "User created successfully"})),
                    )
                        .into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create user"})),
            )
                .into_response()
        }
    }
}

// User login
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = UserRepository::new(&conn);
    
    // Find the user
    match repo.find_by_username(&payload.username) {
        Ok(Some(user)) => {
            // Check if user is suspended
            if user.is_suspended {
                if let Some(until) = user.suspended_until {
                    if until > chrono::Utc::now() {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(serde_json::json!({"error": "Account is suspended"})),
                        )
                            .into_response();
                    }
                }
            }
            
            // Verify password
            match auth::verify_password(&payload.password, &user.password_hash) {
                Ok(true) => {
                    // Update last seen time
                    let _ = repo.update_last_seen(user.id.unwrap());
                    
                    // Generate JWT token
                    match auth::generate_token(&user) {
                        Ok(token) => {
                            let user_response = UserResponse {
                                id: user.id.unwrap(),
                                username: user.username,
                                email: user.email,
                                display_name: user.display_name().to_string(),
                                avatar_url: user.avatar_url,
                                is_admin: user.is_admin,
                                trust_level: user.trust_level,
                            };
                            
                            (
                                StatusCode::OK,
                                Json(AuthResponse {
                                    token,
                                    user: user_response,
                                }),
                            )
                                .into_response()
                        },
                        Err(e) => {
                            eprintln!("Token generation error: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(serde_json::json!({"error": "Failed to generate authentication token"})),
                            )
                                .into_response()
                        }
                    }
                },
                Ok(false) => {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({"error": "Invalid username or password"})),
                    )
                        .into_response()
                },
                Err(e) => {
                    eprintln!("Password verification error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "Authentication error"})),
                    )
                        .into_response()
                }
            }
        },
        Ok(None) => {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid username or password"})),
            )
                .into_response()
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Authentication error"})),
            )
                .into_response()
        }
    }
}

// Get current user info from token
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Extract the token from the Authorization header
    let auth_header = match headers.get("Authorization") {
        Some(header) => header,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "No authorization token provided"})),
            )
                .into_response();
        }
    };
    
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid authorization header"})),
            )
                .into_response();
        }
    };
    
    // Check for "Bearer " prefix
    if !auth_str.starts_with("Bearer ") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid token format"})),
        )
            .into_response();
    }
    
    let token = &auth_str[7..]; // Remove "Bearer " prefix
    
    // Verify the token
    let user_id = match auth::get_user_id_from_token(token) {
        Some(id) => id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid or expired token"})),
            )
                .into_response();
        }
    };
    
    // Get user from database
    let conn = state.conn.lock().await;
    let repo = UserRepository::new(&conn);
    
    match repo.find_by_id(user_id) {
        Ok(Some(user)) => {
            let user_response = UserResponse {
                id: user.id.unwrap(),
                username: user.username,
                email: user.email,
                display_name: user.display_name().to_string(),
                avatar_url: user.avatar_url,
                is_admin: user.is_admin,
                trust_level: user.trust_level,
            };
            
            (StatusCode::OK, Json(user_response)).into_response()
        },
        Ok(None) => {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "User not found"})),
            )
                .into_response()
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve user"})),
            )
                .into_response()
        }
    }
}

// Update user profile
pub async fn update_user_profile(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
    // Extract and verify token (same as get_current_user)
    let auth_header = match headers.get("Authorization") {
        Some(header) => header,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "No authorization token provided"})),
            )
                .into_response();
        }
    };
    
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid authorization header"})),
            )
                .into_response();
        }
    };
    
    if !auth_str.starts_with("Bearer ") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid token format"})),
        )
            .into_response();
    }
    
    let token = &auth_str[7..];
    let user_id = match auth::get_user_id_from_token(token) {
        Some(id) => id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid or expired token"})),
            )
                .into_response();
        }
    };
    
    // Get and update user
    let conn = state.conn.lock().await;
    let repo = UserRepository::new(&conn);
    
    match repo.find_by_id(user_id) {
        Ok(Some(mut user)) => {
            // Update fields if provided
            if let Some(display_name) = payload.display_name {
                user.display_name = Some(display_name);
            }
            
            if let Some(avatar_url) = payload.avatar_url {
                user.avatar_url = Some(avatar_url);
            }
            
            if let Some(bio) = payload.bio {
                user.bio = Some(bio);
            }
            
            if let Some(website) = payload.website {
                user.website = Some(website);
            }
            
            if let Some(location) = payload.location {
                user.location = Some(location);
            }
            
            // Save the updated user
            match repo.update(&user) {
                Ok(_) => {
                    let user_response = UserResponse {
                        id: user.id.unwrap(),
                        username: user.username,
                        email: user.email,
                        display_name: user.display_name().to_string(),
                        avatar_url: user.avatar_url,
                        is_admin: user.is_admin,
                        trust_level: user.trust_level,
                    };
                    
                    (StatusCode::OK, Json(user_response)).into_response()
                },
                Err(e) => {
                    eprintln!("Database error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "Failed to update user profile"})),
                    )
                        .into_response()
                }
            }
        },
        Ok(None) => {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "User not found"})),
            )
                .into_response()
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve user"})),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
}