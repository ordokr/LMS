use crate::config::{Config, CanvasConfig, DiscourseConfig};
use crate::models::*;
use crate::services::errors::ApiError;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

pub struct ApiClient {
    client: Client,
    canvas_config: CanvasConfig,
    discourse_config: DiscourseConfig,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            canvas_config: config.canvas,
            discourse_config: config.discourse,
        }
    }
    
    // Canvas API requests
    async fn canvas_get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.canvas_config.api_url, path);
        
        let mut req = self.client.get(&url);
        
        // Add authentication if available
        if let Some(token) = &self.canvas_config.api_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        let response = req.send().await?;
        
        match response.status() {
            StatusCode::OK => {
                let data = response.json::<T>().await?;
                Ok(data)
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ApiError::AuthError("Authentication failed".to_string()))
            },
            StatusCode::NOT_FOUND => {
                Err(ApiError::NotFound(format!("Resource not found: {}", path)))
            },
            _ => {
                let status = response.status();
                let text = response.text().await.unwrap_or_else(|_| "No error text".to_string());
                Err(ApiError::ServerError(format!("Server returned {}: {}", status, text)))
            }
        }
    }
    
    async fn canvas_post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, ApiError> {
        let url = format!("{}{}", self.canvas_config.api_url, path);
        
        let mut req = self.client.post(&url)
            .json(body);
        
        // Add authentication if available
        if let Some(token) = &self.canvas_config.api_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        
        let response = req.send().await?;
        
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let data = response.json::<T>().await?;
                Ok(data)
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ApiError::AuthError("Authentication failed".to_string()))
            },
            StatusCode::NOT_FOUND => {
                Err(ApiError::NotFound(format!("Resource not found: {}", path)))
            },
            _ => {
                let status = response.status();
                let text = response.text().await.unwrap_or_else(|_| "No error text".to_string());
                Err(ApiError::ServerError(format!("Server returned {}: {}", status, text)))
            }
        }
    }
    
    // Discourse API requests
    async fn discourse_get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.discourse_config.api_url, path);
        
        let mut req = self.client.get(&url);
        
        // Add authentication if available
        if let (Some(key), Some(username)) = (&self.discourse_config.api_key, &self.discourse_config.api_username) {
            req = req.query(&[("api_key", key), ("api_username", username)]);
        }
        
        let response = req.send().await?;
        
        match response.status() {
            StatusCode::OK => {
                let data = response.json::<T>().await?;
                Ok(data)
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ApiError::AuthError("Authentication failed".to_string()))
            },
            StatusCode::NOT_FOUND => {
                Err(ApiError::NotFound(format!("Resource not found: {}", path)))
            },
            _ => {
                let status = response.status();
                let text = response.text().await.unwrap_or_else(|_| "No error text".to_string());
                Err(ApiError::ServerError(format!("Server returned {}: {}", status, text)))
            }
        }
    }
    
    async fn discourse_post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, ApiError> {
        let url = format!("{}{}", self.discourse_config.api_url, path);
        
        let mut req = self.client.post(&url)
            .json(body);
        
        // Add authentication if available
        if let (Some(key), Some(username)) = (&self.discourse_config.api_key, &self.discourse_config.api_username) {
            req = req.query(&[("api_key", key), ("api_username", username)]);
        }
        
        let response = req.send().await?;
        
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let data = response.json::<T>().await?;
                Ok(data)
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ApiError::AuthError("Authentication failed".to_string()))
            },
            StatusCode::NOT_FOUND => {
                Err(ApiError::NotFound(format!("Resource not found: {}", path)))
            },
            _ => {
                let status = response.status();
                let text = response.text().await.unwrap_or_else(|_| "No error text".to_string());
                Err(ApiError::ServerError(format!("Server returned {}: {}", status, text)))
            }
        }
    }
    
    // Canvas endpoint implementations
    pub async fn get_courses(&self) -> Result<Vec<Course>, ApiError> {
        self.canvas_get("/courses").await
    }
    
    pub async fn get_course(&self, id: i64) -> Result<Course, ApiError> {
        self.canvas_get(&format!("/courses/{}", id)).await
    }
    
    pub async fn get_course_assignments(&self, course_id: i64) -> Result<Vec<Assignment>, ApiError> {
        self.canvas_get(&format!("/courses/{}/assignments", course_id)).await
    }
    
    pub async fn get_course_modules(&self, course_id: i64) -> Result<Vec<Module>, ApiError> {
        self.canvas_get(&format!("/courses/{}/modules", course_id)).await
    }
    
    pub async fn get_course_enrollments(&self, course_id: i64) -> Result<Vec<Enrollment>, ApiError> {
        self.canvas_get(&format!("/courses/{}/enrollments", course_id)).await
    }
    
    // Discourse endpoint implementations
    pub async fn get_site_info(&self) -> Result<Site, ApiError> {
        self.discourse_get("/site.json").await
    }
    
    pub async fn get_topics(&self) -> Result<Vec<Topic>, ApiError> {
        // Discourse typically returns a complex structure for topics
        #[derive(serde::Deserialize)]
        struct TopicsResponse {
            topic_list: TopicList,
        }
        
        #[derive(serde::Deserialize)]
        struct TopicList {
            topics: Vec<Topic>,
        }
        
        let response: TopicsResponse = self.discourse_get("/latest.json").await?;
        Ok(response.topic_list.topics)
    }
    
    pub async fn get_topic(&self, id: i64) -> Result<Topic, ApiError> {
        self.discourse_get(&format!("/t/{}.json", id)).await
    }
    
    pub async fn get_categories(&self) -> Result<Vec<Category>, ApiError> {
        #[derive(serde::Deserialize)]
        struct CategoriesResponse {
            category_list: CategoryList,
        }
        
        #[derive(serde::Deserialize)]
        struct CategoryList {
            categories: Vec<Category>,
        }
        
        let response: CategoriesResponse = self.discourse_get("/categories.json").await?;
        Ok(response.category_list.categories)
    }
}