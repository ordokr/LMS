use tauri::{command, State};
use crate::models::integration::{
    IntegrationStatus, DiscourseTopic, DiscourseCategory, SyncHistoryEntry
};
use crate::services::integration_service::{IntegrationService, SyncAllResult};
use std::sync::Arc;

/// Get the current status of the Discourse integration
#[command]
pub async fn get_discourse_integration_status() -> Result<IntegrationStatus, String> {
    IntegrationService::get_discourse_integration_status().await
}

/// Get all topics that have been synchronized with Discourse
#[command]
pub async fn get_discourse_topics() -> Result<Vec<DiscourseTopic>, String> {
    IntegrationService::get_discourse_topics().await
}

/// Get all categories that have been synchronized with Discourse
#[command]
pub async fn get_discourse_categories() -> Result<Vec<DiscourseCategory>, String> {
    IntegrationService::get_discourse_categories().await
}

/// Get the synchronization history
#[command]
pub async fn get_discourse_sync_history() -> Result<Vec<SyncHistoryEntry>, String> {
    IntegrationService::get_discourse_sync_history().await
}

/// Synchronize all topics with Discourse
#[command]
pub async fn sync_all_discourse_topics() -> Result<SyncAllResult, String> {
    IntegrationService::sync_all_discourse_topics().await
}

/// Synchronize a specific topic with Discourse
#[command]
pub async fn sync_discourse_topic(topic_id: String) -> Result<(), String> {
    IntegrationService::sync_discourse_topic(&topic_id).await
}

/// Set up the Discourse integration
#[command]
pub async fn setup_discourse_integration() -> Result<(), String> {
    IntegrationService::setup_discourse_integration().await
}

/// Helper to open URLs in the default browser
#[command]
pub async fn open_url(url: &str) -> Result<(), String> {
    open::that(url).map_err(|e| format!("Failed to open URL: {}", e))
}
