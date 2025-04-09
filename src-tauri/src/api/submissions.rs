use crate::models::submission::{Submission, SubmissionCreate, SubmissionStatus};
use crate::db::submission_repository::SubmissionRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

/// Gets submissions for an assignment
///
/// # Arguments
/// * `assignment_id` - ID of the assignment
///
/// # Returns
/// * `Vec<Submission>` - List of submissions for the assignment
#[tauri::command]
#[instrument(skip(submission_repo), err)]
pub async fn get_submissions(
    assignment_id: String,
    submission_repo: State<'_, Arc<dyn SubmissionRepository + Send + Sync>>
) -> Result<Vec<Submission>, String> {
    info!(event = "api_call", endpoint = "get_submissions", assignment_id = %assignment_id);
    
    match submission_repo.get_submissions_by_assignment(&assignment_id).await {
        Ok(submissions) => {
            info!(
                event = "api_success", 
                endpoint = "get_submissions", 
                assignment_id = %assignment_id,
                count = submissions.len()
            );
            Ok(submissions)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_submissions", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets a specific submission by ID
///
/// # Arguments
/// * `submission_id` - ID of the submission to retrieve
///
/// # Returns
/// * `Submission` - The requested submission
#[tauri::command]
#[instrument(skip(submission_repo), err)]
pub async fn get_submission(
    submission_id: String,
    submission_repo: State<'_, Arc<dyn SubmissionRepository + Send + Sync>>
) -> Result<Submission, String> {
    info!(event = "api_call", endpoint = "get_submission", submission_id = %submission_id);
    
    match submission_repo.get_submission_by_id(&submission_id).await {
        Ok(Some(submission)) => {
            info!(event = "api_success", endpoint = "get_submission", submission_id = %submission_id);
            Ok(submission)
        },
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "get_submission", submission_id = %submission_id);
            Err(format!("Submission not found with ID: {}", submission_id))
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_submission", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new submission
///
/// # Arguments
/// * `submission_create` - Submission creation data
///
/// # Returns
/// * `Submission` - The created submission
#[tauri::command]
#[instrument(skip(submission_repo), err)]
pub async fn create_submission(
    submission_create: SubmissionCreate,
    submission_repo: State<'_, Arc<dyn SubmissionRepository + Send + Sync>>
) -> Result<Submission, String> {
    info!(
        event = "api_call", 
        endpoint = "create_submission", 
        assignment_id = %submission_create.assignment_id,
        user_id = %submission_create.user_id
    );
    
    // Generate ID and create full submission object
    let submission = Submission {
        id: Uuid::new_v4().to_string(),
        assignment_id: submission_create.assignment_id,
        user_id: submission_create.user_id,
        content: submission_create.content,
        attachments: submission_create.attachments,
        status: SubmissionStatus::Submitted,
        score: None,
        feedback: None,
        submitted_at: chrono::Utc::now().to_rfc3339(),
        graded_at: None,
    };
    
    match submission_repo.create_submission(submission).await {
        Ok(created) => {
            info!(event = "api_success", endpoint = "create_submission", submission_id = %created.id);
            Ok(created)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "create_submission", error = %e);
            Err(format!("Failed to create submission: {}", e))
        }
    }
}

/// Updates a submission (for grading)
///
/// # Arguments
/// * `submission` - Updated submission data
///
/// # Returns
/// * `Submission` - The updated submission
#[tauri::command]
#[instrument(skip(submission_repo), fields(submission_id = %submission.id), err)]
pub async fn update_submission(
    submission: Submission,
    submission_repo: State<'_, Arc<dyn SubmissionRepository + Send + Sync>>
) -> Result<Submission, String> {
    info!(event = "api_call", endpoint = "update_submission", submission_id = %submission.id);
    
    match submission_repo.update_submission(submission).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_submission", submission_id = %updated.id);
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_submission", error = %e);
            Err(format!("Failed to update submission: {}", e))
        }
    }
}