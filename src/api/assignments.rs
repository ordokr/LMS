use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteAssignmentRepository, AssignmentRepository, AssignmentRow};
use crate::api::auth_middleware::{AuthUser, require_role};

#[derive(Deserialize)]
pub struct CreateAssignmentRequest {
    pub course_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub points: u32,
}

#[derive(Serialize)]
pub struct AssignmentResponse {
    pub id: i64,
    pub course_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub points: u32,
}

async fn create_assignment(
    auth: AuthUser,
    State(pool): State<Arc<SqlitePool>>,
    Json(req): Json<CreateAssignmentRequest>,
) -> Result<Json<AssignmentResponse>, StatusCode> {
    require_role(&auth, "admin").map_err(|(code, _)| code)?;
    if req.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let repo = SqliteAssignmentRepository { pool: &pool };
    match repo.create_assignment(req.course_id, &req.title, req.description.as_deref(), req.points).await {
        Ok(id) => Ok(Json(AssignmentResponse {
            id,
            course_id: req.course_id,
            title: req.title,
            description: req.description,
            points: req.points,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_assignment(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<Json<AssignmentResponse>, StatusCode> {
    let repo = SqliteAssignmentRepository { pool: &pool };
    match repo.get_assignment(id).await {
        Ok(Some(a)) => Ok(Json(AssignmentResponse {
            id: a.id,
            course_id: a.course_id,
            title: a.title,
            description: a.description,
            points: a.points,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_assignments(
    State(pool): State<Arc<SqlitePool>>,
    Path(course_id): Path<i64>,
) -> Result<Json<Vec<AssignmentResponse>>, StatusCode> {
    let repo = SqliteAssignmentRepository { pool: &pool };
    match repo.list_assignments(course_id).await {
        Ok(assignments) => Ok(Json(assignments.into_iter().map(|a| AssignmentResponse {
            id: a.id,
            course_id: a.course_id,
            title: a.title,
            description: a.description,
            points: a.points,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn assignment_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", post(create_assignment))
        .route("/:id", get(get_assignment))
        .route("/course/:course_id", get(list_assignments))
        .with_state(pool)
}
