use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Submission model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    /// Unique identifier
    pub id: Uuid,
    /// Assignment ID
    pub assignment_id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// Submission content
    pub content: Option<String>,
    /// Submission URL
    pub url: Option<String>,
    /// Submission attachments
    pub attachments: Vec<String>,
    /// Submission grade
    pub grade: Option<f64>,
    /// Submission score
    pub score: Option<f64>,
    /// Submission status
    pub status: SubmissionStatus,
    /// Submitted at
    pub submitted_at: Option<DateTime<Utc>>,
    /// Graded at
    pub graded_at: Option<DateTime<Utc>>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Canvas ID
    pub canvas_id: Option<String>,
    /// Discourse post ID
    pub discourse_post_id: Option<String>,
}

/// Submission status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubmissionStatus {
    /// Not submitted
    NotSubmitted,
    /// Submitted
    Submitted,
    /// Graded
    Graded,
    /// Returned
    Returned,
}

impl Submission {
    /// Create a new submission
    pub fn new(
        assignment_id: Uuid,
        user_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            assignment_id,
            user_id,
            content: None,
            url: None,
            attachments: Vec::new(),
            grade: None,
            score: None,
            status: SubmissionStatus::NotSubmitted,
            submitted_at: None,
            graded_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canvas_id: None,
            discourse_post_id: None,
        }
    }
    
    /// Set content
    pub fn with_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }
    
    /// Set URL
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
    
    /// Add attachment
    pub fn add_attachment(mut self, attachment: &str) -> Self {
        self.attachments.push(attachment.to_string());
        self
    }
    
    /// Set grade
    pub fn with_grade(mut self, grade: f64) -> Self {
        self.grade = Some(grade);
        self
    }
    
    /// Set score
    pub fn with_score(mut self, score: f64) -> Self {
        self.score = Some(score);
        self
    }
    
    /// Set status
    pub fn with_status(mut self, status: SubmissionStatus) -> Self {
        self.status = status;
        self
    }
    
    /// Set submitted at
    pub fn with_submitted_at(mut self, submitted_at: DateTime<Utc>) -> Self {
        self.submitted_at = Some(submitted_at);
        self.status = SubmissionStatus::Submitted;
        self
    }
    
    /// Set graded at
    pub fn with_graded_at(mut self, graded_at: DateTime<Utc>) -> Self {
        self.graded_at = Some(graded_at);
        self.status = SubmissionStatus::Graded;
        self
    }
    
    /// Set Canvas ID
    pub fn with_canvas_id(mut self, canvas_id: &str) -> Self {
        self.canvas_id = Some(canvas_id.to_string());
        self
    }
    
    /// Set Discourse post ID
    pub fn with_discourse_post_id(mut self, discourse_post_id: &str) -> Self {
        self.discourse_post_id = Some(discourse_post_id.to_string());
        self
    }
}
