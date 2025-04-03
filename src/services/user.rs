use crate::models::user::{User, UserUpdateRequest};
use leptos::logging::log;

pub struct UserService;

impl UserService {
    // Get user profile by ID
    pub async fn get_user(id: i64) -> Result<User, String> {
        let url = format!("/api/users/{}", id);
        Self::fetch_json::<User>(&url).await
    }
    
    // Update user profile
    pub async fn update_user(id: i64, update: UserUpdateRequest) -> Result<(), String> {
        let url = format!("/api/users/{}", id);
        Self::fetch_with_json::<(), UserUpdateRequest>("PUT", &url, &update).await
    }
    
    // Helper method for fetching JSON from API
    async fn fetch_json<T>(url: &str) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
    {
        // This is a simplified version - in a real app, use actual fetch logic
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
    
    // Helper method for sending JSON to API
    async fn fetch_with_json<T, U>(method: &str, url: &str, data: &U) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
        U: serde::Serialize,
    {
        // This is a simplified version - in a real app, use actual fetch logic
        let client = reqwest::Client::new();
        
        let request = match method {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "PATCH" => client.patch(url),
            _ => return Err("Unsupported HTTP method".to_string()),
        };
        
        match request.json(data).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<()>() {
                        // If T is (), we don't need to parse the response
                        Ok(unsafe { std::mem::zeroed() })
                    } else {
                        match response.json::<T>().await {
                            Ok(data) => Ok(data),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    }
                } else {
                    Err(format!("API error: {}", response.status()))
                }
            },
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    // Add these methods to your UserService implementation

    pub async fn get_preferences(user_id: i64) -> Result<UserPreferences, AppError> {
        let resp = Request::get(&format!("/api/users/{}/preferences", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let preferences = resp.json::<UserPreferences>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(preferences)
    }

    pub async fn update_preferences(user_id: i64, preferences: UserPreferencesUpdate) -> Result<UserPreferences, AppError> {
        let resp = Request::put(&format!("/api/users/{}/preferences", user_id))
            .json(&preferences)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let updated_preferences = resp.json::<UserPreferences>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(updated_preferences)
    }

    pub async fn reset_preferences(user_id: i64) -> Result<UserPreferences, AppError> {
        let resp = Request::post(&format!("/api/users/{}/preferences/reset", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let default_preferences = resp.json::<UserPreferences>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(default_preferences)
    }

    pub async fn get_topic_subscriptions(user_id: i64) -> Result<Vec<TopicSubscription>, AppError> {
        let resp = Request::get(&format!("/api/users/{}/subscriptions", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let subscriptions = resp.json::<Vec<TopicSubscription>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(subscriptions)
    }

    pub async fn update_topic_subscription(user_id: i64, topic_id: i64, notification_level: String) -> Result<TopicSubscription, AppError> {
        let resp = Request::put(&format!("/api/users/{}/subscriptions/{}", user_id, topic_id))
            .json(&serde_json::json!({ "notification_level": notification_level }))
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let subscription = resp.json::<TopicSubscription>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(subscription)
    }

    pub async fn unsubscribe_from_topic(user_id: i64, topic_id: i64) -> Result<(), AppError> {
        let resp = Request::delete(&format!("/api/users/{}/subscriptions/{}", user_id, topic_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn get_bookmarks(user_id: i64) -> Result<Vec<BookmarkedTopic>, AppError> {
        let resp = Request::get(&format!("/api/users/{}/bookmarks", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let bookmarks = resp.json::<Vec<BookmarkedTopic>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(bookmarks)
    }

    pub async fn add_bookmark(user_id: i64, topic_id: i64, post_id: Option<i64>, note: Option<String>) -> Result<BookmarkedTopic, AppError> {
        let resp = Request::post(&format!("/api/users/{}/bookmarks", user_id))
            .json(&serde_json::json!({
                "topic_id": topic_id,
                "post_id": post_id,
                "note": note
            }))
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 201 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let bookmark = resp.json::<BookmarkedTopic>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(bookmark)
    }

    pub async fn update_bookmark_note(user_id: i64, bookmark_id: i64, note: String) -> Result<BookmarkedTopic, AppError> {
        let resp = Request::put(&format!("/api/users/{}/bookmarks/{}", user_id, bookmark_id))
            .json(&serde_json::json!({ "note": note }))
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let bookmark = resp.json::<BookmarkedTopic>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(bookmark)
    }

    pub async fn remove_bookmark(user_id: i64, bookmark_id: i64) -> Result<(), AppError> {
        let resp = Request::delete(&format!("/api/users/{}/bookmarks/{}", user_id, bookmark_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }
}