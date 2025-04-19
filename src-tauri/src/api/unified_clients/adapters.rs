use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::{
    CanvasApiClient, DiscourseApiClient,
    ApiError, Result, PaginationParams, PaginatedResponse,
};

use crate::api::canvas_client::{CanvasClient as OldCanvasClient, CanvasApiError as OldCanvasApiError};
use crate::api::discourse_client::{DiscourseClient as OldDiscourseClient, DiscourseApiError as OldDiscourseApiError};

/// Adapter for the old Canvas client
#[derive(Debug, Clone)]
pub struct CanvasClientAdapter {
    /// New Canvas API client
    client: Arc<CanvasApiClient>,
}

impl CanvasClientAdapter {
    /// Create a new Canvas client adapter
    pub fn new(client: Arc<CanvasApiClient>) -> Self {
        Self { client }
    }
    
    /// Get the underlying client
    pub fn get_client(&self) -> Arc<CanvasApiClient> {
        self.client.clone()
    }
}

/// Implement the old Canvas client interface
impl OldCanvasClient {
    /// Create a new Canvas client from a unified client
    pub fn from_unified(client: Arc<CanvasApiClient>) -> Arc<Self> {
        Arc::new(Self {
            base_url: client.get_config().base_url.clone(),
            api_key: client.get_config().api_key.clone(),
            client: client.get_http_client().clone(),
        })
    }
}

/// Convert from old Canvas error to new API error
impl From<OldCanvasApiError> for ApiError {
    fn from(error: OldCanvasApiError) -> Self {
        match error {
            OldCanvasApiError::HttpError(e) => ApiError::HttpError(e),
            OldCanvasApiError::AuthError(msg) => ApiError::AuthError(msg),
            OldCanvasApiError::ApiError { status_code, message } => ApiError::ApiError { status_code, message },
            OldCanvasApiError::SerializationError(e) => ApiError::SerializationError(e),
        }
    }
}

/// Convert from new API error to old Canvas error
impl From<ApiError> for OldCanvasApiError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::HttpError(e) => OldCanvasApiError::HttpError(e),
            ApiError::AuthError(msg) => OldCanvasApiError::AuthError(msg),
            ApiError::ApiError { status_code, message } => OldCanvasApiError::ApiError { status_code, message },
            ApiError::SerializationError(e) => OldCanvasApiError::SerializationError(e),
            ApiError::RateLimitError { retry_after: _ } => OldCanvasApiError::ApiError { 
                status_code: reqwest::StatusCode::TOO_MANY_REQUESTS, 
                message: "Rate limit exceeded".to_string() 
            },
            ApiError::TimeoutError => OldCanvasApiError::HttpError(
                reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::TimedOut, "Request timed out"))
            ),
            ApiError::NetworkError(msg) => OldCanvasApiError::HttpError(
                reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::Other, msg))
            ),
            ApiError::UnexpectedError(msg) => OldCanvasApiError::ApiError { 
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR, 
                message: msg 
            },
        }
    }
}

/// Adapter for the old Discourse client
#[derive(Debug, Clone)]
pub struct DiscourseClientAdapter {
    /// New Discourse API client
    client: Arc<DiscourseApiClient>,
}

impl DiscourseClientAdapter {
    /// Create a new Discourse client adapter
    pub fn new(client: Arc<DiscourseApiClient>) -> Self {
        Self { client }
    }
    
    /// Get the underlying client
    pub fn get_client(&self) -> Arc<DiscourseApiClient> {
        self.client.clone()
    }
}

/// Implement the old Discourse client interface
impl OldDiscourseClient {
    /// Create a new Discourse client from a unified client
    pub fn from_unified(client: Arc<DiscourseApiClient>) -> Arc<Self> {
        Arc::new(Self {
            base_url: client.get_config().base_url.clone(),
            api_key: client.get_config().api_key.clone(),
            api_username: client.api_username.clone(),
            client: client.get_http_client().clone(),
        })
    }
}

/// Convert from old Discourse error to new API error
impl From<OldDiscourseApiError> for ApiError {
    fn from(error: OldDiscourseApiError) -> Self {
        match error {
            OldDiscourseApiError::HttpError(e) => ApiError::HttpError(e),
            OldDiscourseApiError::AuthError(msg) => ApiError::AuthError(msg),
            OldDiscourseApiError::ApiError { status_code, message } => ApiError::ApiError { status_code, message },
            OldDiscourseApiError::SerializationError(e) => ApiError::SerializationError(e),
        }
    }
}

