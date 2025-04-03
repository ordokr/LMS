use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::collections::HashMap;

use crate::db::{AppState, DbError};
use crate::models::forum::{Category, Topic, Post, Tag, ForumStats};
use crate::models::auth::User;
use crate::repositories::{
    ForumCategoryRepository, ForumTopicRepository, ForumPostRepository,
    UserRepository, ForumTagRepository
};
use crate::utils::auth::extract_user;

pub mod categories {
    pub use crate::forum::categories::*;
}

pub mod topics {
    pub use crate::forum::topics::*;
}

pub mod posts {
    // This will be implemented later
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

fn default_page() -> usize { 1 }
fn default_per_page() -> usize { 20 }

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub course_id: Option<i64>,
    pub color: Option<String>,
    pub text_color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub category_id: i64,
    pub content: String, // Initial post content
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct TopicWithPosts {
    #[serde(flatten)]
    pub topic: Topic,
    pub posts: Vec<Post>,
}

pub fn forum_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Category routes
        .route("/categories", get(get_categories))
        .route("/categories", post(create_category))
        .route("/categories/:id", get(get_category))
        .route("/categories/:id", put(update_category))
        .route("/categories/:id", delete(delete_category))
        .route("/categories/:id/topics", get(get_topics_by_category))
        .route("/courses/:id/categories", get(get_categories_by_course))
        
        // Topic routes
        .route("/topics", get(get_topics))
        .route("/topics", post(create_topic))
        .route("/topics/:id", get(get_topic))
        .route("/topics/:id", put(update_topic))
        .route("/topics/:id", delete(delete_topic))
        .route("/topics/:id/posts", get(get_posts_by_topic))
        .route("/topics/:id/posts", post(create_post))
        .route("/topics/recent", get(get_recent_topics))
        
        // Post routes
        .route("/posts/:id", get(get_post))
        .route("/posts/:id", put(update_post))
        .route("/posts/:id", delete(delete_post))
        .route("/posts/:id/like", post(like_post))
        
        // Tag routes
        .route("/tags", get(get_tags))
        .route("/tags/:name", get(get_topics_by_tag))
        
        // Stats route
        .route("/stats", get(get_forum_stats))
        
        // Search route
        .route("/search", get(search_forum))
        
        // Sync routes
        .route("/updates/categories", get(get_updated_categories))
        .route("/updates/topics", get(get_updated_topics))
        .route("/updates/posts", get(get_updated_posts))
}

// Category handlers
async fn get_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, AppError> {
    let repo = ForumCategoryRepository::new(state.pool.clone());
    let categories = repo.get_all().await?;
    Ok(Json(categories))
}

async fn create_category(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    // Ensure user is authenticated and has permission
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin privileges required".to_string()));
    }
    
    let repo = ForumCategoryRepository::new(state.pool.clone());
    
    // Generate slug if not provided
    let slug = match payload.slug {
        Some(slug) => slug,
        None => slugify(&payload.name),
    };
    
    let category = repo.create(
        &payload.name,
        &slug,
        payload.description.as_deref(),
        payload.parent_id,
        payload.course_id,
        payload.color.as_deref(),
        payload.text_color.as_deref(),
    ).await?;
    
    Ok(Json(category))
}

async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Category>, AppError> {
    let repo = ForumCategoryRepository::new(state.pool.clone());
    let category = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Category not found".to_string()))?;
    
    Ok(Json(category))
}

async fn update_category(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    // Ensure user is authenticated and has permission
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin privileges required".to_string()));
    }
    
    let repo = ForumCategoryRepository::new(state.pool.clone());
    
    // Check if category exists
    let _ = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Category not found".to_string()))?;
    
    // Generate slug if not provided
    let slug = match &payload.slug {
        Some(slug) => slug.clone(),
        None => slugify(&payload.name),
    };
    
    let updated = repo.update(
        id,
        &payload.name,
        &slug,
        payload.description.as_deref(),
        payload.parent_id,
        payload.course_id,
        payload.color.as_deref(),
        payload.text_color.as_deref(),
    ).await?;
    
    Ok(Json(updated))
}

