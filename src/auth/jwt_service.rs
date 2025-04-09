use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at time
    pub role: String,       // User role (student, instructor, admin)
    pub canvas_id: String,  // Canvas user ID
    pub discourse_id: Option<String>, // Discourse user ID (optional, may not exist yet)
    pub jti: Option<String>, // JWT ID for token identification
    pub email: Option<String>, // User email (for SSO)
    pub name: Option<String>,  // User's display name (for SSO)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenData {
    pub user_id: String,
    pub token_id: String,
    pub expires_at: usize,
    pub revoked: bool,
    pub canvas_id: String,
    pub discourse_id: Option<String>,
    pub role: String,
}

pub struct JwtService {
    secret: Vec<u8>,
    refresh_tokens: Arc<Mutex<HashMap<String, RefreshTokenData>>>,
}

impl JwtService {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            secret: secret.to_vec(),
            refresh_tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }    pub fn generate_token(
        &self,
        user_id: &str, 
        role: &str,
        canvas_id: &str,
        discourse_id: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow!("Time error: {}", e))?
            .as_secs() + 24 * 3600; // 24 hours from now
        
        let issued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow!("Time error: {}", e))?
            .as_secs();
            
        let token_id = Uuid::new_v4().to_string();
            
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
            iat: issued_at as usize,
            role: role.to_string(),
            canvas_id: canvas_id.to_string(),
            discourse_id: discourse_id.map(|id| id.to_string()),
            jti: Some(token_id),
            email: email.map(|e| e.to_string()),
            name: name.map(|n| n.to_string()),
        };

        encode(
            &Header::default(), 
            &claims, 
            &EncodingKey::from_secret(&self.secret)
        ).map_err(|e| anyhow!("JWT encoding error: {}", e))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        
        let token_data = decode::<Claims>(
            token, 
            &DecodingKey::from_secret(&self.secret), 
            &validation
        ).map_err(|e| anyhow!("JWT validation error: {}", e))?;
        
        Ok(token_data.claims)
    }
    
    // Generate a refresh token for the specified user
    pub fn generate_refresh_token(
        &self,
        user_id: &str, 
        role: &str,
        canvas_id: &str,
        discourse_id: Option<&str>,
    ) -> Result<String> {
        let token = Uuid::new_v4().to_string();
        let token_id = Uuid::new_v4().to_string();
        
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow!("Time error: {}", e))?
            .as_secs() + 30 * 24 * 3600; // 30 days from now
            
        let token_data = RefreshTokenData {
            user_id: user_id.to_string(),
            token_id,
            expires_at: expiration as usize,
            revoked: false,
            canvas_id: canvas_id.to_string(),
            discourse_id: discourse_id.map(ToString::to_string),
            role: role.to_string(),
        };
        
        let mut refresh_tokens = self.refresh_tokens.lock().unwrap();
        refresh_tokens.insert(token.clone(), token_data);
        
        Ok(token)
    }
    
    // Validate a refresh token and return user information if valid
    pub fn validate_refresh_token(&self, token: &str) -> Result<RefreshTokenData> {
        let refresh_tokens = self.refresh_tokens.lock().unwrap();
        
        if let Some(token_data) = refresh_tokens.get(token) {
            // Check if token is expired or revoked
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| anyhow!("Time error: {}", e))?
                .as_secs() as usize;
                
            if token_data.revoked || token_data.expires_at < now {
                return Err(anyhow!("Refresh token is expired or revoked"));
            }
            
            return Ok(token_data.clone());
        }
        
        Err(anyhow!("Invalid refresh token"))
    }
    
    // Revoke a specific refresh token
    pub fn revoke_refresh_token(&self, token: &str) -> Result<()> {
        let mut refresh_tokens = self.refresh_tokens.lock().unwrap();
        
        if let Some(token_data) = refresh_tokens.get_mut(token) {
            token_data.revoked = true;
            return Ok(());
        }
        
        Err(anyhow!("Token not found"))
    }
    
    // Revoke all refresh tokens for a specific user
    pub fn revoke_all_user_tokens(&self, user_id: &str) -> Result<usize> {
        let mut refresh_tokens = self.refresh_tokens.lock().unwrap();
        let mut count = 0;
        
        for token_data in refresh_tokens.values_mut() {
            if token_data.user_id == user_id && !token_data.revoked {
                token_data.revoked = true;
                count += 1;
            }
        }
        
        Ok(count)
    }
      // Generate both access token and refresh token for a complete authentication response
    pub fn generate_auth_tokens(
        &self,
        user_id: &str, 
        role: &str,
        canvas_id: &str,
        discourse_id: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<(String, String)> {
        let access_token = self.generate_token(user_id, role, canvas_id, discourse_id, email, name)?;
        let refresh_token = self.generate_refresh_token(user_id, role, canvas_id, discourse_id)?;
        
        Ok((access_token, refresh_token))
    }
      // Refresh an access token using a refresh token
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String> {
        let token_data = self.validate_refresh_token(refresh_token)?;
        
        // For a refresh operation, we'll pass None for email and name as they aren't
        // stored in the refresh token data. They will be fetched from the database
        // if needed in a real-world implementation.
        self.generate_token(
            &token_data.user_id,
            &token_data.role,
            &token_data.canvas_id,
            token_data.discourse_id.as_deref(),
            None,
            None
        )
    }
    
    // Clean up expired refresh tokens
    pub fn cleanup_expired_tokens(&self) -> Result<usize> {
        let mut refresh_tokens = self.refresh_tokens.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow!("Time error: {}", e))?
            .as_secs() as usize;
            
        let token_keys: Vec<String> = refresh_tokens
            .iter()
            .filter(|(_, data)| data.expires_at < now)
            .map(|(key, _)| key.clone())
            .collect();
            
        let count = token_keys.len();
        
        for key in token_keys {
            refresh_tokens.remove(&key);
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let secret = b"test_secret_key_for_jwt_authentication";
        let jwt_service = JwtService::new(secret);
        
        let user_id = "user123";
        let role = "student";
        let canvas_id = "canvas_user_456";
        let discourse_id = Some("discourse_user_789");
        let email = Some("student@example.com");
        let name = Some("John Student");
        
        let token = jwt_service.generate_token(user_id, role, canvas_id, discourse_id, email, name).unwrap();
        
        let claims = jwt_service.validate_token(&token).unwrap();
        
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, role);
        assert_eq!(claims.canvas_id, canvas_id);
        assert_eq!(claims.discourse_id, discourse_id.map(String::from));
        assert_eq!(claims.email, email.map(String::from));
        assert_eq!(claims.name, name.map(String::from));
    }
      #[test]
    fn test_refresh_token_flow() {
        let secret = b"test_secret_key_for_jwt_authentication";
        let jwt_service = JwtService::new(secret);
        
        let user_id = "user123";
        let role = "student";
        let canvas_id = "canvas_user_456";
        let discourse_id = Some("discourse_user_789");
        let email = Some("student@example.com");
        let name = Some("Test Student");
        
        // Generate both access and refresh tokens
        let (access_token, refresh_token) = jwt_service
            .generate_auth_tokens(user_id, role, canvas_id, discourse_id, email, name)
            .unwrap();
        
        // Validate both tokens
        let access_claims = jwt_service.validate_token(&access_token).unwrap();
        assert_eq!(access_claims.sub, user_id);
        
        let refresh_data = jwt_service.validate_refresh_token(&refresh_token).unwrap();
        assert_eq!(refresh_data.user_id, user_id);
        
        // Generate new access token using refresh token
        let new_access_token = jwt_service.refresh_access_token(&refresh_token).unwrap();
        let new_claims = jwt_service.validate_token(&new_access_token).unwrap();
        assert_eq!(new_claims.sub, user_id);
        
        // Test revoking the refresh token
        jwt_service.revoke_refresh_token(&refresh_token).unwrap();
        assert!(jwt_service.validate_refresh_token(&refresh_token).is_err());
    }
}