/// Convert from new API error to old Discourse error
impl From<ApiError> for OldDiscourseApiError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::HttpError(e) => OldDiscourseApiError::HttpError(e),
            ApiError::AuthError(msg) => OldDiscourseApiError::AuthError(msg),
            ApiError::ApiError { status_code, message } => OldDiscourseApiError::ApiError { status_code, message },
            ApiError::SerializationError(e) => OldDiscourseApiError::SerializationError(e),
            ApiError::RateLimitError { retry_after: _ } => OldDiscourseApiError::ApiError { 
                status_code: reqwest::StatusCode::TOO_MANY_REQUESTS, 
                message: "Rate limit exceeded".to_string() 
            },
            ApiError::TimeoutError => OldDiscourseApiError::HttpError(
                reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::TimedOut, "Request timed out"))
            ),
            ApiError::NetworkError(msg) => OldDiscourseApiError::HttpError(
                reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::Other, msg))
            ),
            ApiError::UnexpectedError(msg) => OldDiscourseApiError::ApiError { 
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR, 
                message: msg 
            },
        }
    }
}

/// Factory function to create a Canvas client adapter
pub fn create_canvas_client_adapter(base_url: &str, api_key: &str) -> Result<Arc<OldCanvasClient>> {
    let client = super::create_canvas_client(base_url, api_key)?;
    Ok(OldCanvasClient::from_unified(client))
}

/// Factory function to create a Discourse client adapter
pub fn create_discourse_client_adapter(base_url: &str, api_key: &str, api_username: &str) -> Result<Arc<OldDiscourseClient>> {
    let client = super::create_discourse_client(base_url, api_key, api_username)?;
    Ok(OldDiscourseClient::from_unified(client))
}

/// Adapter for the Canvas service client
#[derive(Debug, Clone)]
pub struct CanvasServiceClientAdapter {
    /// New Canvas API client
    client: Arc<CanvasApiClient>,
}

impl CanvasServiceClientAdapter {
    /// Create a new Canvas service client adapter
    pub fn new(client: Arc<CanvasApiClient>) -> Self {
        Self { client }
    }
    
    /// Get the underlying client
    pub fn get_client(&self) -> Arc<CanvasApiClient> {
        self.client.clone()
    }
}

/// Adapter for the Discourse service client
#[derive(Debug, Clone)]
pub struct DiscourseServiceClientAdapter {
    /// New Discourse API client
    client: Arc<DiscourseApiClient>,
}

impl DiscourseServiceClientAdapter {
    /// Create a new Discourse service client adapter
    pub fn new(client: Arc<DiscourseApiClient>) -> Self {
        Self { client }
    }
    
    /// Get the underlying client
    pub fn get_client(&self) -> Arc<DiscourseApiClient> {
        self.client.clone()
    }
}

/// Factory function to create a Canvas service client adapter
pub fn create_canvas_service_client_adapter(base_url: &str, api_key: &str) -> Result<Arc<crate::services::canvas_client::CanvasClient>> {
    let config = crate::services::canvas_client::CanvasConfig {
        url: base_url.to_string(),
        api_token: api_key.to_string(),
        timeout_seconds: 30,
    };
    
    Ok(crate::services::canvas_client::CanvasClient::new(config))
}

/// Factory function to create a Discourse service client adapter
pub fn create_discourse_service_client_adapter(base_url: &str, api_key: &str, api_username: &str) -> Result<Arc<crate::services::discourse_client::DiscourseClient>> {
    let config = crate::services::discourse_client::DiscourseConfig {
        url: base_url.to_string(),
        api_key: api_key.to_string(),
        api_username: api_username.to_string(),
        timeout_seconds: 30,
    };
    
    Ok(crate::services::discourse_client::DiscourseClient::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_canvas_client_adapter() {
        let _m = mock("GET", "/api/v1/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"123","name":"Test User","email":"test@example.com","login_id":"testuser"}"#)
            .create();
            
        let client = super::super::create_canvas_client(&server_url(), "test_key").unwrap();
        let adapter = CanvasClientAdapter::new(client);
        
        let unified_client = adapter.get_client();
        let user = unified_client.get_user("123").await.unwrap();
        
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
    }
    
    #[tokio::test]
    async fn test_discourse_client_adapter() {
        let _m = mock("GET", "/users/testuser.json")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("api_key".into(), "test_key".into()),
                mockito::Matcher::UrlEncoded("api_username".into(), "test_admin".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"user":{"id":123,"username":"testuser","name":"Test User","email":"test@example.com"}}"#)
            .create();
            
        let client = super::super::create_discourse_client(&server_url(), "test_key", "test_admin").unwrap();
        let adapter = DiscourseClientAdapter::new(client);
        
        let unified_client = adapter.get_client();
        let user = unified_client.get_user_by_username("testuser").await.unwrap();
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
}