async fn delete_category(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    // Ensure user is authenticated and has permission
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin privileges required".to_string()));
    }
    
    let repo = ForumCategoryRepository::new(state.pool.clone());
    
    // Check if category exists
    let _ = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Category not found".to_string()))?;
    
    repo.delete(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn get_categories_by_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<i64>,
) -> Result<Json<Vec<Category>>, AppError> {
    let repo = ForumCategoryRepository::new(state.pool.clone());
    let categories = repo.get_by_course_id(course_id).await?;
    
    Ok(Json(categories))
}

// Topic handlers
async fn get_topics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let repo = ForumTopicRepository::new(state.pool.clone());
    let topics = repo.get_all(params.page, params.per_page).await?;
    
    Ok(Json(topics))
}

async fn create_topic(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Json(payload): Json<CreateTopicRequest>,
) -> Result<Json<Topic>, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let topic_repo = ForumTopicRepository::new(state.pool.clone());
    let post_repo = ForumPostRepository::new(state.pool.clone());
    let tag_repo = ForumTagRepository::new(state.pool.clone());
    
    // Check if category exists
    let category_repo = ForumCategoryRepository::new(state.pool.clone());
    let _ = category_repo.get_by_id(payload.category_id).await?
        .ok_or(AppError::NotFound("Category not found".to_string()))?;
    
    // Create slug from title
    let slug = slugify(&payload.title);
    
    // Start a transaction
    let mut tx = state.pool.begin().await.map_err(DbError::from)?;
    
    // Create the topic
    let topic = topic_repo.create_with_tx(
        &mut tx,
        &payload.title,
        &slug,
        payload.category_id,
        user.id,
        false, // pinned
        false, // locked
    ).await?;
    
    // Create the initial post
    post_repo.create_with_tx(
        &mut tx,
        topic.id,
        user.id,
        &payload.content,
        None, // parent_id
    ).await?;
    
    // Add tags if provided
    if let Some(tags) = payload.tags {
        for tag_name in tags {
            tag_repo.add_tag_to_topic_with_tx(&mut tx, &tag_name, topic.id).await?;
        }
    }
    
    // Commit the transaction
    tx.commit().await.map_err(DbError::from)?;
    
    // Get the complete topic with all relations
    let topic = topic_repo.get_by_id(topic.id).await?
        .ok_or(AppError::NotFound("Topic not found after creation".to_string()))?;
    
    Ok(Json(topic))
}

async fn get_topic(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Topic>, AppError> {
    let repo = ForumTopicRepository::new(state.pool.clone());
    let topic = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Topic not found".to_string()))?;
    
    // Increment view count
    repo.increment_view_count(id).await?;
    
    Ok(Json(topic))
}

async fn update_topic(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateTopicRequest>,
) -> Result<Json<Topic>, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let repo = ForumTopicRepository::new(state.pool.clone());
    
    // Check if topic exists and user has permission
    let topic = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Topic not found".to_string()))?;
    
    if topic.user_id != user.id && !user.is_admin {
        return Err(AppError::Forbidden("You don't have permission to update this topic".to_string()));
    }
    
    // Generate slug if title changed
    let slug = slugify(&payload.title);
    
    let updated = repo.update(
        id,
        &payload.title,
        &slug,
        payload.category_id,
    ).await?;
    
    Ok(Json(updated))
}

