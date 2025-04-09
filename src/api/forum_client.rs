use leptos::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use reqwasm::http::{Request, Response, Error as ReqwError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::api::forum::{Category, Topic, NewCategory, NewTopic};
use tauri::State;
use std::error::Error;

// API result type with detailed error information
#[derive(Debug)]
pub enum ApiError {
    Network(String),
    Server(u16, String),
    Deserialization(String),
    NotFound,
    Unauthorized,
    RateLimit,
    Timeout,
    Unknown,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::Server(code, msg) => write!(f, "Server error {}: {}", code, msg),
            Self::Deserialization(msg) => write!(f, "Failed to parse response: {}", msg),
            Self::NotFound => write!(f, "Resource not found"),
            Self::Unauthorized => write!(f, "Unauthorized access"),
            Self::RateLimit => write!(f, "Rate limit exceeded"),
            Self::Timeout => write!(f, "Request timed out"),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl From<ReqwError> for ApiError {
    fn from(err: ReqwError) -> Self {
        Self::Network(err.to_string())
    }
}

impl From<JsValue> for ApiError {
    fn from(err: JsValue) -> Self {
        Self::Network(format!("{:?}", err))
    }
}

// Base API client with robust error handling
#[derive(Clone)]
pub struct ForumApiClient {
    base_url: String,
    timeout_ms: u32,
}

impl ForumApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "/api/forum".to_string(),
            timeout_ms: 10000, // 10 seconds
        }
    }
    
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }
    
    pub fn with_timeout(mut self, timeout_ms: u32) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
    
    // GET request with automatic deserialization
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ApiError> 
    where 
        T: for<'de> Deserialize<'de> + 'static
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        // Create a timeout promise
        let timeout_promise = Promise::new(&mut |resolve, _| {
            let window = web_sys::window().unwrap();
            let closure = Closure::once_into_js(move || {
                resolve.call0(&JsValue::NULL).unwrap();
            });
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                self.timeout_ms as i32,
            ).unwrap();
        });
        
        // Race between fetch and timeout
        let request_promise = Request::get(&url)
            .header("Content-Type", "application/json")
            .header("X-Client-Version", "1.0.0")
            .send()
            .await?;
            
        // Check status code for common errors
        match request_promise.status() {
            200 => {}, // OK
            401 => return Err(ApiError::Unauthorized),
            403 => return Err(ApiError::Unauthorized),
            404 => return Err(ApiError::NotFound),
            429 => return Err(ApiError::RateLimit),
            status if status >= 500 => {
                let text = request_promise.text().await.unwrap_or_else(|_| "Unknown server error".to_string());
                return Err(ApiError::Server(status, text));
            },
            _ => return Err(ApiError::Unknown),
        }
        
        // Parse response
        request_promise.json::<T>().await
            .map_err(|e| ApiError::Deserialization(e.to_string()))
    }
    
    // POST request with serialization & deserialization
    pub async fn post<T, R>(&self, endpoint: &str, data: &T) -> Result<R, ApiError>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de> + 'static,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let json_data = serde_json::to_string(data)
            .map_err(|e| ApiError::Deserialization(e.to_string()))?;
            
        let request_promise = Request::post(&url)
            .header("Content-Type", "application/json")
            .header("X-Client-Version", "1.0.0")
            .body(json_data)
            .send()
            .await?;
            
        // Check status code for common errors
        match request_promise.status() {
            200 | 201 | 204 => {}, // OK
            401 => return Err(ApiError::Unauthorized),
            403 => return Err(ApiError::Unauthorized),
            404 => return Err(ApiError::NotFound),
            429 => return Err(ApiError::RateLimit),
            status if status >= 500 => {
                let text = request_promise.text().await.unwrap_or_else(|_| "Unknown server error".to_string());
                return Err(ApiError::Server(status, text));
            },
            _ => return Err(ApiError::Unknown),
        }
        
        // Parse response (or return empty for 204 No Content)
        if request_promise.status() == 204 {
            // Create an empty JSON object for deserializing when no content
            let empty_json = serde_json::json!({});
            serde_json::from_value(empty_json)
                .map_err(|e| ApiError::Deserialization(e.to_string()))
        } else {
            request_promise.json::<R>().await
                .map_err(|e| ApiError::Deserialization(e.to_string()))
        }
    }
    
    // Resource API methods
    
    pub async fn get_categories(&self) -> Result<Vec<Category>, ApiError> {
        self.get("/categories").await
    }
    
    pub async fn get_category(&self, id: i64) -> Result<Category, ApiError> {
        self.get(&format!("/categories/{}", id)).await
    }
    
    pub async fn get_topics_in_category(&self, category_id: i64, page: u32, per_page: u32) -> Result<TopicPage, ApiError> {
        self.get(&format!("/categories/{}/topics?page={}&per_page={}", category_id, page, per_page)).await
    }
    
    pub async fn get_topic(&self, id: i64) -> Result<Topic, ApiError> {
        self.get(&format!("/topics/{}", id)).await
    }
    
    pub async fn get_posts_in_topic(&self, topic_id: i64, page: u32, per_page: u32) -> Result<PostPage, ApiError> {
        self.get(&format!("/topics/{}/posts?page={}&per_page={}", topic_id, page, per_page)).await
    }
    
    pub async fn create_topic(&self, data: CreateTopicRequest) -> Result<Topic, ApiError> {
        self.post("/topics", &data).await
    }
    
    pub async fn create_post(&self, data: CreatePostRequest) -> Result<Post, ApiError> {
        self.post("/posts", &data).await
    }
}

