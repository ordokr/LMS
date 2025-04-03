use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;

use crate::core::auth::Claims;
use crate::core::errors::AppError;
use crate::database::repositories::CourseRepository;
use crate::lms::models::{Course, CourseStatus, Enrollment, EnrollmentRole, EnrollmentStatus};
use crate::sync::engine::SyncEngine;
use crate::sync::operations::OperationType;

#[derive(Debug, Deserialize)]
pub struct CourseFilter {
    pub status: Option<String>,
    pub user_id: Option<i64>,
}

// Create a new course
pub async fn create_course(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Json(course): Json<Course>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Set the instructor ID to the current user
    let mut new_course = course;
    new_course.instructor_id = user_id;
    
    // Create the course
    let course_id = course_repo.create_course(&new_course).await?;
    
    // Create sync operation
    let mut sync_course = new_course.clone();
    sync_course.id = Some(course_id);
    
    sync_engine.queue_operation(
        user_id,
        OperationType::Create,
        "course",
        Some(&course_id.to_string()),
        serde_json::to_value(sync_course).unwrap(),
    ).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "course_id": course_id,
        "message": "Course created successfully"
    }))))
}

// Get courses (filtered by status and/or user)
pub async fn get_courses(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    Query(params): Query<CourseFilter>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Parse status filter
    let status = match params.status.as_deref() {
        Some("draft") => Some(CourseStatus::Draft),
        Some("active") => Some(CourseStatus::Active),
        Some("archived") => Some(CourseStatus::Archived),
        _ => None,
    };
    
    // By default, show only courses for the current user
    let filter_user_id = params.user_id.unwrap_or(user_id);
    
    // Get courses
    let courses = course_repo.get_courses(Some(filter_user_id), status).await?;
    
    Ok((StatusCode::OK, Json(courses)))
}

// Get a specific course by ID
pub async fn get_course(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    Path(course_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has access to this course
    let is_enrolled = course_repo.check_enrollment(course_id, user_id, None).await?;
    if !is_enrolled {
        return Err(AppError::AuthorizationError("You do not have access to this course".to_string()));
    }
    
    // Get course
    let course = course_repo.get_course_by_id(course_id).await?;
    
    Ok((StatusCode::OK, Json(course)))
}

// Update a course
pub async fn update_course(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path(course_id): Path<i64>,
    Json(course_update): Json<Course>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Get the existing course
    let existing_course = course_repo.get_course_by_id(course_id).await?;
    
    // Check if user is the instructor
    if existing_course.instructor_id != user_id {
        return Err(AppError::AuthorizationError("Only the course instructor can update this course".to_string()));
    }
    
    // Update the course with provided fields
    let mut updated_course = existing_course.clone();
    
    if !course_update.code.is_empty() {
        updated_course.code = course_update.code;
    }
    
    if !course_update.name.is_empty() {
        updated_course.name = course_update.name;
    }
    
    updated_course.description = course_update.description;
    updated_course.start_date = course_update.start_date;
    updated_course.end_date = course_update.end_date;
    updated_course.status = course_update.status;
    
    // Save the changes
    course_repo.update_course(&updated_course).await?;
    
    // Create sync operation
    sync_engine.queue_operation(
        user_id,
        OperationType::Update,
        "course",
        Some(&course_id.to_string()),
        serde_json::to_value(updated_course).unwrap(),
    ).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Course updated successfully"
    }))))
}

// Delete a course
pub async fn delete_course(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path(course_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Get the existing course
    let existing_course = course_repo.get_course_by_id(course_id).await?;
    
    // Check if user is the instructor
    if existing_course.instructor_id != user_id {
        return Err(AppError::AuthorizationError("Only the course instructor can delete this course".to_string()));
    }
    
    // Delete the course
    course_repo.delete_course(course_id).await?;
    
    // Create sync operation
    sync_engine.queue_operation(
        user_id,
        OperationType::Delete,
        "course",
        Some(&course_id.to_string()),
        serde_json::json!({}),
    ).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Course deleted successfully"
    }))))
}

