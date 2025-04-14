use crate::auth::jwt_service::{self, JwtService, Claims, JwtError};
use crate::models::user::User;
use crate::repositories::user::UserRepository;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, 
    TokenUrl, Scope, TokenResponse, AuthorizationCode, CsrfToken, 
    PkceCodeChallenge, PkceCodeVerifier, reqwest::async_http_client
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::env;
use tracing::{info, error};

/// Unified Authentication Service
/// Combines OAuth2 authentication with JWT token management
pub struct UnifiedAuthService {
    /// JWT service for token management
    jwt_service: Arc<JwtService>,
    /// OAuth2 client for Canvas authentication
    oauth2_client: BasicClient,
}

/// OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub scopes: Vec<String>,
}

impl UnifiedAuthService {
    /// Create a new unified authentication service
    pub fn new(jwt_service: Arc<JwtService>, oauth2_config: OAuth2Config) -> Self {
        let oauth2_client = BasicClient::new(
            ClientId::new(oauth2_config.client_id),
            Some(ClientSecret::new(oauth2_config.client_secret)),
            AuthUrl::new(oauth2_config.auth_url).expect("Invalid authorization URL"),
            Some(TokenUrl::new(oauth2_config.token_url).expect("Invalid token URL")),
        )
        .set_redirect_uri(RedirectUrl::new(oauth2_config.redirect_url).expect("Invalid redirect URL"));

        Self {
            jwt_service,
            oauth2_client,
        }
    }

    /// Start OAuth2 authentication flow
    pub fn start_oauth_flow(&self) -> (String, CsrfToken, PkceCodeVerifier) {
        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        
        // Generate authorization URL
        let (auth_url, csrf_token) = self.oauth2_client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .url();
            
        (auth_url.to_string(), csrf_token, pkce_verifier)
    }
    
    /// Complete OAuth2 authentication flow
    pub async fn complete_oauth_flow(
        &self,
        code: &str,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<String, String> {
        // Exchange authorization code for access token
        let token_result = self.oauth2_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await;
            
        match token_result {
            Ok(token) => {
                let access_token = token.access_token().secret();
                Ok(access_token.clone())
            },
            Err(e) => {
                error!("OAuth2 token exchange error: {}", e);
                Err(format!("OAuth2 token exchange error: {}", e))
            }
        }
    }
    
    /// Authenticate user with Canvas and generate JWT token
    pub async fn authenticate_with_canvas(
        &self,
        oauth_token: &str,
        canvas_client: &dyn CanvasClient,
    ) -> Result<(String, String, User), String> {
        // Get user info from Canvas using OAuth token
        let canvas_user = canvas_client.get_user_info(oauth_token).await
            .map_err(|e| format!("Failed to get Canvas user info: {}", e))?;
            
        // Convert Canvas user to internal user model
        let user = self.map_canvas_user_to_internal(canvas_user)?;
        
        // Generate JWT token and refresh token for authenticated user
        let (token, refresh_token) = self.generate_auth_tokens(&user).await?;
        
        Ok((token, refresh_token, user))
    }
    
    /// Authenticate user with username/password and generate JWT token
    pub async fn authenticate_with_credentials(
        &self,
        username: &str,
        password: &str,
        user_repository: &dyn UserRepository,
    ) -> Result<(String, String, User), String> {
        // Authenticate against database
        let user = user_repository.authenticate_user(username, password).await
            .ok_or_else(|| "Invalid credentials".to_string())?;
            
        // Generate JWT token and refresh token
        let (token, refresh_token) = self.generate_auth_tokens(&user).await?;
        
        Ok((token, refresh_token, user))
    }
    
    /// Refresh an authentication token
    pub async fn refresh_token(&self, token: &str) -> Result<(String, String), String> {
        // Validate the existing token
        let claims = self.validate_token(token)
            .map_err(|e| format!("Invalid token: {}", e))?;
            
        // Get user from database to ensure they still exist
        // In a real implementation, you would fetch the user from the database
        
        // Generate new tokens
        let new_token = self.jwt_service.generate_token(
            &claims.sub,
            &claims.role,
            &claims.canvas_id,
            claims.discourse_id.as_deref(),
            claims.email.as_deref(),
            claims.name.as_deref(),
        ).map_err(|e| format!("Failed to generate new token: {}", e))?;
        
        // Generate new refresh token
        // In a real implementation, you would generate a proper refresh token
        let new_refresh_token = "simulated_refresh_token".to_string();
        
        Ok((new_token, new_refresh_token))
    }
    
    /// Generate both access and refresh tokens for a user
    async fn generate_auth_tokens(&self, user: &User) -> Result<(String, String), String> {
        // Generate access token
        let token = self.jwt_service.generate_token(
            &user.id,
            &user.role,
            &user.canvas_id,
            user.discourse_id.as_deref(),
            Some(&user.email),
            user.display_name.as_deref(),
        ).map_err(|e| format!("Failed to generate JWT token: {}", e))?;
        
        // Generate refresh token
        // In a real implementation, you would generate a proper refresh token
        let refresh_token = "simulated_refresh_token".to_string();
        
        Ok((token, refresh_token))
    }
    
    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        self.jwt_service.validate_token(token)
    }
    
