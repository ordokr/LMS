use std::collections::HashMap;
use async_trait::async_trait;
use anyhow::{Result, Context};
use log::{info, error, debug};
use reqwest::{Client, header};
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// Canvas API client interface
#[async_trait]
pub trait CanvasApi: Send + Sync {
    /// Get a user by ID
    async fn get_user(&self, id: &str) -> Result<Value>;
    
    /// Get a course by ID
    async fn get_course(&self, id: &str) -> Result<Value>;
    
    /// Get an assignment by ID
    async fn get_assignment(&self, course_id: &str, assignment_id: &str) -> Result<Value>;
    
    /// Get a submission by assignment and user ID
    async fn get_submission(&self, course_id: &str, assignment_id: &str, user_id: &str) -> Result<Value>;
    
    /// Get a discussion topic by ID
    async fn get_discussion_topic(&self, course_id: &str, topic_id: &str) -> Result<Value>;
    
    /// Create a submission comment
    async fn create_submission_comment(&self, course_id: &str, assignment_id: &str, user_id: &str, comment: &str) -> Result<Value>;
    
    /// Get enrollment for a user in a course
    async fn get_enrollment(&self, course_id: &str, user_id: &str) -> Result<Value>;
}

/// Configuration for Canvas API client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasApiConfig {
    /// Base URL of the Canvas API (e.g., "https://canvas.example.com/api/v1")
    pub base_url: String,
    
    /// API access token
    pub access_token: String,
    
    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    
    /// Maximum number of retries
    #[serde(default = "default_retries")]
    pub max_retries: usize,
}

fn default_timeout() -> u64 {
    30
}

fn default_retries() -> usize {
    3
}

/// Implementation of Canvas API client
pub struct CanvasApiClient {
    config: CanvasApiConfig,
    client: Client,
}

impl CanvasApiClient {
    /// Create a new Canvas API client
    pub fn new(config: CanvasApiConfig) -> Result<Self> {
        // Create custom headers with authentication
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.access_token))
                .context("Invalid access token")?
        );
        
        // Create HTTP client with appropriate configuration
        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            config,
            client,
        })
    }
    
    /// Make a GET request to Canvas API
    async fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Canvas API request: GET {}", url);
        
        let mut retries = 0;
        let mut last_error = None;
        
        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }
            
            match self.client.get(&url).send().await {
                Ok(response) => {
                    let status = response.status();
                    let response_text = response.text().await?;
                    
                    if status.is_success() {
                        match serde_json::from_str(&response_text) {
                            Ok(json) => return Ok(json),
                            Err(e) => {
                                last_error = Some(anyhow::anyhow!("Failed to parse JSON: {}", e));
                                retries += 1;
                                continue;
                            }
                        }
                    } else {
                        last_error = Some(anyhow::anyhow!(
                            "Canvas API error: {} - {}", 
                            status, 
                            response_text
                        ));
                        
                        // Don't retry if client error (except rate limiting)
                        if status.is_client_error() && status.as_u16() != 429 {
                            break;
                        }
                        
                        retries += 1;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    retries += 1;
                }
            }
        }
        
        // If we get here, all retries failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Canvas API request failed")))
    }
    
    /// Make a POST request to Canvas API
    async fn post(&self, path: &str, data: &Value) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Canvas API request: POST {}", url);
        
        let mut retries = 0;
        let mut last_error = None;
        
        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }
            
            match self.client.post(&url).json(data).send().await {
                Ok(response) => {
                    let status = response.status();
                    let response_text = response.text().await?;
                    
                    if status.is_success() {
                        match serde_json::from_str(&response_text) {
                            Ok(json) => return Ok(json),
                            Err(e) => {
                                last_error = Some(anyhow::anyhow!("Failed to parse JSON: {}", e));
                                retries += 1;
                                continue;
                            }
                        }
                    } else {
                        last_error = Some(anyhow::anyhow!(
                            "Canvas API error: {} - {}", 
                            status, 
                            response_text
                        ));
                        
                        // Don't retry if client error (except rate limiting)
                        if status.is_client_error() && status.as_u16() != 429 {
                            break;
                        }
                        
                        retries += 1;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    retries += 1;
                }
            }
        }
        
        // If we get here, all retries failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Canvas API request failed")))
    }
}

#[async_trait]
impl CanvasApi for CanvasApiClient {
    async fn get_user(&self, id: &str) -> Result<Value> {
        self.get(&format!("/users/{}", id)).await
    }
    
    async fn get_course(&self, id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}", id)).await
    }
    
    async fn get_assignment(&self, course_id: &str, assignment_id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}/assignments/{}", course_id, assignment_id)).await
    }
    
    async fn get_submission(&self, course_id: &str, assignment_id: &str, user_id: &str) -> Result<Value> {
        self.get(&format!(
            "/courses/{}/assignments/{}/submissions/{}", 
            course_id, assignment_id, user_id
        )).await
    }
    
    async fn get_discussion_topic(&self, course_id: &str, topic_id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}/discussion_topics/{}", course_id, topic_id)).await
    }
    
    async fn create_submission_comment(&self, course_id: &str, assignment_id: &str, user_id: &str, comment: &str) -> Result<Value> {
        let data = serde_json::json!({
            "comment": {
                "text_comment": comment
            }
        });
        
        self.post(&format!(
            "/courses/{}/assignments/{}/submissions/{}", 
            course_id, assignment_id, user_id
        ), &data).await
    }
    
    async fn get_enrollment(&self, course_id: &str, user_id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}/enrollments?user_id={}", course_id, user_id)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_get_user() {
        let mock_server = server_url();
        
        // Setup mock response
        let _m = mock("GET", "/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 123, "name": "Test User", "email": "test@example.com"}"#)
            .create();
        
        // Create client with mock server URL
        let config = CanvasApiConfig {
            base_url: mock_server,
            access_token: "fake-token".to_string(),
            timeout_seconds: 5,
            max_retries: 1,
        };
        
        let client = CanvasApiClient::new(config).unwrap();
        
        // Test API call
        let result = client.get_user("123").await.unwrap();
        
        assert_eq!(result["name"], "Test User");
        assert_eq!(result["email"], "test@example.com");
    }
}
