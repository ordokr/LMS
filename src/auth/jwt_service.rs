// Auto-generated from src/auth/jwtService.js
// JWT authentication service

use crate::config;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// JWT authentication errors
#[derive(Error, Debug)]
pub enum JwtError {
    #[error("JWT encoding error: {0}")]
    EncodingError(#[from] jsonwebtoken::errors::Error),
    
    #[error("JWT validation error")]
    ValidationError,
    
    #[error("Invalid token")]
    InvalidToken,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    /// User ID
    pub id: String,
    
    /// User email
    pub email: String,
    
    /// User roles
    pub roles: Vec<String>,
    
    /// Expiration time (unix timestamp)
    pub exp: usize,
    
    /// Issued at time (unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<usize>,
}

/// Generate a JWT token for authenticated users
///
/// # Arguments
/// * `user` - User object with authentication details
///
/// # Returns
/// JWT token as a string
pub fn generate_jwt_token(user_id: &str, email: &str, roles: Vec<String>) -> Result<String, JwtError> {
    let config = config::get_config();
    
    // Calculate expiration time (24 hours from now)
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = JwtClaims {
        id: user_id.to_string(),
        email: email.to_string(),
        roles,
        exp: expiration,
        iat: Some(Utc::now().timestamp() as usize),
    };
    
    // Sign the token with the secret key
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes())
    )?;
    
    Ok(token)
}

/// Verify and decode a JWT token
///
/// # Arguments
/// * `token` - JWT token to verify
///
/// # Returns
/// Decoded token claims or error if invalid
pub fn verify_jwt_token(token: &str) -> Result<JwtClaims, JwtError> {
    let config = config::get_config();
    
    match decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default()
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => {
            error!("JWT verification error: {}", err);
            Err(JwtError::ValidationError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_token_lifecycle() {
        // Generate a token
        let user_id = "123";
        let email = "test@example.com";
        let roles = vec!["user".to_string(), "admin".to_string()];
        
        let token = generate_jwt_token(user_id, email, roles.clone()).unwrap();
        
        // Verify the token is valid
        let claims = verify_jwt_token(&token).unwrap();
        
        // Verify the claims match what we put in
        assert_eq!(claims.id, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.roles, roles);
    }
    
    #[test]
    fn test_invalid_token() {
        // Try to verify an invalid token
        let result = verify_jwt_token("invalid.token.here");
        assert!(result.is_err());
    }
}
