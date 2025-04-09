use leptos::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use std::collections::HashMap;
use web_sys::AbortController;
use futures::future::Abortable;
use crate::state::app_state::{AppStore, StateSection};
use gloo_storage::LocalStorage;
use futures::{stream, StreamExt};

// Models matching backend
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub category_id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewTopic {
    pub title: String,
    pub slug: String,
    pub category_id: i64,
    pub user_id: i64,
}

// API client functions
#[server(GetCategories, "/api")]
pub async fn get_categories(page: i64, per_page: i64) -> Result<Vec<Category>, ServerFnError> {
    use crate::api::forum_server::get_categories_handler;
    
    get_categories_handler(page, per_page).await
}

#[server(GetCategory, "/api")]
pub async fn get_category(id: i64) -> Result<Category, ServerFnError> {
    use crate::api::forum_server::get_category_handler;
    
    get_category_handler(id).await
}

#[server(GetTopics, "/api")]
pub async fn get_topics(page: i64, per_page: i64) -> Result<Vec<Topic>, ServerFnError> {
    use crate::api::forum_server::get_topics_handler;
    
    get_topics_handler(page, per_page).await
}

#[server(GetTopicsByCategory, "/api")]
pub async fn get_topics_by_category(
    category_id: i64, 
    page: i64, 
    per_page: i64
) -> Result<Vec<Topic>, ServerFnError> {
    use crate::api::forum_server::get_topics_by_category_handler;
    
    get_topics_by_category_handler(category_id, page, per_page).await
}

#[server(GetTopic, "/api")]
pub async fn get_topic(id: i64) -> Result<Topic, ServerFnError> {
    use crate::api::forum_server::get_topic_handler;
    
    get_topic_handler(id).await
}

#[server(CreateTopic, "/api")]
pub async fn create_topic(new_topic: NewTopic) -> Result<Topic, ServerFnError> {
    use crate::api::forum_server::create_topic_handler;
    
    create_topic_handler(new_topic).await
}

#[server(CreateCategory, "/api")]
pub async fn create_category(new_category: NewCategory) -> Result<Category, ServerFnError> {
    use crate::api::forum_server::create_category_handler;
    
    create_category_handler(new_category).await
}

// Implement request cache and deduplication
struct RequestManager {
    ongoing_requests: HashMap<String, AbortController>,
    cache_ttl: HashMap<String, (f64, serde_json::Value)>, // (expiry_timestamp, response)
}

impl RequestManager {
    fn new() -> Self {
        Self {
            ongoing_requests: HashMap::new(),
            cache_ttl: HashMap::new(),
        }
    }
    
    fn abort_existing(&mut self, key: &str) {
        if let Some(controller) = self.ongoing_requests.remove(key) {
            controller.abort();
        }
    }
    
    fn register_request(&mut self, key: String) -> AbortController {
        self.abort_existing(&key);
        let controller = AbortController::new().unwrap();
        self.ongoing_requests.insert(key, controller.clone());
        controller
    }
    
    fn complete_request(&mut self, key: &str) {
        self.ongoing_requests.remove(key);
    }
    
    fn get_cached<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        if let Some((expiry, value)) = self.cache_ttl.get(key) {
            let now = js_sys::Date::now();
            if *expiry > now {
                return serde_json::from_value(value.clone()).ok();
            }
        }
        None
    }
    
    fn set_cached(&mut self, key: String, value: serde_json::Value, ttl_ms: f64) {
        let expiry = js_sys::Date::now() + ttl_ms;
        self.cache_ttl.insert(key, (expiry, value));
    }
}

// Singleton instance
static REQUEST_MANAGER: once_cell::sync::Lazy<std::sync::Mutex<RequestManager>> = 
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(RequestManager::new()));

