use serde::{Serialize, Deserialize};
use reqwest::Client;
use crate::utils::errors::ApiError;
use crate::utils::auth::get_auth_token;
use crate::utils::offline::is_online;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        // For Tauri - this will be a local endpoint
        let base_url = "/api".to_string();
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, ApiError> 
    where
        T: for<'de> Deserialize<'de>,
    {
        if !is_online() {
            return Err(ApiError::Offline);
        }

        let url = format!("{}{}", self.base_url, path);
        let token = get_auth_token().ok();
        
        let mut req = self.client.get(&url);
        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        match req.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<T>().await {
                        Ok(data) => Ok(data),
                        Err(e) => Err(ApiError::Deserialization(e.to_string())),
                    }
                } else {
                    let status = response.status().as_u16();
                    let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(ApiError::ServerError(status, text))
                }
            }
            Err(e) => Err(ApiError::NetworkError(e.to_string())),
        }
    }

    pub async fn post<Req, Res>(&self, path: &str, body: Req) -> Result<Res, ApiError>
    where
        Req: Serialize,
        Res: for<'de> Deserialize<'de>,
    {
        if !is_online() {
            return Err(ApiError::Offline);
        }

        let url = format!("{}{}", self.base_url, path);
        let token = get_auth_token().ok();
        
        let mut req = self.client.post(&url);
        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        match req.json(&body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Res>().await {
                        Ok(data) => Ok(data),
                        Err(e) => Err(ApiError::Deserialization(e.to_string())),
                    }
                } else {
                    let status = response.status().as_u16();
                    let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(ApiError::ServerError(status, text))
                }
            }
            Err(e) => Err(ApiError::NetworkError(e.to_string())),
        }
    }
    
    pub async fn put<Req, Res>(&self, path: &str, body: Req) -> Result<Res, ApiError>
    where
        Req: Serialize,
        Res: for<'de> Deserialize<'de>,
    {
        if !is_online() {
            return Err(ApiError::Offline);
        }

        let url = format!("{}{}", self.base_url, path);
        let token = get_auth_token().ok();
        
        let mut req = self.client.put(&url);
        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        match req.json(&body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Res>().await {
                        Ok(data) => Ok(data),
                        Err(e) => Err(ApiError::Deserialization(e.to_string())),
                    }
                } else {
                    let status = response.status().as_u16();
                    let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(ApiError::ServerError(status, text))
                }
            }
            Err(e) => Err(ApiError::NetworkError(e.to_string())),
        }
    }
    
    pub async fn delete<T>(&self, path: &str) -> Result<T, ApiError> 
    where
        T: for<'de> Deserialize<'de>,
    {
        if !is_online() {
            return Err(ApiError::Offline);
        }

        let url = format!("{}{}", self.base_url, path);
        let token = get_auth_token().ok();
        
        let mut req = self.client.delete(&url);
        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        match req.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<T>().await {
                        Ok(data) => Ok(data),
                        Err(e) => Err(ApiError::Deserialization(e.to_string())),
                    }
                } else {
                    let status = response.status().as_u16();
                    let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(ApiError::ServerError(status, text))
                }
            }
            Err(e) => Err(ApiError::NetworkError(e.to_string())),
        }
    }
}