use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use log::{info, error};

/// JWT claims structure for authentication tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (as UTC timestamp)
    pub exp: i64,
    /// Issued at (as UTC timestamp)
    pub iat: i64,
    /// Role(s) assigned to the user
    pub role: Vec<String>,
    /// Canvas ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_id: Option<String>,
    /// Discourse ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discourse_id: Option<String>,
    /// JWT ID (unique identifier for this token)
    pub jti: String,
    /// User email
    pub email: String,
    /// User name
    pub name: String,
}

/// Refresh token data structure for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenData {
    /// User ID associated with this refresh token
    pub user_id: String,
    /// Unique token identifier
    pub token_id: String,
    /// Token expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Whether the token has been revoked
    pub revoked: bool,
    /// Canvas ID if available
    pub canvas_id: Option<String>,
    /// Discourse ID if available
    pub discourse_id: Option<String>,
    /// User role
    pub role: Vec<String>,
}

/// Error type for JWT service operations
#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Type alias for result with JwtError
pub type Result<T> = std::result::Result<T, JwtError>;

/// JWT authentication service
#[derive(Clone)]
pub struct JwtService {
    /// Secret key for signing tokens
    jwt_secret: String,
    /// Access token validity duration in seconds
    access_token_expiry: i64,
    /// Refresh token validity duration in seconds
    refresh_token_expiry: i64,
    /// Database connection for refresh tokens, optional
    #[cfg(feature = "database")]
    db_pool: Option<Arc<sqlx::SqlitePool>>,
}

