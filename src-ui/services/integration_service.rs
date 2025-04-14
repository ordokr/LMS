use crate::models::integration::{
    IntegrationStatus,
    SyncHistoryEntry,
    Topic,
    Category,
    SyncConflict,
    ConflictResolutionStrategy,
    CanvasCourse,
    CanvasAssignment
};
use leptos::*;
use wasm_bindgen::prelude::*;
use crate::models::integration::{IntegrationStatus, SyncHistoryEntry, SyncConflict};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct IntegrationService;

impl IntegrationService {
    // Get Discourse integration status
    pub async fn get_discourse_integration_status() -> Result<IntegrationStatus, String> {
        let result = invoke("get_discourse_integration_status", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(status) => Ok(status),
            Err(e) => Err(format!("Failed to parse integration status: {}", e)),
        }
    }

    // Get Discourse topics
    pub async fn get_discourse_topics() -> Result<Vec<Topic>, String> {
        let result = invoke("get_discourse_topics", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(topics) => Ok(topics),
            Err(e) => Err(format!("Failed to parse topics: {}", e)),
        }
    }

    // Get Discourse categories
    pub async fn get_discourse_categories() -> Result<Vec<Category>, String> {
        let result = invoke("get_discourse_categories", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(categories) => Ok(categories),
            Err(e) => Err(format!("Failed to parse categories: {}", e)),
        }
    }

    // Get Discourse sync history
    pub async fn get_discourse_sync_history() -> Result<Vec<SyncHistoryEntry>, String> {
        let result = invoke("get_discourse_sync_history", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(history) => Ok(history),
            Err(e) => Err(format!("Failed to parse sync history: {}", e)),
        }
    }

    // Get sync conflicts
    pub async fn get_sync_conflicts() -> Result<Vec<SyncConflict>, String> {
        let result = invoke("get_sync_conflicts", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(conflicts) => Ok(conflicts),
            Err(e) => Err(format!("Failed to parse sync conflicts: {}", e)),
        }
    }

    // Resolve a sync conflict
    pub async fn resolve_sync_conflict(
        conflict_id: &str,
        strategy: ConflictResolutionStrategy
    ) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "conflict_id": conflict_id,
            "strategy": strategy,
        })).unwrap();

        let result = invoke("resolve_sync_conflict", args).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Sync all Discourse topics
    pub async fn sync_all_discourse_topics() -> Result<(), String> {
        let result = invoke("sync_all_discourse_topics", JsValue::NULL).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Sync a specific Discourse topic
    pub async fn sync_discourse_topic(topic_id: &str) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "topic_id": topic_id,
        })).unwrap();

        let result = invoke("sync_discourse_topic", args).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Setup Discourse integration
    pub async fn setup_discourse_integration() -> Result<(), String> {
        let result = invoke("setup_discourse_integration", JsValue::NULL).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Canvas Integration Methods

    // Get Canvas integration status
    pub async fn get_canvas_integration_status() -> Result<IntegrationStatus, String> {
        let result = invoke("get_canvas_integration_status", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(status) => Ok(status),
            Err(e) => Err(format!("Failed to parse integration status: {}", e)),
        }
    }

    // Get Canvas courses
    pub async fn get_canvas_courses() -> Result<Vec<CanvasCourse>, String> {
        let result = invoke("get_canvas_courses", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(courses) => Ok(courses),
            Err(e) => Err(format!("Failed to parse courses: {}", e)),
        }
    }

    // Get Canvas assignments
    pub async fn get_canvas_assignments() -> Result<Vec<CanvasAssignment>, String> {
        let result = invoke("get_canvas_assignments", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(assignments) => Ok(assignments),
            Err(e) => Err(format!("Failed to parse assignments: {}", e)),
        }
    }

    // Get Canvas sync history
    pub async fn get_canvas_sync_history() -> Result<Vec<SyncHistoryEntry>, String> {
        let result = invoke("get_canvas_sync_history", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(history) => Ok(history),
            Err(e) => Err(format!("Failed to parse sync history: {}", e)),
        }
    }

    // Sync all Canvas courses
    pub async fn sync_all_canvas_courses() -> Result<(), String> {
        let result = invoke("sync_all_canvas_courses", JsValue::NULL).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Sync a specific Canvas course
    pub async fn sync_canvas_course(course_id: &str) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "course_id": course_id,
        })).unwrap();

        let result = invoke("sync_canvas_course", args).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Setup Canvas integration
    pub async fn setup_canvas_integration() -> Result<(), String> {
        let result = invoke("setup_canvas_integration", JsValue::NULL).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Common Integration Methods

    // Sync all pending items
    pub async fn sync_all_pending() -> Result<(), String> {
        let result = invoke("sync_all_pending", JsValue::NULL).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Clear sync errors
    pub async fn clear_sync_errors() -> Result<(), String> {
        let result = invoke("clear_sync_errors", JsValue::NULL).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Get sync history stats
    pub async fn get_sync_history_stats() -> Result<serde_json::Value, String> {
        let result = invoke("get_sync_history_stats", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(format!("Failed to parse sync history stats: {}", e)),
        }
    }

    // Get sync history
    pub async fn get_sync_history() -> Result<Vec<SyncHistoryEntry>, String> {
        let result = invoke("get_sync_history", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(history) => Ok(history),
            Err(e) => Err(format!("Failed to parse sync history: {}", e)),
        }
    }

    // Get sync conflicts
    pub async fn get_sync_conflicts() -> Result<Vec<SyncConflict>, String> {
        let result = invoke("get_sync_conflicts", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(conflicts) => Ok(conflicts),
            Err(e) => Err(format!("Failed to parse sync conflicts: {}", e)),
        }
    }

    // Integration Settings Methods

    // Get integration settings
    pub async fn get_integration_settings() -> Result<serde_json::Value, String> {
        let result = invoke("get_integration_settings", JsValue::NULL).await;

        match serde_wasm_bindgen::from_value(result) {
            Ok(settings) => Ok(settings),
            Err(e) => Err(format!("Failed to parse integration settings: {}", e)),
        }
    }

    // Save Canvas settings
    pub async fn save_canvas_settings(api_url: &str, api_token: &str) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "api_url": api_url,
            "api_token": api_token,
        })).unwrap();

        let result = invoke("save_canvas_settings", args).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Save Discourse settings
    pub async fn save_discourse_settings(api_url: &str, api_key: &str, username: &str) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "api_url": api_url,
            "api_key": api_key,
            "username": username,
        })).unwrap();

        let result = invoke("save_discourse_settings", args).await;

        if result.is_null() || result.is_undefined() {
            Ok(())
        } else {
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(error) => Err(error),
                Err(e) => Err(format!("Failed to parse error: {}", e)),
            }
        }
    }

    // Save sync settings
    pub async fn save_sync_settings(sync_interval: i32) -> Result<(), String> {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "sync_interval": sync_interval,
        })).unwrap();

        let result = invoke("save_sync_settings", args).await;

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
