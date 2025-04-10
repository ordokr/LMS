use crate::models::user::User;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use log::{info, error};
use thiserror::Error;

/// Canvas user data structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CanvasUser {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(rename = "sortable_name")]
    pub sortable_name: Option<String>,
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(rename = "time_zone")]
    pub time_zone: Option<String>,
    #[serde(rename = "locale")]
    pub locale: Option<String>,
    #[serde(rename = "bio")]
    pub bio: Option<String>,
}

/// Error type for Canvas authentication operations
#[derive(Debug, Error)]
pub enum CanvasAuthError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Invalid Canvas token")]
    InvalidToken,
    
    #[error("Canvas API error: {0}")]
    ApiError(String),
    
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Type alias for result with CanvasAuthError
pub type Result<T> = std::result::Result<T, CanvasAuthError>;

/// Canvas authentication service
#[derive(Clone)]
pub struct CanvasAuthService {
    api_url: String,
    client: reqwest::Client,
}

impl CanvasAuthService {
    /// Create a new Canvas authentication service
    pub fn new(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    /// Authenticate a user through Canvas OAuth
    pub async fn authenticate_canvas_user(&self, token: &str) -> Result<User> {
        info!("Authenticating Canvas user with token");
        
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|_| CanvasAuthError::InvalidToken)?,
        );
        
        let url = format!("{}/users/self", self.api_url);
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("Canvas authentication failed: {} - {}", status, error_text);
            return Err(CanvasAuthError::ApiError(format!("HTTP {}: {}", status, error_text)));
        }
        
        let canvas_user: CanvasUser = response.json().await?;
        info!("Successfully authenticated Canvas user: {}", canvas_user.name);
        
        Ok(self.map_canvas_user_to_internal(canvas_user))
    }
    
    /// Map Canvas user data to internal user model
    fn map_canvas_user_to_internal(&self, canvas_user: CanvasUser) -> User {
        let roles = self.determine_user_roles(&canvas_user);
        
        User {
            id: Some(canvas_user.id.clone()),
            name: Some(canvas_user.name),
            email: Some(canvas_user.email),
            username: None, // Canvas doesn't provide a separate username
            avatar: canvas_user.avatar_url,
            canvas_id: Some(canvas_user.id),
            discourse_id: None,
            last_login: None,
            source: Some("canvas".to_string()),
            roles: Some(roles),
            metadata: None,
        }
    }
    
    /// Determine user roles based on Canvas user data
    fn determine_user_roles(&self, canvas_user: &CanvasUser) -> Vec<String> {
        let mut roles = canvas_user.roles.clone();
        
        // If no roles are provided, default to "student"
        if roles.is_empty() {
            roles.push("student".to_string());
        }
        
        roles
    }
    
    /// Factory function to create a new service instance
    pub fn create(api_url: &str) -> Arc<Self> {
        Arc::new(Self::new(api_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_authenticate_canvas_user_success() {
        let mock_server_url = server_url();
        
        let _m = mock("GET", "/users/self")
            .match_header("Authorization", "Bearer test_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "id": "12345",
                    "name": "Test User",
                    "email": "test@example.com",
                    "roles": ["student"],
                    "avatar_url": "https://example.com/avatar.jpg"
                }
            "#)
            .create();
        
        let service = CanvasAuthService::new(&mock_server_url);
        let user = service.authenticate_canvas_user("test_token").await.unwrap();
        
        assert_eq!(user.id, Some("12345".to_string()));
        assert_eq!(user.name, Some("Test User".to_string()));
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert_eq!(user.canvas_id, Some("12345".to_string()));
        assert_eq!(user.source, Some("canvas".to_string()));
        assert_eq!(user.roles, Some(vec!["student".to_string()]));
    }
    
    #[tokio::test]
    async fn test_authenticate_canvas_user_failure() {
        let mock_server_url = server_url();
        
        let _m = mock("GET", "/users/self")
            .match_header("Authorization", "Bearer invalid_token")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"errors": [{"message": "Invalid access token"}]}"#)
            .create();
        
        let service = CanvasAuthService::new(&mock_server_url);
        let result = service.authenticate_canvas_user("invalid_token").await;
        
        assert!(result.is_err());
        match result {
            Err(CanvasAuthError::ApiError(msg)) => {
                assert!(msg.contains("401"));
            },
            _ => panic!("Expected ApiError variant"),
        }
    }
    
    #[tokio::test]
    async fn test_determine_user_roles_default() {
        let service = CanvasAuthService::new("https://example.com");
        let canvas_user = CanvasUser {
            id: "12345".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            roles: vec![],
            sortable_name: None,
            short_name: None,
            avatar_url: None,
            time_zone: None,
            locale: None,
            bio: None,
        };
        
        let roles = service.determine_user_roles(&canvas_user);
        
        assert_eq!(roles, vec!["student".to_string()]);
    }
    
    #[tokio::test]
    async fn test_determine_user_roles_custom() {
        let service = CanvasAuthService::new("https://example.com");
        let canvas_user = CanvasUser {
            id: "12345".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            roles: vec!["teacher".to_string(), "admin".to_string()],
            sortable_name: None,
            short_name: None,
            avatar_url: None,
            time_zone: None,
            locale: None,
            bio: None,
        };
        
        let roles = service.determine_user_roles(&canvas_user);
        
        assert_eq!(roles, vec!["teacher".to_string(), "admin".to_string()]);
    }
}
