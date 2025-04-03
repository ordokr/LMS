use sqlx::{Pool, Sqlite};
use serde_json::Value;

use crate::core::errors::AppError;
use crate::lms::models::{Assignment, GradingType, SubmissionType, Submission, SubmissionFile};
use crate::sync::engine::SyncEngine;
use crate::sync::operations::OperationType;
use std::sync::Arc;

pub struct AssignmentRepository {
    db: Pool<Sqlite>,
    sync_engine: Arc<SyncEngine>,
}

impl AssignmentRepository {
    pub fn new(db: Pool<Sqlite>, sync_engine: Arc<SyncEngine>) -> Self {
        Self { db, sync_engine }
    }
    
    // Create a new assignment
    pub async fn create_assignment(&self, assignment: &Assignment) -> Result<i64, AppError> {
        // Serialize submission types to JSON
        let submission_types = serde_json::to_string(&assignment.submission_types)
            .map_err(|e| AppError::ValidationError(format!("Invalid submission types: {}", e)))?;
        
        let result = sqlx::query!(
            r#"
            INSERT INTO assignments
            (course_id, title, description, due_date, available_from, available_until, 
             points_possible, submission_types, published)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            assignment.course_id,
            assignment.title,
            assignment.description,
            assignment.due_date,
            assignment.available_from,
            assignment.available_until,
            assignment.points_possible,
            submission_types,
            assignment.published
        )
        .fetch_one(&self.db)
        .await?;
        
        Ok(result.id)
    }
    
