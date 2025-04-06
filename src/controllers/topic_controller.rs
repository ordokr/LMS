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
    models::{topic::Topic, post::Post},
    auth::middleware,
};

#[derive(Deserialize)]
pub struct CreateTopicRequest {
    title: String,
    category_id: String,
    initial_post_content: String,
    pinned: Option<bool>,
}

#[derive(Serialize)]
pub struct TopicResponse {
    topic: Topic,
    initial_post: Post,
}

pub async fn create_topic(
    State(state): State<Arc<AppState>>,
    user_id: String, // From auth middleware
    Json(payload): Json<CreateTopicRequest>,
) -> Result<Json<TopicResponse>, StatusCode> {
    let author_id = Uuid::parse_str(&user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let category_id = Uuid::parse_str(&payload.category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Create topic
    let topic = Topic::new(
        payload.title,
        category_id,
        author_id,
        payload.pinned.unwrap_or(false),
        false, // Not closed initially
    );
    
    // Create initial post
    let post = Post::new(
        topic.id,
        author_id,
        payload.initial_post_content,
    );
    
    // Save topic
    let saved_topic = state.topic_repo.create_topic(&topic)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Save post
    let saved_post = state.post_repo.create_post(&post)
        .await
        .map_err(|_| {
            // Cleanup: If post creation fails, attempt to delete the topic
            // This is not ideal for production - should use transactions
            let topic_id = saved_topic.id;
            tokio::spawn(async move {
                let _ = sqlx::query!("DELETE FROM topics WHERE id = $1", topic_id)
                    .execute(&state.pool)
                    .await;
            });
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(TopicResponse {
        topic: saved_topic,
        initial_post: saved_post,
    }))
}

pub async fn get_topic(
    State(state): State<Arc<AppState>>,
    Path(topic_id): Path<String>,
) -> Result<Json<Topic>, StatusCode> {
    let id = Uuid::parse_str(&topic_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Increment view count
    state.topic_repo.increment_view_count(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Get topic
    let topic = state.topic_repo.find_topic_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(topic))
}

#[derive(Serialize)]
pub struct TopicWithPostsResponse {
    topic: Topic,
    posts: Vec<Post>,
}

pub async fn get_topic_with_posts(
    State(state): State<Arc<AppState>>,
    Path(topic_id): Path<String>,
) -> Result<Json<TopicWithPostsResponse>, StatusCode> {
    let id = Uuid::parse_str(&topic_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Increment view count
    state.topic_repo.increment_view_count(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Get topic
    let topic = state.topic_repo.find_topic_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Get all posts for topic
    let posts = state.post_repo.find_posts_by_topic(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(TopicWithPostsResponse { topic, posts }))
}

#[derive(Deserialize)]
pub struct UpdateTopicRequest {
    title: Option<String>,
    pinned: Option<bool>,
    closed: Option<bool>,
}

pub async fn update_topic(
    State(state): State<Arc<AppState>>,
    Path(topic_id): Path<String>,
    Json(payload): Json<UpdateTopicRequest>,
) -> Result<Json<Topic>, StatusCode> {
    let id = Uuid::parse_str(&topic_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Get existing topic
    let mut topic = state.topic_repo.find_topic_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Update fields if provided
    if let Some(title) = payload.title {
        topic.title = title;
        topic.slug = crate::models::topic::generate_slug(&topic.title);
    }
    
    if let Some(pinned) = payload.pinned {
        topic.pinned = pinned;
    }
    
    if let Some(closed) = payload.closed {
        topic.closed = closed;
    }
    
    // Update timestamp
    topic.updated_at = chrono::Utc::now();
    
    // Save updated topic
    state.topic_repo.update_topic(&topic)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(topic))
}

pub async fn get_topics_by_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<String>,
) -> Result<Json<Vec<Topic>>, StatusCode> {
    let id = Uuid::parse_str(&category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Get topics
    let topics = state.topic_repo.find_topics_by_category(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(topics))
}