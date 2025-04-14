use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use crate::db::{Category, Db, Post, Topic};

pub fn forum_routes(db: Arc<Db>) -> Router {
    Router::new()
        .route("/categories", get(list_categories).post(create_category))
        .route("/topics", post(create_topic))
        .route("/posts", post(create_post))
        .with_state(db)
}

async fn list_categories(State(db): State<Arc<Db>>) -> Result<Json<Vec<Category>>, axum::http::StatusCode> {
    sqlx::query_as!(
        Category,
        "SELECT id, name, description, created_at FROM categories"
    )
    .fetch_all(&**db)
    .await
    .map(Json)
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_category(
    State(db): State<Arc<Db>>,
    Json(payload): Json<NewCategory>,
) -> Result<Json<Category>, axum::http::StatusCode> {
    sqlx::query_as!(
        Category,
        "INSERT INTO categories (name, description) VALUES (?, ?) RETURNING id, name, description, created_at",
        payload.name,
        payload.description
    )
    .fetch_one(&**db)
    .await
    .map(Json)
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_topic(
    State(db): State<Arc<Db>>,
    Json(payload): Json<NewTopic>,
) -> Result<Json<Topic>, axum::http::StatusCode> {
    sqlx::query_as!(
        Topic,
        "INSERT INTO topics (category_id, title, content, user_id) VALUES (?, ?, ?, ?) RETURNING id, category_id, title, content, user_id, created_at",
        payload.category_id,
        payload.title,
        payload.content,
        payload.user_id
    )
    .fetch_one(&**db)
    .await
    .map(Json)
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_post(
    State(db): State<Arc<Db>>,
    Json(payload): Json<NewPost>,
) -> Result<Json<Post>, axum::http::StatusCode> {
    sqlx::query_as!(
        Post,
        "INSERT INTO posts (topic_id, user_id, content) VALUES (?, ?, ?) RETURNING id, topic_id, user_id, content, created_at",
        payload.topic_id,
        payload.user_id,
        payload.content
    )
    .fetch_one(&**db)
    .await
    .map(Json)
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(serde::Deserialize)]
struct NewCategory {
    name: String,
    description: Option<String>,
}

#[derive(serde::Deserialize)]
struct NewTopic {
    category_id: i64,
    title: String,
    content: Option<String>,
    user_id: i64,
}

#[derive(serde::Deserialize)]
struct NewPost {
    topic_id: i64,
    user_id: i64,
    content: String,
}
