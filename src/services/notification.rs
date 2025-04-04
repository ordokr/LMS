use leptos::*;
use gloo_net::http::Request;
use crate::models::notification::{Notification, NotificationPreferences, NotificationSummary};
use crate::utils::errors::AppError;

pub struct NotificationService;

impl NotificationService {
    pub async fn get_notifications(user_id: i64) -> Result<Vec<Notification>, AppError> {
        let resp = Request::get(&format!("/api/users/{}/notifications", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let notifications = resp.json::<Vec<Notification>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(notifications)
    }

    pub async fn get_notification_summary(user_id: i64) -> Result<NotificationSummary, AppError> {
        let resp = Request::get(&format!("/api/users/{}/notifications/summary", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let summary = resp.json::<NotificationSummary>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(summary)
    }

    pub async fn mark_notification_read(user_id: i64, notification_id: i64) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/users/{}/notifications/{}/read", user_id, notification_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn mark_all_notifications_read(user_id: i64) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/users/{}/notifications/read-all", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn get_notification_preferences(user_id: i64) -> Result<NotificationPreferences, AppError> {
        let resp = Request::get(&format!("/api/users/{}/notification-preferences", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let preferences = resp.json::<NotificationPreferences>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(preferences)
    }

    pub async fn update_notification_preferences(user_id: i64, preferences: NotificationPreferences) -> Result<NotificationPreferences, AppError> {
        let resp = Request::put(&format!("/api/users/{}/notification-preferences", user_id))
            .json(&preferences)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let updated_preferences = resp.json::<NotificationPreferences>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(updated_preferences)
    }
}