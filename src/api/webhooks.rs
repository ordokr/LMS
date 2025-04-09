use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CanvasWebhookPayload {
    pub event_name: String,
    pub event_data: CanvasEventData,
}

#[derive(Debug, Deserialize)]
pub struct CanvasEventData {
    pub discussion_topic_id: Option<String>,
    pub discussion_entry_id: Option<String>,
    pub updated_at: Option<String>,
    // Additional fields as needed
}

#[derive(Debug, Deserialize)]
pub struct DiscourseWebhookPayload {
    pub event_name: String,
    pub topic_id: Option<i64>,
    pub post_id: Option<i64>,
    pub updated_at: Option<String>,
    // Additional fields as needed
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message: String,
}

async fn canvas_webhook(
    State(state): State<AppState>,
    Json(payload): Json<CanvasWebhookPayload>,
) -> impl IntoResponse {
    let sync_service = &state.sync_service;
    
    // Handle discussion topic updates
    if payload.event_name == "discussion_topic_updated" {
        if let Some(topic_id) = payload.event_data.discussion_topic_id {
            // Parse updated_at timestamp
            if let Some(updated_at_str) = payload.event_data.updated_at {
                if let Ok(updated_at) = chrono::DateTime::parse_from_rfc3339(&updated_at_str) {
                    let updated_at_utc = updated_at.with_timezone(&Utc);
                    
                    // Record update in our mapping system
                    if let Ok(_) = sync_service.record_canvas_topic_update(&topic_id, updated_at_utc).await {
                        return (StatusCode::OK, Json(WebhookResponse {
                            success: true,
                            message: format!("Successfully recorded update for Canvas topic {}", topic_id),
                        }));
                    }
                }
            }
        }
    }
    
    // Default response for unhandled events
    (StatusCode::OK, Json(WebhookResponse {
        success: true,
        message: "Webhook received, but no action taken".to_string(),
    }))
}

async fn discourse_webhook(
    State(state): State<AppState>,
    Json(payload): Json<DiscourseWebhookPayload>,
) -> impl IntoResponse {
    let sync_service = &state.sync_service;
    
    // Handle topic updates
    if payload.event_name == "topic_updated" {
        if let Some(topic_id) = payload.topic_id {
            // Parse updated_at timestamp
            if let Some(updated_at_str) = payload.updated_at {
                if let Ok(updated_at) = chrono::DateTime::parse_from_rfc3339(&updated_at_str) {
                    let updated_at_utc = updated_at.with_timezone(&Utc);
                    
                    // Record update in our mapping system
                    if let Ok(_) = sync_service.record_discourse_topic_update(&topic_id.to_string(), updated_at_utc).await {
                        return (StatusCode::OK, Json(WebhookResponse {
                            success: true,
                            message: format!("Successfully recorded update for Discourse topic {}", topic_id),
                        }));
                    }
                }
            }
        }
    }
    
    // Default response for unhandled events
    (StatusCode::OK, Json(WebhookResponse {
        success: true,
        message: "Webhook received, but no action taken".to_string(),
    }))
}

pub fn webhook_routes() -> Router<AppState> {
    Router::new()
        .route("/api/webhooks/canvas", post(canvas_webhook))
        .route("/api/webhooks/discourse", post(discourse_webhook))
}