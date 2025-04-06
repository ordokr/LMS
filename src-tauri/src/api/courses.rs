use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::State,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::AppState;
use crate::database::repositories::course::CourseRepository;
use crate::shared::models::Course;

// API DATA STRUCTURES

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseRequest {
    name: String,
    code: String,
    description: Option<String>
}

// ROUTE HANDLERS

async fn get_courses(
    State(state): State<Arc<AppState>>,
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    let courses = repo.get_all().await?;
    Ok(axum::Json(courses))
}

async fn get_course(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    let course = repo.get_by_id(id).await?;
    Ok(axum::Json(course))
}

async fn create_course(
    State(state): State<Arc<AppState>>,
    axum::Json(course_req): axum::Json<CourseRequest>
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    let course = repo.create(course_req.name, course_req.code, course_req.description).await?;
    Ok(axum::Json(course))
}

async fn update_course(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    axum::Json(course_req): axum::Json<CourseRequest>
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    let course = repo.update(id, course_req.name, course_req.code, course_req.description).await?;
    Ok(axum::Json(course))
}

async fn delete_course(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    repo.delete(id).await?;
    Ok(axum::Json(()))
}

// Configure course API routes
pub fn course_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/courses", get(get_courses))
        .route("/courses/:id", get(get_course))
        .route("/courses", post(create_course))
        .route("/courses/:id", put(update_course))
        .route("/courses/:id", delete(delete_course))
}