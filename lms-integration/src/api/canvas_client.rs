//! Canvas API client module
//!
//! Provides type-safe interactions with the Canvas LMS API

use anyhow::{Context, Result};
use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Canvas API client for interacting with Canvas LMS
pub struct CanvasClient {
    client: Client,
    api_url: String,
    api_key: String,
}

impl CanvasClient {
    /// Create a new Canvas API client
    pub async fn new(api_url: &str, api_key: &str) -> Result<Self> {
        let client = Client::new();
        
        Ok(Self {
            client,
            api_url: api_url.to_string(),
            api_key: api_key.to_string(),
        })
    }
    
    /// Fetch a course by ID
    pub async fn get_course(&self, course_id: &str) -> Result<Course> {
        let url = format!("{}/api/v1/courses/{}", self.api_url, course_id);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to send request to Canvas API")?;
            
        if !response.status().is_success() {
            error!("Canvas API error: {:?}", response.status());
            anyhow::bail!("Failed to fetch course: HTTP {}", response.status());
        }
        
        let course = response.json::<Course>().await
            .context("Failed to deserialize course data")?;
            
        Ok(course)
    }
    
    /// List all courses
    pub async fn list_courses(&self) -> Result<Vec<Course>> {
        let url = format!("{}/api/v1/courses", self.api_url);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to send request to Canvas API")?;
            
        if !response.status().is_success() {
            error!("Canvas API error: {:?}", response.status());
            anyhow::bail!("Failed to list courses: HTTP {}", response.status());
        }
        
        let courses = response.json::<Vec<Course>>().await
            .context("Failed to deserialize courses data")?;
            
        Ok(courses)
    }
    
    // Add more Canvas API methods as needed
}

/// Canvas Course model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub course_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}
