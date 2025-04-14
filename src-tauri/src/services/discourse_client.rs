use serde::{Serialize, Deserialize};
use reqwest::{Client, header};
use std::sync::Arc;
use crate::models::integration::{DiscourseTopic, DiscourseCategory, SyncStatus};
use crate::db::DB;
use log::{info, error, warn};
use chrono::{DateTime, Utc};
use std::time::Duration;
use thiserror::Error;

/// Error types specific to Discourse API operations
#[derive(Debug, Error)]
pub enum DiscourseError {
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

/// Result type for Discourse operations
pub type DiscourseResult<T> = Result<T, DiscourseError>;

/// Configuration for connecting to Discourse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseConfig {
    /// Base URL of the Discourse instance
    pub url: String,
    
    /// API key for authentication
    pub api_key: String,
    
    /// Username to use for API calls
    pub api_username: String,
    
    /// Timeout for API calls in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    
    /// Polling interval in seconds for sync operations
    #[serde(default = "default_poll_interval")]
    pub poll_interval_seconds: u64,
}

fn default_timeout() -> u64 {
    30
}

fn default_poll_interval() -> u64 {
    300 // 5 minutes
}

impl Default for DiscourseConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            api_key: "".to_string(),
            api_username: "system".to_string(),
            timeout_seconds: default_timeout(),
            poll_interval_seconds: default_poll_interval(),
        }
    }
}

/// Client for interacting with the Discourse API
pub struct DiscourseClient {
    client: Client,
    config: DiscourseConfig,
    db: DB,
}

