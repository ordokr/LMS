use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Submission status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmissionStatus {
    /// Not submitted yet
    NotSubmitted,
    /// Draft (saved but not submitted)
    Draft,
    /// Submitted and awaiting grading
    Submitted,
    /// Submitted late
    Late,
    /// Missing (past due and not submitted)
    Missing,
    /// Graded
    Graded,
    /// Returned to student
    Returned,
    /// Pending review
    PendingReview,
    /// Excused from submission
    Excused,
}

impl std::fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmissionStatus::NotSubmitted => write!(f, "not_submitted"),
            SubmissionStatus::Draft => write!(f, "draft"),
            SubmissionStatus::Submitted => write!(f, "submitted"),
            SubmissionStatus::Late => write!(f, "late"),
            SubmissionStatus::Missing => write!(f, "missing"),
            SubmissionStatus::Graded => write!(f, "graded"),
            SubmissionStatus::Returned => write!(f, "returned"),
            SubmissionStatus::PendingReview => write!(f, "pending_review"),
            SubmissionStatus::Excused => write!(f, "excused"),
        }
    }
}

impl From<&str> for SubmissionStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "not_submitted" => SubmissionStatus::NotSubmitted,
            "draft" => SubmissionStatus::Draft,
            "submitted" => SubmissionStatus::Submitted,
            "late" => SubmissionStatus::Late,
            "missing" => SubmissionStatus::Missing,
            "graded" => SubmissionStatus::Graded,
            "returned" => SubmissionStatus::Returned,
            "pending_review" => SubmissionStatus::PendingReview,
            "excused" => SubmissionStatus::Excused,
            _ => SubmissionStatus::NotSubmitted,
        }
    }
}

/// Submission type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmissionType {
    /// No submission
    None,
    /// Online text entry
    OnlineTextEntry,
    /// Online URL
    OnlineUrl,
    /// Online upload
    OnlineUpload,
    /// Media recording
    MediaRecording,
    /// Discussion topic
    DiscussionTopic,
    /// Quiz
    Quiz,
    /// External tool
    ExternalTool,
    /// On paper
    OnPaper,
    /// Attendance
    Attendance,
}

impl std::fmt::Display for SubmissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmissionType::None => write!(f, "none"),
            SubmissionType::OnlineTextEntry => write!(f, "online_text_entry"),
            SubmissionType::OnlineUrl => write!(f, "online_url"),
            SubmissionType::OnlineUpload => write!(f, "online_upload"),
            SubmissionType::MediaRecording => write!(f, "media_recording"),
            SubmissionType::DiscussionTopic => write!(f, "discussion_topic"),
            SubmissionType::Quiz => write!(f, "quiz"),
            SubmissionType::ExternalTool => write!(f, "external_tool"),
            SubmissionType::OnPaper => write!(f, "on_paper"),
            SubmissionType::Attendance => write!(f, "attendance"),
        }
    }
}

impl From<&str> for SubmissionType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => SubmissionType::None,
            "online_text_entry" => SubmissionType::OnlineTextEntry,
            "online_url" => SubmissionType::OnlineUrl,
            "online_upload" => SubmissionType::OnlineUpload,
            "media_recording" => SubmissionType::MediaRecording,
            "discussion_topic" => SubmissionType::DiscussionTopic,
            "quiz" => SubmissionType::Quiz,
            "external_tool" => SubmissionType::ExternalTool,
            "on_paper" => SubmissionType::OnPaper,
            "attendance" => SubmissionType::Attendance,
            _ => SubmissionType::None,
        }
    }
}

/// Submission comment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionComment {
    /// Unique identifier
    pub id: String,
    /// Submission ID
    pub submission_id: String,
    /// Author ID
    pub author_id: String,
    /// Comment text
    pub comment: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Attachment IDs
    pub attachment_ids: Vec<String>,
    /// Whether the comment is hidden
    pub is_hidden: bool,
    /// Whether the comment is draft
    pub is_draft: bool,
}

