use crate::models::course::{Assignment, AssignmentStatus};
use crate::db::assignment_repository::AssignmentRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};

/// Gets all assignments, with optional filtering by course
///
/// # Arguments
/// * `course_id` - Optional course ID to filter assignments by
///
/// # Returns
/// * `Vec<Assignment>` - List of assignments matching the filters
#[tauri::command]
#[instrument(skip(assignment_repo), err)]
pub async fn get_assignments(
    course_id: Option<String>,
    assignment_repo: State<'_, Arc<dyn AssignmentRepository + Send + Sync>>
) -> Result<Vec<Assignment>, String> {
    info!(
        event = "api_call", 
        endpoint = "get_assignments",
        ?course_id
    );
    
    // Call repository to get assignments
    match assignment_repo.get_assignments(course_id).await {
        Ok(assignments) => {
            info!(
                event = "api_success", 
                endpoint = "get_assignments", 
                count = assignments.len()
            );
            Ok(assignments)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_assignments", 
                error = %e
            );
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets a single assignment by ID
///
/// # Arguments
/// * `assignment_id` - The ID of the assignment to retrieve
///
/// # Returns
/// * `Assignment` - The requested assignment
#[tauri::command]
#[instrument(skip(assignment_repo), err)]
pub async fn get_assignment(
    assignment_id: String,
    assignment_repo: State<'_, Arc<dyn AssignmentRepository + Send + Sync>>
) -> Result<Assignment, String> {
    info!(
        event = "api_call", 
        endpoint = "get_assignment", 
        assignment_id = %assignment_id
    );
    
    match assignment_repo.get_assignment_by_id(&assignment_id).await {
        Ok(Some(assignment)) => {
            info!(
                event = "api_success", 
                endpoint = "get_assignment", 
                assignment_id = %assignment_id
            );
            Ok(assignment)
        },
        Ok(None) => {
            warn!(
                event = "api_not_found", 
                endpoint = "get_assignment", 
                assignment_id = %assignment_id
            );
            Err(format!("Assignment not found with ID: {}", assignment_id))
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_assignment", 
                error = %e
            );
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new assignment
///
/// # Arguments
/// * `assignment` - Assignment data to create
///
/// # Returns
/// * `Assignment` - The created assignment
#[tauri::command]
#[instrument(skip(assignment_repo), err)]
pub async fn create_assignment(
    assignment: Assignment,
    assignment_repo: State<'_, Arc<dyn AssignmentRepository + Send + Sync>>
) -> Result<Assignment, String> {
    info!(
        event = "api_call", 
        endpoint = "create_assignment", 
        title = %assignment.title,
        course_id = %assignment.course_id
    );
    
    match assignment_repo.create_assignment(assignment).await {
        Ok(created) => {
            info!(
                event = "api_success", 
                endpoint = "create_assignment", 
                assignment_id = %created.id
            );
            Ok(created)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "create_assignment", 
                error = %e
            );
            Err(format!("Failed to create assignment: {}", e))
        }
    }
}

/// Updates an existing assignment
///
/// # Arguments
/// * `assignment` - Updated assignment data
///
/// # Returns
/// * `Assignment` - The updated assignment
#[tauri::command]
#[instrument(skip(assignment_repo), fields(assignment_id = %assignment.id), err)]
pub async fn update_assignment(
    assignment: Assignment,
    assignment_repo: State<'_, Arc<dyn AssignmentRepository + Send + Sync>>
) -> Result<Assignment, String> {
    info!(
        event = "api_call", 
        endpoint = "update_assignment", 
        assignment_id = %assignment.id
    );
    
    match assignment_repo.update_assignment(assignment).await {
        Ok(updated) => {
            info!(
                event = "api_success", 
                endpoint = "update_assignment", 
                assignment_id = %updated.id
            );
            Ok(updated)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "update_assignment", 
                error = %e
            );
            Err(format!("Failed to update assignment: {}", e))
        }
    }
}

/// Deletes an assignment
///
/// # Arguments
/// * `assignment_id` - ID of the assignment to delete
///
/// # Returns
/// * `bool` - Whether the deletion was successful
#[tauri::command]
#[instrument(skip(assignment_repo), err)]
pub async fn delete_assignment(
    assignment_id: String,
    assignment_repo: State<'_, Arc<dyn AssignmentRepository + Send + Sync>>
) -> Result<bool, String> {
    info!(
        event = "api_call", 
        endpoint = "delete_assignment", 
        assignment_id = %assignment_id
    );
    
    match assignment_repo.delete_assignment(&assignment_id).await {
        Ok(deleted) => {
            if deleted {
                info!(
                    event = "api_success", 
                    endpoint = "delete_assignment", 
                    assignment_id = %assignment_id
                );
            } else {
                warn!(
                    event = "api_not_found", 
                    endpoint = "delete_assignment", 
                    assignment_id = %assignment_id
                );
            }
            Ok(deleted)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "delete_assignment", 
                error = %e
            );
            Err(format!("Failed to delete assignment: {}", e))
        }
    }
}