impl DiscourseClient {
    /// Create a new Discourse client
    pub fn new(config: DiscourseConfig, db: DB) -> Arc<Self> {
        // Set up headers for authentication
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Api-Key",
            header::HeaderValue::from_str(&config.api_key).unwrap_or_default(),
        );
        headers.insert(
            "Api-Username", 
            header::HeaderValue::from_str(&config.api_username).unwrap_or_default(),
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
            db,
        })
    }
    
    /// Check if the client is properly configured
    pub fn is_configured(&self) -> bool {
        !self.config.url.is_empty() && !self.config.api_key.is_empty()
    }
    
    /// Get the base URL of the Discourse instance
    pub fn base_url(&self) -> &str {
        &self.config.url
    }
    
    /// Test connectivity to the Discourse API
    pub async fn test_connection(&self) -> DiscourseResult<bool> {
        if !self.is_configured() {
            return Err(DiscourseError::Configuration("Discourse client is not configured".to_string()));
        }
        
        let url = format!("{}/site.json", self.config.url.trim_end_matches('/'));
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(true)
                } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                    Err(DiscourseError::Authentication("Invalid API key or username".to_string()))
                } else {
                    Err(DiscourseError::Api(format!("Failed to connect: status code {}", response.status())))
                }
            },
            Err(e) => Err(DiscourseError::Network(e)),
        }
    }
    
    /// Get categories from Discourse
    pub async fn get_categories(&self) -> DiscourseResult<Vec<DiscourseCategory>> {
        if !self.is_configured() {
            return Err(DiscourseError::Configuration("Discourse client is not configured".to_string()));
        }
        
        let url = format!("{}/categories.json", self.config.url.trim_end_matches('/'));
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        let categories_response: serde_json::Value = response.json().await?;
        
        // Extract categories from the response and convert to our model
        let category_list = categories_response
            .get("category_list")
            .and_then(|list| list.get("categories"));
        
        if let Some(categories) = category_list {
            if let Some(categories_array) = categories.as_array() {
                let mut result = Vec::with_capacity(categories_array.len());
                
                for category in categories_array {
                    let id = category.get("id").and_then(|v| v.as_i64());
                    let name = category.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    
                    if let Some(id) = id {
                        let description = category.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let color = category.get("color").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let topic_count = category.get("topic_count").and_then(|v| v.as_i64()).map(|n| n as i32);
                        let parent_id = category.get("parent_category_id").and_then(|v| v.as_i64());
                        
                        // Map permission settings to a string representation
                        let read_restricted = category.get("read_restricted").and_then(|v| v.as_bool()).unwrap_or(false);
                        let permissions = if read_restricted {
                            Some("Restricted".to_string())
                        } else {
                            Some("Everyone".to_string())
                        };
                        
                        // Generate a unique local ID for this category
                        let local_id = format!("discourse_category_{}", id);
                        
                        result.push(DiscourseCategory {
                            id: local_id,
                            discourse_category_id: Some(id),
                            name,
                            description,
                            color,
                            topic_count,
                            parent_id,
                            sync_status: Some(SyncStatus::Synced.to_string()),
                            permissions,
                            canvas_course_id: None, // Will be populated from mapping table
                        });
                    }
                }
                
                return Ok(result);
            }
        }
        
        Err(DiscourseError::InvalidResponse("Failed to parse categories response".to_string()))
    }
    
    /// Get recent topics from Discourse
    pub async fn get_recent_topics(&self, limit: usize) -> DiscourseResult<Vec<DiscourseTopic>> {
        if !self.is_configured() {
            return Err(DiscourseError::Configuration("Discourse client is not configured".to_string()));
        }
        
        let url = format!("{}/latest.json?order=created&limit={}", 
            self.config.url.trim_end_matches('/'), 
            limit
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        let topics_response: serde_json::Value = response.json().await?;
        
        // Extract topics from the response and convert to our model
        let topics_list = topics_response.get("topic_list").and_then(|list| list.get("topics"));
        
        if let Some(topics) = topics_list {
            if let Some(topics_array) = topics.as_array() {
                let mut result = Vec::with_capacity(topics_array.len());
                
                for topic in topics_array {
                    let id = topic.get("id").and_then(|v| v.as_i64());
                    let title = topic.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    
                    if let Some(id) = id {
                        let category_id = topic.get("category_id").and_then(|v| v.as_i64());
                        let post_count = topic.get("posts_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                        let slug = topic.get("slug").and_then(|v| v.as_str()).unwrap_or("");
                        
                        // Get category name from the DB based on category_id
                        let category = if let Some(cat_id) = category_id {
                            // In a real implementation, this would query the DB for the category name
                            Some(format!("Category {}", cat_id))
                        } else {
                            None
                        };
                        
                        // Generate a unique local ID for this topic
                        let local_id = format!("discourse_topic_{}", id);
                        
                        // Construct the Discourse URL
                        let discourse_url = Some(format!("{}/t/{}/{}", self.config.url.trim_end_matches('/'), slug, id));
                        
                        result.push(DiscourseTopic {
                            id: local_id,
                            discourse_topic_id: Some(id),
                            title,
                            category,
                            post_count,
                            sync_status: SyncStatus::Synced.to_string(),
                            last_synced_at: Some(Utc::now().to_rfc3339()),
                            discourse_url,
                            exists_in_canvas: false, // Will be populated from mapping table
                            canvas_entity_id: None,
                            canvas_entity_type: None,
                        });
                    }
                }
                
                return Ok(result);
            }
        }
        
        Err(DiscourseError::InvalidResponse("Failed to parse topics response".to_string()))
    }
    
    /// Get topic details by ID
    pub async fn get_topic(&self, topic_id: i64) -> DiscourseResult<DiscourseTopic> {
        if !self.is_configured() {
            return Err(DiscourseError::Configuration("Discourse client is not configured".to_string()));
        }
        
        let url = format!("{}/t/{}.json", self.config.url.trim_end_matches('/'), topic_id);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        let topic_response: serde_json::Value = response.json().await?;
        
        let id = topic_response.get("id").and_then(|v| v.as_i64());
        let title = topic_response.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        if let Some(id) = id {
            let category_id = topic_response.get("category_id").and_then(|v| v.as_i64());
            let post_count = topic_response.get("posts_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let slug = topic_response.get("slug").and_then(|v| v.as_str()).unwrap_or("");
            
            // Get category name from the DB based on category_id
            let category = if let Some(cat_id) = category_id {
                // In a real implementation, this would query the DB for the category name
                Some(format!("Category {}", cat_id))
            } else {
                None
            };
            
            // Generate a unique local ID for this topic
            let local_id = format!("discourse_topic_{}", id);
            
            // Construct the Discourse URL
            let discourse_url = Some(format!("{}/t/{}/{}", self.config.url.trim_end_matches('/'), slug, id));
            
            return Ok(DiscourseTopic {
                id: local_id,
                discourse_topic_id: Some(id),
                title,
                category,
                post_count,
                sync_status: SyncStatus::Synced.to_string(),
                last_synced_at: Some(Utc::now().to_rfc3339()),
                discourse_url,
                exists_in_canvas: false, // Will be populated from mapping table
                canvas_entity_id: None,
                canvas_entity_type: None,
            });
        }
        
        Err(DiscourseError::InvalidResponse("Failed to parse topic response".to_string()))
    }
    
    /// Create a new topic in Discourse
    pub async fn create_topic(&self, title: &str, content: &str, category_id: Option<i64>) -> DiscourseResult<i64> {
        if !self.is_configured() {
            return Err(DiscourseError::Configuration("Discourse client is not configured".to_string()));
        }
        
        let url = format!("{}/posts.json", self.config.url.trim_end_matches('/'));
        
        // Prepare the request body
        let mut body = serde_json::json!({
            "title": title,
            "raw": content,
        });
        
        if let Some(cat_id) = category_id {
            body["category"] = serde_json::json!(cat_id);
        }
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        
        let result: serde_json::Value = response.json().await?;
        
        // Extract the topic ID from the response
        if let Some(topic_id) = result.get("topic_id").and_then(|v| v.as_i64()) {
            Ok(topic_id)
        } else {
            Err(DiscourseError::InvalidResponse("No topic ID in response".to_string()))
        }
    }
    
    /// Update a topic in Discourse
    pub async fn update_topic(&self, topic_id: i64, title: &str) -> DiscourseResult<()> {
        if !self.is_configured() {
            return Err(DiscourseError::Configuration("Discourse client is not configured".to_string()));
        }
        
        let url = format!("{}/t/{}.json", self.config.url.trim_end_matches('/'), topic_id);
        
        // Prepare the request body
        let body = serde_json::json!({
            "title": title,
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
    async fn handle_error_response<T>(&self, response: reqwest::Response) -> DiscourseResult<T> {
        let status = response.status();
        
        match status.as_u16() {
            401 => Err(DiscourseError::Authentication("Invalid API key or username".to_string())),
            404 => Err(DiscourseError::NotFound("Resource not found".to_string())),
            429 => Err(DiscourseError::RateLimit),
            _ => {
                // Try to get error message from response
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(DiscourseError::Api(format!("API error ({}) {}", status, error_text)))
            }
        }
    }
}