// Resource-specific hook with automatic error handling
#[hook]
pub fn use_forum_resource<T, F>(
    resource_fn: F,
    deps: Vec<ReadSignal<String>>,
) -> Resource<(), Result<T, ApiError>>
where
    T: 'static,
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, ApiError>>>> + 'static,
{
    let deps_signal = create_memo(move |_| {
        // Create a dependency signal that changes when any input signal changes
        deps.iter().map(|sig| sig.get()).collect::<Vec<_>>().join(",")
    });
    
    // Create resource with retry and automatic error handling
    let resource = create_resource(
        move || deps_signal.get(),
        move |_| {
            resource_fn()
        },
    );
    
    resource
}

#[tauri::command]
pub async fn get_categories(
    page: i64, 
    per_page: i64, 
) -> Result<Vec<Category>, String> {
    match crate::api::forum_server::get_categories_handler(page, per_page).await {
        Ok(categories) => Ok(categories),
        Err(e) => Err(format!("Failed to load categories: {}", e)),
    }
}

#[tauri::command]
pub async fn get_category(id: i64) -> Result<Category, String> {
    match crate::api::forum_server::get_category_handler(id).await {
        Ok(category) => Ok(category),
        Err(e) => Err(format!("Failed to load category: {}", e)),
    }
}

#[tauri::command]
pub async fn get_topics(
    page: i64, 
    per_page: i64, 
) -> Result<Vec<Topic>, String> {
    match crate::api::forum_server::get_topics_handler(page, per_page).await {
        Ok(topics) => Ok(topics),
        Err(e) => Err(format!("Failed to load topics: {}", e)),
    }
}

#[tauri::command]
pub async fn get_topics_by_category(
    category_id: i64,
    page: i64, 
    per_page: i64, 
) -> Result<Vec<Topic>, String> {
    match crate::api::forum_server::get_topics_by_category_handler(category_id, page, per_page).await {
        Ok(topics) => Ok(topics),
        Err(e) => Err(format!("Failed to load topics: {}", e)),
    }
}

#[tauri::command]
pub async fn get_topic(id: i64) -> Result<Topic, String> {
    match crate::api::forum_server::get_topic_handler(id).await {
        Ok(topic) => Ok(topic),
        Err(e) => Err(format!("Failed to load topic: {}", e)),
    }
}

#[tauri::command]
pub async fn create_topic(new_topic: NewTopic) -> Result<Topic, String> {
    match crate::api::forum_server::create_topic_handler(new_topic).await {
        Ok(topic) => Ok(topic),
        Err(e) => Err(format!("Failed to create topic: {}", e)),
    }
}

#[tauri::command]
pub async fn update_topic(id: i64, updated_topic: NewTopic) -> Result<Topic, String> {
    match crate::api::forum_server::update_topic_handler(id, updated_topic).await {
        Ok(topic) => Ok(topic),
        Err(e) => Err(format!("Failed to update topic: {}", e)),
    }
}

#[tauri::command]
pub async fn delete_topic(id: i64) -> Result<(), String> {
    match crate::api::forum_server::delete_topic_handler(id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete topic: {}", e)),
    }
}

#[tauri::command]
pub async fn create_category(new_category: NewCategory) -> Result<Category, String> {
    match crate::api::forum_server::create_category_handler(new_category).await {
        Ok(category) => Ok(category),
        Err(e) => Err(format!("Failed to create category: {}", e)),
    }
}