/// Submission model that harmonizes all existing submission implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    // Core fields
    pub id: String,                           // Primary identifier (UUID)
    pub assignment_id: String,                // Assignment ID
    pub user_id: String,                      // User ID
    pub created_at: DateTime<Utc>,            // Creation timestamp
    pub updated_at: DateTime<Utc>,            // Last update timestamp

    // Submission content
    pub submission_type: Option<SubmissionType>, // Type of submission
    pub content: Option<String>,              // Text content
    pub url: Option<String>,                  // URL content
    pub attachment_ids: Vec<String>,          // Attachment IDs

    // Submission status
    pub status: SubmissionStatus,             // Submission status
    pub submitted_at: Option<DateTime<Utc>>,  // When submitted
    pub attempt: i32,                         // Attempt number
    pub late: bool,                           // Whether submission is late
    pub missing: bool,                        // Whether submission is missing
    pub excused: bool,                        // Whether submission is excused

    // Grading
    pub grade: Option<String>,                // Grade (letter, percentage, etc.)
    pub score: Option<f64>,                   // Numeric score
    pub points_deducted: Option<f64>,         // Points deducted (late policy)
    pub graded_at: Option<DateTime<Utc>>,     // When graded
    pub grader_id: Option<String>,            // Grader ID
    pub grade_matches_current: bool,          // Whether grade matches current submission
    pub posted_at: Option<DateTime<Utc>>,     // When grade was posted

    // External system IDs
    pub canvas_id: Option<String>,            // Canvas submission ID
    pub discourse_id: Option<String>,         // Discourse post ID

    // Related content
    pub quiz_submission_id: Option<String>,   // Quiz submission ID

    // Metadata and extensibility
    pub source_system: Option<String>,        // Source system (canvas, discourse, etc.)
    pub metadata: HashMap<String, serde_json::Value>, // Extensible metadata

    // Comments (not stored directly in DB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<SubmissionComment>>, // Submission comments
}

