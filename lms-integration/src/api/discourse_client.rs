//! Discourse API client module
//!
//! Provides type-safe interactions with the Discourse forum API

use anyhow::{Context, Result};
use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Discourse API client for interacting with Discourse forums
pub struct DiscourseClient {
    client: Client,
    api_url: String,
    api_key: String,
    api_username: String,
}

impl DiscourseClient {
    /// Create a new Discourse API client
    pub async fn new(api_url: &str, api_key: &str, api_username: &str) -> Result<Self> {
        let client = Client::new();
        
        Ok(Self {
            client,
            api_url: api_url.to_string(),
            api_key: api_key.to_string(),
            api_username: api_username.to_string(),
        })
    }
    
    /// Fetch a category by ID
    pub async fn get_category(&self, category_id: u64) -> Result<Category> {
        let url = format!("{}/c/{}.json", self.api_url, category_id);
        
        let response = self.client.get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await
            .context("Failed to send request to Discourse API")?;
            
        if !response.status().is_success() {
            error!("Discourse API error: {:?}", response.status());
            anyhow::bail!("Failed to fetch category: HTTP {}", response.status());
        }
        
        let response_data = response.json::<CategoryResponse>().await
            .context("Failed to deserialize category data")?;
            
        Ok(response_data.category)
    }
    
    /// Create a new category
    pub async fn create_category(&self, name: &str, color: &str, text_color: &str, description: &str, parent_category_id: Option<u64>) -> Result<Category> {
        let url = format!("{}/categories.json", self.api_url);
        
        let mut params = serde_json::json!({
            "name": name,
            "color": color,
            "text_color": text_color,
            "description": description,
        });
        
        if let Some(parent_id) = parent_category_id {
            params["parent_category_id"] = parent_id.into();
        }
        
        let response = self.client.post(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .json(&params)
            .send()
            .await
            .context("Failed to send request to Discourse API")?;
            
        if !response.status().is_success() {
            error!("Discourse API error: {:?}", response.status());
            anyhow::bail!("Failed to create category: HTTP {}", response.status());
        }
        
        let response_data = response.json::<CategoryResponse>().await
            .context("Failed to deserialize category data")?;
            
        Ok(response_data.category)
    }
    
    // Add more Discourse API methods as needed
}

/// Discourse Category model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub text_color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<u64>,
}

/// Response wrapper for category
#[derive(Debug, Deserialize)]
struct CategoryResponse {
    category: Category,
}
