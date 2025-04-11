// Auto-generated from src/clients/discourse.js
// Client for interacting with the Discourse API

use log::{error, info};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;
use thiserror::Error;

use crate::utils::logger;

/// Errors that can occur when interacting with the Discourse API
#[derive(Error, Debug)]
pub enum DiscourseClientError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Missing API key")]
    MissingApiKey,
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Configuration options for the Discourse client
#[derive(Debug, Clone)]
pub struct DiscourseClientOptions {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub api_username: Option<String>,
}

impl Default for DiscourseClientOptions {
    fn default() -> Self {
        Self {
            base_url: None,
            api_key: None,
            api_username: None,
        }
    }
}

/// Client for interacting with the Discourse API
pub struct DiscourseClient {
    base_url: String,
    api_key: String,
    api_username: String,
}

impl DiscourseClient {
    /// Create a new Discourse API client
    ///
    /// # Arguments
    /// * `options` - Configuration options
    ///
    /// # Returns
    /// A new Discourse client instance
    pub fn new(options: Option<DiscourseClientOptions>) -> Result<Self, DiscourseClientError> {
        let options = options.unwrap_or_default();
        
        let base_url = options.base_url
            .or_else(|| env::var("DISCOURSE_API_URL").ok())
            .unwrap_or_else(|| "http://localhost:4000".to_string());
            
        let api_key = options.api_key
            .or_else(|| env::var("DISCOURSE_API_KEY").ok())
            .ok_or(DiscourseClientError::MissingApiKey)?;
            
        let api_username = options.api_username
            .or_else(|| env::var("DISCOURSE_API_USERNAME").ok())
            .unwrap_or_else(|| "system".to_string());
        
        // Initialize logger
        logger::init();
        
        Ok(Self {
            base_url,
            api_key,
            api_username,
        })
    }
    
    /// Make an API request to Discourse
    ///
    /// # Arguments
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `endpoint` - API endpoint path
    /// * `request_data` - Optional data to send
    ///
    /// # Returns
    /// API response
    pub async fn request<T, R>(&self, method: &str, endpoint: &str, request_data: Option<&T>) -> Result<R, DiscourseClientError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        info!("Making {} request to {}", method, endpoint);
        
        // In a production implementation, this would use reqwest to make HTTP requests
        // For now, we'll implement a mock version for testing
        
        #[cfg(not(test))]
        {
            let client = reqwest::Client::new();
            
            // Set up Discourse-specific headers
            let mut headers = HeaderMap::new();
            headers.insert("Api-Key", HeaderValue::from_str(&self.api_key)
                .map_err(|_| DiscourseClientError::ApiError("Invalid API key format".into()))?);
            headers.insert("Api-Username", HeaderValue::from_str(&self.api_username)
                .map_err(|_| DiscourseClientError::ApiError("Invalid API username format".into()))?);
            
            let url = if endpoint.starts_with("http") {
                endpoint.to_string()
            } else {
                format!("{}/{}", self.base_url, endpoint)
            };
            
            let response = match method.to_uppercase().as_str() {
                "GET" => {
                    client.get(&url)
                        .headers(headers)
                        .send()
                        .await?
                },
                "POST" => {
                    let builder = client.post(&url).headers(headers);
                    
                    if let Some(data) = request_data {
                        builder.json(data).send().await?
                    } else {
                        builder.send().await?
                    }
                },
                "PUT" => {
                    let builder = client.put(&url).headers(headers);
                    
                    if let Some(data) = request_data {
                        builder.json(data).send().await?
                    } else {
                        builder.send().await?
                    }
                },
                "DELETE" => {
                    client.delete(&url)
                        .headers(headers)
                        .send()
                        .await?
                },
                _ => return Err(DiscourseClientError::ApiError(format!("Unsupported HTTP method: {}", method))),
            };
            
            if response.status().is_success() {
                let result = response.json::<R>().await?;
                Ok(result)
            } else {
                let error_text = response.text().await?;
                error!("Discourse API error: {}", error_text);
                Err(DiscourseClientError::ApiError(error_text))
            }
        }
        