// Optimized API client
pub async fn fetch_topics(
    category_id: i64, 
    page: u32,
    per_page: u32,
    use_cache: bool
) -> Result<Vec<Topic>, String> {
    let key = format!("topics:{}:{}:{}", category_id, page, per_page);
    
    // Check cache first if enabled
    if use_cache {
        let manager = REQUEST_MANAGER.lock().unwrap();
        if let Some(cached) = manager.get_cached::<Vec<Topic>>(&key) {
            return Ok(cached);
        }
    }
    
    // Register new request and abort any existing ones for the same key
    let controller = {
        let mut manager = REQUEST_MANAGER.lock().unwrap();
        manager.register_request(key.clone())
    };
    
    // Make request with abort signal
    let signal = controller.signal();
    let response = Request::get(&format!("/api/forum/categories/{}/topics?page={}&per_page={}", category_id, page, per_page))
        .header("Content-Type", "application/json")
        .abort_signal(Some(&signal))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.ok() {
        return Err(format!("Error fetching topics: {}", response.status()));
    }
    
    let topics: Vec<Topic> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;
    
    // Cache the result
    {
        let mut manager = REQUEST_MANAGER.lock().unwrap();
        manager.complete_request(&key);
        if use_cache {
            manager.set_cached(
                key, 
                serde_json::to_value(&topics).unwrap(), 
                60000.0 // 1 minute TTL
            );
        }
    }
    
    Ok(topics)
}

// Offline-first data fetch with sync capability
pub async fn fetch_topics_offline_first(
    app_store: &AppStore,
    category_id: i64,
    page: u32,
    per_page: u32
) -> Result<Vec<Topic>, String> {
    // Try from local storage first
    let storage_key = format!("offline:topics:{}:{}:{}", category_id, page, per_page);
    let offline_topics: Option<Vec<Topic>> = LocalStorage::get(&storage_key).ok();
    
    // Set from local storage while we fetch fresh data
    if let Some(topics) = &offline_topics {
        app_store.update(StateSection::Forum, |state| {
            state.forum.current_topics = topics.clone();
        });
    }
    
    // Check if we're online
    if app_store.is_online().get() {
        // Fetch fresh data
        match fetch_topics(category_id, page, per_page, true).await {
            Ok(topics) => {
                // Update local storage for offline use
                let _ = LocalStorage::set(&storage_key, &topics);
                
                // Update app state with fresh data
                app_store.update(StateSection::Forum, |state| {
                    state.forum.current_topics = topics.clone();
                });
                
                Ok(topics)
            },
            Err(e) => {
                if let Some(topics) = offline_topics {
                    // Return offline data on error
                    Ok(topics)
                } else {
                    Err(e)
                }
            }
        }
    } else if let Some(topics) = offline_topics {
        // We're offline but have cached data
        Ok(topics)
    } else {
        // We're offline and have no data
        Err("No internet connection and no cached data available".to_string())
    }
}

// Batched request structure
#[derive(Serialize)]
struct BatchRequest {
    requests: Vec<SingleRequest>,
}

#[derive(Serialize)]
struct SingleRequest {
    id: String,
    path: String,
    method: String,
}

#[derive(Deserialize)]
struct BatchResponse {
    responses: HashMap<String, SingleResponse>,
}

#[derive(Deserialize)]
struct SingleResponse {
    status: u16,
    body: serde_json::Value,
}

// Batch multiple forum requests together
async fn batch_api_requests(requests: Vec<(String, String)>) -> Result<HashMap<String, serde_json::Value>, String> {
    // Build batch request
    let batch_requests = requests.iter()
        .map(|(id, path)| SingleRequest {
            id: id.clone(),
            path: path.clone(),
            method: "GET".to_string(),
        })
        .collect();

    let batch = BatchRequest {
        requests: batch_requests,
    };

    // Send batch request
    let response = reqwasm::http::Request::post("/api/forum/batch")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&batch).map_err(|e| e.to_string())?)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Batch request failed with status: {}", response.status()));
    }

    let batch_response: BatchResponse = response.json()
        .await
        .map_err(|e| e.to_string())?;

    // Extract results
    let mut results = HashMap::new();
    for (id, response) in batch_response.responses {
        if response.status >= 200 && response.status < 300 {
            results.insert(id, response.body);
        }
    }

    Ok(results)
}