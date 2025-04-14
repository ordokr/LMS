use crate::models::notification::Notification;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct NotificationService;

impl NotificationService {
    // Get all notifications
    pub async fn get_notifications() -> Result<Vec<Notification>, String> {
        let result = invoke("get_notifications", JsValue::NULL).await;
        
        match serde_wasm_bindgen::from_value(result) {
            Ok(notifications) => Ok(notifications),
            Err(e) => Err(format!("Failed to parse notifications: {}", e)),
        }
    }
    
    // Get unread notifications
    pub async fn get_unread_notifications() -> Result<Vec<Notification>, String> {
        let result = invoke("get_unread_notifications", JsValue::NULL).await;
        
        match serde_wasm_bindgen::from_value(result) {
            Ok(notifications) => Ok(notifications),
            Err(e) => Err(format!("Failed to parse notifications: {}", e)),
        }
    }
    
    // Get unread notification count
    pub async fn get_unread_count() -> Result<u32, String> {
        let result = invoke("get_unread_notification_count", JsValue::NULL).await;
        
        match serde_wasm_bindgen::from_value(result) {
            Ok(count) => Ok(count),
            Err(e) => Err(format!("Failed to parse notification count: {}", e)),
        }
    }
    
    // Mark notification as read
    pub async fn mark_notification_read(notification_id: &str) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "notification_id": notification_id,
        })).unwrap();
        
        let result = invoke("mark_notification_read", args).await;
        
        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }
    
    // Mark all notifications as read
    pub async fn mark_all_notifications_read() -> Result<(), String> {
        let result = invoke("mark_all_notifications_read", JsValue::NULL).await;
        
        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }
    
    // Dismiss notification
    pub async fn dismiss_notification(notification_id: &str) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "notification_id": notification_id,
        })).unwrap();
        
        let result = invoke("dismiss_notification", args).await;
        
        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }
    
    // Dismiss all notifications
    pub async fn dismiss_all_notifications() -> Result<(), String> {
        let result = invoke("dismiss_all_notifications", JsValue::NULL).await;
        
        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }
}
