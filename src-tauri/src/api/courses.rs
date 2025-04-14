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
    description: Option<String>,
    status: Option<CourseStatus>, // Added field for status
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

async fn update_course_status(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    axum::Json(new_status): axum::Json<CourseStatus>,
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    let mut course = repo.get_by_id(id).await?;
    match new_status {
        CourseStatus::Active => course.activate(),
        CourseStatus::Archived => course.archive(),
        _ => (),
    }
    repo.update_status(id, new_status).await?;
    Ok(axum::Json(course))
}

async fn create_course(
    State(state): State<Arc<AppState>>,
    axum::Json(course_req): axum::Json<CourseRequest>
) -> Result<impl axum::response::IntoResponse, crate::api::forum::AppError> {
    let repo = CourseRepository::new(&state.db);
    let course = repo.create(
        course_req.name,
        course_req.code,
        course_req.description,
        course_req.status.unwrap_or(CourseStatus::Draft), // Default to Draft
    ).await?;
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

use crate::models::course::{Course, CourseStatus, CourseCreate};
use crate::db::course_repository::CourseRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

/// Gets all courses, with optional filtering by status
///
/// # Arguments
/// * `status` - Optional filter for course status
///
/// # Returns
/// * `Vec<Course>` - List of courses matching the filters
#[tauri::command]
pub async fn get_courses(
    status: Option<CourseStatus>,
    course_repo: State<'_, Arc<dyn CourseRepository + Send + Sync>>
) -> Result<Vec<Course>, String> {
    // Log the API call
    info!(event = "api_call", endpoint = "get_courses", ?status);
    
    // Call repository method to fetch courses
    match course_repo.get_courses(status).await {
        Ok(courses) => {
            info!(event = "api_success", endpoint = "get_courses", count = courses.len());
            Ok(courses)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_courses", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets a single course by ID
///
/// # Arguments
/// * `course_id` - The ID of the course to retrieve
///
/// # Returns
/// * `Course` - The requested course
#[tauri::command]
pub async fn get_course(
    course_id: String,
    course_repo: State<'_, Arc<dyn CourseRepository + Send + Sync>>
) -> Result<Course, String> {
    info!(event = "api_call", endpoint = "get_course", course_id = %course_id);
    
    // Call repository to fetch the specific course
    match course_repo.get_course_by_id(&course_id).await {
        Ok(Some(course)) => {
            info!(event = "api_success", endpoint = "get_course", course_id = %course_id);
            Ok(course)
        },
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "get_course", course_id = %course_id);
            Err(format!("Course not found with ID: {}", course_id))
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_course", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new course
///
/// # Arguments
/// * `course_create` - Course creation data
///
/// # Returns
/// * `Course` - The newly created course
#[tauri::command]
#[instrument(skip(course_repo), err)]
pub async fn create_course(
    course_create: CourseCreate,
    course_repo: State<'_, Arc<dyn CourseRepository + Send + Sync>>
) -> Result<Course, String> {
    info!(event = "api_call", endpoint = "create_course", title = %course_create.title);
    
    // Generate a new UUID for the course
    let course_id = Uuid::new_v4().to_string();
    
    // Create the full course object from the creation data
    let new_course = Course {
        id: course_id,
        title: course_create.title,
        description: course_create.description,
        status: course_create.status,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        modules: course_create.modules,
    };
    
    // Insert into repository
    match course_repo.create_course(new_course).await {
        Ok(course) => {
            info!(event = "api_success", endpoint = "create_course", course_id = %course.id);
            Ok(course)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "create_course", error = %e);
            Err(format!("Failed to create course: {}", e))
        }
    }
}

/// Updates an existing course
///
/// # Arguments
/// * `course` - Updated course data
///
/// # Returns
/// * `Course` - The updated course
#[tauri::command]
#[instrument(skip(course_repo), fields(course_id = %course.id), err)]
pub async fn update_course(
    course: Course,
    course_repo: State<'_, Arc<dyn CourseRepository + Send + Sync>>
) -> Result<Course, String> {
    info!(event = "api_call", endpoint = "update_course", course_id = %course.id);
    
    // Update the course's timestamp
    let mut updated_course = course;
    updated_course.updated_at = chrono::Utc::now().to_rfc3339();
    
    // Update in repository
    match course_repo.update_course(updated_course).await {
        Ok(course) => {
            info!(event = "api_success", endpoint = "update_course", course_id = %course.id);
            Ok(course)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_course", error = %e);
            Err(format!("Failed to update course: {}", e))
        }
    }
}

/// Deletes a course
///
/// # Arguments
/// * `course_id` - ID of the course to delete
///
/// # Returns
/// * `bool` - Whether the deletion was successful
#[tauri::command]
#[instrument(skip(course_repo), err)]
pub async fn delete_course(
    course_id: String,
    course_repo: State<'_, Arc<dyn CourseRepository + Send + Sync>>
) -> Result<bool, String> {
    info!(event = "api_call", endpoint = "delete_course", course_id = %course_id);
    
    // Delete from repository
    match course_repo.delete_course(&course_id).await {
        Ok(deleted) => {
            if deleted {
                info!(event = "api_success", endpoint = "delete_course", course_id = %course_id);
            } else {
                warn!(event = "api_not_found", endpoint = "delete_course", course_id = %course_id);
            }
            Ok(deleted)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "delete_course", error = %e);
            Err(format!("Failed to delete course: {}", e))
        }
    }
}