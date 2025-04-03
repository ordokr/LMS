use crate::models::User;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // Subject (user ID)
    pub name: String,      // Username
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
    pub is_admin: bool,    // Admin flag
}

// Constants
const JWT_SECRET: &[u8] = b"your_jwt_secret_key"; // In production, use an environment variable
const JWT_EXPIRY_HOURS: i64 = 24;

// Hash a password
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

// Generate a JWT token for a user
pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expires_at = now + Duration::hours(JWT_EXPIRY_HOURS);
    
    let user_id = user.id
        .expect("User must have an ID to generate token")
        .to_string();
    
    let claims = Claims {
        sub: user_id,
        name: user.username.clone(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        is_admin: user.is_admin,
    };
    
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(JWT_SECRET)
    )
}

// Verify and decode a JWT token
pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
}

// Get user ID from a token
pub fn get_user_id_from_token(token: &str) -> Option<i64> {
    match verify_token(token) {
        Ok(claims) => claims.sub.parse::<i64>().ok(),
        Err(_) => None,
    }
}

// Check if a token is valid and not expired
pub fn is_token_valid(token: &str) -> bool {
    verify_token(token).is_ok()
}