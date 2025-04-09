use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::services::content_sync_service::DiscourseClient;

pub struct DiscourseApiClient {
    client: Client,
    base_url: String,
    api_key: String,
    api_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiscourseTopic {
    id: i64,
    title: String,
    post_stream: PostStream,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostStream {
    posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: i64,
    cooked: String, // HTML content
    raw: String,    // Markdown content
    post_number: i32,
}

impl DiscourseApiClient {
    pub fn new(base_url: &str, api_key: &str, api_username: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            api_username: api_username.to_string(),
        }
    }
}

#[async_trait]
impl DiscourseClient for DiscourseApiClient {
    async fn get_topic_content(&self, topic_id: &str) -> Result<String> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        
        let response = self.client.get(&url)
            .query(&[
                ("api_key", &self.api_key),
                ("api_username", &self.api_username),
            ])
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to get Discourse topic: status code {}",
                response.status()
            ));
        }
        
        let topic: DiscourseTopic = response.json().await?;
        
        // Get the first post (opening post)
        let first_post = topic.post_stream.posts.iter()
            .find(|post| post.post_number == 1)
            .ok_or_else(|| anyhow!("Could not find opening post"))?;
            
        Ok(first_post.raw.clone())
    }
    
    async fn update_topic_content(&self, topic_id: &str, content: &str) -> Result<()> {
        // First need to get the post_id of the first post
        let topic_url = format!("{}/t/{}.json", self.base_url, topic_id);
        
        let response = self.client.get(&topic_url)
            .query(&[
                ("api_key", &self.api_key),
                ("api_username", &self.api_username),
            ])
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to get Discourse topic: status code {}",
                response.status()
            ));
        }
        
        let topic: DiscourseTopic = response.json().await?;
        
        // Get the first post (opening post)
        let first_post = topic.post_stream.posts.iter()
            .find(|post| post.post_number == 1)
            .ok_or_else(|| anyhow!("Could not find opening post"))?;
        
        // Now update the post
        let post_url = format!("{}/posts/{}.json", self.base_url, first_post.id);
        
        let response = self.client.put(&post_url)
            .query(&[
                ("api_key", &self.api_key),
                ("api_username", &self.api_username),
            ])
            .form(&[
                ("raw", content),
            ])
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to update Discourse post: status code {}",
                response.status()
            ));
        }
        
        Ok(())
    }
}