// Get enrollments for a course
pub async fn get_enrollments(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    Path(course_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has access to this course
    let is_instructor = course_repo.check_enrollment(
        course_id, 
        user_id, 
        Some(EnrollmentRole::Teacher)
    ).await?;
    
    if !is_instructor {
        return Err(AppError::AuthorizationError("Only instructors can view all enrollments".to_string()));
    }
    
    // Get enrollments
    let enrollments = course_repo.get_enrollments(course_id).await?;
    
    Ok((StatusCode::OK, Json(enrollments)))
}

#[derive(Debug, Deserialize)]
pub struct EnrollmentRequest {
    pub user_id: i64,
    pub role: String,
    pub status: Option<String>,
}

// Enroll a user in a course
pub async fn enroll_user(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path(course_id): Path<i64>,
    Json(enrollment_req): Json<EnrollmentRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user is the instructor
    let is_instructor = course_repo.check_enrollment(
        course_id, 
        user_id, 
        Some(EnrollmentRole::Teacher)
    ).await?;
    
    if !is_instructor {
        return Err(AppError::AuthorizationError("Only instructors can enroll users".to_string()));
    }
    
    // Parse role
    let role = match enrollment_req.role.as_str() {
        "student" => EnrollmentRole::Student,
        "teacher" => EnrollmentRole::Teacher,
        "teaching_assistant" => EnrollmentRole::TeachingAssistant,
        "observer" => EnrollmentRole::Observer,
        _ => return Err(AppError::ValidationError("Invalid role".to_string())),
    };
    
    // Parse status (default to Active)
    let status = match enrollment_req.status.as_deref() {
        Some("invited") => EnrollmentStatus::Invited,
        Some("completed") => EnrollmentStatus::Completed,
        Some("rejected") => EnrollmentStatus::Rejected,
        _ => EnrollmentStatus::Active,
    };
    
    // Enroll the user
    let enrollment_id = course_repo.enroll_user(
        course_id,
        enrollment_req.user_id,
        role,
        status,
    ).await?;
    
    // Create sync operation
    let enrollment = Enrollment {
        id: Some(enrollment_id),
        user_id: enrollment_req.user_id,
        course_id,
        role,
        status,
        created_at: None,
        updated_at: None,
    };
    
    sync_engine.queue_operation(
        user_id,
        OperationType::Create,
        "enrollment",
        Some(&enrollment_id.to_string()),
        serde_json::to_value(enrollment).unwrap(),
    ).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "enrollment_id": enrollment_id,
        "message": "User enrolled successfully"
    }))))
}

#[derive(Debug, Deserialize)]
pub struct EnrollmentUpdateRequest {
    pub role: Option<String>,
    pub status: Option<String>,
}

// Update a user's enrollment
pub async fn update_enrollment(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path((course_id, enroll_user_id)): Path<(i64, i64)>,
    Json(update_req): Json<EnrollmentUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user is the instructor
    let is_instructor = course_repo.check_enrollment(
        course_id, 
        user_id, 
        Some(EnrollmentRole::Teacher)
    ).await?;
    
    if !is_instructor {
        return Err(AppError::AuthorizationError("Only instructors can update enrollments".to_string()));
    }
    
    // Get current enrollment to merge changes
    let enrollments = course_repo.get_enrollments(course_id).await?;
    let current_enrollment = enrollments.iter()
        .find(|e| e.user_id == enroll_user_id)
        .ok_or_else(|| AppError::NotFound(format!("Enrollment for user {} not found", enroll_user_id)))?;
    
    // Parse role (use current if not provided)
    let role = match update_req.role.as_deref() {
        Some("student") => EnrollmentRole::Student,
        Some("teacher") => EnrollmentRole::Teacher,
        Some("teaching_assistant") => EnrollmentRole::TeachingAssistant,
        Some("observer") => EnrollmentRole::Observer,
        _ => current_enrollment.role.clone(),
    };
    
    // Parse status (use current if not provided)
    let status = match update_req.status.as_deref() {
        Some("active") => EnrollmentStatus::Active,
        Some("invited") => EnrollmentStatus::Invited,
        Some("completed") => EnrollmentStatus::Completed,
        Some("rejected") => EnrollmentStatus::Rejected,
        _ => current_enrollment.status.clone(),
    };
    
    // Update enrollment
    course_repo.update_enrollment(
        course_id,
        enroll_user_id,
        role.clone(),
        status.clone(),
    ).await?;
    
    // Create sync operation
    let enrollment = Enrollment {
        id: current_enrollment.id,
        user_id: enroll_user_id,
        course_id,
        role,
        status,
        created_at: current_enrollment.created_at.clone(),
        updated_at: None,
    };
    
    sync_engine.queue_operation(
        user_id,
        OperationType::Update,
        "enrollment",
        current_enrollment.id.map(|id| id.to_string()).as_deref(),
        serde_json::to_value(enrollment).unwrap(),
    ).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Enrollment updated successfully"
    }))))
}

// Remove a user from a course
pub async fn remove_enrollment(
    claims: Claims,
    State(course_repo): State<Arc<CourseRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path((course_id, enroll_user_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user is the instructor or the user themselves
    let is_instructor = course_repo.check_enrollment(
        course_id, 
        user_id, 
        Some(EnrollmentRole::Teacher)
    ).await?;
    
    if !is_instructor && user_id != enroll_user_id {
        return Err(AppError::AuthorizationError("Only instructors can remove other users".to_string()));
    }
    
    // Get current enrollment for sync operation
    let enrollments = course_repo.get_enrollments(course_id).await?;
    let current_enrollment = enrollments.iter()
        .find(|e| e.user_id == enroll_user_id)
        .ok_or_else(|| AppError::NotFound(format!("Enrollment for user {} not found", enroll_user_id)))?;
    
    // Remove enrollment
    course_repo.remove_enrollment(course_id, enroll_user_id).await?;
    
    // Create sync operation
    sync_engine.queue_operation(
        user_id,
        OperationType::Delete,
        "enrollment",
        current_enrollment.id.map(|id| id.to_string()).as_deref(),
        serde_json::json!({
            "user_id": enroll_user_id,
            "course_id": course_id
        }),
    ).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "User removed from course successfully"
    }))))
}