    // Get all assignments for a course
    pub async fn get_assignments_by_course_id(&self, course_id: i64) -> Result<Vec<Assignment>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, course_id, title, description, due_date, available_from, available_until,
                   points_possible, submission_types, published, created_at, updated_at
            FROM assignments
            WHERE course_id = ?
            ORDER BY due_date, title
            "#,
            course_id
        )
        .fetch_all(&self.db)
        .await?;
        
        let mut assignments = Vec::with_capacity(rows.len());
        
        for row in rows {
            // Parse submission types from JSON
            let submission_types: Vec<String> = serde_json::from_str(&row.submission_types)
                .unwrap_or_else(|_| Vec::new());
            
            assignments.push(Assignment {
                id: Some(row.id),
                course_id: row.course_id,
                title: row.title,
                description: row.description,
                due_date: row.due_date,
                available_from: row.available_from,
                available_until: row.available_until,
                points_possible: row.points_possible,
                submission_types,
                published: row.published,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(assignments)
    }
    
    // Get a specific assignment by ID
    pub async fn get_assignment_by_id(&self, assignment_id: i64) -> Result<Assignment, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT id, course_id, title, description, due_date, available_from, available_until,
                   points_possible, submission_types, published, created_at, updated_at
            FROM assignments
            WHERE id = ?
            "#,
            assignment_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Assignment with id {} not found", assignment_id)))?;
        
        // Parse submission types from JSON
        let submission_types: Vec<String> = serde_json::from_str(&row.submission_types)
            .unwrap_or_else(|_| Vec::new());
        
        let assignment = Assignment {
            id: Some(row.id),
            course_id: row.course_id,
            title: row.title,
            description: row.description,
            due_date: row.due_date,
            available_from: row.available_from,
            available_until: row.available_until,
            points_possible: row.points_possible,
            submission_types,
            published: row.published,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };
        
        Ok(assignment)
    }
    
    // Update an assignment
    pub async fn update_assignment(&self, assignment: &Assignment) -> Result<(), AppError> {
        // Serialize submission types to JSON
        let submission_types = serde_json::to_string(&assignment.submission_types)
            .map_err(|e| AppError::ValidationError(format!("Invalid submission types: {}", e)))?;
        
        sqlx::query!(
            r#"
            UPDATE assignments
            SET title = ?, description = ?, due_date = ?, available_from = ?,
                available_until = ?, points_possible = ?, submission_types = ?, 
                published = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            assignment.title,
            assignment.description,
            assignment.due_date,
            assignment.available_from,
            assignment.available_until,
            assignment.points_possible,
            submission_types,
            assignment.published,
            assignment.id
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    // Delete an assignment
    pub async fn delete_assignment(&self, assignment_id: i64) -> Result<(), AppError> {
        // Start a transaction to delete assignment and related data
        let mut tx = self.db.begin().await?;
        
        // Delete submissions first (would need to be implemented)
        // sqlx::query!("DELETE FROM submissions WHERE assignment_id = ?", assignment_id)
        //    .execute(&mut *tx)
        //    .await?;
        
        // Delete module items that reference this assignment
        sqlx::query!(
            r#"
            DELETE FROM module_items
            WHERE content_type = 'assignment' AND content_id = ?
            "#,
            assignment_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Delete the assignment
        sqlx::query!(
            r#"
            DELETE FROM assignments
            WHERE id = ?
            "#,
            assignment_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    // Verify that a user has instructor permissions for a course
    pub async fn verify_course_instructor(&self, course_id: i64, user_id: i64) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            SELECT 1 FROM enrollments 
            WHERE course_id = ? AND user_id = ? AND role IN ('teacher', 'teaching_assistant')
            "#,
            course_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        if result.is_none() {
            return Err(AppError::AuthorizationError("You do not have instructor permissions for this course".to_string()));
        }
        
        Ok(())
    }
    
    pub async fn create_submission(&self, user_id: i64, submission: Submission) -> Result<i64, AppError> {
        // Convert enums to strings
        let submission_type = submission.submission_type
            .map(|st| submission_type_to_string(st))
            .unwrap_or_else(|| "none".to_string());
            
        // Convert file list to JSON
        let files_json = serde_json::to_string(&submission.feedback_files)
            .map_err(|e| AppError::ServerError(format!("Failed to serialize files: {}", e)))?;
        
        // Insert into database
        let submission_id = sqlx::query!(
            r#"
            INSERT INTO submissions (
                assignment_id, user_id, submitted_at, submission_type,
                submission_data, url, grade, score, graded_at,
                grader_id, attempt, feedback_comment, feedback_files
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            submission.assignment_id,
            submission.user_id,
            submission.submitted_at,
            submission_type,
            submission.submission_data,
            submission.url,
            submission.grade,
            submission.score,
            submission.graded_at,
            submission.grader_id,
            submission.attempt,
            submission.feedback_comment,
            files_json
        )
        .fetch_one(&self.db)
        .await?
        .id;
        
        // Create sync operation
        let payload = serde_json::to_value(&submission)
            .map_err(|e| AppError::ServerError(format!("Failed to serialize submission: {}", e)))?;
            
        self.sync_engine.queue_operation(
            user_id,
            OperationType::Create,
            "submission",
            Some(&submission_id.to_string()),
            payload,
        ).await?;
        
        Ok(submission_id)
    }
    
    pub async fn get_submission(&self, assignment_id: i64, user_id: i64) -> Result<Option<Submission>, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT 
                id, assignment_id, user_id, submitted_at, submission_type,
                submission_data, url, grade, score, graded_at,
                grader_id, attempt, feedback_comment, feedback_files,
                created_at, updated_at
            FROM submissions
            WHERE assignment_id = ? AND user_id = ?
            ORDER BY attempt DESC
            LIMIT 1
            "#,
            assignment_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;
        
        match record {
            Some(record) => {
                // Convert strings to enums
                let submission_type = match record.submission_type.as_deref() {
                    Some(st) => Some(string_to_submission_type(st)),
                    None => None,
                };
                
                // Parse feedback files
                let feedback_files: Vec<SubmissionFile> = match serde_json::from_str(&record.feedback_files) {
                    Ok(files) => files,
                    Err(_) => Vec::new(),
                };
                
                Ok(Some(Submission {
                    id: Some(record.id),
                    assignment_id: record.assignment_id,
                    user_id: record.user_id,
                    submitted_at: record.submitted_at,
                    submission_type,
                    submission_data: record.submission_data,
                    url: record.url,
                    grade: record.grade,
                    score: record.score,
                    graded_at: record.graded_at,
                    grader_id: record.grader_id,
                    attempt: record.attempt,
                    feedback_comment: record.feedback_comment,
                    feedback_files,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                }))
            },
            None => Ok(None),
        }
    }
    
    pub async fn grade_submission(&self, grader_id: i64, submission_id: i64, 
                              grade: &str, score: Option<f64>, feedback: Option<&str>) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        
        // Update submission
        sqlx::query!(
            r#"
            UPDATE submissions
            SET grade = ?, score = ?, feedback_comment = ?,
                graded_at = ?, grader_id = ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            grade,
            score,
            feedback,
            now,
            grader_id,
            submission_id
        )
        .execute(&self.db)
        .await?;
        
        // Create sync operation
        let payload = serde_json::json!({
            "id": submission_id,
            "grade": grade,
            "score": score,
            "feedback_comment": feedback,
            "graded_at": now,
            "grader_id": grader_id
        });
        
        self.sync_engine.queue_operation(
            grader_id,
            OperationType::Update,
            "submission",
            Some(&submission_id.to_string()),
            payload,
        ).await?;
        
        Ok(())
    }
    
    pub async fn get_submissions_for_assignment(&self, assignment_id: i64) -> Result<Vec<Submission>, AppError> {
        let records = sqlx::query!(
            r#"
            SELECT 
                id, assignment_id, user_id, submitted_at, submission_type,
                submission_data, url, grade, score, graded_at,
                grader_id, attempt, feedback_comment, feedback_files,
                created_at, updated_at
            FROM submissions
            WHERE assignment_id = ?
            ORDER BY user_id, attempt DESC
            "#,
            assignment_id
        )
        .fetch_all(&self.db)
        .await?;
        
        let mut submissions = Vec::new();
        let mut seen_users = std::collections::HashSet::new();
        
        for record in records {
            // Only include the latest submission for each user
            if seen_users.contains(&record.user_id) {
                continue;
            }
            seen_users.insert(record.user_id);
            
            // Convert strings to enums
            let submission_type = match record.submission_type.as_deref() {
                Some(st) => Some(string_to_submission_type(st)),
                None => None,
            };
            
            // Parse feedback files
            let feedback_files: Vec<SubmissionFile> = match serde_json::from_str(&record.feedback_files) {
                Ok(files) => files,
                Err(_) => Vec::new(),
            };
            
            submissions.push(Submission {
                id: Some(record.id),
                assignment_id: record.assignment_id,
                user_id: record.user_id,
                submitted_at: record.submitted_at,
                submission_type,
                submission_data: record.submission_data,
                url: record.url,
                grade: record.grade,
                score: record.score,
                graded_at: record.graded_at,
                grader_id: record.grader_id,
                attempt: record.attempt,
                feedback_comment: record.feedback_comment,
                feedback_files,
                created_at: record.created_at,
                updated_at: record.updated_at,
            });
        }
        
        Ok(submissions)
    }
}

// Helper functions for enum conversions
fn grading_type_to_string(grading_type: GradingType) -> String {
    match grading_type {
        GradingType::Points => "points".to_string(),
        GradingType::Percentage => "percent".to_string(),
        GradingType::LetterGrade => "letter_grade".to_string(),
        GradingType::GpaScale => "gpa_scale".to_string(),
        GradingType::PassFail => "pass_fail".to_string(),
        GradingType::NotGraded => "not_graded".to_string(),
    }
}

fn string_to_grading_type(grading_type: &str) -> GradingType {
    match grading_type {
        "points" => GradingType::Points,
        "percent" => GradingType::Percentage,
        "letter_grade" => GradingType::LetterGrade,
        "gpa_scale" => GradingType::GpaScale,
        "pass_fail" => GradingType::PassFail,
        "not_graded" => GradingType::NotGraded,
        _ => GradingType::Points, // Default
    }
}

fn submission_type_to_string(submission_type: SubmissionType) -> String {
    match submission_type {
        SubmissionType::None => "none".to_string(),
        SubmissionType::OnlineText => "online_text_entry".to_string(),
        SubmissionType::OnlineUrl => "online_url".to_string(),
        SubmissionType::OnlineUpload => "online_upload".to_string(),
        SubmissionType::MediaRecording => "media_recording".to_string(),
        SubmissionType::Discussion => "discussion_topic".to_string(),
        SubmissionType::Quiz => "online_quiz".to_string(),
        SubmissionType::ExternalTool => "external_tool".to_string(),
        SubmissionType::NotGraded => "not_graded".to_string(),
    }
}

fn string_to_submission_type(submission_type: &str) -> SubmissionType {
    match submission_type {
        "none" => SubmissionType::None,
        "online_text_entry" => SubmissionType::OnlineText,
        "online_url" => SubmissionType::OnlineUrl,
        "online_upload" => SubmissionType::OnlineUpload,
        "media_recording" => SubmissionType::MediaRecording,
        "discussion_topic" => SubmissionType::Discussion,
        "online_quiz" => SubmissionType::Quiz,
        "external_tool" => SubmissionType::ExternalTool,
        "not_graded" => SubmissionType::NotGraded,
        _ => SubmissionType::None, // Default
    }
}