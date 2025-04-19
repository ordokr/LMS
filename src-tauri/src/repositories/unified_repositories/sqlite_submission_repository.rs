use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use crate::error::Error;
use crate::models::unified_models::{Submission, SubmissionStatus, SubmissionContentType, SubmissionComment};
use super::repository::Repository;
use super::submission_repository::SubmissionRepository;

/// SQLite implementation of the submission repository
pub struct SqliteSubmissionRepository {
    pool: Pool<Sqlite>,
}

impl SqliteSubmissionRepository {
    /// Create a new SQLite submission repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to a Submission
    async fn row_to_submission(&self, id: &str) -> Result<Option<Submission>, Error> {
        let submission_row = sqlx::query!(
            r#"
            SELECT 
                id, assignment_id, user_id, created_at, updated_at, submission_type,
                content, url, attachment_ids, status, submitted_at, attempt, late,
                missing, excused, grade, score, points_deducted, graded_at, grader_id,
                grade_matches_current, posted_at, canvas_id, discourse_id, quiz_submission_id,
                source_system, metadata
            FROM submissions
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = submission_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
            
            // Parse optional dates
            let submitted_at = if let Some(date_str) = row.submitted_at {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse submitted_at: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            let graded_at = if let Some(date_str) = row.graded_at {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse graded_at: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            let posted_at = if let Some(date_str) = row.posted_at {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse posted_at: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            // Parse submission type
            let submission_type = if let Some(type_str) = row.submission_type {
                Some(SubmissionContentType::from(type_str.as_str()))
            } else {
                None
            };
            
            // Parse status
            let status = SubmissionStatus::from(row.status.as_str());
            
            // Parse attachment IDs
            let attachment_ids: Vec<String> = if let Some(attachments_str) = row.attachment_ids {
                serde_json::from_str(&attachments_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse attachment_ids: {}", e)))?
            } else {
                Vec::new()
            };
            
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse metadata: {}", e)))?
            } else {
                HashMap::new()
            };
            
            // Load comments
            let comments = self.load_comments(&row.id).await?;
            
            // Create submission
            let submission = Submission {
                id: row.id,
                assignment_id: row.assignment_id,
                user_id: row.user_id,
                created_at,
                updated_at,
                submission_type,
                content: row.content,
                url: row.url,
                attachment_ids,
                status,
                submitted_at,
                attempt: row.attempt,
                late: row.late != 0,
                missing: row.missing != 0,
                excused: row.excused != 0,
                grade: row.grade,
                score: row.score,
                points_deducted: row.points_deducted,
                graded_at,
                grader_id: row.grader_id,
                grade_matches_current: row.grade_matches_current != 0,
                posted_at,
                canvas_id: row.canvas_id,
                discourse_id: row.discourse_id,
                quiz_submission_id: row.quiz_submission_id,
                source_system: row.source_system,
                metadata,
                comments: Some(comments),
            };
            
            Ok(Some(submission))
        } else {
            Ok(None)
        }
    }
    
    /// Helper method to load comments for a submission
    async fn load_comments(&self, submission_id: &str) -> Result<Vec<SubmissionComment>, Error> {
        let comment_rows = sqlx::query!(
            r#"
            SELECT 
                id, submission_id, author_id, comment, created_at, attachment_ids,
                is_hidden, is_draft
            FROM submission_comments
            WHERE submission_id = ?
            ORDER BY created_at ASC
            "#,
            submission_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut comments = Vec::new();
        
        for row in comment_rows {
            // Parse timestamp
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse comment created_at: {}", e)))?
                .with_timezone(&Utc);
            
            // Parse attachment IDs
            let attachment_ids: Vec<String> = if let Some(attachments_str) = row.attachment_ids {
                serde_json::from_str(&attachments_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse comment attachment_ids: {}", e)))?
            } else {
                Vec::new()
            };
            
            // Create comment
            let comment = SubmissionComment {
                id: row.id,
                submission_id: row.submission_id,
                author_id: row.author_id,
                comment: row.comment,
                created_at,
                attachment_ids,
                is_hidden: row.is_hidden != 0,
                is_draft: row.is_draft != 0,
            };
            
            comments.push(comment);
        }
        
        Ok(comments)
    }
    
    /// Helper method to save a comment
    async fn save_comment(&self, comment: &SubmissionComment) -> Result<(), Error> {
        // Serialize attachment IDs
        let attachment_ids_json = serde_json::to_string(&comment.attachment_ids)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize comment attachment_ids: {}", e)))?;
        
        // Insert comment
        sqlx::query!(
            r#"
            INSERT INTO submission_comments (
                id, submission_id, author_id, comment, created_at, attachment_ids,
                is_hidden, is_draft
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            comment.id,
            comment.submission_id,
            comment.author_id,
            comment.comment,
            comment.created_at.to_rfc3339(),
            attachment_ids_json,
            if comment.is_hidden { 1 } else { 0 },
            if comment.is_draft { 1 } else { 0 }
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

#[async_trait]
impl Repository<Submission, String> for SqliteSubmissionRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Submission>, Error> {
        self.row_to_submission(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<Submission>, Error> {
        let submission_ids = sqlx::query!(
            "SELECT id FROM submissions"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_ids {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn create(&self, submission: &Submission) -> Result<Submission, Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Serialize attachment IDs
        let attachment_ids_json = serde_json::to_string(&submission.attachment_ids)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize attachment_ids: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&submission.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert submission
        sqlx::query!(
            r#"
            INSERT INTO submissions (
                id, assignment_id, user_id, created_at, updated_at, submission_type,
                content, url, attachment_ids, status, submitted_at, attempt, late,
                missing, excused, grade, score, points_deducted, graded_at, grader_id,
                grade_matches_current, posted_at, canvas_id, discourse_id, quiz_submission_id,
                source_system, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            submission.id,
            submission.assignment_id,
            submission.user_id,
            submission.created_at.to_rfc3339(),
            submission.updated_at.to_rfc3339(),
            submission.submission_type.as_ref().map(|st| st.to_string()),
            submission.content,
            submission.url,
            attachment_ids_json,
            submission.status.to_string(),
            submission.submitted_at.map(|dt| dt.to_rfc3339()),
            submission.attempt,
            if submission.late { 1 } else { 0 },
            if submission.missing { 1 } else { 0 },
            if submission.excused { 1 } else { 0 },
            submission.grade,
            submission.score,
            submission.points_deducted,
            submission.graded_at.map(|dt| dt.to_rfc3339()),
            submission.grader_id,
            if submission.grade_matches_current { 1 } else { 0 },
            submission.posted_at.map(|dt| dt.to_rfc3339()),
            submission.canvas_id,
            submission.discourse_id,
            submission.quiz_submission_id,
            submission.source_system,
            metadata_json
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert comments
        if let Some(comments) = &submission.comments {
            for comment in comments {
                // Serialize attachment IDs
                let comment_attachment_ids_json = serde_json::to_string(&comment.attachment_ids)
                    .map_err(|e| Error::SerializationError(format!("Failed to serialize comment attachment_ids: {}", e)))?;
                
                // Insert comment
                sqlx::query!(
                    r#"
                    INSERT INTO submission_comments (
                        id, submission_id, author_id, comment, created_at, attachment_ids,
                        is_hidden, is_draft
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    comment.id,
                    comment.submission_id,
                    comment.author_id,
                    comment.comment,
                    comment.created_at.to_rfc3339(),
                    comment_attachment_ids_json,
                    if comment.is_hidden { 1 } else { 0 },
                    if comment.is_draft { 1 } else { 0 }
                )
                .execute(&mut *tx)
                .await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Return the created submission
        Ok(submission.clone())
    }
    
    async fn update(&self, submission: &Submission) -> Result<Submission, Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Serialize attachment IDs
        let attachment_ids_json = serde_json::to_string(&submission.attachment_ids)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize attachment_ids: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&submission.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Update submission
        sqlx::query!(
            r#"
            UPDATE submissions SET
                assignment_id = ?, user_id = ?, updated_at = ?, submission_type = ?,
                content = ?, url = ?, attachment_ids = ?, status = ?, submitted_at = ?,
                attempt = ?, late = ?, missing = ?, excused = ?, grade = ?, score = ?,
                points_deducted = ?, graded_at = ?, grader_id = ?, grade_matches_current = ?,
                posted_at = ?, canvas_id = ?, discourse_id = ?, quiz_submission_id = ?,
                source_system = ?, metadata = ?
            WHERE id = ?
            "#,
            submission.assignment_id,
            submission.user_id,
            submission.updated_at.to_rfc3339(),
            submission.submission_type.as_ref().map(|st| st.to_string()),
            submission.content,
            submission.url,
            attachment_ids_json,
            submission.status.to_string(),
            submission.submitted_at.map(|dt| dt.to_rfc3339()),
            submission.attempt,
            if submission.late { 1 } else { 0 },
            if submission.missing { 1 } else { 0 },
            if submission.excused { 1 } else { 0 },
            submission.grade,
            submission.score,
            submission.points_deducted,
            submission.graded_at.map(|dt| dt.to_rfc3339()),
            submission.grader_id,
            if submission.grade_matches_current { 1 } else { 0 },
            submission.posted_at.map(|dt| dt.to_rfc3339()),
            submission.canvas_id,
            submission.discourse_id,
            submission.quiz_submission_id,
            submission.source_system,
            metadata_json,
            submission.id
        )
        .execute(&mut *tx)
        .await?;
        
        // Update comments (delete and reinsert)
        if let Some(comments) = &submission.comments {
            // Delete existing comments
            sqlx::query!(
                "DELETE FROM submission_comments WHERE submission_id = ?",
                submission.id
            )
            .execute(&mut *tx)
            .await?;
            
            // Insert new comments
            for comment in comments {
                // Serialize attachment IDs
                let comment_attachment_ids_json = serde_json::to_string(&comment.attachment_ids)
                    .map_err(|e| Error::SerializationError(format!("Failed to serialize comment attachment_ids: {}", e)))?;
                
                // Insert comment
                sqlx::query!(
                    r#"
                    INSERT INTO submission_comments (
                        id, submission_id, author_id, comment, created_at, attachment_ids,
                        is_hidden, is_draft
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    comment.id,
                    comment.submission_id,
                    comment.author_id,
                    comment.comment,
                    comment.created_at.to_rfc3339(),
                    comment_attachment_ids_json,
                    if comment.is_hidden { 1 } else { 0 },
                    if comment.is_draft { 1 } else { 0 }
                )
                .execute(&mut *tx)
                .await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Return the updated submission
        Ok(submission.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<(), Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Delete comments
        sqlx::query!(
            "DELETE FROM submission_comments WHERE submission_id = ?",
            id
        )
        .execute(&mut *tx)
        .await?;
        
        // Delete submission
        sqlx::query!(
            "DELETE FROM submissions WHERE id = ?",
            id
        )
        .execute(&mut *tx)
        .await?;
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM submissions")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl SubmissionRepository for SqliteSubmissionRepository {
    async fn find_by_assignment_id(&self, assignment_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE assignment_id = ? ORDER BY submitted_at DESC",
            assignment_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE user_id = ? ORDER BY submitted_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_by_assignment_and_user(&self, assignment_id: &str, user_id: &str) -> Result<Option<Submission>, Error> {
        let submission_row = sqlx::query!(
            "SELECT id FROM submissions WHERE assignment_id = ? AND user_id = ?",
            assignment_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = submission_row {
            self.row_to_submission(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_course_id(&self, course_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT s.id FROM submissions s
            JOIN assignments a ON s.assignment_id = a.id
            WHERE a.course_id = ?
            ORDER BY s.submitted_at DESC",
            course_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_by_course_and_user(&self, course_id: &str, user_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT s.id FROM submissions s
            JOIN assignments a ON s.assignment_id = a.id
            WHERE a.course_id = ? AND s.user_id = ?
            ORDER BY s.submitted_at DESC",
            course_id,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_by_status(&self, status: SubmissionStatus) -> Result<Vec<Submission>, Error> {
        let status_str = status.to_string();
        
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE status = ? ORDER BY submitted_at DESC",
            status_str
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_by_submission_type(&self, submission_type: SubmissionContentType) -> Result<Vec<Submission>, Error> {
        let type_str = submission_type.to_string();
        
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE submission_type = ? ORDER BY submitted_at DESC",
            type_str
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Submission>, Error> {
        let submission_row = sqlx::query!(
            "SELECT id FROM submissions WHERE canvas_id = ?",
            canvas_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = submission_row {
            self.row_to_submission(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Submission>, Error> {
        let submission_row = sqlx::query!(
            "SELECT id FROM submissions WHERE discourse_id = ?",
            discourse_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = submission_row {
            self.row_to_submission(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_late_submissions(&self, assignment_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE assignment_id = ? AND late = 1 ORDER BY submitted_at DESC",
            assignment_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_missing_submissions(&self, assignment_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE assignment_id = ? AND missing = 1 ORDER BY submitted_at DESC",
            assignment_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn find_needs_grading(&self, assignment_id: &str) -> Result<Vec<Submission>, Error> {
        let submission_rows = sqlx::query!(
            "SELECT id FROM submissions WHERE assignment_id = ? AND status IN ('submitted', 'pending_review') ORDER BY submitted_at ASC",
            assignment_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut submissions = Vec::new();
        for row in submission_rows {
            if let Some(submission) = self.row_to_submission(&row.id).await? {
                submissions.push(submission);
            }
        }
        
        Ok(submissions)
    }
    
    async fn submit(&self, id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Submit the submission
        submission.submit();
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn grade(&self, id: &str, grader_id: &str, grade: &str, score: Option<f64>) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Grade the submission
        submission.grade(grader_id, grade, score);
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn return_to_student(&self, id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Return the submission to the student
        submission.return_to_student();
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn mark_late(&self, id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Mark the submission as late
        submission.mark_late();
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn mark_missing(&self, id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Mark the submission as missing
        submission.mark_missing();
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn excuse(&self, id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Excuse the submission
        submission.excuse();
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn add_comment(&self, id: &str, author_id: &str, comment: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Add the comment
        submission.add_comment(author_id, comment);
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn add_attachment(&self, id: &str, attachment_id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Add the attachment
        submission.add_attachment(attachment_id);
        
        // Update the submission
        self.update(&submission).await
    }
    
    async fn remove_attachment(&self, id: &str, attachment_id: &str) -> Result<Submission, Error> {
        // Get the submission
        let mut submission = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Submission with ID {} not found", id)))?;
        
        // Remove the attachment
        submission.remove_attachment(attachment_id);
        
        // Update the submission
        self.update(&submission).await
    }
}
