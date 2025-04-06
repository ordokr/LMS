use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    AppState,
    services::discussion_topic_integration::DiscussionTopicIntegration,
    auth::jwt,
};

#[derive(Deserialize)]
pub struct CanvasDiscussionPayload {
    id: String,
    title: String,
    message: String,
    user_id: String,
    course_id: String,
    // Other Canvas discussion fields
}

#[derive(Deserialize)]
pub struct CanvasReplyPayload {
    id: String,
    discussion_id: String,
    message: String,
    user_id: String,
    parent_id: Option<String>,  // Parent reply ID if this is a nested reply
    // Other Canvas reply fields
}

#[derive(Serialize)]
pub struct ImportResponse {
    success: bool,
    topic_id: Option<String>,
    post_id: Option<String>,
    message: String,
}

// Import a new discussion from Canvas
pub async fn import_canvas_discussion(
    State(state): State<Arc<AppState>>,
    auth_header: axum::headers::Authorization<axum::headers::authorization::Bearer>,
    Json(payload): Json<CanvasDiscussionPayload>,
) -> Result<Json<ImportResponse>, StatusCode> {
    // Get user ID from JWT token and verify admin/instructor role
    let token = auth_header.token();
    let claims = match jwt::validate_token(token) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Only instructors can import discussions
    if claims.role != "instructor" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Find the course
    let course_uuid = match state.course_repo.find_course_by_canvas_id(&payload.course_id).await {
        Ok(Some(course)) => course.id,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // Find the importing user
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Create integration service
    let integration = DiscussionTopicIntegration::new(
        state.topic_repo.clone(),
        state.post_repo.clone(),
        state.category_repo.clone(),
        state.course_repo.clone(),
    );
    
    // Import the discussion
    match integration.import_canvas_discussion(
        &payload.id,
        &payload.title,
        &payload.message,
        &course_uuid,
        &user_id,
    ).await {
        Ok(topic) => {
            Ok(Json(ImportResponse {
                success: true,
                topic_id: Some(topic.id.to_string()),
                post_id: None,
                message: "Discussion successfully imported".to_string(),
            }))
        },
        Err(e) => {
            log::error!("Failed to import Canvas discussion: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Import a new reply from Canvas
pub async fn import_canvas_reply(
    State(state): State<Arc<AppState>>,
    auth_header: axum::headers::Authorization<axum::headers::authorization::Bearer>,
    Json(payload): Json<CanvasReplyPayload>,
) -> Result<Json<ImportResponse>, StatusCode> {
    // Get user ID from JWT token
    let token = auth_header.token();
    let claims = match jwt::validate_token(token) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Find the importing user
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Find the topic from the Canvas discussion ID
    let topic = match state.topic_repo.find_topic_by_canvas_discussion_id(&payload.discussion_id).await {
        Ok(Some(topic)) => topic,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // Create integration service
    let integration = DiscussionTopicIntegration::new(
        state.topic_repo.clone(),
        state.post_repo.clone(),
        state.category_repo.clone(),
        state.course_repo.clone(),
    );
    
    // Import the reply
    match integration.import_canvas_reply(
        &payload.id,
        &payload.message,
        &topic.id,
        &user_id,
        payload.parent_id.as_deref(),
    ).await {
        Ok(post) => {
            Ok(Json(ImportResponse {
                success: true,
                topic_id: Some(topic.id.to_string()),
                post_id: Some(post.id.to_string()),
                message: "Reply successfully imported".to_string(),
            }))
        },
        Err(e) => {
            log::error!("Failed to import Canvas reply: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Handle Canvas webhook events
pub async fn canvas_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    // Extract event type from payload
    let event_type = payload["metadata"]["event_type"].as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    log::info!("Received Canvas webhook event: {}", event_type);
    
    // Process different event types
    match event_type {
        "discussion_created" => {
            // Handle discussion creation event
            if let Some(discussion_data) = payload["data"].as_object() {
                // Extract discussion data and process
                log::info!("Processing discussion creation event");
                // Implementation would go here
            }
        },
        "discussion_updated" => {
            // Handle discussion update event
            if let Some(discussion_data) = payload["data"].as_object() {
                // Extract discussion data and process
                log::info!("Processing discussion update event");
                // Implementation would go here
            }
        },
        "discussion_reply_created" => {
            // Handle new reply event
            if let Some(reply_data) = payload["data"].as_object() {
                // Extract reply data and process
                log::info!("Processing reply creation event");
                // Implementation would go here
            }
        },
        // Add more event types as needed
        _ => {
            log::warn!("Unhandled Canvas event type: {}", event_type);
        }
    }
    
    // Return success response
    Ok(StatusCode::OK)
}