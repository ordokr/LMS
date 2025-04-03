use crate::core::errors::AppError;
use axum::{
    extract::{Extension, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub name: String,       // User's full name
    pub email: String,      // User's email
    pub roles: Vec<String>, // User roles
    pub exp: usize,         // Expiration time (as UTC timestamp)
    pub iat: usize,         // Issued at (as UTC timestamp)
    pub jti: String,        // JWT ID (unique identifier for this token)
}

#[derive(Debug, Clone)]
pub struct AuthService {
    jwt_secret: String,
    token_expiration: Duration,
}

impl AuthService {
    pub fn new(jwt_secret: String, token_expiration_secs: u64) -> Self {
        Self {
            jwt_secret,
            token_expiration: Duration::seconds(token_expiration_secs as i64),
        }
    }

    pub fn create_token(&self, user_id: i64, name: String, email: String, roles: Vec<String>) -> Result<String, AppError> {
        let now = OffsetDateTime::now_utc();
        let expiration = now + self.token_expiration;

        let claims = Claims {
            sub: user_id.to_string(),
            name,
            email,
            roles,
            exp: expiration.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::AuthError(format!("Failed to create token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthError(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
}

// Middleware extractor for authenticated user
pub async fn auth_middleware(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> Result<Claims, AppError> {
    auth_service.verify_token(bearer.token())
}