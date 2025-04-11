use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

use crate::utils::logger::create_logger;

/// Discourse API client configuration
#[derive(Debug, Clone)]
pub struct DiscourseClientConfig {
    /// Base URL for the Discourse API
    pub base_url: String,
    
    /// API key for authenticating with Discourse
    pub api_key: String,
    
    /// Username for the API
    pub api_username: String,
}

impl Default for DiscourseClientConfig {
    fn default() -> Self {
        Self {
            base_url: env::var("DISCOURSE_API_URL")
                .unwrap_or_else(|_| "http://localhost:4000".to_string()),
            api_key: env::var("DISCOURSE_API_KEY")
                .unwrap_or_default(),
            api_username: env::var("DISCOURSE_API_USERNAME")
                .unwrap_or_else(|_| "system".to_string()),
        }
    }
}

/// Client for interacting with the Discourse API
pub struct DiscourseClient {
    config: DiscourseClientConfig,
    http_client: Client,
    logger: slog::Logger,
}

/// Discourse topic data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicData {
    pub title: String,
    pub raw: String,
    pub category: u32,
}

/// Discourse user data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
}

/// Discourse SSO data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoData {
    pub sso: String,
    pub sig: String,
}

impl DiscourseClient {
    /// Create a new Discourse API client
    pub fn new(config: Option<DiscourseClientConfig>) -> Self {
        let config = config.unwrap_or_default();
        let http_client = Client::new();
        let logger = create_logger("discourse-client");
        
        Self {
            config,
            http_client,
            logger,
        }
    }
    
    /// Make an API request to Discourse
    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        endpoint: &str,
        request_data: Option<&impl Serialize>,
    ) -> Result<T> {
        slog::info!(self.logger, "Making {} request to {}", method, endpoint);
        
        // In a real implementation, this would use the HTTP client
        // This is a mock implementation for testing
        
        // TODO: Replace with actual HTTP request implementation
        // For now, just return mock data
        
        #[derive(Serialize, Deserialize)]
        struct MockResponse<T> {
            success: bool,
            data: T,
        }
        
        // Sample mock data based on endpoint
        let mock_json = if endpoint.contains("topics") {
            r#"
            {
                "success": true,
                "data": {
                    "id": "discourse123",
                    "result": "success",
                    "topic_id": 12345,
                    "slug": "sample-topic"
                }
            }
            "#
        } else if endpoint.contains("users") {
            r#"
            {
                "success": true,
                "data": {
                    "id": "discourse123",
                    "result": "success",
                    "id": 67890,
                    "username": "sample_user"
                }
            }
            "#
        } else {
            r#"
            {
                "success": true,
                "data": {
                    "id": "discourse123",
                    "result": "success"
                }
            }
            "#
        };
        
        // Parse the mock JSON
        let mock_response: MockResponse<T> = serde_json::from_str(mock_json)
            .with_context(|| format!("Failed to parse mock response for endpoint {}", endpoint))?;
        
        Ok(mock_response.data)
    }
    
    /// Create a new topic
    pub async fn create_topic(&self, topic_data: &TopicData) -> Result<serde_json::Value> {
        self.request::<serde_json::Value>("POST", "topics", Some(topic_data)).await
    }
    
    /// Get a topic by ID
    pub async fn get_topic(&self, topic_id: u32) -> Result<serde_json::Value> {
        self.request::<serde_json::Value>("GET", &format!("topics/{}", topic_id), None).await
    }
    
    /// Create a new user
    pub async fn create_user(&self, user_data: &UserData) -> Result<serde_json::Value> {
        self.request::<serde_json::Value>("POST", "users", Some(user_data)).await
    }
    
    /// Authenticate a user via SSO
    pub async fn authenticate_sso(&self, sso_data: &SsoData) -> Result<serde_json::Value> {
        self.request::<serde_json::Value>("POST", "admin/users/sync_sso", Some(sso_data)).await
    }
}

/// Create a default Discourse client
pub fn create_discourse_client() -> DiscourseClient {
    DiscourseClient::new(None)
}
