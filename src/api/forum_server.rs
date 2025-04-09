use crate::api::forum::{Category, Topic, NewCategory, NewTopic};
use axum::http::StatusCode;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

const API_BASE: &str = "http://localhost:3000/api/forum";

// Helper for HTTP requests
async fn make_request<T: serde::de::DeserializeOwned>(
    url: &str,
    method: &str,
    body: Option<serde_json::Value>,
) -> Result<T, Box<dyn Error>> {
    let client = Client::new();
    
    let mut req = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        _ => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unsupported HTTP method: {}", method)
        ))),
    };
    
    if let Some(json_body) = body {
        req = req.json(&json_body);
    }
    
    let res = req.send().await?;
    
    if !res.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("API error: {}", res.status())
        )));
    }
    
    let data = res.json::<T>().await?;
    Ok(data)
}

// API implementation functions
pub async fn get_categories_handler(page: i64, per_page: i64) -> Result<Vec<Category>, Box<dyn Error>> {
    let url = format!("{}/categories?page={}&per_page={}", API_BASE, page, per_page);
    make_request::<Vec<Category>>(&url, "GET", None).await
}

pub async fn get_category_handler(id: i64) -> Result<Category, Box<dyn Error>> {
    let url = format!("{}/categories/{}", API_BASE, id);
    make_request::<Category>(&url, "GET", None).await
}

pub async fn get_topics_handler(page: i64, per_page: i64) -> Result<Vec<Topic>, Box<dyn Error>> {
    let url = format!("{}/topics?page={}&per_page={}", API_BASE, page, per_page);
    make_request::<Vec<Topic>>(&url, "GET", None).await
}

pub async fn get_topics_by_category_handler(
    category_id: i64, 
    page: i64, 
    per_page: i64
) -> Result<Vec<Topic>, Box<dyn Error>> {
    let url = format!(
        "{}/topics/by-category?category_id={}&page={}&per_page={}", 
        API_BASE, category_id, page, per_page
    );
    make_request::<Vec<Topic>>(&url, "GET", None).await
}

pub async fn get_topic_handler(id: i64) -> Result<Topic, Box<dyn Error>> {
    let url = format!("{}/topics/{}", API_BASE, id);
    make_request::<Topic>(&url, "GET", None).await
}

pub async fn create_topic_handler(new_topic: NewTopic) -> Result<Topic, Box<dyn Error>> {
    let url = format!("{}/topics", API_BASE);
    make_request::<Topic>(&url, "POST", Some(json!(new_topic))).await
}

pub async fn create_category_handler(new_category: NewCategory) -> Result<Category, Box<dyn Error>> {
    let url = format!("{}/categories", API_BASE);
    make_request::<Category>(&url, "POST", Some(json!(new_category))).await
}

// Add update topic handler
pub async fn update_topic_handler(id: i64, updated_topic: NewTopic) -> Result<Topic, Box<dyn Error>> {
    let url = format!("{}/topics/{}", API_BASE, id);
    make_request::<Topic>(&url, "PUT", Some(json!(updated_topic))).await
}

// Add delete topic handler
pub async fn delete_topic_handler(id: i64) -> Result<StatusCode, Box<dyn Error>> {
    let url = format!("{}/topics/{}", API_BASE, id);
    // We expect a successful status code response for deletion
    let _ = make_request::<serde_json::Value>(&url, "DELETE", None).await?;
    Ok(StatusCode::NO_CONTENT)
}