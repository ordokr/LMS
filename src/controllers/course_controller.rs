use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    models::{course::Course, category::Category},
    services::course_category_mapper::CourseCategoryMapper,
    auth::middleware,
};

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    canvas_id: String,
    name: String,
    code: String,
    description: Option<String>,
    instructor_id: String,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Serialize)]
pub struct CourseResponse {
    course: Course,
    category: Option<Category>,
}

pub async fn create_course(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCourseRequest>,
) -> Result<Json<Course>, StatusCode> {
    let instructor_id = Uuid::parse_str(&payload.instructor_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Parse dates if provided
    let start_date = payload.start_date
        .as_deref()
        .map(chrono::DateTime::parse_from_rfc3339)
        .transpose()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let end_date = payload.end_date
        .as_deref()
        .map(chrono::DateTime::parse_from_rfc3339)
        .transpose()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .map(|dt| dt.with_timezone(&chrono::Utc));

    // Create course
    let course = Course::new(
        payload.canvas_id,
        payload.name,
        payload.code,
        payload.description,
        instructor_id,
        start_date,
        end_date,
    );

    // Save course
    let created_course = state.course_repo.create_course(&course)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_course))
}

pub async fn get_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<CourseResponse>, StatusCode> {
    let course_id = Uuid::parse_str(&course_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mapper = CourseCategoryMapper::new(
        state.course_repo.clone(),
        state.category_repo.clone(),
    );
    
    let (course, category) = mapper.get_course_with_category(&course_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(CourseResponse { course, category }))
}

#[derive(Deserialize)]
pub struct MapCategoryRequest {
    category_name: Option<String>,
}

pub async fn map_course_to_category(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
    Json(payload): Json<MapCategoryRequest>,
) -> Result<Json<CourseResponse>, StatusCode> {
    let course_id = Uuid::parse_str(&course_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mapper = CourseCategoryMapper::new(
        state.course_repo.clone(),
        state.category_repo.clone(),
    );
    
    let (course, category) = mapper.map_course_to_category(&course_id, payload.category_name)
        .await
        .map_err(|e| {
            eprintln!("Error mapping course to category: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(CourseResponse { 
        course, 
        category: Some(category) 
    }))
}

pub async fn list_courses(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Course>>, StatusCode> {
    let courses = state.course_repo.list_courses()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(courses))
}