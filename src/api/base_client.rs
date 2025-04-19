use reqwest::{Client, Response, StatusCode, Error as ReqwestError};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::time::Duration;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use async_trait::async_trait;

/// API client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiClientConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub api_token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub timeout_seconds: u64,
    pub user_agent: String,
}

/// API client trait
#[async_trait]
pub trait ApiClient {
    /// Get the client configuration
    fn get_config(&self) -> &ApiClientConfig;
    
    /// Get the HTTP client
    fn get_client(&self) -> &Client;
    
    /// Make a GET request
    async fn get<T: DeserializeOwned + Send + Sync>(&self, path: &str, params: Option<HashMap<String, String>>) -> Result<T>;
    
    /// Make a POST request
    async fn post<T: DeserializeOwned + Send + Sync, B: Serialize + Send + Sync>(&self, path: &str, body: &B) -> Result<T>;
    
    /// Make a PUT request
    async fn put<T: DeserializeOwned + Send + Sync, B: Serialize + Send + Sync>(&self, path: &str, body: &B) -> Result<T>;
    
    /// Make a DELETE request
    async fn delete<T: DeserializeOwned + Send + Sync>(&self, path: &str) -> Result<T>;
}

/// Base API client implementation
pub struct BaseApiClient {
    config: ApiClientConfig,
    client: Client,
}

impl BaseApiClient {
    /// Create a new base API client
    pub fn new(config: ApiClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .build()?;
        
        Ok(Self {
            config,
            client,
        })
    }
    
    /// Add authentication headers to a request
    pub fn add_auth_headers(&self, headers: &mut reqwest::header::HeaderMap) -> Result<()> {
        if let Some(api_key) = &self.config.api_key {
            headers.insert("X-API-Key", api_key.parse()?);
        }
        
        if let Some(api_token) = &self.config.api_token {
            headers.insert("Authorization", format!("Bearer {}", api_token).parse()?);
        }
        
        Ok(())
    }
    
    /// Handle API response
    pub async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        let response_text = response.text().await?;
        
        if status.is_success() {
            match serde_json::from_str::<T>(&response_text) {
                Ok(data) => Ok(data),
                Err(e) => Err(anyhow!("Failed to deserialize response: {}, Response: {}", e, response_text)),
            }
        } else {
            Err(anyhow!("API request failed with status {}: {}", status, response_text))
        }
    }
}

#[async_trait]
impl ApiClient for BaseApiClient {
    fn get_config(&self) -> &ApiClientConfig {
        &self.config
    }
    
    fn get_client(&self) -> &Client {
        &self.client
    }
    
    async fn get<T: DeserializeOwned + Send + Sync>(&self, path: &str, params: Option<HashMap<String, String>>) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self.client.get(&url);
        
        if let Some(params) = params {
            request = request.query(&params);
        }
        
        let mut headers = reqwest::header::HeaderMap::new();
        self.add_auth_headers(&mut headers)?;
        request = request.headers(headers);
        
        let response = request.send().await?;
        self.handle_response::<T>(response).await
    }
    
    async fn post<T: DeserializeOwned + Send + Sync, B: Serialize + Send + Sync>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self.client.post(&url);
        
        let mut headers = reqwest::header::HeaderMap::new();
        self.add_auth_headers(&mut headers)?;
        request = request.headers(headers);
        
        let response = request.json(body).send().await?;
        self.handle_response::<T>(response).await
    }
    
    async fn put<T: DeserializeOwned + Send + Sync, B: Serialize + Send + Sync>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self.client.put(&url);
        
        let mut headers = reqwest::header::HeaderMap::new();
        self.add_auth_headers(&mut headers)?;
        request = request.headers(headers);
        
        let response = request.json(body).send().await?;
        self.handle_response::<T>(response).await
    }
    
    async fn delete<T: DeserializeOwned + Send + Sync>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self.client.delete(&url);
        
        let mut headers = reqwest::header::HeaderMap::new();
        self.add_auth_headers(&mut headers)?;
        request = request.headers(headers);
        
        let response = request.send().await?;
        self.handle_response::<T>(response).await
    }
}
