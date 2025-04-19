use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context, anyhow};
use uuid::Uuid;

/// Claims structure for JWT tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at timestamp
    pub iat: u64,
    /// Expiration timestamp
    pub exp: u64,
    /// JWT ID (unique token identifier)
    pub jti: String,
    /// User role
    pub role: String,
    /// User name
    pub name: Option<String>,
    /// User email
    pub email: Option<String>,
}

/// JWT token service for authentication
pub struct JwtService {
    /// Secret key for JWT encoding/decoding
    secret: Vec<u8>,
    /// Token expiration time in seconds
    expiration: u64,
}

impl JwtService {
    /// Create a new JWT service
    ///
    /// # Arguments
    /// * `secret` - Secret key for JWT encoding/decoding
    /// * `expiration` - Token expiration time in seconds (default: 24 hours)
    pub fn new(secret: Vec<u8>, expiration: Option<u64>) -> Self {
        Self {
            secret,
            expiration: expiration.unwrap_or(86400), // 24 hours default
        }
    }
    
    /// Generate a JWT token for a user
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `role` - User role
    /// * `name` - Optional user name
    /// * `email` - Optional user email
    ///
    /// # Returns
    /// A Result containing the JWT token or an error
    pub fn generate_token(&self, user_id: &str, role: &str, name: Option<&str>, email: Option<&str>) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs();
        
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: now + self.expiration,
            jti: Uuid::new_v4().to_string(),
            role: role.to_string(),
            name: name.map(|s| s.to_string()),
            email: email.map(|s| s.to_string()),
        };
        
        let header = Header::new(Algorithm::HS256);
        
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .context("Failed to encode JWT token")
    }
    
    /// Validate a JWT token
    ///
    /// # Arguments
    /// * `token` - JWT token to validate
    ///
    /// # Returns
    /// A Result containing the validated claims or an error
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &validation,
        )
        .context("Failed to decode JWT token")?;
        
        Ok(token_data.claims)
    }
    
    /// Generate a refresh token for a user
    ///
    /// # Arguments
    /// * `user_id` - User ID
    ///
    /// # Returns
    /// A Result containing the refresh token or an error
    pub fn generate_refresh_token(&self, user_id: &str) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs();
        
        // Refresh tokens have longer expiration (7 days)
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: now + (self.expiration * 7), // 7 times longer than access token
            jti: Uuid::new_v4().to_string(),
            role: "refresh".to_string(), // Special role for refresh tokens
            name: None,
            email: None,
        };
        
        let header = Header::new(Algorithm::HS256);
        
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .context("Failed to encode refresh token")
    }
    
    /// Refresh an access token using a refresh token
    ///
    /// # Arguments
    /// * `refresh_token` - Refresh token
    ///
    /// # Returns
    /// A Result containing the new access token or an error
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String> {
        let claims = self.validate_token(refresh_token)?;
        
        // Verify this is a refresh token
        if claims.role != "refresh" {
            return Err(anyhow!("Invalid refresh token"));
        }
        
        // Generate a new access token
        self.generate_token(&claims.sub, "user", None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_token_generation_and_validation() {
        let secret = b"test_secret".to_vec();
        let jwt_service = JwtService::new(secret, Some(3600));
        
        let token = jwt_service.generate_token("user123", "admin", Some("Test User"), Some("test@example.com"))
            .expect("Failed to generate token");
        
        let claims = jwt_service.validate_token(&token)
            .expect("Failed to validate token");
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.name, Some("Test User".to_string()));
        assert_eq!(claims.email, Some("test@example.com".to_string()));
    }
    
    #[test]
    fn test_refresh_token() {
        let secret = b"test_secret".to_vec();
        let jwt_service = JwtService::new(secret, Some(3600));
        
        let refresh_token = jwt_service.generate_refresh_token("user123")
            .expect("Failed to generate refresh token");
        
        let claims = jwt_service.validate_token(&refresh_token)
            .expect("Failed to validate refresh token");
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "refresh");
        
        let new_token = jwt_service.refresh_access_token(&refresh_token)
            .expect("Failed to refresh access token");
        
        let new_claims = jwt_service.validate_token(&new_token)
            .expect("Failed to validate new token");
        
        assert_eq!(new_claims.sub, "user123");
        assert_eq!(new_claims.role, "user");
    }
}
