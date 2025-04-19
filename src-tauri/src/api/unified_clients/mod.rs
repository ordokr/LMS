// Unified API clients module
// This module contains consolidated API client implementations that replace redundant clients

mod base_client;
mod canvas_client;
mod discourse_client;
mod adapters;

#[cfg(test)]
mod tests;

// Re-export base client
pub use base_client::{
    ApiClient, ApiClientConfig, ApiError, Result,
    PaginationParams, PaginatedResponse,
    create_api_client, create_default_api_client,
};

// Re-export Canvas client
pub use canvas_client::{
    CanvasApiClient, CanvasNotification,
    create_canvas_client,
};

// Re-export Discourse client
pub use discourse_client::{
    DiscourseApiClient, DiscourseNotification, DiscourseCategory, DiscoursePost,
    create_discourse_client,
};

// Re-export adapters
pub use adapters::{
    CanvasClientAdapter, DiscourseClientAdapter,
    CanvasServiceClientAdapter, DiscourseServiceClientAdapter,
    create_canvas_client_adapter, create_discourse_client_adapter,
    create_canvas_service_client_adapter, create_discourse_service_client_adapter,
};

// Factory function to create API clients based on type
pub fn create_client(client_type: &str, base_url: &str, api_key: &str, api_username: Option<&str>) -> Result<std::sync::Arc<dyn ApiClient>> {
    match client_type.to_lowercase().as_str() {
        "canvas" => {
            let client = canvas_client::CanvasApiClient::new(base_url, api_key)?;
            Ok(std::sync::Arc::new(client))
        },
        "discourse" => {
            let username = api_username.unwrap_or("system");
            let client = discourse_client::DiscourseApiClient::new(base_url, api_key, username)?;
            Ok(std::sync::Arc::new(client))
        },
        _ => {
            // Default to base client
            let config = ApiClientConfig {
                base_url: base_url.to_string(),
                api_key: api_key.to_string(),
                api_username: api_username.map(|s| s.to_string()),
                ..Default::default()
            };
            create_api_client(config)
        }
    }
}
