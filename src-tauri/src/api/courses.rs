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
use crate::models::course::{Course, CourseStatus};
use crate::utils::database::DatabaseConnection;
use crate::utils::error::ApiResult;
use tauri::State;

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

#[tauri::command]
pub async fn get_courses(db: State<'_, DatabaseConnection>) -> ApiResult<Vec<Course>> {
    let conn = db.get_connection().await?;
    
    // Query to fetch all courses from the database
    let courses = sqlx::query_as!(
        Course,
        "SELECT * FROM courses ORDER BY created_at DESC"
    )
    .fetch_all(&conn)
    .await?;
    
    Ok(courses)
}

#[tauri::command]
pub async fn get_course(course_id: i64, db: State<'_, DatabaseConnection>) -> ApiResult<Course> {
    let conn = db.get_connection().await?;
    
    // Query to fetch a specific course by ID
    let course = sqlx::query_as!(
        Course,
        "SELECT * FROM courses WHERE id = $1",
        course_id
    )
    .fetch_optional(&conn)
    .await?
    .ok_or_else(|| anyhow::anyhow!("Course not found"))?;
    
    Ok(course)
}

#[derive(Serialize, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: CourseStatus,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

#[tauri::command]
pub async fn create_course(
    request: CreateCourseRequest, 
    db: State<'_, DatabaseConnection>
) -> ApiResult<Course> {
    let conn = db.get_connection().await?;
    
    // Insert new course into database
    let course = sqlx::query_as!(
        Course,
        r#"
        INSERT INTO courses (title, description, status, start_date, end_date, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        request.title,
        request.description,
        request.status as _,
        request.start_date,
        request.end_date
    )
    .fetch_one(&conn)
    .await?;
    
    Ok(course)
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCourseRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<CourseStatus>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

#[tauri::command]
pub async fn update_course(
    course_id: i64,
    request: UpdateCourseRequest,
    db: State<'_, DatabaseConnection>
) -> ApiResult<Course> {
    let conn = db.get_connection().await?;
    
    // First, fetch the existing course
    let existing = sqlx::query_as!(
        Course,
        "SELECT * FROM courses WHERE id = $1",
        course_id
    )
    .fetch_optional(&conn)
    .await?
    .ok_or_else(|| anyhow::anyhow!("Course not found"))?;
    
    // Update the course with new values or keep existing ones
    let updated = sqlx::query_as!(
        Course,
        r#"
        UPDATE courses 
        SET 
            title = $1,
            description = $2,
            status = $3,
            start_date = $4,
            end_date = $5,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $6
        RETURNING *
        "#,
        request.title.unwrap_or(existing.title),
        request.description.or(existing.description),
        request.status.unwrap_or(existing.status) as _,
        request.start_date.or(existing.start_date),
        request.end_date.or(existing.end_date),
        course_id
    )
    .fetch_one(&conn)
    .await?;
    
    Ok(updated)
}

#[tauri::command]
pub async fn delete_course(course_id: i64, db: State<'_, DatabaseConnection>) -> ApiResult<()> {
    let conn = db.get_connection().await?;
    
    // Delete the course
    sqlx::query!("DELETE FROM courses WHERE id = $1", course_id)
        .execute(&conn)
        .await?;
    
    Ok(())
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