    /// Generate SSO token for Discourse
    pub fn generate_sso_token(
        &self,
        user: &User,
        return_url: Option<&str>,
    ) -> Result<String, String> {
        self.jwt_service.generate_token(
            &user.id,
            &user.role,
            &user.canvas_id,
            user.discourse_id.as_deref(),
            Some(&user.email),
            user.display_name.as_deref(),
        ).map_err(|e| format!("Failed to generate SSO token: {}", e))
    }
    
    /// Map Canvas user to internal user model
    fn map_canvas_user_to_internal(&self, canvas_user: serde_json::Value) -> Result<User, String> {
        // Extract user info from Canvas response
        let id = canvas_user["id"].as_u64()
            .ok_or_else(|| "Missing Canvas user ID".to_string())?
            .to_string();
        let name = canvas_user["name"].as_str()
            .ok_or_else(|| "Missing Canvas user name".to_string())?
            .to_string();
        let email = canvas_user["email"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Missing Canvas user email".to_string())?;
        let role = canvas_user["roles"].as_array()
            .and_then(|roles| roles.first())
            .and_then(|role| role.as_str())
            .unwrap_or("student")
            .to_string();
            
        // Create internal user model
        let user = User {
            id: format!("canvas_{}", id.clone()),
            username: email.clone(),
            email,
            display_name: Some(name),
            role,
            canvas_id: id,
            discourse_id: None,
            // Additional fields would be populated here based on your User struct
        };
        
        Ok(user)
    }
}

/// Canvas client trait for testability
pub trait CanvasClient {
    async fn get_user_info(&self, token: &str) -> Result<serde_json::Value, String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    
    mock! {
        CanvasClient {}
        
        async fn get_user_info(&self, token: &str) -> Result<serde_json::Value, String>;
    }
    
    mock! {
        UserRepository {}
        
        async fn authenticate_user(&self, username: &str, password: &str) -> Option<User>;
    }
    
    #[tokio::test]
    async fn test_authenticate_with_canvas() {
        // Create mock Canvas client
        let mut mock_canvas_client = MockCanvasClient::new();
        mock_canvas_client
            .expect_get_user_info()
            .with(eq("oauth_token"))
            .returning(|_| {
                Ok(serde_json::json!({
                    "id": 123,
                    "name": "Test User",
                    "email": "test@example.com",
                    "roles": ["student"]
                }))
            });
            
        // Create JWT service
        let jwt_service = Arc::new(JwtService::new("test_secret", 3600, 86400));
        
        // Create OAuth2 config
        let oauth2_config = OAuth2Config {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            auth_url: "https://canvas.example.com/oauth2/auth".to_string(),
            token_url: "https://canvas.example.com/oauth2/token".to_string(),
            redirect_url: "https://example.com/callback".to_string(),
            scopes: vec!["read".to_string()],
        };
        
        // Create unified auth service
        let auth_service = UnifiedAuthService::new(jwt_service, oauth2_config);
        
        // Authenticate with Canvas
        let result = auth_service.authenticate_with_canvas("oauth_token", &mock_canvas_client).await;
        assert!(result.is_ok());
        
        let (token, refresh_token, user) = result.unwrap();
        assert!(!token.is_empty());
        assert!(!refresh_token.is_empty());
        assert_eq!(user.id, "canvas_123");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.display_name, Some("Test User".to_string()));
        assert_eq!(user.role, "student");
        assert_eq!(user.canvas_id, "123");
    }
}