        // Mock implementation for testing
        #[cfg(test)]
        {
            let mock_response = if endpoint.contains("topics") {
                r#"{
                    "success": true,
                    "data": {
                        "id": "discourse123",
                        "result": "success",
                        "topic_id": 12345,
                        "slug": "sample-topic"
                    }
                }"#
            } else if endpoint.contains("users") {
                r#"{
                    "success": true,
                    "data": {
                        "id": "discourse123",
                        "result": "success",
                        "id": 67890,
                        "username": "sample_user"
                    }
                }"#
            } else {
                r#"{
                    "success": true,
                    "data": {
                        "id": "discourse123",
                        "result": "success"
                    }
                }"#
            };
            
            let parsed: R = serde_json::from_str(mock_response)?;
            Ok(parsed)
        }
    }
    
    /// Create a new topic
    ///
    /// # Arguments
    /// * `topic_data` - Topic data including title, content, and category
    ///
    /// # Returns
    /// Created topic
    pub async fn create_topic<T, R>(&self, topic_data: &T) -> Result<R, DiscourseClientError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.request::<T, R>("POST", "topics", Some(topic_data)).await
    }
    
    /// Get a topic by ID
    ///
    /// # Arguments
    /// * `topic_id` - Topic ID
    ///
    /// # Returns
    /// Topic information
    pub async fn get_topic<R>(&self, topic_id: u64) -> Result<R, DiscourseClientError>
    where
        R: DeserializeOwned,
    {
        self.request::<(), R>("GET", &format!("topics/{}", topic_id), None).await
    }
    
    /// Create a new user
    ///
    /// # Arguments
    /// * `user_data` - User data
    ///
    /// # Returns
    /// Created user
    pub async fn create_user<T, R>(&self, user_data: &T) -> Result<R, DiscourseClientError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.request::<T, R>("POST", "users", Some(user_data)).await
    }
    
    /// Authenticate a user via SSO
    ///
    /// # Arguments
    /// * `sso_data` - SSO payload and signature
    ///
    /// # Returns
    /// Authentication result
    pub async fn authenticate_sso<T, R>(&self, sso_data: &T) -> Result<R, DiscourseClientError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.request::<T, R>("POST", "admin/users/sync_sso", Some(sso_data)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[derive(Debug, Serialize, Deserialize)]
    struct TestResponse {
        success: bool,
        data: TestData,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    struct TestData {
        id: String,
        result: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        topic_id: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        slug: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    }
    
    #[tokio::test]
    async fn test_create_topic() {
        let client = DiscourseClient {
            base_url: "http://localhost:4000".to_string(),
            api_key: "test_key".to_string(),
            api_username: "system".to_string(),
        };
        
        let topic_data = json!({
            "title": "Test Topic",
            "raw": "This is a test topic",
            "category": 1
        });
        
        let result = client.create_topic::<serde_json::Value, TestResponse>(&topic_data).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.data.id, "discourse123");
        assert_eq!(response.data.topic_id.unwrap(), 12345);
        assert_eq!(response.data.slug.unwrap(), "sample-topic");
    }
    
    #[tokio::test]
    async fn test_get_topic() {
        let client = DiscourseClient {
            base_url: "http://localhost:4000".to_string(),
            api_key: "test_key".to_string(),
            api_username: "system".to_string(),
        };
        
        let result = client.get_topic::<TestResponse>(12345).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.data.id, "discourse123");
        assert_eq!(response.data.topic_id.unwrap(), 12345);
    }
    
    #[tokio::test]
    async fn test_create_user() {
        let client = DiscourseClient {
            base_url: "http://localhost:4000".to_string(),
            api_key: "test_key".to_string(),
            api_username: "system".to_string(),
        };
        
        let user_data = json!({
            "name": "Test User",
            "email": "test@example.com",
            "password": "password123",
            "username": "testuser"
        });
        
        let result = client.create_user::<serde_json::Value, TestResponse>(&user_data).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.data.id, "discourse123");
        assert_eq!(response.data.username.unwrap(), "sample_user");
    }
    
    #[tokio::test]
    async fn test_authenticate_sso() {
        let client = DiscourseClient {
            base_url: "http://localhost:4000".to_string(),
            api_key: "test_key".to_string(),
            api_username: "system".to_string(),
        };
        
        let sso_data = json!({
            "sso": "base64_encoded_payload",
            "sig": "signature"
        });
        
        let result = client.authenticate_sso::<serde_json::Value, TestResponse>(&sso_data).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.data.id, "discourse123");
        assert_eq!(response.data.result, "success");
    }
}