async fn delete_topic(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let repo = ForumTopicRepository::new(state.pool.clone());
    
    // Check if topic exists and user has permission
    let topic = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Topic not found".to_string()))?;
    
    if topic.user_id != user.id && !user.is_admin {
        return Err(AppError::Forbidden("You don't have permission to delete this topic".to_string()));
    }
    
    repo.delete(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn get_topics_by_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let repo = ForumTopicRepository::new(state.pool.clone());
    let topics = repo.get_by_category_id(category_id, params.page, params.per_page).await?;
    
    Ok(Json(topics))
}

async fn get_recent_topics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let repo = ForumTopicRepository::new(state.pool.clone());
    let topics = repo.get_recent(params.page, params.per_page).await?;
    
    Ok(Json(topics))
}

// Post handlers
async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Post>, AppError> {
    let repo = ForumPostRepository::new(state.pool.clone());
    let post = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Post not found".to_string()))?;
    
    Ok(Json(post))
}

async fn get_posts_by_topic(
    State(state): State<Arc<AppState>>,
    Path(topic_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Post>>, AppError> {
    let repo = ForumPostRepository::new(state.pool.clone());
    let posts = repo.get_by_topic_id(topic_id, params.page, params.per_page).await?;
    
    Ok(Json(posts))
}

async fn create_post(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(topic_id): Path<i64>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<Post>, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let post_repo = ForumPostRepository::new(state.pool.clone());
    let topic_repo = ForumTopicRepository::new(state.pool.clone());
    
    // Check if topic exists
    let _ = topic_repo.get_by_id(topic_id).await?
        .ok_or(AppError::NotFound("Topic not found".to_string()))?;
    
    // Create the post
    let post = post_repo.create(
        topic_id,
        user.id,
        &payload.content,
        payload.parent_id,
    ).await?;
    
    // Update topic's last post information
    topic_repo.update_last_post(topic_id, post.id, user.id, Utc::now()).await?;
    
    Ok(Json(post))
}

async fn update_post(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Post>, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let repo = ForumPostRepository::new(state.pool.clone());
    
    // Check if post exists and user has permission
    let post = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Post not found".to_string()))?;
    
    if post.user_id != user.id && !user.is_admin {
        return Err(AppError::Forbidden("You don't have permission to update this post".to_string()));
    }
    
    let updated = repo.update(id, &payload.content).await?;
    
    Ok(Json(updated))
}

async fn delete_post(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let repo = ForumPostRepository::new(state.pool.clone());
    
    // Check if post exists and user has permission
    let post = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Post not found".to_string()))?;
    
    if post.user_id != user.id && !user.is_admin {
        return Err(AppError::Forbidden("You don't have permission to delete this post".to_string()));
    }
    
    repo.delete(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn like_post(
    State(state): State<Arc<AppState>>,
    user: Option<User>,
    Path(id): Path<i64>,
) -> Result<Json<Post>, AppError> {
    // Ensure user is authenticated
    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    let repo = ForumPostRepository::new(state.pool.clone());
    
    // Check if post exists
    let post = repo.get_by_id(id).await?
        .ok_or(AppError::NotFound("Post not found".to_string()))?;
    
    // Toggle like status
    let updated = repo.toggle_like(id, user.id).await?;
    
    Ok(Json(updated))
}

// Tag handlers
async fn get_tags(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Tag>>, AppError> {
    let repo = ForumTagRepository::new(state.pool.clone());
    let tags = repo.get_all().await?;
    
    Ok(Json(tags))
}

async fn get_topics_by_tag(
    State(state): State<Arc<AppState>>,
    Path(tag_name): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let repo = ForumTopicRepository::new(state.pool.clone());
    let topics = repo.get_by_tag(&tag_name, params.page, params.per_page).await?;
    
    Ok(Json(topics))
}

// Stats handler
async fn get_forum_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ForumStats>, AppError> {
    let topic_repo = ForumTopicRepository::new(state.pool.clone());
    let post_repo = ForumPostRepository::new(state.pool.clone());
    let user_repo = UserRepository::new(state.pool.clone());
    
    let total_topics = topic_repo.count().await?;
    let total_posts = post_repo.count().await?;
    let total_users = user_repo.count().await?;
    
    let posts_today = post_repo.count_today().await?;
    let active_users_today = user_repo.count_active_today().await?;
    
    let stats = ForumStats {
        total_posts,
        total_topics,
        total_users,
        posts_today,
        active_users_today,
    };
    
    Ok(Json(stats))
}

// Search handler
async fn search_forum(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let query = params.get("q").cloned().unwrap_or_default();
    if query.trim().is_empty() {
        return Ok(Json(vec![]));
    }
    
    let page = params.get("page")
        .and_then(|p| p.parse::<usize>().ok())
        .unwrap_or(1);
    let per_page = params.get("per_page")
        .and_then(|p| p.parse::<usize>().ok())
        .unwrap_or(20);
    
    let repo = ForumTopicRepository::new(state.pool.clone());
    let results = repo.search(&query, page, per_page).await?;
    
    Ok(Json(results))
}

// Sync handlers
async fn get_updated_categories(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Category>>, AppError> {
    let since = params.get("since")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    
    let repo = ForumCategoryRepository::new(state.pool.clone());
    let categories = repo.get_updated_since(since).await?;
    
    Ok(Json(categories))
}

async fn get_updated_topics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let since = params.get("since")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    
    let repo = ForumTopicRepository::new(state.pool.clone());
    let topics = repo.get_updated_since(since).await?;
    
    Ok(Json(topics))
}

async fn get_updated_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Post>>, AppError> {
    let since = params.get("since")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    
    let repo = ForumPostRepository::new(state.pool.clone());
    let posts = repo.get_updated_since(since).await?;
    
    Ok(Json(posts))
}

// Helper function to create URL-friendly slugs
fn slugify(text: &str) -> String {
    // Convert to lowercase
    let text = text.to_lowercase();
    
    // Replace non-alphanumeric characters with hyphens
    let slug: String = text
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect();
    
    // Replace multiple consecutive hyphens with a single one
    let slug = slug.replace("--", "-");
    
    // Remove leading and trailing hyphens
    let slug = slug.trim_matches('-').to_string();
    
    slug
}

// Error handling
#[derive(Debug)]
pub enum AppError {
    Database(DbError),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };
        
        let body = Json(serde_json::json!({
            "error": message
        }));
        
        (status, body).into_response()
    }
}

impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        AppError::Database(err)
    }
}

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::AppState;

mod forum_categories;
mod forum_topics;
mod forum_posts;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Category routes
        .route("/categories", get(forum_categories::get_categories))
        .route("/categories", post(forum_categories::create_category))
        .route("/categories/:id", get(forum_categories::get_category))
        .route("/categories/:id", put(forum_categories::update_category))
        .route("/categories/:id", delete(forum_categories::delete_category))
        
        // Topic routes
        .route("/topics", get(forum_topics::get_topics))
        .route("/topics", post(forum_topics::create_topic))
        .route("/topics/:id", get(forum_topics::get_topic))
        .route("/topics/:id", put(forum_topics::update_topic))
        .route("/topics/:id", delete(forum_topics::delete_topic))
        .route("/topics/recent", get(forum_topics::get_recent_topics))
        .route("/categories/:id/topics", get(forum_topics::get_topics_by_category))
        
        // Post routes
        .route("/topics/:id/posts", get(forum_posts::get_posts_for_topic))
        .route("/topics/:id/posts", post(forum_posts::create_post))
        .route("/posts/:id", get(forum_posts::get_post))
        .route("/posts/:id", put(forum_posts::update_post))
        .route("/posts/:id", delete(forum_posts::delete_post))
        .route("/posts/:id/solution", post(forum_posts::mark_as_solution))
        .route("/posts/:id/like", post(forum_posts::like_post))
        .route("/posts/:id/unlike", post(forum_posts::unlike_post))
        .route("/posts/recent", get(forum_posts::get_recent_posts))
}