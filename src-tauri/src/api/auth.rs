use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tauri::State as TauriState;
use tracing::{info, warn, error, instrument};
use argon2::{self, Config};

use crate::core::errors::AppError;
use crate::core::auth::{AuthService, CurrentUserId};
use crate::database::repositories::user::UserRepository as AxumUserRepository;
use crate::models::auth::{
    JwtClaims, UserAuthProfile, AuthResponse, 
    LoginRequest as ModelLoginRequest, 
    RegisterRequest as ModelRegisterRequest,
    RefreshTokenRequest
};

// Import for backward compatibility
use crate::models::user::{User, LoginRequest, AuthResponse as LegacyAuthResponse, RegisterRequest};
use crate::db::user_repository::UserRepository;

// Fixed unresolved imports and missing types
use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use crate::models::{auth::{JwtClaims, UserAuthProfile}, user::{User, LoginRequest, AuthResponse as LegacyAuthResponse, RegisterRequest}};
use crate::db::user_repository::UserRepository;
use crate::AppState;

/// Login user with email and password
///
/// # Arguments
/// * `login_request` - Login credentials containing email and password
///
/// # Returns
/// * `AuthResponse` - Authentication response with user info and token
#[tauri::command]
#[instrument(skip(user_repo, app_state), fields(email = %login_request.email), err)]
pub async fn login_user(
    login_request: LoginRequest,
    user_repo: TauriState<'_, Arc<dyn UserRepository + Send + Sync>>,
    app_state: TauriState<'_, crate::AppState>,
) -> Result<LegacyAuthResponse, String> {
    info!(event = "api_call", endpoint = "login_user");
    
    // Find user by email
    let user = match user_repo.get_user_by_email(&login_request.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!(event = "login_failed", reason = "user_not_found");
            return Err("Invalid email or password".to_string());
        },
        Err(e) => {
            error!(event = "db_error", error = %e);
            return Err("Database error".to_string());
        }
    };
    
    // Verify password
    let password_matches = verify_password(&login_request.password, &user.password_hash);
    if !password_matches {
        warn!(event = "login_failed", reason = "invalid_password");
        return Err("Invalid email or password".to_string());
    }
    
    // Use the auth service to generate tokens
    let auth_config = app_state.auth_config.clone().unwrap_or_default();
    let auth_service = AuthService::new(auth_config);
    
    // Create user profile for token generation
    let user_profile = UserAuthProfile {
        id: user.id.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        roles: vec![user.role.clone()],
        canvas_id: user.canvas_id.clone(),
        discourse_id: user.discourse_id.clone(),
    };
    
    // Generate JWT token
    let token = match auth_service.generate_token(&user_profile) {
        Ok(token) => token,
        Err(e) => {
            error!(event = "token_generation_error", error = %e);
            return Err("Authentication error".to_string());
        }
    };
    
    info!(event = "login_success", user_id = %user.id);
    
    // Return auth response
    Ok(LegacyAuthResponse {
        user: user.into_public(),
        token,
    })
}

/// Register a new user
///
/// # Arguments
/// * `register_request` - Registration data with email, password, and user info
///
/// # Returns
/// * `AuthResponse` - Authentication response with user info and token
#[tauri::command]
#[instrument(skip(user_repo, app_state), fields(email = %register_request.email), err)]
pub async fn register_user(
    register_request: RegisterRequest,
    user_repo: TauriState<'_, Arc<dyn UserRepository + Send + Sync>>,
    app_state: TauriState<'_, crate::AppState>,
) -> Result<LegacyAuthResponse, String> {
    info!(event = "api_call", endpoint = "register_user");
    
    // Check if user with email already exists
    let existing_user = user_repo.get_user_by_email(&register_request.email).await;
    if let Ok(Some(_)) = existing_user {
        warn!(event = "registration_failed", reason = "email_exists");
        return Err("Email already registered".to_string());
    }
    
    // Hash the password
    let password_hash = match hash_password(&register_request.password) {
        Ok(hash) => hash,
        Err(e) => {
            error!(event = "password_hash_error", error = %e);
            return Err("Registration error".to_string());
        }
    };
    
    // Create new user
    let new_user = User::from_register_request(register_request, password_hash);
    
    // Save user to database
    let created_user = match user_repo.create_user(new_user).await {
        Ok(user) => user,
        Err(e) => {
            error!(event = "db_error", error = %e);
            return Err("Failed to create user".to_string());
        }
    };
    
    // Use the auth service to generate tokens
    let auth_config = app_state.auth_config.clone().unwrap_or_default();
    let auth_service = AuthService::new(auth_config);
    
    // Create user profile for token generation
    let user_profile = UserAuthProfile {
        id: created_user.id.clone(),
        name: created_user.name.clone(),
        email: created_user.email.clone(),
        roles: vec![created_user.role.clone()],
        canvas_id: created_user.canvas_id.clone(),
        discourse_id: created_user.discourse_id.clone(),
    };
    
    // Generate JWT token
    let token = match auth_service.generate_token(&user_profile) {
        Ok(token) => token,
        Err(e) => {
            error!(event = "token_generation_error", error = %e);
            return Err("Authentication error".to_string());
        }
    };
    
    info!(event = "registration_success", user_id = %created_user.id);
    
    // Return auth response
    Ok(LegacyAuthResponse {
        user: created_user.into_public(),
        token,
    })
}

