use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::models::content::attachment::Attachment;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmissionStatus {
    NotSubmitted,
    Submitted,
    Graded,
    Returned,
    PendingReview,
}

/// Submission model based on Canvas Submission
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Submission {
    pub id: Uuid,
    pub assignment_id: Uuid,
    pub user_id: Uuid,
    
    // Optional content fields
    pub content: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
    pub submission_type: String,
    
    // Grading information
    pub grade: Option<String>,
    pub score: Option<f64>,
    pub grader_id: Option<Uuid>,
    
    // Status information
    pub status: SubmissionStatus,
    pub is_late: bool,
    pub attempt: i32,
    pub canvas_submission_id: Option<String>,
    
    // Timestamps with proper DateTime handling
    #[serde(default)]
    pub submitted_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub graded_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
    
    // Related data (not stored directly in DB)
    #[sqlx(skip)]
    pub attachment_ids: Vec<Uuid>,
    
    #[sqlx(skip)]
    pub comments: Vec<SubmissionComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionComment {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub user_id: Uuid,
    pub comment: String,
    
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    
    pub attachment_ids: Vec<Uuid>,
}

impl Submission {
    pub fn new(assignment_id: Uuid, user_id: Uuid) -> Self {
        let now = Utc::now();
        
        Submission {
            id: Uuid::new_v4(),
            assignment_id,
            user_id,
            content: None,
            body: None,
            url: None,
            submission_type: String::new(),
            grade: None,
            score: None,
            grader_id: None,
            status: SubmissionStatus::NotSubmitted,
            is_late: false,
            attempt: 1,
            canvas_submission_id: None,
            submitted_at: None,
            graded_at: None,
            created_at: now,
            updated_at: now,
            attachment_ids: Vec::new(),
            comments: Vec::new(),
        }
    }

    /// Validate submission data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Must have at least one content field populated
        if self.content.is_none() && self.body.is_none() && self.url.is_none() && self.attachment_ids.is_empty() {
            errors.push("Submission must include content, attachment, or URL".to_string());
        }
        
        // Score should be non-negative if provided
        if let Some(score) = self.score {
            if score < 0.0 {
                errors.push("Score cannot be negative".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    // Canvas API integration method
    pub fn from_canvas_api(canvas_submission: &serde_json::Value) -> Result<Self, String> {
        use crate::utils::date_utils::parse_date_string;
        
        let now = Utc::now();
        
        // Extract ids
        let id = match canvas_submission["id"].as_str() {
            Some(id_str) => Uuid::parse_str(id_str).map_err(|_| "Invalid UUID for id")?,
            None => Uuid::new_v4(), // Generate new UUID if not provided
        };
        
        let assignment_id = match canvas_submission["assignment_id"].as_str() {
            Some(id_str) => Uuid::parse_str(id_str).map_err(|_| "Invalid UUID for assignment_id")?,
            None => return Err("Missing assignment_id".to_string()),
        };
        
        let user_id = match canvas_submission["user_id"].as_str() {
            Some(id_str) => Uuid::parse_str(id_str).map_err(|_| "Invalid UUID for user_id")?,
            None => return Err("Missing user_id".to_string()),
        };
        
        // Extract grader_id if present
        let grader_id = if let Some(grader_id_str) = canvas_submission["grader_id"].as_str() {
            Some(Uuid::parse_str(grader_id_str).map_err(|_| "Invalid UUID for grader_id")?)
        } else {
            None
        };
        
        // Extract content fields
        let content = canvas_submission["body"].as_str().map(String::from);
        let body = canvas_submission["body"].as_str().map(String::from);
        let url = canvas_submission["url"].as_str().map(String::from);
        let submission_type = canvas_submission["submission_type"]
            .as_str()
            .unwrap_or("online_text_entry")
            .to_string();
        
        // Extract grading information
        let grade = canvas_submission["grade"].as_str().map(String::from);
        let score = canvas_submission["score"].as_f64();
        
        // Parse status
        let workflow_state = canvas_submission["workflow_state"].as_str().unwrap_or("unsubmitted");
        let status = match workflow_state {
            "submitted" => SubmissionStatus::Submitted,
            "graded" => SubmissionStatus::Graded,
            "pending_review" => SubmissionStatus::PendingReview,
            _ => SubmissionStatus::NotSubmitted,
        };
        
        // Parse other fields
        let is_late = canvas_submission["late"].as_bool().unwrap_or(false);
        let attempt = canvas_submission["attempt"].as_i64().unwrap_or(1) as i32;
        let canvas_submission_id = canvas_submission["id"].as_str().map(String::from);
        
        // Parse dates
        let submitted_at = parse_date_string(canvas_submission["submitted_at"].as_str());
        let graded_at = parse_date_string(canvas_submission["graded_at"].as_str());
        
        // Create the submission
        let submission = Self {
            id,
            assignment_id,
            user_id,
            content,
            body,
            url,
            submission_type,
            grade,
            score,
            grader_id,
            status,
            is_late,
            attempt,
            canvas_submission_id,
            submitted_at,
            graded_at,
            created_at: now,
            updated_at: now,
            attachment_ids: Vec::new(), // Will be populated separately
            comments: Vec::new(),       // Will be populated separately
        };
        
        Ok(submission)
    }

    // Database operations
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let mut submission: Submission = sqlx::query_as::<_, Submission>(
            "SELECT * FROM submissions WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load related attachments
        submission.attachment_ids = sqlx::query_scalar(
            "SELECT attachment_id FROM submission_attachments WHERE submission_id = ?"
        )
        .bind(submission.id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load comments
        submission.comments = Self::load_comments(db, submission.id).await?;
        
        Ok(submission)
    }
    
    pub async fn find_by_assignment_id(db: &DB, assignment_id: Uuid) -> Result<Vec<Self>, Error> {
        let submissions = sqlx::query_as::<_, Self>(
            "SELECT * FROM submissions WHERE assignment_id = ?"
        )
        .bind(assignment_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load related data for each submission
        let mut complete_submissions = Vec::with_capacity(submissions.len());
        
        for mut submission in submissions {
            // Load attachments
            submission.attachment_ids = sqlx::query_scalar(
                "SELECT attachment_id FROM submission_attachments WHERE submission_id = ?"
            )
            .bind(submission.id)
            .fetch_all(&db.pool)
            .await?;
            
            // Load comments
            submission.comments = Self::load_comments(db, submission.id).await?;
            
            complete_submissions.push(submission);
        }
        
        Ok(complete_submissions)
    }
    
    pub async fn find_by_user_id(db: &DB, user_id: Uuid) -> Result<Vec<Self>, Error> {
        let submissions = sqlx::query_as::<_, Self>(
            "SELECT * FROM submissions WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load related data for each submission
        let mut complete_submissions = Vec::with_capacity(submissions.len());
        
        for mut submission in submissions {
            // Load attachments
            submission.attachment_ids = sqlx::query_scalar(
                "SELECT attachment_id FROM submission_attachments WHERE submission_id = ?"
            )
            .bind(submission.id)
            .fetch_all(&db.pool)
            .await?;
            
            // Load comments
            submission.comments = Self::load_comments(db, submission.id).await?;
            
            complete_submissions.push(submission);
        }
        
        Ok(complete_submissions)
    }
    
    pub async fn find_by_user_and_assignment(db: &DB, user_id: Uuid, assignment_id: Uuid) -> Result<Self, Error> {
        let mut submission = sqlx::query_as::<_, Self>(
            "SELECT * FROM submissions WHERE user_id = ? AND assignment_id = ?"
        )
        .bind(user_id)
        .bind(assignment_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load attachments
        submission.attachment_ids = sqlx::query_scalar(
            "SELECT attachment_id FROM submission_attachments WHERE submission_id = ?"
        )
        .bind(submission.id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load comments
        submission.comments = Self::load_comments(db, submission.id).await?;
        
        Ok(submission)
    }
    
    async fn load_comments(db: &DB, submission_id: Uuid) -> Result<Vec<SubmissionComment>, Error> {
        let comment_rows: Vec<CommentRow> = sqlx::query_as(
            "SELECT * FROM submission_comments WHERE submission_id = ? ORDER BY created_at"
        )
        .bind(submission_id)
        .fetch_all(&db.pool)
        .await?;
        
        let mut comments: Vec<SubmissionComment> = Vec::with_capacity(comment_rows.len());
        
        for row in comment_rows {
            let mut comment = SubmissionComment {
                id: row.id,
                submission_id: row.submission_id,
                user_id: row.user_id,
                comment: row.comment,
                created_at: row.created_at,
                attachment_ids: Vec::new(),
            };
            
            // Load comment attachments
            let attachment_ids: Vec<Uuid> = sqlx::query_scalar(
                "SELECT attachment_id FROM comment_attachments WHERE comment_id = ?"
            )
            .bind(comment.id)
            .fetch_all(&db.pool)
            .await?;
            
            comment.attachment_ids = attachment_ids;
            comments.push(comment);
        }
        
        Ok(comments)
    }
    
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // Insert submission
        sqlx::query(
            "INSERT INTO submissions 
            (id, assignment_id, user_id, submitted_at, graded_at, score, grade,
            submission_type, body, url, attempt, status, created_at, updated_at,
            canvas_submission_id, grader_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.assignment_id)
        .bind(self.user_id)
        .bind(self.submitted_at)
        .bind(self.graded_at)
        .bind(self.score)
        .bind(&self.grade)
        .bind(&self.submission_type)
        .bind(&self.body)
        .bind(&self.url)
        .bind(self.attempt)
        .bind(self.status as i32)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.canvas_submission_id)
        .bind(self.grader_id)
        .execute(&mut *tx)
        .await?;
        
        // Insert attachments
        for attachment_id in &self.attachment_ids {
            sqlx::query(
                "INSERT INTO submission_attachments 
                (submission_id, attachment_id) VALUES (?, ?)"
            )
            .bind(self.id)
            .bind(attachment_id)
            .execute(&mut *tx)
            .await?;
        }
        
        // Insert comments
        for comment in &self.comments {
            sqlx::query(
                "INSERT INTO submission_comments 
                (id, submission_id, user_id, comment, created_at)
                VALUES (?, ?, ?, ?, ?)"
            )
            .bind(comment.id)
            .bind(comment.submission_id)
            .bind(comment.user_id)
            .bind(&comment.comment)
            .bind(comment.created_at)
            .execute(&mut *tx)
            .await?;
            
            // Insert comment attachments
            for attachment_id in &comment.attachment_ids {
                sqlx::query(
                    "INSERT INTO comment_attachments 
                    (comment_id, attachment_id) VALUES (?, ?)"
                )
                .bind(comment.id)
                .bind(attachment_id)
                .execute(&mut *tx)
                .await?;
            }
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(self.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // Update submission
        sqlx::query(
            "UPDATE submissions SET
            assignment_id = ?, user_id = ?, submitted_at = ?, graded_at = ?,
            score = ?, grade = ?, submission_type = ?, body = ?, url = ?,
            attempt = ?, status = ?, updated_at = ?, canvas_submission_id = ?,
            grader_id = ?
            WHERE id = ?"
        )
        .bind(self.assignment_id)
        .bind(self.user_id)
        .bind(self.submitted_at)
        .bind(self.graded_at)
        .bind(self.score)
        .bind(&self.grade)
        .bind(&self.submission_type)
        .bind(&self.body)
        .bind(&self.url)
        .bind(self.attempt)
        .bind(self.status as i32)
        .bind(Utc::now())
        .bind(&self.canvas_submission_id)
        .bind(self.grader_id)
        .bind(self.id)
        .execute(&mut *tx)
        .await?;
        
        // Update attachments (delete and reinsert)
        sqlx::query("DELETE FROM submission_attachments WHERE submission_id = ?")
            .bind(self.id)
            .execute(&mut *tx)
            .await?;
            
        for attachment_id in &self.attachment_ids {
            sqlx::query(
                "INSERT INTO submission_attachments 
                (submission_id, attachment_id) VALUES (?, ?)"
            )
            .bind(self.id)
            .bind(attachment_id)
            .execute(&mut *tx)
            .await?;
        }
        
        // Note: We don't update existing comments, only add new ones
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // Delete comment attachments
        sqlx::query(
            "DELETE FROM comment_attachments WHERE comment_id IN (
                SELECT id FROM submission_comments WHERE submission_id = ?
            )"
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        
        // Delete comments
        sqlx::query("DELETE FROM submission_comments WHERE submission_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Delete submission attachments
        sqlx::query("DELETE FROM submission_attachments WHERE submission_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Delete submission
        sqlx::query("DELETE FROM submissions WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    // Comment operations
    
    pub async fn add_comment(&mut self, db: &DB, comment: SubmissionComment) -> Result<Uuid, Error> {
        // Insert comment
        sqlx::query(
            "INSERT INTO submission_comments 
            (id, submission_id, user_id, comment, created_at)
            VALUES (?, ?, ?, ?, ?)"
        )
        .bind(comment.id)
        .bind(comment.submission_id)
        .bind(comment.user_id)
        .bind(&comment.comment)
        .bind(comment.created_at)
        .execute(&db.pool)
        .await?;
        
        // Insert comment attachments
        for attachment_id in &comment.attachment_ids {
            sqlx::query(
                "INSERT INTO comment_attachments 
                (comment_id, attachment_id) VALUES (?, ?)"
            )
            .bind(comment.id)
            .bind(attachment_id)
            .execute(&db.pool)
            .await?;
        }
        
        // Add to in-memory comments
        self.comments.push(comment.clone());
        
        // Update updated_at
        sqlx::query(
            "UPDATE submissions SET updated_at = ? WHERE id = ?"
        )
        .bind(Utc::now())
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(comment.id)
    }
    
    // Submission status operations
    
    pub async fn submit(&mut self, db: &DB) -> Result<(), Error> {
        self.submitted_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.status = SubmissionStatus::Submitted;
        
        sqlx::query(
            "UPDATE submissions SET 
            submitted_at = ?, updated_at = ?, status = ? WHERE id = ?"
        )
        .bind(self.submitted_at)
        .bind(self.updated_at)
        .bind(self.status as i32)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn grade(
        &mut self, 
        db: &DB, 
        grader_id: Uuid,
        score: f64, 
        grade: Option<String>
    ) -> Result<(), Error> {
        self.graded_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.status = SubmissionStatus::Graded;
        self.score = Some(score);
        self.grade = grade;
        self.grader_id = Some(grader_id);
        
        sqlx::query(
            "UPDATE submissions SET 
            graded_at = ?, updated_at = ?, status = ?, score = ?, grade = ?,
            grader_id = ? WHERE id = ?"
        )
        .bind(self.graded_at)
        .bind(self.updated_at)
        .bind(self.status as i32)
        .bind(self.score)
        .bind(&self.grade)
        .bind(self.grader_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
}

// Helper structs for SQLx
#[derive(FromRow)]
struct CommentRow {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub user_id: Uuid,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}