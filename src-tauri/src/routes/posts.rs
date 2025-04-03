use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    models::Post,
    database::{PostRepository, TopicRepository},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreatePostPayload {
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostPayload {
    pub content: String,
}

// Get a single post
pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = PostRepository::new(&conn);
    
    match repo.find_by_id(id) {
        Ok(Some(post)) => (StatusCode::OK, Json(post)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Post not found"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve post"})),
            )
                .into_response()
        }
    }
}

// Create a new post
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePostPayload>,
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
    
    // Create post
    let post = Post::new(payload.topic_id, payload.user_id, payload.content);
    let post_repo = PostRepository::new(&tx);
    
    match post_repo.create(&post) {
        Ok(post_id) => {
            // Update the topic's last_posted_at time
            let topic_repo = TopicRepository::new(&tx);
            if let Err(e) = topic_repo.update_last_posted(payload.topic_id) {
                eprintln!("Failed to update topic last_posted_at: {}", e);
                let _ = tx.rollback();
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Failed to update topic"})),
                )
                    .into_response();
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
            
            // Return the created post ID
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "id": post_id,
                    "message": "Post created successfully"
                })),
            )
                .into_response()
        },
        Err(e) => {
            eprintln!("Failed to create post: {}", e);
            let _ = tx.rollback();
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create post"})),
            )
                .into_response()
        }
    }
}

// Update an existing post
pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = PostRepository::new(&conn);
    
    // First get the existing post
    match repo.find_by_id(id) {
        Ok(Some(mut post)) => {
            // Update the content
            post.edit(payload.content);
            
            // Save the updated post
            match repo.update(&post) {
                Ok(_) => (StatusCode::OK, Json(post)).into_response(),
                Err(e) => {
                    eprintln!("Database error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "Failed to update post"})),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Post not found"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve post for update"})),
            )
                .into_response()
        }
    }
}

// Delete a post (soft delete)
pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = PostRepository::new(&conn);
    
    match repo.delete(id) {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Post deleted successfully"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to delete post"})),
            )
                .into_response()
        }
    }
}