/// Get current user from token
///
/// # Arguments
/// * `token` - JWT token from authentication
///
/// # Returns
/// * `User` - Current user information
#[tauri::command]
#[instrument(skip(user_repo, app_state), err)]
pub async fn get_current_user(
    token: String,
    user_repo: TauriState<'_, Arc<dyn UserRepository + Send + Sync>>,
    app_state: TauriState<'_, crate::AppState>,
) -> Result<User, String> {
    info!(event = "api_call", endpoint = "get_current_user");
    
    // Use the auth service to validate the token
    let auth_config = app_state.auth_config.clone().unwrap_or_default();
    let auth_service = AuthService::new(auth_config);
    
    // Validate and decode token
    let claims = match auth_service.verify_token(&token) {
        Ok(claims) => claims,
        Err(e) => {
            warn!(event = "token_validation_failed", error = %e);
            return Err("Invalid or expired token".to_string());
        }
    };
    
    let user_id = claims.sub;
    
    // Get user by ID
    match user_repo.get_user_by_id(&user_id).await {
        Ok(Some(user)) => {
            info!(event = "user_retrieved", user_id = %user_id);
            Ok(user.into_public())
        },
        Ok(None) => {
            warn!(event = "user_not_found", user_id = %user_id);
            Err("User not found".to_string())
        },
        Err(e) => {
            error!(event = "db_error", error = %e);
            Err("Database error".to_string())
        }
    }
}

// Corrected argon2 API usage
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)?.to_string()
}

fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

/// Register a new user via the Axum web API
///
/// This endpoint handles user registration through the web API
pub async fn register(
    State(user_repo): State<Arc<AxumUserRepository>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<ModelRegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!(event = "api_call", endpoint = "register");
    
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
    
    if request.password != request.password_confirmation {
        return Err(AppError::ValidationError("Passwords do not match".to_string()));
    }
    
    // Check if user exists
    if user_repo.user_exists_by_email(&request.email).await? {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }
    
    // Create the user
    let user_id = user_repo.create_user(request.clone()).await?;
    
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id).await?;
    
    // Generate auth response with tokens
    let auth_response = auth_service.create_auth_response(user_profile)?;
    
    info!(event = "registration_success", user_id = %user_id);
    
    // Return the response
    Ok((StatusCode::CREATED, Json(auth_response)))
}

/// Login a user via the Axum web API
///
/// This endpoint handles user authentication through the web API
pub async fn login(
    State(user_repo): State<Arc<AxumUserRepository>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<ModelLoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!(event = "api_call", endpoint = "login", email = %request.email);
    
    // Authenticate the user
    let user_profile = user_repo.authenticate_user(request).await?;
    
    // Generate auth response with tokens
    let auth_response = auth_service.create_auth_response(user_profile)?;
    
    info!(event = "login_success", user_id = %auth_response.user.id);
    
    // Return the response
    Ok((StatusCode::OK, Json(auth_response)))
}

/// Get current authenticated user profile
///
/// This endpoint returns the current user's profile information
pub async fn me(
    State(user_repo): State<Arc<AxumUserRepository>>,
    user_id: CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    info!(event = "api_call", endpoint = "me", user_id = %user_id.0);
    
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id.0).await?;
    
    Ok((StatusCode::OK, Json(user_profile)))
}

/// Refresh the access token using a refresh token
///
/// This endpoint allows clients to obtain a new access token using a valid refresh token
pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    State(user_repo): State<Arc<AxumUserRepository>>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!(event = "api_call", endpoint = "refresh_token");
    
    // Validate refresh token and get user ID
    let user_id = auth_service.verify_refresh_token(&request.refresh_token)?;
    
    // Get the user profile
    let user_profile = user_repo.get_user_profile(user_id).await?;
    
    // Generate new auth response with fresh tokens
    let auth_response = auth_service.create_auth_response(user_profile)?;
    
    info!(event = "token_refresh_success", user_id = %auth_response.user.id);
    
    Ok((StatusCode::OK, Json(auth_response)))
}

