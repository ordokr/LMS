use crate::api::{ApiClientConfig, CanvasApiClient, DiscourseApiClient, CanvasApi, DiscourseApi};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::fs;
use std::path::Path;

/// API configuration service
/// 
/// Manages the configuration for Canvas and Discourse API clients
pub struct ApiConfigService {
    canvas_config: ApiClientConfig,
    discourse_config: ApiClientConfig,
    canvas_client: Option<Arc<CanvasApiClient>>,
    discourse_client: Option<Arc<DiscourseApiClient>>,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub canvas: CanvasConfig,
    pub discourse: DiscourseConfig,
}

/// Canvas API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasConfig {
    pub base_url: String,
    pub api_token: String,
    pub timeout_seconds: Option<u64>,
}

/// Discourse API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_username: String,
    pub timeout_seconds: Option<u64>,
}

impl ApiConfigService {
    /// Create a new API configuration service from a configuration file
    pub fn from_file<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_str = fs::read_to_string(config_path)?;
        let config: ApiConfig = serde_json::from_str(&config_str)?;
        
        Self::from_config(config)
    }
    
    /// Create a new API configuration service from a configuration object
    pub fn from_config(config: ApiConfig) -> Result<Self> {
        // Create Canvas API client configuration
        let canvas_config = ApiClientConfig {
            base_url: config.canvas.base_url,
            api_key: None,
            api_token: Some(config.canvas.api_token),
            username: None,
            password: None,
            timeout_seconds: config.canvas.timeout_seconds.unwrap_or(30),
            user_agent: "Canvas LMS Integration".to_string(),
        };
        
        // Create Discourse API client configuration
        let discourse_config = ApiClientConfig {
            base_url: config.discourse.base_url,
            api_key: Some(config.discourse.api_key),
            api_token: None,
            username: Some(config.discourse.api_username),
            password: None,
            timeout_seconds: config.discourse.timeout_seconds.unwrap_or(30),
            user_agent: "Discourse Integration".to_string(),
        };
        
        Ok(Self {
            canvas_config,
            discourse_config,
            canvas_client: None,
            discourse_client: None,
        })
    }
    
    /// Initialize the API clients
    pub fn initialize(&mut self) -> Result<()> {
        // Create Canvas API client
        let canvas_client = CanvasApiClient::new(self.canvas_config.clone())?;
        self.canvas_client = Some(Arc::new(canvas_client));
        
        // Create Discourse API client
        let discourse_client = DiscourseApiClient::new(self.discourse_config.clone())?;
        self.discourse_client = Some(Arc::new(discourse_client));
        
        Ok(())
    }
    
    /// Get the Canvas API client
    pub fn get_canvas_client(&self) -> Result<Arc<CanvasApiClient>> {
        self.canvas_client.clone().ok_or_else(|| anyhow!("Canvas API client not initialized"))
    }
    
    /// Get the Discourse API client
    pub fn get_discourse_client(&self) -> Result<Arc<DiscourseApiClient>> {
        self.discourse_client.clone().ok_or_else(|| anyhow!("Discourse API client not initialized"))
    }
    
    /// Update the Canvas API configuration
    pub fn update_canvas_config(&mut self, config: CanvasConfig) -> Result<()> {
        self.canvas_config = ApiClientConfig {
            base_url: config.base_url,
            api_key: None,
            api_token: Some(config.api_token),
            username: None,
            password: None,
            timeout_seconds: config.timeout_seconds.unwrap_or(30),
            user_agent: "Canvas LMS Integration".to_string(),
        };
        
        // Re-initialize the Canvas API client
        let canvas_client = CanvasApiClient::new(self.canvas_config.clone())?;
        self.canvas_client = Some(Arc::new(canvas_client));
        
        Ok(())
    }
    
    /// Update the Discourse API configuration
    pub fn update_discourse_config(&mut self, config: DiscourseConfig) -> Result<()> {
        self.discourse_config = ApiClientConfig {
            base_url: config.base_url,
            api_key: Some(config.api_key),
            api_token: None,
            username: Some(config.api_username),
            password: None,
            timeout_seconds: config.timeout_seconds.unwrap_or(30),
            user_agent: "Discourse Integration".to_string(),
        };
        
        // Re-initialize the Discourse API client
        let discourse_client = DiscourseApiClient::new(self.discourse_config.clone())?;
        self.discourse_client = Some(Arc::new(discourse_client));
        
        Ok(())
    }
    
    /// Test the Canvas API connection
    pub async fn test_canvas_connection(&self) -> Result<bool> {
        let client = self.get_canvas_client()?;
        
        // Try to get the current user
        match client.get_user("self").await {
            Ok(_) => Ok(true),
            Err(e) => {
                log::error!("Canvas API connection test failed: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Test the Discourse API connection
    pub async fn test_discourse_connection(&self) -> Result<bool> {
        let client = self.get_discourse_client()?;
        
        // Try to get the site info
        match client.get("/site.json", None).await {
            Ok(_) => Ok(true),
            Err(e) => {
                log::error!("Discourse API connection test failed: {}", e);
                Ok(false)
            }
        }
    }
}
