use crate::core::errors::AppError;
use crate::models::auth::{JwtClaims, UserAuthProfile, AuthResponse};
use axum::{
    extract::{Extension, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

/// Extract user ID from Claims for convenience in route handlers
pub struct CurrentUserId(pub String);

/// Configuration for the authentication service
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: Option<String>,
    pub access_token_expiration: Duration,
    pub refresh_token_expiration: Duration,
    pub refresh_token_enabled: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "default_secret_please_change_in_config".to_string(),
            jwt_issuer: "lms-integration".to_string(),
            jwt_audience: None,
            access_token_expiration: Duration::minutes(15),
            refresh_token_expiration: Duration::hours(24),
            refresh_token_enabled: true,
        }
    }
}

/// Authentication service for token management
#[derive(Debug, Clone)]
pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }
    
    /// Generate a new JWT token for a user
    pub fn generate_token(&self, user_profile: &UserAuthProfile) -> Result<String, AppError> {
        let now = OffsetDateTime::now_utc();
        let expiration = now + self.config.access_token_expiration;

        let claims = JwtClaims {
            sub: user_profile.id.to_string(),
            name: user_profile.name.clone(),
            email: user_profile.email.clone(),
            roles: user_profile.roles.clone(),
            exp: expiration.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            iss: self.config.jwt_issuer.clone(),
            aud: self.config.jwt_audience.clone(),
            canvas_id: user_profile.canvas_id.clone(),
            discourse_id: user_profile.discourse_id.clone(),
        };

        let mut header = Header::default();
        header.typ = Some("JWT".to_string());
        header.alg = Algorithm::HS256;

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::AuthError(format!("Failed to create token: {}", e)))
    }
    
    /// Generate a refresh token for extending sessions
    pub fn generate_refresh_token(&self, user_id: &str) -> Result<String, AppError> {
        if (!self.config.refresh_token_enabled) {
            return Err(AppError::AuthError("Refresh tokens are disabled".to_string()));
        }
        
        let now = OffsetDateTime::now_utc();
        let expiration = now + self.config.refresh_token_expiration;

        #[derive(Debug, Serialize)]
        struct RefreshClaims {
            sub: String,
            exp: usize,
            iat: usize,
            jti: String,
            r: bool, // Marker for refresh token
        }

        let claims = RefreshClaims {
            sub: user_id.to_string(),
            exp: expiration.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            r: true,
        };

        let mut header = Header::default();
        header.typ = Some("JWT".to_string());
        header.alg = Algorithm::HS256;

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::AuthError(format!("Failed to create refresh token: {}", e)))
    }

    /// Verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> Result<JwtClaims, AppError> {
        let mut validation = Validation::new(Algorithm::HS256);
        
        if let Some(audience) = &self.config.jwt_audience {
            validation.set_audience(&[audience]);
        }
        validation.validate_exp = true;
        validation.set_required_spec_claims(&["exp", "iat", "sub", "jti", "iss"]);
        
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| AppError::AuthError(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
    
    /// Verify a refresh token and return the user ID
    pub fn verify_refresh_token(&self, token: &str) -> Result<String, AppError> {
        if !self.config.refresh_token_enabled {
            return Err(AppError::AuthError("Refresh tokens are disabled".to_string()));
        }
        
        #[derive(Debug, Deserialize)]
        struct RefreshClaims {
            sub: String,
            exp: usize,
            r: bool,
        }
        
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        
        let token_data = decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| AppError::AuthError(format!("Invalid refresh token: {}", e)))?;
        
        // Verify this is actually a refresh token
        if !token_data.claims.r {
            return Err(AppError::AuthError("Not a valid refresh token".to_string()));
        }
        
        Ok(token_data.claims.sub)
    }
    
    /// Create a full authentication response with tokens and user info
    pub fn create_auth_response(&self, user_profile: UserAuthProfile) -> Result<AuthResponse, AppError> {
        let token = self.generate_token(&user_profile)?;
        
        let refresh_token = if self.config.refresh_token_enabled {
            Some(self.generate_refresh_token(&user_profile.id)?)
        } else {
            None
        };
        
        let now = OffsetDateTime::now_utc();
        let expiration = now + self.config.access_token_expiration;
        
        Ok(AuthResponse {
            token,
            refresh_token,
            expires_at: expiration.unix_timestamp(),
            user: user_profile,
        })
    }
}

/// Middleware for extracting authenticated user from request headers
pub async fn auth_middleware(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> Result<JwtClaims, AppError> {
    auth_service.verify_token(bearer.token())
}

/// Middleware for extracting just the user ID from authenticated request
pub async fn user_id_from_token(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> Result<CurrentUserId, AppError> {
    let claims = auth_service.verify_token(bearer.token())?;
    Ok(CurrentUserId(claims.sub))
}