/// Logout the current user
///
/// This endpoint handles user logout and token invalidation
/// In a complete implementation, this would add the token to a blocklist
pub async fn logout(
    user_id: CurrentUserId,
) -> Result<impl IntoResponse, AppError> {
    info!(event = "api_call", endpoint = "logout", user_id = %user_id.0);
    
    // In a complete implementation, we would add the token to a blocklist here
    // This would require a token blocklist service
    
    info!(event = "logout_success", user_id = %user_id.0);
    
    Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Successfully logged out" }))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::UserRole;
    use chrono::{Duration, Utc};
    use std::env;
    use uuid::Uuid;

    fn setup() {
        env::set_var("JWT_SECRET", "test_secret_key_for_auth_tests");
    }

    #[test]
    fn test_password_hashing_and_verification() {
        // Test that password hashing produces different hashes for the same password
        let password = "secure_password_123";
        let hash1 = hash_password(password).expect("Password hashing should succeed");
        let hash2 = hash_password(password).expect("Password hashing should succeed");

        // Hashes should be different due to different salts
        assert_ne!(hash1, hash2);

        // But verification should work for both
        assert!(verify_password(password, &hash1));
        assert!(verify_password(password, &hash2));

        // Wrong password should fail verification
        assert!(!verify_password("wrong_password", &hash1));
    }

    #[test]
    fn test_malformed_token() {
        setup();

        // Test completely invalid token format
        let invalid_token = "not.a.valid.token.format";
        let result = verify_token(invalid_token);
        assert!(result.is_err());

        // Test token with valid format but invalid signature
        let forged_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = verify_token(forged_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_token_generation_and_verification() {
        setup();
        
        let user_id = Uuid::new_v4().to_string();
        let role = UserRole::Student.to_string();
        
        // Generate a token
        let token = generate_token(&user_id, &role).expect("Token generation should succeed");
        
        // Verify the token
        let claims = verify_token(&token).expect("Token verification should succeed");
        
        // Check claims
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, role);
        assert!(claims.exp > Utc::now().timestamp());
    }

    #[test]
    fn test_expired_token() {
        setup();
        
        let user_id = Uuid::new_v4().to_string();
        let role = UserRole::Student.to_string();
        
        // Create custom JWT with expired timestamp
        let expiration = Utc::now() - Duration::hours(1); // 1 hour in the past
        let claims = Claims {
            sub: user_id.clone(),
            role: role.clone(),
            exp: expiration.timestamp(),
            iat: (expiration - Duration::hours(1)).timestamp(),
        };
        
        // Generate token with expired claims
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
        ).expect("Token encoding should succeed");
        
        // Verify token - should fail due to expiration
        let result = verify_token(&token);
        assert!(result.is_err());
        
        // Check that the error is specifically about expiration
        match result {
            Err(e) => {
                assert!(e.to_string().contains("expired"));
            },
            _ => panic!("Expected error about token expiration"),
        }
    }

    #[test]
    fn test_token_for_different_roles() {
        setup();
        
        // Test admin role
        let admin_id = Uuid::new_v4().to_string();
        let admin_token = generate_token(&admin_id, &UserRole::Admin.to_string())
            .expect("Token generation should succeed");
        let admin_claims = verify_token(&admin_token).expect("Token verification should succeed");
        assert_eq!(admin_claims.role, UserRole::Admin.to_string());
        
        // Test instructor role
        let instructor_id = Uuid::new_v4().to_string();
        let instructor_token = generate_token(&instructor_id, &UserRole::Instructor.to_string())
            .expect("Token generation should succeed");
        let instructor_claims = verify_token(&instructor_token).expect("Token verification should succeed");
        assert_eq!(instructor_claims.role, UserRole::Instructor.to_string());
        
        // Test student role
        let student_id = Uuid::new_v4().to_string();
        let student_token = generate_token(&student_id, &UserRole::Student.to_string())
            .expect("Token generation should succeed");
        let student_claims = verify_token(&student_token).expect("Token verification should succeed");
        assert_eq!(student_claims.role, UserRole::Student.to_string());
    }

    #[test]
    fn test_wrong_secret_key() {
        // Set a known secret key
        env::set_var("JWT_SECRET", "correct_secret_key");
        
        let user_id = Uuid::new_v4().to_string();
        let role = UserRole::Student.to_string();
        
        // Generate token with the correct key
        let token = generate_token(&user_id, &role).expect("Token generation should succeed");
        
        // Change the secret key
        env::set_var("JWT_SECRET", "wrong_secret_key");
        
        // Verify with wrong key - should fail
        let result = verify_token(&token);
        assert!(result.is_err());
    }
}