use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::{ApiConfigService, ApiConfig, CanvasConfig, DiscourseConfig};
use crate::AppState;

/// API configuration response
#[derive(Debug, Serialize)]
pub struct ApiConfigResponse {
    pub canvas: CanvasConfigResponse,
    pub discourse: DiscourseConfigResponse,
}

/// Canvas configuration response
#[derive(Debug, Serialize)]
pub struct CanvasConfigResponse {
    pub base_url: String,
    pub api_token: String,
    pub timeout_seconds: Option<u64>,
}

/// Discourse configuration response
#[derive(Debug, Serialize)]
pub struct DiscourseConfigResponse {
    pub base_url: String,
    pub api_key: String,
    pub api_username: String,
    pub timeout_seconds: Option<u64>,
}

/// API configuration request
#[derive(Debug, Deserialize)]
pub struct ApiConfigRequest {
    pub canvas: CanvasConfigRequest,
    pub discourse: DiscourseConfigRequest,
}

/// Canvas configuration request
#[derive(Debug, Deserialize)]
pub struct CanvasConfigRequest {
    pub base_url: String,
    pub api_token: String,
    pub timeout_seconds: Option<u64>,
}

/// Discourse configuration request
#[derive(Debug, Deserialize)]
pub struct DiscourseConfigRequest {
    pub base_url: String,
    pub api_key: String,
    pub api_username: String,
    pub timeout_seconds: Option<u64>,
}

/// Connection test response
#[derive(Debug, Serialize)]
pub struct ConnectionTestResponse {
    pub success: bool,
    pub message: String,
}

/// Get the current API configuration
async fn get_api_config(
    State(state): State<Arc<AppState>>,
) -> Json<ApiConfigResponse> {
    let api_config = state.api_config.lock().await;
    
    let canvas_config = api_config.get_canvas_config();
    let discourse_config = api_config.get_discourse_config();
    
    Json(ApiConfigResponse {
        canvas: CanvasConfigResponse {
            base_url: canvas_config.base_url.clone(),
            api_token: canvas_config.api_token.clone(),
            timeout_seconds: canvas_config.timeout_seconds,
        },
        discourse: DiscourseConfigResponse {
            base_url: discourse_config.base_url.clone(),
            api_key: discourse_config.api_key.clone(),
            api_username: discourse_config.api_username.clone(),
            timeout_seconds: discourse_config.timeout_seconds,
        },
    })
}

/// Update the API configuration
async fn update_api_config(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ApiConfigRequest>,
) -> Json<ApiConfigResponse> {
    let mut api_config = state.api_config.lock().await;
    
    // Update Canvas configuration
    api_config.update_canvas_config(CanvasConfig {
        base_url: request.canvas.base_url,
        api_token: request.canvas.api_token,
        timeout_seconds: request.canvas.timeout_seconds,
    }).unwrap();
    
    // Update Discourse configuration
    api_config.update_discourse_config(DiscourseConfig {
        base_url: request.discourse.base_url,
        api_key: request.discourse.api_key,
        api_username: request.discourse.api_username,
        timeout_seconds: request.discourse.timeout_seconds,
    }).unwrap();
    
    // Get the updated configuration
    let canvas_config = api_config.get_canvas_config();
    let discourse_config = api_config.get_discourse_config();
    
    Json(ApiConfigResponse {
        canvas: CanvasConfigResponse {
            base_url: canvas_config.base_url.clone(),
            api_token: canvas_config.api_token.clone(),
            timeout_seconds: canvas_config.timeout_seconds,
        },
        discourse: DiscourseConfigResponse {
            base_url: discourse_config.base_url.clone(),
            api_key: discourse_config.api_key.clone(),
            api_username: discourse_config.api_username.clone(),
            timeout_seconds: discourse_config.timeout_seconds,
        },
    })
}

/// Test the Canvas API connection
async fn test_canvas_connection(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CanvasConfigRequest>,
) -> Json<ConnectionTestResponse> {
    let mut api_config = state.api_config.lock().await;
    
    // Temporarily update the Canvas configuration
    api_config.update_canvas_config(CanvasConfig {
        base_url: request.base_url,
        api_token: request.api_token,
        timeout_seconds: request.timeout_seconds,
    }).unwrap();
    
    // Test the connection
    match api_config.test_canvas_connection().await {
        Ok(true) => Json(ConnectionTestResponse {
            success: true,
            message: "Connection successful".to_string(),
        }),
        Ok(false) => Json(ConnectionTestResponse {
            success: false,
            message: "Connection failed".to_string(),
        }),
        Err(e) => Json(ConnectionTestResponse {
            success: false,
            message: format!("Connection error: {}", e),
        }),
    }
}

/// Test the Discourse API connection
async fn test_discourse_connection(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DiscourseConfigRequest>,
) -> Json<ConnectionTestResponse> {
    let mut api_config = state.api_config.lock().await;
    
    // Temporarily update the Discourse configuration
    api_config.update_discourse_config(DiscourseConfig {
        base_url: request.base_url,
        api_key: request.api_key,
        api_username: request.api_username,
        timeout_seconds: request.timeout_seconds,
    }).unwrap();
    
    // Test the connection
    match api_config.test_discourse_connection().await {
        Ok(true) => Json(ConnectionTestResponse {
            success: true,
            message: "Connection successful".to_string(),
        }),
        Ok(false) => Json(ConnectionTestResponse {
            success: false,
            message: "Connection failed".to_string(),
        }),
        Err(e) => Json(ConnectionTestResponse {
            success: false,
            message: format!("Connection error: {}", e),
        }),
    }
}

/// Create the configuration API routes
pub fn config_api_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/config", get(get_api_config))
        .route("/config", put(update_api_config))
        .route("/config/test-canvas", post(test_canvas_connection))
        .route("/config/test-discourse", post(test_discourse_connection))
        .with_state(state)
}
