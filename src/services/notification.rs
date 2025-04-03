use crate::models::notification::{Notification, NotificationType, NotificationData};

pub struct NotificationService;

impl NotificationService {
    // Get unread notification count for the current user
    pub async fn get_unread_count() -> Result<i64, String> {
        let url = "/api/notifications/unread/count";
        Self::fetch_json::<UnreadResponse>(url)
            .await
            .map(|response| response.count)
    }
    
    // Get recent notifications for the current user
    pub async fn get_recent(limit: usize) -> Result<Vec<Notification>, String> {
        let url = format!("/api/notifications/recent?limit={}", limit);
        Self::fetch_json::<Vec<Notification>>(&url).await
    }
    
    // Get all notifications for the current user with pagination
    pub async fn get_all(page: usize, per_page: usize) -> Result<NotificationsPage, String> {
        let url = format!("/api/notifications?page={}&per_page={}", page, per_page);
        Self::fetch_json::<NotificationsPage>(&url).await
    }
    
    // Mark a notification as read
    pub async fn mark_as_read(id: i64) -> Result<(), String> {
        let url = format!("/api/notifications/{}/read", id);
        Self::fetch_empty("POST", &url).await
    }
    
    // Mark all notifications as read
    pub async fn mark_all_as_read() -> Result<(), String> {
        let url = "/api/notifications/read-all";
        Self::fetch_empty("POST", &url).await
    }
    
    // Helper method for fetching JSON from API
    async fn fetch_json<T>(url: &str) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
    {
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<T>().await {
                        Ok(data) => Ok(data),
                        Err(e) => Err(format!("Failed to parse response: {}", e)),
                    }
                } else {
                    Err(format!("API error: {}", response.status()))
                }
            },
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
    
    // Helper method for requests that don't need response data
    async fn fetch_empty(method: &str, url: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        
        let request = match method {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => return Err("Unsupported HTTP method".to_string()),
        };
        
        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("API error: {}", response.status()))
                }
            },
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[derive(serde::Deserialize)]
struct UnreadResponse {
    count: i64,
}

#[derive(serde::Deserialize)]
pub struct NotificationsPage {
    pub notifications: Vec<Notification>,
    pub total: usize,
    pub page: usize,
    pub total_pages: usize,
}