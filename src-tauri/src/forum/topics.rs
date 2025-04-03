use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::core::errors::AppError;
use crate::database::repositories::forum::ForumTopicRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub category_id: i64,
    pub title: String,
    pub content: String,
}

pub async fn create_topic(
    State(repo): State<Arc<ForumTopicRepository>>,
    Json(payload): Json<CreateTopicRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract user_id from auth context in a real implementation
    let user_id = 1; // Placeholder

    tracing::info!("Creating forum topic: {}", payload.title);
    
    // Create the topic
    let topic_id = repo.create_topic(
        payload.category_id,
        &payload.title,
        user_id,
    ).await?;
    
    // Create the initial post
    let post_id = repo.create_post(
        topic_id,
        user_id,
        &payload.content,
    ).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "topic_id": topic_id,
        "post_id": post_id,
        "message": "Topic created successfully"
    }))))
}

pub async fn get_topics(
    State(repo): State<Arc<ForumTopicRepository>>,
    Path(category_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let topics = repo.get_topics_by_category(category_id).await?;
    Ok((StatusCode::OK, Json(topics)))
}

pub async fn get_topic(
    State(repo): State<Arc<ForumTopicRepository>>,
    Path(topic_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let topic = repo.get_topic_by_id(topic_id).await?;
    let posts = repo.get_posts_by_topic_id(topic_id).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "topic": topic,
        "posts": posts
    }))))
}