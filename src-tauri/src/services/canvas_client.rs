use serde::{Serialize, Deserialize};
use reqwest::{Client, header};
use std::sync::Arc;
use log::{info, error, warn};
use chrono::{DateTime, Utc};
use std::time::Duration;
use thiserror::Error;

/// Error types specific to Canvas API operations
#[derive(Debug, Error)]
pub enum CanvasError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}

/// Result type for Canvas operations
pub type CanvasResult<T> = Result<T, CanvasError>;

/// Configuration for connecting to Canvas LMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasConfig {
    /// Base URL of the Canvas instance
    pub url: String,
    
    /// API token for authentication
    pub api_token: String,
    
    /// Timeout for API calls in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

fn default_timeout() -> u64 {
    30
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            api_token: "".to_string(),
            timeout_seconds: default_timeout(),
        }
    }
}

/// Client for interacting with the Canvas LMS API
pub struct CanvasClient {
    client: Client,
    config: CanvasConfig,
}

impl CanvasClient {
    /// Create a new Canvas client
    pub fn new(config: CanvasConfig) -> Arc<Self> {
        // Set up headers for authentication
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", config.api_token)).unwrap_or_default(),
        );
        
        // Create HTTP client with appropriate timeout and headers
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .default_headers(headers)
            .build()
            .unwrap_or_default();
        
        Arc::new(Self {
            client,
            config,
        })
    }
    
    /// Check if the client is properly configured
    pub fn is_configured(&self) -> bool {
        !self.config.url.is_empty() && !self.config.api_token.is_empty()
    }
    
    /// Get the base URL of the Canvas instance
    pub fn base_url(&self) -> &str {
        &self.config.url
    }
    
    /// Test connectivity to the Canvas API
    pub async fn test_connection(&self) -> CanvasResult<bool> {
        if !self.is_configured() {
            return Err(CanvasError::Configuration("Canvas client is not configured".to_string()));
        }
        
        let url = format!("{}/api/v1/courses", self.config.url.trim_end_matches('/'));
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(true)
                } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                    Err(CanvasError::Authentication("Invalid API token".to_string()))
                } else {
                    Err(CanvasError::Api(format!("Failed to connect: status code {}", response.status())))
                }
            },
            Err(e) => Err(CanvasError::Network(e)),
        }
    }
    
    /// Get a course announcement by ID
    pub async fn get_announcement(&self, course_id: &str, announcement_id: &str) -> CanvasResult<serde_json::Value> {
        if !self.is_configured() {
            return Err(CanvasError::Configuration("Canvas client is not configured".to_string()));
        }
        
        let url = format!("{}/api/v1/courses/{}/discussion_topics/{}", 
            self.config.url.trim_end_matches('/'), 
            course_id,
            announcement_id
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        let announcement: serde_json::Value = response.json().await?;
        Ok(announcement)
    }
    
    /// Create a course announcement
    pub async fn create_announcement(&self, course_id: &str, title: &str, message: &str) -> CanvasResult<String> {
        if !self.is_configured() {
            return Err(CanvasError::Configuration("Canvas client is not configured".to_string()));
        }
        
        let url = format!("{}/api/v1/courses/{}/discussion_topics", 
            self.config.url.trim_end_matches('/'),
            course_id
        );
        
        // Prepare the request body
        let body = serde_json::json!({
            "title": title,
            "message": message,
            "is_announcement": true,
            "published": true,
        });
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        let result: serde_json::Value = response.json().await?;
        
        // Extract the announcement ID from the response
        if let Some(id) = result.get("id").and_then(|v| v.as_str()) {
            Ok(id.to_string())
        } else if let Some(id) = result.get("id").and_then(|v| v.as_i64()) {
            Ok(id.to_string())
        } else {
            Err(CanvasError::InvalidResponse("No announcement ID in response".to_string()))
        }
    }
    
    /// Update a course announcement
    pub async fn update_announcement(&self, course_id: &str, announcement_id: &str, title: &str, message: &str) -> CanvasResult<()> {
        if !self.is_configured() {
            return Err(CanvasError::Configuration("Canvas client is not configured".to_string()));
        }
        
        let url = format!("{}/api/v1/courses/{}/discussion_topics/{}", 
            self.config.url.trim_end_matches('/'),
            course_id,
            announcement_id
        );
        
        // Prepare the request body
        let body = serde_json::json!({
            "title": title,
            "message": message,
        });
        
        let response = self.client.put(&url)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        Ok(())
    }
    
    /// Helper function to handle error responses
    async fn handle_error_response<T>(&self, response: reqwest::Response) -> CanvasResult<T> {
        let status = response.status();
        
        match status.as_u16() {
            401 => Err(CanvasError::Authentication("Invalid API token".to_string())),
            404 => Err(CanvasError::NotFound("Resource not found".to_string())),
            429 => Err(CanvasError::RateLimit),
            _ => {
                // Try to get error message from response
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(CanvasError::Api(format!("API error ({}) {}", status, error_text)))
            }
        }
    }
}
