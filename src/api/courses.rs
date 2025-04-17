use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqliteCourseRepository, CourseRepository, CourseRow};
use sqlx::SqlitePool;
use crate::api::auth_middleware::{AuthUser, require_role};

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CourseResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

async fn create_course(
    auth: AuthUser,
    State(pool): State<Arc<SqlitePool>>,
    Json(req): Json<CreateCourseRequest>,
) -> Result<Json<CourseResponse>, StatusCode> {
    require_role(&auth, "admin").map_err(|(code, _)| code)?;
    if req.name.trim().is_empty() || req.code.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let repo = SqliteCourseRepository { pool: &pool };
    match repo.create_course(&req.name, &req.code, req.description.as_deref()).await {
        Ok(id) => Ok(Json(CourseResponse {
            id,
            name: req.name,
            code: req.code,
            description: req.description,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_course(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<Json<CourseResponse>, StatusCode> {
    let repo = SqliteCourseRepository { pool: &pool };
    match repo.get_course(id).await {
        Ok(Some(course)) => Ok(Json(CourseResponse {
            id: course.id,
            name: course.name,
            code: course.code,
            description: course.description,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_courses(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Vec<CourseResponse>>, StatusCode> {
    let repo = SqliteCourseRepository { pool: &pool };
    match repo.list_courses().await {
        Ok(courses) => Ok(Json(courses.into_iter().map(|c| CourseResponse {
            id: c.id,
            name: c.name,
            code: c.code,
            description: c.description,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn course_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", post(create_course).get(list_courses))
        .route("/:id", get(get_course))
        .with_state(pool)
}
