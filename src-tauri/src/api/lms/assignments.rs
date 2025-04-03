use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::core::auth::Claims;
use crate::core::errors::AppError;
use crate::database::repositories::AssignmentRepository;
use crate::lms::models::Assignment;
use crate::sync::engine::SyncEngine;
use crate::sync::operations::OperationType;

// Create a new assignment
pub async fn create_assignment(
    claims: Claims,
    State(assignment_repo): State<Arc<AssignmentRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path(course_id): Path<i64>,
    Json(mut assignment): Json<Assignment>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has permission to add assignments to this course
    assignment_repo.verify_course_instructor(course_id, user_id).await?;
    
    // Set the course ID
    assignment.course_id = course_id;
    
    // Create the assignment
    let assignment_id = assignment_repo.create_assignment(&assignment).await?;
    
    // Create sync operation
    let mut sync_assignment = assignment;
    sync_assignment.id = Some(assignment_id);
    
    sync_engine.queue_operation(
        user_id,
        OperationType::Create,
        "assignment",
        Some(&assignment_id.to_string()),
        serde_json::to_value(sync_assignment).unwrap(),
    ).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "assignment_id": assignment_id,
        "message": "Assignment created successfully"
    }))))
}

// Get all assignments for a course
pub async fn get_assignments(
    claims: Claims,
    State(assignment_repo): State<Arc<AssignmentRepository>>,
    Path(course_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has permission to view assignments for this course
    // In a real implementation, this should check course enrollment
    
    // Get assignments
    let assignments = assignment_repo.get_assignments_by_course_id(course_id).await?;
    
    Ok((StatusCode::OK, Json(assignments)))
}

// Get a specific assignment
pub async fn get_assignment(
    claims: Claims,
    State(assignment_repo): State<Arc<AssignmentRepository>>,
    Path((course_id, assignment_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has permission to view assignments for this course
    // In a real implementation, this should check course enrollment
    
    // Get assignment
    let assignment = assignment_repo.get_assignment_by_id(assignment_id).await?;
    
    // Verify the assignment belongs to the specified course
    if assignment.course_id != course_id {
        return Err(AppError::AuthorizationError("Assignment does not belong to the specified course".to_string()));
    }
    
    Ok((StatusCode::OK, Json(assignment)))
}

// Update an assignment
pub async fn update_assignment(
    claims: Claims,
    State(assignment_repo): State<Arc<AssignmentRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path((course_id, assignment_id)): Path<(i64, i64)>,
    Json(assignment_update): Json<Assignment>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has permission to update assignments in this course
    assignment_repo.verify_course_instructor(course_id, user_id).await?;
    
    // Get the existing assignment
    let existing_assignment = assignment_repo.get_assignment_by_id(assignment_id).await?;
    
    // Verify the assignment belongs to the specified course
    if existing_assignment.course_id != course_id {
        return Err(AppError::AuthorizationError("Assignment does not belong to the specified course".to_string()));
    }
    
    // Update the assignment with provided fields
    let mut updated_assignment = existing_assignment.clone();
    updated_assignment.title = assignment_update.title;
    updated_assignment.description = assignment_update.description;
    updated_assignment.due_date = assignment_update.due_date;
    updated_assignment.available_from = assignment_update.available_from;
    updated_assignment.available_until = assignment_update.available_until;
    updated_assignment.points_possible = assignment_update.points_possible;
    updated_assignment.submission_types = assignment_update.submission_types;
    updated_assignment.published = assignment_update.published;
    
    // Save the changes
    assignment_repo.update_assignment(&updated_assignment).await?;
    
    // Create sync operation
    sync_engine.queue_operation(
        user_id,
        OperationType::Update,
        "assignment",
        Some(&assignment_id.to_string()),
        serde_json::to_value(updated_assignment).unwrap(),
    ).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Assignment updated successfully"
    }))))
}

// Delete an assignment
pub async fn delete_assignment(
    claims: Claims,
    State(assignment_repo): State<Arc<AssignmentRepository>>,
    State(sync_engine): State<Arc<SyncEngine>>,
    Path((course_id, assignment_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    // Parse user ID from claims
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| AppError::AuthError("Invalid user ID in token".to_string()))?;
    
    // Check if user has permission to delete assignments in this course
    assignment_repo.verify_course_instructor(course_id, user_id).await?;
    
    // Get the existing assignment
    let existing_assignment = assignment_repo.get_assignment_by_id(assignment_id).await?;
    
    // Verify the assignment belongs to the specified course
    if existing_assignment.course_id != course_id {
        return Err(AppError::AuthorizationError("Assignment does not belong to the specified course".to_string()));
    }
    
    // Delete the assignment
    assignment_repo.delete_assignment(assignment_id).await?;
    
    // Create sync operation
    sync_engine.queue_operation(
        user_id,
        OperationType::Delete,
        "assignment",
        Some(&assignment_id.to_string()),
        serde_json::json!({}),
    ).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Assignment deleted successfully"
    }))))
}