impl JwtService {
    /// Create a new JWT service instance
    pub fn new(jwt_secret: &str, access_token_expiry: i64, refresh_token_expiry: i64) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
            access_token_expiry,
            refresh_token_expiry,
            #[cfg(feature = "database")]
            db_pool: None,
        }
    }
    
    #[cfg(feature = "database")]
    /// Configure database pool for refresh token persistence
    pub fn with_database_pool(mut self, db_pool: Arc<sqlx::SqlitePool>) -> Self {
        self.db_pool = Some(db_pool);
        self
    }
    
    /// Generate a JWT token for the given user
    pub fn generate_token(&self, 
        user_id: &str, 
        email: &str, 
        name: &str, 
        role: Vec<String>,
        canvas_id: Option<String>,
        discourse_id: Option<String>
    ) -> Result<String> {
        let now = Utc::now();
        let expiry = now + Duration::seconds(self.access_token_expiry);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
            role,
            canvas_id,
            discourse_id,
            jti: Uuid::new_v4().to_string(),
            email: email.to_string(),
            name: name.to_string(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )?;
        
        Ok(token)
    }
    
    /// Verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default()
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => {
                error!("JWT verification error: {}", err);
                match err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(JwtError::TokenExpired),
                    _ => Err(JwtError::InvalidToken),
                }
            }
        }
    }
    
    /// Generate a refresh token for the given user
    #[cfg(feature = "database")]
    pub async fn generate_refresh_token(
        &self,
        user_id: &str,
        canvas_id: Option<String>,
        discourse_id: Option<String>,
        role: Vec<String>,
    ) -> Result<String> {
        let token_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_expiry);
        
        let token_data = RefreshTokenData {
            user_id: user_id.to_string(),
            token_id: token_id.clone(),
            expires_at,
            revoked: false,
            canvas_id,
            discourse_id,
            role,
        };
        
        // Store refresh token in database if available
        if let Some(pool) = &self.db_pool {
            let result = sqlx::query!(
                r#"
                INSERT INTO refresh_tokens (
                    token_id, user_id, expires_at, revoked, canvas_id, discourse_id, role_json
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
                token_data.token_id,
                token_data.user_id,
                token_data.expires_at,
                token_data.revoked,
                token_data.canvas_id,
                token_data.discourse_id,
                serde_json::to_string(&token_data.role).unwrap_or_else(|_| "[]".to_string())
            )
            .execute(pool.as_ref())
            .await;
            
            if let Err(err) = result {
                error!("Failed to store refresh token: {}", err);
                return Err(JwtError::DatabaseError(err.to_string()));
            }
        }
        
        Ok(token_id)
    }
    
    /// Verify a refresh token and generate a new access token
    #[cfg(feature = "database")]
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<String> {
        let pool = self.db_pool.as_ref()
            .ok_or_else(|| JwtError::DatabaseError("Database connection not configured".to_string()))?;
        
        // Retrieve refresh token from database
        let token_data = sqlx::query_as!(
            RefreshTokenData,
            r#"
            SELECT 
                user_id, 
                token_id, 
                expires_at as "expires_at: DateTime<Utc>", 
                revoked, 
                canvas_id, 
                discourse_id,
                json(role_json) as "role: Vec<String>"
            FROM refresh_tokens
            WHERE token_id = ? AND revoked = false
            "#,
            refresh_token
        )
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| JwtError::DatabaseError(e.to_string()))?;
        
        // Verify token exists and is not expired
        match token_data {
            Some(token) => {
                if token.expires_at < Utc::now() {
                    return Err(JwtError::TokenExpired);
                }
                
                // Get user details for new token
                let user = sqlx::query!(
                    "SELECT email, name FROM users WHERE id = ?",
                    token.user_id
                )
                .fetch_optional(pool.as_ref())
                .await
                .map_err(|e| JwtError::DatabaseError(e.to_string()))?;
                
                // Generate new access token
                match user {
                    Some(user) => {
                        self.generate_token(
                            &token.user_id,
                            &user.email,
                            &user.name,
                            token.role,
                            token.canvas_id,
                            token.discourse_id
                        )
                    },
                    None => Err(JwtError::InvalidToken),
                }
            },
            None => Err(JwtError::InvalidToken),
        }
    }
    
    /// Revoke a refresh token
    #[cfg(feature = "database")]
    pub async fn revoke_refresh_token(&self, token_id: &str) -> Result<()> {
        let pool = self.db_pool.as_ref()
            .ok_or_else(|| JwtError::DatabaseError("Database connection not configured".to_string()))?;
        
        let result = sqlx::query!(
            "UPDATE refresh_tokens SET revoked = true WHERE token_id = ?",
            token_id
        )
        .execute(pool.as_ref())
        .await
        .map_err(|e| JwtError::DatabaseError(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(JwtError::InvalidToken);
        }
        
        Ok(())
    }
    
    /// Revoke all refresh tokens for a user
    #[cfg(feature = "database")]
    pub async fn revoke_all_user_tokens(&self, user_id: &str) -> Result<()> {
        let pool = self.db_pool.as_ref()
            .ok_or_else(|| JwtError::DatabaseError("Database connection not configured".to_string()))?;
        
        sqlx::query!(
            "UPDATE refresh_tokens SET revoked = true WHERE user_id = ?",
            user_id
        )
        .execute(pool.as_ref())
        .await
        .map_err(|e| JwtError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Create database tables required for JWT operations
    #[cfg(feature = "database")]
    pub async fn create_tables(db_pool: &sqlx::SqlitePool) -> Result<()> {
        // Create refresh tokens table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS refresh_tokens (
                token_id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                revoked BOOLEAN NOT NULL DEFAULT 0,
                canvas_id TEXT,
                discourse_id TEXT,
                role_json TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
            "#
        )
        .execute(db_pool)
        .await
        .map_err(|e| JwtError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup_service() -> JwtService {
        JwtService::new("test_secret_key", 3600, 86400)
    }
    
    #[test]
    fn test_generate_and_verify_token() {
        let service = setup_service();
        
        let token = service.generate_token(
            "user123",
            "test@example.com",
            "Test User",
            vec!["user".to_string()],
            None,
            None
        ).unwrap();
        
        let claims = service.verify_token(&token).unwrap();
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.name, "Test User");
        assert_eq!(claims.role, vec!["user"]);
    }
    
    #[test]
    fn test_invalid_token() {
        let service = setup_service();
        
        let result = service.verify_token("invalid.token.here");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_token_with_optional_fields() {
        let service = setup_service();
        
        let token = service.generate_token(
            "user456",
            "test@example.com",
            "Test User",
            vec!["user".to_string(), "admin".to_string()],
            Some("canvas123".to_string()),
            Some("discourse456".to_string())
        ).unwrap();
        
        let claims = service.verify_token(&token).unwrap();
        
        assert_eq!(claims.sub, "user456");
        assert_eq!(claims.role, vec!["user", "admin"]);
        assert_eq!(claims.canvas_id, Some("canvas123".to_string()));
        assert_eq!(claims.discourse_id, Some("discourse456".to_string()));
    }
}
