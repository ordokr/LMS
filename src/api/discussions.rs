use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteDiscussionRepository, DiscussionRepository, DiscussionRow};
use crate::api::auth_middleware::{AuthUser, require_role};

#[derive(Deserialize)]
pub struct CreateDiscussionRequest {
    pub course_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct DiscussionResponse {
    pub id: i64,
    pub course_id: i64,
    pub title: String,
    pub content: String,
}

async fn create_discussion(
    auth: AuthUser,
    State(pool): State<Arc<SqlitePool>>,
    Json(req): Json<CreateDiscussionRequest>,
) -> Result<Json<DiscussionResponse>, StatusCode> {
    require_role(&auth, "user").map_err(|(code, _)| code)?;
    if req.title.trim().is_empty() || req.content.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let repo = SqliteDiscussionRepository { pool: &pool };
    match repo.create_discussion(req.course_id, &req.title, &req.content).await {
        Ok(id) => Ok(Json(DiscussionResponse {
            id,
            course_id: req.course_id,
            title: req.title,
            content: req.content,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_discussion(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<Json<DiscussionResponse>, StatusCode> {
    let repo = SqliteDiscussionRepository { pool: &pool };
    match repo.get_discussion(id).await {
        Ok(Some(d)) => Ok(Json(DiscussionResponse {
            id: d.id,
            course_id: d.course_id,
            title: d.title,
            content: d.content,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_discussions(
    State(pool): State<Arc<SqlitePool>>,
    Path(course_id): Path<i64>,
) -> Result<Json<Vec<DiscussionResponse>>, StatusCode> {
    let repo = SqliteDiscussionRepository { pool: &pool };
    match repo.list_discussions(course_id).await {
        Ok(discussions) => Ok(Json(discussions.into_iter().map(|d| DiscussionResponse {
            id: d.id,
            course_id: d.course_id,
            title: d.title,
            content: d.content,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn discussion_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", post(create_discussion))
        .route("/:id", get(get_discussion))
        .route("/course/:course_id", get(list_discussions))
        .with_state(pool)
}
