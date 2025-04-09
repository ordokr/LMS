use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::services::content_sync_service::CanvasClient;

pub struct CanvasApiClient {
    client: Client,
    base_url: String,
    api_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CanvasTopic {
    id: String,
    title: String,
    message: String,
}

impl CanvasApiClient {
    pub fn new(base_url: &str, api_token: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_token: api_token.to_string(),
        }
    }
}

#[async_trait]
impl CanvasClient for CanvasApiClient {
    async fn get_topic_content(&self, topic_id: &str) -> Result<String> {
        let url = format!("{}/api/v1/discussion_topics/{}", self.base_url, topic_id);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to get Canvas topic: status code {}",
                response.status()
            ));
        }
        
        let topic: CanvasTopic = response.json().await?;
        Ok(topic.message)
    }
    
    async fn update_topic_content(&self, topic_id: &str, content: &str) -> Result<()> {
        let url = format!("{}/api/v1/discussion_topics/{}", self.base_url, topic_id);
        
        let response = self.client.put(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&serde_json::json!({
                "message": content
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to update Canvas topic: status code {}",
                response.status()
            ));
        }
        
        Ok(())
    }
}