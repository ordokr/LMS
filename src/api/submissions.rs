use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteSubmissionRepository, SubmissionRepository, SubmissionRow};
use crate::api::auth_middleware::{AuthUser, require_role};

#[derive(Deserialize)]
pub struct CreateSubmissionRequest {
    pub assignment_id: i64,
    pub user_id: i64,
    pub content: String,
}

#[derive(Serialize)]
pub struct SubmissionResponse {
    pub id: i64,
    pub assignment_id: i64,
    pub user_id: i64,
    pub content: String,
}

async fn create_submission(
    auth: AuthUser,
    State(pool): State<Arc<SqlitePool>>,
    Json(req): Json<CreateSubmissionRequest>,
) -> Result<Json<SubmissionResponse>, StatusCode> {
    require_role(&auth, "user").map_err(|(code, _)| code)?;
    if req.content.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let repo = SqliteSubmissionRepository { pool: &pool };
    match repo.create_submission(req.assignment_id, req.user_id, &req.content).await {
        Ok(id) => Ok(Json(SubmissionResponse {
            id,
            assignment_id: req.assignment_id,
            user_id: req.user_id,
            content: req.content,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_submission(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<Json<SubmissionResponse>, StatusCode> {
    let repo = SqliteSubmissionRepository { pool: &pool };
    match repo.get_submission(id).await {
        Ok(Some(s)) => Ok(Json(SubmissionResponse {
            id: s.id,
            assignment_id: s.assignment_id,
            user_id: s.user_id,
            content: s.content,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_submissions(
    State(pool): State<Arc<SqlitePool>>,
    Path(assignment_id): Path<i64>,
) -> Result<Json<Vec<SubmissionResponse>>, StatusCode> {
    let repo = SqliteSubmissionRepository { pool: &pool };
    match repo.list_submissions(assignment_id).await {
        Ok(submissions) => Ok(Json(submissions.into_iter().map(|s| SubmissionResponse {
            id: s.id,
            assignment_id: s.assignment_id,
            user_id: s.user_id,
            content: s.content,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn submission_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", post(create_submission))
        .route("/:id", get(get_submission))
        .route("/assignment/:assignment_id", get(list_submissions))
        .with_state(pool)
}