impl Submission {
    /// Create a new Submission with default values
    pub fn new(
        id: Option<String>,
        assignment_id: String,
        user_id: String,
    ) -> Self {
        let now = Utc::now();
        let id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        Self {
            id,
            assignment_id,
            user_id,
            created_at: now,
            updated_at: now,
            submission_type: None,
            content: None,
            url: None,
            attachment_ids: Vec::new(),
            status: SubmissionStatus::NotSubmitted,
            submitted_at: None,
            attempt: 1,
            late: false,
            missing: false,
            excused: false,
            grade: None,
            score: None,
            points_deducted: None,
            graded_at: None,
            grader_id: None,
            grade_matches_current: true,
            posted_at: None,
            canvas_id: None,
            discourse_id: None,
            quiz_submission_id: None,
            source_system: None,
            metadata: HashMap::new(),
            comments: Some(Vec::new()),
        }

    /// Create a Submission from a Canvas submission JSON
    pub fn from_canvas_submission(canvas_submission: &serde_json::Value, assignment_id: &str, user_id: &str) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let canvas_id = canvas_submission["id"].as_str()
            .or_else(|| canvas_submission["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();

        // Parse timestamps
        let created_at = canvas_submission["created_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let updated_at = canvas_submission["updated_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let submitted_at = canvas_submission["submitted_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let graded_at = canvas_submission["graded_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let posted_at = canvas_submission["posted_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Parse submission content
        let submission_type_str = canvas_submission["submission_type"].as_str().unwrap_or("none");
        let submission_type = match submission_type_str {
            "" => None,
            _ => Some(SubmissionType::from(submission_type_str)),
        };

        let content = canvas_submission["body"].as_str().map(|s| s.to_string());
        let url = canvas_submission["url"].as_str().map(|s| s.to_string());

        // Parse attachment IDs
        let attachment_ids = if let Some(attachments) = canvas_submission["attachments"].as_array() {
            attachments.iter()
                .filter_map(|a| a["id"].as_str().or_else(|| a["id"].as_i64().map(|id| id.to_string())))
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };

        // Parse status
        let workflow_state = canvas_submission["workflow_state"].as_str().unwrap_or("unsubmitted");
        let status = match workflow_state {
            "submitted" => SubmissionStatus::Submitted,
            "graded" => SubmissionStatus::Graded,
            "pending_review" => SubmissionStatus::PendingReview,
            "unsubmitted" => SubmissionStatus::NotSubmitted,
            _ => SubmissionStatus::NotSubmitted,
        };

        // Parse attempt
        let attempt = canvas_submission["attempt"].as_i64().unwrap_or(1) as i32;

        // Parse late/missing/excused flags
        let late = canvas_submission["late"].as_bool().unwrap_or(false);
        let missing = canvas_submission["missing"].as_bool().unwrap_or(false);
        let excused = canvas_submission["excused"].as_bool().unwrap_or(false);

        // Parse grading information
        let grade = canvas_submission["grade"].as_str().map(|s| s.to_string());
        let score = canvas_submission["score"].as_f64();
        let points_deducted = canvas_submission["points_deducted"].as_f64();
        let grader_id = canvas_submission["grader_id"].as_str()
            .or_else(|| canvas_submission["grader_id"].as_i64().map(|id| id.to_string()));
        let grade_matches_current = canvas_submission["grade_matches_current_submission"].as_bool().unwrap_or(true);

        // Parse quiz submission ID
        let quiz_submission_id = canvas_submission["quiz_submission_id"].as_str()
            .or_else(|| canvas_submission["quiz_submission_id"].as_i64().map(|id| id.to_string()));

        // Parse comments
        let comments = if let Some(submission_comments) = canvas_submission["submission_comments"].as_array() {
            Some(submission_comments.iter()
                .filter_map(|c| {
                    let comment_id = c["id"].as_str()
                        .or_else(|| c["id"].as_i64().map(|id| id.to_string()))
                        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

                    let author_id = c["author_id"].as_str()
                        .or_else(|| c["author_id"].as_i64().map(|id| id.to_string()))?
                        .to_string();

                    let comment_text = c["comment"].as_str()?.to_string();

                    let comment_created_at = c["created_at"].as_str()
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(Utc::now);

                    let attachment_ids = if let Some(attachments) = c["attachments"].as_array() {
                        attachments.iter()
                            .filter_map(|a| a["id"].as_str().or_else(|| a["id"].as_i64().map(|id| id.to_string())))
                            .map(|s| s.to_string())
                            .collect()
                    } else {
                        Vec::new()
                    };

                    let is_hidden = c["hidden"].as_bool().unwrap_or(false);
                    let is_draft = c["draft"].as_bool().unwrap_or(false);

                    Some(SubmissionComment {
                        id: comment_id,
                        submission_id: id.clone(),
                        author_id,
                        comment: comment_text,
                        created_at: comment_created_at,
                        attachment_ids,
                        is_hidden,
                        is_draft,
                    })
                })
                .collect())
        } else {
            Some(Vec::new())
        };

        // Convert the canvas_submission to a HashMap for metadata
        let metadata = serde_json::to_value(canvas_submission).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();

        Self {
            id,
            assignment_id: assignment_id.to_string(),
            user_id: user_id.to_string(),
            created_at,
            updated_at,
            submission_type,
            content,
            url,
            attachment_ids,
            status,
            submitted_at,
            attempt,
            late,
            missing,
            excused,
            grade,
            score,
            points_deducted,
            graded_at,
            grader_id,
            grade_matches_current,
            posted_at,
            canvas_id: Some(canvas_id),
            discourse_id: None,
            quiz_submission_id,
            source_system: Some("canvas".to_string()),
            metadata,
            comments,
        }
    }

    /// Create a Submission from a Discourse post JSON
    pub fn from_discourse_post(discourse_post: &serde_json::Value, assignment_id: &str, user_id: &str) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let discourse_id = discourse_post["id"].as_str()
            .or_else(|| discourse_post["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();

        // Parse timestamps
        let created_at = discourse_post["created_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let updated_at = discourse_post["updated_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        // Parse content
        let content = discourse_post["cooked"].as_str()
            .or_else(|| discourse_post["raw"].as_str())
            .map(|s| s.to_string());

        // Parse status
        let status = SubmissionStatus::Submitted;

        // Convert the discourse_post to a HashMap for metadata
        let metadata = serde_json::to_value(discourse_post).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();

        Self {
            id,
            assignment_id: assignment_id.to_string(),
            user_id: user_id.to_string(),
            created_at,
            updated_at,
            submission_type: Some(SubmissionType::DiscussionTopic),
            content,
            url: None,
            attachment_ids: Vec::new(),
            status,
            submitted_at: Some(created_at),
            attempt: 1,
            late: false,
            missing: false,
            excused: false,
            grade: None,
            score: None,
            points_deducted: None,
            graded_at: None,
            grader_id: None,
            grade_matches_current: true,
            posted_at: None,
            canvas_id: None,
            discourse_id: Some(discourse_id),
            quiz_submission_id: None,
            source_system: Some("discourse".to_string()),
            metadata,
            comments: Some(Vec::new()),
        }
    }

    /// Convert Submission to Canvas submission JSON
    pub fn to_canvas_submission(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.canvas_id,
            "assignment_id": self.assignment_id,
            "user_id": self.user_id,
            "created_at": self.created_at.to_rfc3339(),
            "updated_at": self.updated_at.to_rfc3339(),
            "submitted_at": self.submitted_at.map(|dt| dt.to_rfc3339()),
            "graded_at": self.graded_at.map(|dt| dt.to_rfc3339()),
            "posted_at": self.posted_at.map(|dt| dt.to_rfc3339()),
            "submission_type": self.submission_type.as_ref().map(|st| st.to_string()),
            "body": self.content,
            "url": self.url,
            "grade": self.grade,
            "score": self.score,
            "points_deducted": self.points_deducted,
            "grader_id": self.grader_id,
            "grade_matches_current_submission": self.grade_matches_current,
            "workflow_state": match self.status {
                SubmissionStatus::NotSubmitted => "unsubmitted",
                SubmissionStatus::Draft => "unsubmitted",
                SubmissionStatus::Submitted => "submitted",
                SubmissionStatus::Late => "submitted",
                SubmissionStatus::Missing => "unsubmitted",
                SubmissionStatus::Graded => "graded",
                SubmissionStatus::Returned => "graded",
                SubmissionStatus::PendingReview => "pending_review",
                SubmissionStatus::Excused => "graded"
            },
            "attempt": self.attempt,
            "late": self.late,
            "missing": self.missing,
            "excused": self.excused,
            "quiz_submission_id": self.quiz_submission_id,
            "submission_comments": self.comments.as_ref().map(|comments| {
                comments.iter().map(|c| {
                    serde_json::json!({
                        "id": c.id,
                        "author_id": c.author_id,
                        "comment": c.comment,
                        "created_at": c.created_at.to_rfc3339(),
                        "hidden": c.is_hidden,
                        "draft": c.is_draft
                    })
                }).collect::<Vec<_>>()
            })
        })
    }

    /// Convert Submission to Discourse post JSON
    pub fn to_discourse_post(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "raw": self.content,
            "created_at": self.created_at.to_rfc3339(),
            "updated_at": self.updated_at.to_rfc3339(),
            "user_id": self.user_id
        })
    }

    /// Add a comment to the submission
    pub fn add_comment(&mut self, author_id: &str, comment: &str) -> SubmissionComment {
        let now = Utc::now();
        let comment = SubmissionComment {
            id: uuid::Uuid::new_v4().to_string(),
            submission_id: self.id.clone(),
            author_id: author_id.to_string(),
            comment: comment.to_string(),
            created_at: now,
            attachment_ids: Vec::new(),
            is_hidden: false,
            is_draft: false,
        };

        if let Some(comments) = &mut self.comments {
            comments.push(comment.clone());
        } else {
            self.comments = Some(vec![comment.clone()]);
        }

        self.updated_at = now;

        comment
    }

    /// Add an attachment to the submission
    pub fn add_attachment(&mut self, attachment_id: &str) {
        if !self.attachment_ids.contains(&attachment_id.to_string()) {
            self.attachment_ids.push(attachment_id.to_string());
            self.updated_at = Utc::now();
        }
    }

    /// Remove an attachment from the submission
    pub fn remove_attachment(&mut self, attachment_id: &str) {
        self.attachment_ids.retain(|id| id != attachment_id);
        self.updated_at = Utc::now();
    }

    /// Submit the submission
    pub fn submit(&mut self) {
        self.status = SubmissionStatus::Submitted;
        self.submitted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Mark the submission as late
    pub fn mark_late(&mut self) {
        self.late = true;
        self.updated_at = Utc::now();
    }

    /// Mark the submission as missing
    pub fn mark_missing(&mut self) {
        self.missing = true;
        self.updated_at = Utc::now();
    }

    /// Excuse the submission
    pub fn excuse(&mut self) {
        self.excused = true;
        self.status = SubmissionStatus::Excused;
        self.updated_at = Utc::now();
    }

    /// Grade the submission
    pub fn grade(&mut self, grader_id: &str, grade: &str, score: Option<f64>) {
        self.grade = Some(grade.to_string());
        self.score = score;
        self.grader_id = Some(grader_id.to_string());
        self.graded_at = Some(Utc::now());
        self.status = SubmissionStatus::Graded;
        self.updated_at = Utc::now();
    }

    /// Return the submission to the student
    pub fn return_to_student(&mut self) {
        self.status = SubmissionStatus::Returned;
        self.posted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Check if the submission has been submitted
    pub fn is_submitted(&self) -> bool {
        matches!(self.status,
            SubmissionStatus::Submitted |
            SubmissionStatus::Late |
            SubmissionStatus::Graded |
            SubmissionStatus::Returned |
            SubmissionStatus::PendingReview
        )
    }

    /// Check if the submission has been graded
    pub fn is_graded(&self) -> bool {
        matches!(self.status,
            SubmissionStatus::Graded |
            SubmissionStatus::Returned
        )
    }

    /// Check if the submission is late
    pub fn is_late(&self) -> bool {
        self.late
    }

    /// Check if the submission is missing
    pub fn is_missing(&self) -> bool {
        self.missing
    }

    /// Check if the submission is excused
    pub fn is_excused(&self) -> bool {
        self.excused
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_submission() {
        let submission = Submission::new(
            None,
            "assignment123".to_string(),
            "user456".to_string(),
        );

        assert_eq!(submission.assignment_id, "assignment123");
        assert_eq!(submission.user_id, "user456");
        assert_eq!(submission.status, SubmissionStatus::NotSubmitted);
        assert_eq!(submission.attempt, 1);
        assert_eq!(submission.late, false);
        assert_eq!(submission.missing, false);
        assert_eq!(submission.excused, false);
        assert_eq!(submission.grade, None);
        assert_eq!(submission.score, None);
        assert_eq!(submission.attachment_ids.len(), 0);
        assert_eq!(submission.comments.unwrap().len(), 0);
    }

    #[test]
    fn test_from_canvas_submission() {
        let canvas_json = serde_json::json!({
            "id": "12345",
            "assignment_id": "assignment123",
            "user_id": "user456",
            "created_at": "2023-12-01T00:00:00Z",
            "updated_at": "2023-12-01T12:34:56Z",
            "submitted_at": "2023-12-01T10:00:00Z",
            "graded_at": "2023-12-02T10:00:00Z",
            "submission_type": "online_text_entry",
            "body": "This is my submission",
            "grade": "A",
            "score": 95.5,
            "workflow_state": "graded",
            "attempt": 2,
            "late": true,
            "missing": false,
            "excused": false,
            "submission_comments": [
                {
                    "id": "comment1",
                    "author_id": "teacher789",
                    "comment": "Good work!",
                    "created_at": "2023-12-02T11:00:00Z"
                }
            ]
        });

        let submission = Submission::from_canvas_submission(&canvas_json, "assignment123", "user456");

        assert_eq!(submission.assignment_id, "assignment123");
        assert_eq!(submission.user_id, "user456");
        assert_eq!(submission.canvas_id, Some("12345".to_string()));
        assert_eq!(submission.content, Some("This is my submission".to_string()));
        assert_eq!(submission.submission_type, Some(SubmissionType::OnlineTextEntry));
        assert_eq!(submission.status, SubmissionStatus::Graded);
        assert_eq!(submission.grade, Some("A".to_string()));
        assert_eq!(submission.score, Some(95.5));
        assert_eq!(submission.attempt, 2);
        assert_eq!(submission.late, true);
        assert_eq!(submission.missing, false);
        assert_eq!(submission.excused, false);

        // Check comment
        let comments = submission.comments.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].author_id, "teacher789");
        assert_eq!(comments[0].comment, "Good work!");
    }

    #[test]
    fn test_from_discourse_post() {
        let discourse_json = serde_json::json!({
            "id": "67890",
            "raw": "This is my discussion post",
            "created_at": "2023-12-01T00:00:00Z",
            "updated_at": "2023-12-01T12:34:56Z",
            "user_id": "user456"
        });

        let submission = Submission::from_discourse_post(&discourse_json, "assignment123", "user456");

        assert_eq!(submission.assignment_id, "assignment123");
        assert_eq!(submission.user_id, "user456");
        assert_eq!(submission.discourse_id, Some("67890".to_string()));
        assert_eq!(submission.content, Some("This is my discussion post".to_string()));
        assert_eq!(submission.submission_type, Some(SubmissionType::DiscussionTopic));
        assert_eq!(submission.status, SubmissionStatus::Submitted);
        assert_eq!(submission.source_system, Some("discourse".to_string()));
    }

    #[test]
    fn test_to_canvas_submission() {
        let mut submission = Submission::new(
            Some("submission123".to_string()),
            "assignment123".to_string(),
            "user456".to_string(),
        );

        submission.canvas_id = Some("12345".to_string());
        submission.content = Some("This is my submission".to_string());
        submission.submission_type = Some(SubmissionType::OnlineTextEntry);
        submission.status = SubmissionStatus::Graded;
        submission.grade = Some("A".to_string());
        submission.score = Some(95.5);
        submission.grader_id = Some("teacher789".to_string());

        let canvas_submission = submission.to_canvas_submission();

        assert_eq!(canvas_submission["id"], "12345");
        assert_eq!(canvas_submission["assignment_id"], "assignment123");
        assert_eq!(canvas_submission["user_id"], "user456");
        assert_eq!(canvas_submission["body"], "This is my submission");
        assert_eq!(canvas_submission["submission_type"], "online_text_entry");
        assert_eq!(canvas_submission["workflow_state"], "graded");
        assert_eq!(canvas_submission["grade"], "A");
        assert_eq!(canvas_submission["score"], 95.5);
        assert_eq!(canvas_submission["grader_id"], "teacher789");
    }

    #[test]
    fn test_to_discourse_post() {
        let mut submission = Submission::new(
            Some("submission123".to_string()),
            "assignment123".to_string(),
            "user456".to_string(),
        );

        submission.discourse_id = Some("67890".to_string());
        submission.content = Some("This is my discussion post".to_string());

        let discourse_post = submission.to_discourse_post();

        assert_eq!(discourse_post["id"], "67890");
        assert_eq!(discourse_post["raw"], "This is my discussion post");
        assert_eq!(discourse_post["user_id"], "user456");
    }

    #[test]
    fn test_add_comment() {
        let mut submission = Submission::new(
            None,
            "assignment123".to_string(),
            "user456".to_string(),
        );

        let comment = submission.add_comment("teacher789", "Good work!");

        assert_eq!(comment.author_id, "teacher789");
        assert_eq!(comment.comment, "Good work!");
        assert_eq!(comment.is_hidden, false);
        assert_eq!(comment.is_draft, false);

        let comments = submission.comments.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].author_id, "teacher789");
        assert_eq!(comments[0].comment, "Good work!");
    }

    #[test]
    fn test_add_attachment() {
        let mut submission = Submission::new(
            None,
            "assignment123".to_string(),
            "user456".to_string(),
        );

        submission.add_attachment("attachment1");
        submission.add_attachment("attachment2");

        assert_eq!(submission.attachment_ids.len(), 2);
        assert!(submission.attachment_ids.contains(&"attachment1".to_string()));
        assert!(submission.attachment_ids.contains(&"attachment2".to_string()));

        // Test duplicate attachment
        submission.add_attachment("attachment1");
        assert_eq!(submission.attachment_ids.len(), 2);
    }

    #[test]
    fn test_remove_attachment() {
        let mut submission = Submission::new(
            None,
            "assignment123".to_string(),
            "user456".to_string(),
        );

        submission.add_attachment("attachment1");
        submission.add_attachment("attachment2");
        submission.remove_attachment("attachment1");

        assert_eq!(submission.attachment_ids.len(), 1);
        assert!(!submission.attachment_ids.contains(&"attachment1".to_string()));
        assert!(submission.attachment_ids.contains(&"attachment2".to_string()));
    }

    #[test]
    fn test_submission_status_changes() {
        let mut submission = Submission::new(
            None,
            "assignment123".to_string(),
            "user456".to_string(),
        );

        // Initially not submitted
        assert_eq!(submission.status, SubmissionStatus::NotSubmitted);
        assert!(!submission.is_submitted());
        assert!(!submission.is_graded());

        // Submit
        submission.submit();
        assert_eq!(submission.status, SubmissionStatus::Submitted);
        assert!(submission.is_submitted());
        assert!(!submission.is_graded());
        assert!(submission.submitted_at.is_some());

        // Grade
        submission.grade("teacher789", "A", Some(95.5));
        assert_eq!(submission.status, SubmissionStatus::Graded);
        assert!(submission.is_submitted());
        assert!(submission.is_graded());
        assert_eq!(submission.grade, Some("A".to_string()));
        assert_eq!(submission.score, Some(95.5));
        assert_eq!(submission.grader_id, Some("teacher789".to_string()));
        assert!(submission.graded_at.is_some());

        // Return to student
        submission.return_to_student();
        assert_eq!(submission.status, SubmissionStatus::Returned);
        assert!(submission.is_submitted());
        assert!(submission.is_graded());
        assert!(submission.posted_at.is_some());
    }

    #[test]
    fn test_submission_flags() {
        let mut submission = Submission::new(
            None,
            "assignment123".to_string(),
            "user456".to_string(),
        );

        // Initially not late, missing, or excused
        assert!(!submission.is_late());
        assert!(!submission.is_missing());
        assert!(!submission.is_excused());

        // Mark as late
        submission.mark_late();
        assert!(submission.is_late());
        assert!(!submission.is_missing());
        assert!(!submission.is_excused());

        // Mark as missing
        submission.mark_missing();
        assert!(submission.is_late());
        assert!(submission.is_missing());
        assert!(!submission.is_excused());

        // Excuse
        submission.excuse();
        assert!(submission.is_late());
        assert!(submission.is_missing());
        assert!(submission.is_excused());
        assert_eq!(submission.status, SubmissionStatus::Excused);
    }
}