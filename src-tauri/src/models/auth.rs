use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// JWT claims structure for authentication tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    // Subject (user ID)
    pub sub: String,
    
    // User's full name
    pub name: String,
    
    // User's email
    pub email: String,
    
    // User roles for permission checking
    pub roles: Vec<String>,
    
    // Expiration time (as UTC timestamp)
    pub exp: usize,
    
    // Issued at (as UTC timestamp)
    pub iat: usize,
    
    // JWT ID (unique identifier for this token)
    pub jti: String,
    
    // Issuer (who created the token)
    pub iss: String,
    
    // Audience (who the token is intended for)
    pub aud: Option<String>,
    
    // Canvas user ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_id: Option<String>,
    
    // Discourse user ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discourse_id: Option<String>,
}

/// User authentication profile for tokens
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuthProfile {
    // User ID
    pub id: String,
    
    // User's full name
    pub name: String,
    
    // User's email address
    pub email: String,
    
    // User's roles
    pub roles: Vec<String>,
    
    // Canvas user ID if available
    pub canvas_id: Option<String>,
    
    // Discourse user ID if available
    pub discourse_id: Option<String>,
}

/// Standard response for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    // The JWT token
    pub token: String,
    
    // Refresh token for obtaining new JWT without full login
    pub refresh_token: Option<String>,
    
    // Token expiration timestamp
    pub expires_at: i64,
    
    // User information
    pub user: UserAuthProfile,
}

/// Login request structure
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    // Email address for login
    pub email: String,
    
    // Password for login
    pub password: String,
    
    // Remember user for extended session
    pub remember_me: Option<bool>,
}

/// Registration request structure
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    // User's full name
    pub name: String,
    
    // Email address
    pub email: String,
    
    // Password
    pub password: String,
    
    // Password confirmation
    pub password_confirmation: String,
}

/// Token refresh request
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    // The refresh token
    pub refresh_token: String,
}
