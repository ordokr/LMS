use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    models::{Topic, Post},
    database::{TopicRepository, PostRepository},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateTopicPayload {
    pub title: String,
    pub category_id: i64,
    pub content: String, // First post content
    pub user_id: i64,    // Author ID
}

#[derive(Debug, Deserialize)]
pub struct UpdateTopicPayload {
    pub title: Option<String>,
    pub is_closed: Option<bool>,
    pub is_pinned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TopicQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

// List topics in a category
pub async fn list_topics(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i64>,
    Query(params): Query<TopicQuery>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = TopicRepository::new(&conn);
    
    let page = params.page.max(1);
    let per_page = params.per_page.min(100).max(5);
    let offset = (page - 1) * per_page;
    
    match repo.find_by_category_id(category_id, per_page, offset) {
        Ok(topics) => (StatusCode::OK, Json(topics)).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve topics"})),
            )
                .into_response()
        }
    }
}

// Get a single topic
pub async fn get_topic(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = TopicRepository::new(&conn);
    
    // Increment view count
    if let Err(e) = repo.increment_view_count(id) {
        eprintln!("Failed to increment view count: {}", e);
    }
    
    match repo.find_by_id(id) {
        Ok(Some(topic)) => (StatusCode::OK, Json(topic)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Topic not found"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve topic"})),
            )
                .into_response()
        }
    }
}

// Create a new topic with initial post
pub async fn create_topic(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTopicPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    
    // Start a transaction
    let tx = match conn.transaction() {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Transaction error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Database transaction error"})),
            )
                .into_response();
        }
    };
    
    // Create slug from title
    let slug = payload.title
        .to_lowercase()
        .replace(" ", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    
    // Create topic
    let topic = Topic::new(payload.title, slug, payload.category_id, payload.user_id);
    let topic_repo = TopicRepository::new(&tx);
    
    let topic_id = match topic_repo.create(&topic) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to create topic: {}", e);
            let _ = tx.rollback();
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create topic"})),
            )
                .into_response();
        }
    };
    
    // Create initial post
    let post = Post::new(topic_id, payload.user_id, payload.content);
    let post_repo = PostRepository::new(&tx);
    
    match post_repo.create(&post) {
        Ok(_) => {
            // Update last posted time
            if let Err(e) = topic_repo.update_last_posted(topic_id) {
                eprintln!("Failed to update last posted time: {}", e);
            }
            
            // Commit the transaction
            if let Err(e) = tx.commit() {
                eprintln!("Failed to commit transaction: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Failed to commit transaction"})),
                )
                    .into_response();
            }
            
            // Return the created topic
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "id": topic_id,
                    "message": "Topic created successfully"
                })),
            )
                .into_response()
        },
        Err(e) => {
            eprintln!("Failed to create post: {}", e);
            let _ = tx.rollback();
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create initial post"})),
            )
                .into_response()
        }
    }
}

// Update a topic
pub async fn update_topic(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTopicPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = TopicRepository::new(&conn);
    
    // First get the existing topic
    match repo.find_by_id(id) {
        Ok(Some(mut topic)) => {
            // Update fields if provided
            if let Some(title) = payload.title {
                topic.title = title;
                
                // Update slug too if title changed
                topic.slug = topic.title
                    .to_lowercase()
                    .replace(" ", "-")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '-')
                    .collect::<String>();
            }
            
            if let Some(is_closed) = payload.is_closed {
                topic.is_closed = is_closed;
            }
            
            if let Some(is_pinned) = payload.is_pinned {
                topic.is_pinned = is_pinned;
            }
            
            // Save the updated topic
            match repo.update(&topic) {
                Ok(_) => (StatusCode::OK, Json(topic)).into_response(),
                Err(e) => {
                    eprintln!("Database error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "Failed to update topic"})),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Topic not found"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve topic for update"})),
            )
                .into_response()
        }
    }
}

// Get posts for a topic
pub async fn list_topic_posts(
    State(state): State<Arc<AppState>>,