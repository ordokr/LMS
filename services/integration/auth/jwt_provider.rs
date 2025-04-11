use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Serialize, Deserialize};
use std::env;
use thiserror::Error;
use url::Url;

/// Error types for the JWT auth provider
#[derive(Error, Debug)]
pub enum JwtError {
    #[error("JWT configuration error: {0}")]
    ConfigError(String),
    
    #[error("Invalid user: {0}")]
    InvalidUserError(String),
    
    #[error("JWT generation error: {0}")]
    TokenGenerationError(String),
    
    #[error("JWT verification error: {0}")]
    TokenVerificationError(String),
    
    #[error("URL generation error: {0}")]
    UrlGenerationError(String),
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub external_id: String,
    pub admin: bool,
    pub roles: String,
    pub exp: usize,  // Expiration time (as UTC timestamp)
    pub iat: usize,  // Issued at (as UTC timestamp)
    pub aud: String, // Audience
    pub iss: String, // Issuer
}

/// User object for token generation
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub admin: Option<bool>,
    pub roles: Option<Vec<String>>,
}

/// Configuration options for JWT auth provider
#[derive(Debug, Clone)]
pub struct JwtAuthProviderOptions {
    pub secret: Option<String>,
    pub issuer: Option<String>,
    pub audience: Option<String>,
    pub expires_in: Option<u64>,
}

/// JWT Authentication Provider for Canvas-Discourse Integration
/// Handles generation and verification of JWT tokens for SSO between systems
pub struct JwtAuthProvider {
    secret: String,
    issuer: String,
    audience: String,
    expires_in: u64,
}

impl JwtAuthProvider {
    /// Initialize the JWT authentication provider
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options
    ///
    /// # Returns
    ///
    /// * `Result<Self, JwtError>` - New JWT auth provider or error
    pub fn new(options: Option<JwtAuthProviderOptions>) -> Result<Self, JwtError> {
        let options = options.unwrap_or(JwtAuthProviderOptions {
            secret: None,
            issuer: None,
            audience: None,
            expires_in: None,
        });
        
        let secret = options.secret
            .or_else(|| env::var("JWT_SECRET").ok())
            .ok_or_else(|| JwtError::ConfigError("JWT secret is required".to_string()))?;
            
        let issuer = options.issuer.unwrap_or_else(|| "canvas".to_string());
        let audience = options.audience.unwrap_or_else(|| "discourse".to_string());
        let expires_in = options.expires_in.unwrap_or(3600); // 1 hour default
        
        Ok(JwtAuthProvider {
            secret,
            issuer,
            audience,
            expires_in,
        })
    }
    
    /// Generate a JWT token for Canvas user to authenticate with Discourse
    ///
    /// # Arguments
    ///
    /// * `user` - Canvas user object
    ///
    /// # Returns
    ///
    /// * `Result<String, JwtError>` - JWT token or error
    pub async fn generate_token(&self, user: &User) -> Result<String, JwtError> {
        if user.id.is_empty() {
            return Err(JwtError::InvalidUserError("Valid user ID is required".to_string()));
        }
        
        let now = chrono::Utc::now().timestamp() as usize;
        let expiry = now + self.expires_in as usize;
        
        let name = user.display_name.clone()
            .or_else(|| user.name.clone())
            .unwrap_or_else(|| "Unknown User".to_string());
            
        let roles = match &user.roles {
            Some(roles) => roles.join(","),
            None => String::new(),
        };
        
        let claims = JwtClaims {
            user_id: user.id.clone(),
            email: user.email.clone(),
            name,
            external_id: format!("canvas_{}", user.id),
            admin: user.admin.unwrap_or(false),
            roles,
            exp: expiry,
            iat: now,
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes())
        )
        .map_err(|e| JwtError::TokenGenerationError(e.to_string()))
    }
    
    /// Verify a JWT token from Discourse
    ///
    /// # Arguments
    ///
    /// * `token` - JWT token
    ///
    /// # Returns
    ///
    /// * `Result<JwtClaims, JwtError>` - Decoded token payload or error
    pub async fn verify_token(&self, token: &str) -> Result<JwtClaims, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&self.issuer]); // Reversed for verification direction
        validation.set_issuer(&[&self.audience]);
        
        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation
        )
        .map(|token_data| token_data.claims)
        .map_err(|e| JwtError::TokenVerificationError(e.to_string()))
    }
    
    /// Generate SSO redirect URL for Canvas user to Discourse
    ///
    /// # Arguments
    ///
    /// * `user` - Canvas user object
    /// * `return_url` - URL to return to after authentication
    ///
    /// # Returns
    ///
    /// * `Result<String, JwtError>` - Discourse SSO URL with token or error
    pub async fn generate_sso_url(&self, user: &User, return_url: Option<&str>) -> Result<String, JwtError> {
        let token = self.generate_token(user).await?;
        
        let discourse_url = env::var("DISCOURSE_URL").unwrap_or_else(|_| "http://discourse.example.com".to_string());
        let sso_endpoint = "/canvas-sso";
        
        let mut url = Url::parse(&format!("{}{}", discourse_url, sso_endpoint))
            .map_err(|e| JwtError::UrlGenerationError(e.to_string()))?;
        
        url.query_pairs_mut().append_pair("token", &token);
        
        if let Some(return_url) = return_url {
            url.query_pairs_mut().append_pair("return_url", return_url);
        }
        
        Ok(url.to_string())
    }
}
