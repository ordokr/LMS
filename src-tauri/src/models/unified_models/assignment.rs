use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Assignment submission type enum
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
    /// Online quiz
    OnlineQuiz,
    /// Discussion topic
    DiscussionTopic,
    /// Media recording
    MediaRecording,
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
            SubmissionType::OnlineQuiz => write!(f, "online_quiz"),
            SubmissionType::DiscussionTopic => write!(f, "discussion_topic"),
            SubmissionType::MediaRecording => write!(f, "media_recording"),
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
            "online_quiz" => SubmissionType::OnlineQuiz,
            "discussion_topic" => SubmissionType::DiscussionTopic,
            "media_recording" => SubmissionType::MediaRecording,
            "external_tool" => SubmissionType::ExternalTool,
            "on_paper" => SubmissionType::OnPaper,
            "attendance" => SubmissionType::Attendance,
            _ => SubmissionType::None,
        }
    }
}

/// Assignment grading type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GradingType {
    /// Points
    Points,
    /// Percentage
    Percentage,
    /// Letter grade
    LetterGrade,
    /// GPA scale
    GpaScale,
    /// Pass/fail
    PassFail,
    /// Not graded
    NotGraded,
}

impl std::fmt::Display for GradingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GradingType::Points => write!(f, "points"),
            GradingType::Percentage => write!(f, "percent"),
            GradingType::LetterGrade => write!(f, "letter_grade"),
            GradingType::GpaScale => write!(f, "gpa_scale"),
            GradingType::PassFail => write!(f, "pass_fail"),
            GradingType::NotGraded => write!(f, "not_graded"),
        }
    }
}

impl From<&str> for GradingType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "points" => GradingType::Points,
            "percent" | "percentage" => GradingType::Percentage,
            "letter_grade" => GradingType::LetterGrade,
            "gpa_scale" => GradingType::GpaScale,
            "pass_fail" => GradingType::PassFail,
            "not_graded" => GradingType::NotGraded,
            _ => GradingType::Points,
        }
    }
}

/// Assignment status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentStatus {
    /// Published
    Published,
    /// Unpublished
    Unpublished,
    /// Deleted
    Deleted,
}

impl std::fmt::Display for AssignmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentStatus::Published => write!(f, "published"),
            AssignmentStatus::Unpublished => write!(f, "unpublished"),
            AssignmentStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl From<&str> for AssignmentStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "published" => AssignmentStatus::Published,
            "unpublished" => AssignmentStatus::Unpublished,
            "deleted" => AssignmentStatus::Deleted,
            _ => AssignmentStatus::Unpublished,
        }
    }
}

/// Assignment model that harmonizes all existing assignment implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    // Core fields
    pub id: String,                           // Primary identifier (UUID)
    pub title: String,                        // Assignment title
    pub description: Option<String>,          // Assignment description
    pub created_at: DateTime<Utc>,            // Creation timestamp
    pub updated_at: DateTime<Utc>,            // Last update timestamp

    // Course relationship
    pub course_id: Option<String>,            // Course ID

    // Dates
    pub due_date: Option<DateTime<Utc>>,      // Due date
    pub unlock_date: Option<DateTime<Utc>>,   // Unlock date
    pub lock_date: Option<DateTime<Utc>>,     // Lock date

    // Grading
    pub points_possible: Option<f64>,         // Points possible
    pub grading_type: GradingType,            // Grading type
    pub submission_types: Vec<SubmissionType>, // Submission types

    // Status
    pub status: AssignmentStatus,             // Assignment status
    pub is_published: bool,                   // Whether the assignment is published

    // Group
    pub group_category_id: Option<String>,    // Group category ID
    pub assignment_group_id: Option<String>,  // Assignment group ID

    // Peer review
    pub peer_reviews: bool,                   // Whether peer reviews are enabled
    pub automatic_peer_reviews: bool,         // Whether automatic peer reviews are enabled
    pub peer_review_count: Option<i32>,       // Number of peer reviews

    // External system IDs
    pub canvas_id: Option<String>,            // Canvas assignment ID
    pub discourse_id: Option<String>,         // Discourse topic ID

    // Related content
    pub quiz_id: Option<String>,              // Quiz ID
    pub discussion_topic_id: Option<String>,  // Discussion topic ID

    // Position
    pub position: Option<i32>,                // Position in assignment group

    // Metadata and extensibility
    pub source_system: Option<String>,        // Source system (canvas, discourse, etc.)
    pub metadata: HashMap<String, serde_json::Value>, // Extensible metadata
}

impl Assignment {
    /// Create a new Assignment with default values
    pub fn new(
        id: Option<String>,
        title: String,
    ) -> Self {
        let now = Utc::now();
        let id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        Self {
            id,
            title,
            description: None,
            created_at: now,
            updated_at: now,
            course_id: None,
            due_date: None,
            unlock_date: None,
            lock_date: None,
            points_possible: Some(0.0),
            grading_type: GradingType::Points,
            submission_types: vec![SubmissionType::None],
            status: AssignmentStatus::Unpublished,
            is_published: false,
            group_category_id: None,
            assignment_group_id: None,
            peer_reviews: false,
            automatic_peer_reviews: false,
            peer_review_count: None,
            canvas_id: None,
            discourse_id: None,
            quiz_id: None,
            discussion_topic_id: None,
            position: None,
            source_system: None,
            metadata: HashMap::new(),
        }

    /// Create an Assignment from a Canvas assignment JSON
    pub fn from_canvas_assignment(canvas_assignment: &serde_json::Value) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let canvas_id = canvas_assignment["id"].as_str()
            .or_else(|| canvas_assignment["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let title = canvas_assignment["name"].as_str().unwrap_or("").to_string();
        let description = canvas_assignment["description"].as_str().map(|s| s.to_string());

        // Parse dates
        let due_date = canvas_assignment["due_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let unlock_date = canvas_assignment["unlock_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let lock_date = canvas_assignment["lock_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Parse course ID
        let course_id = canvas_assignment["course_id"].as_str()
            .or_else(|| canvas_assignment["course_id"].as_i64().map(|id| id.to_string()));

        // Parse points possible
        let points_possible = canvas_assignment["points_possible"].as_f64();

        // Parse grading type
        let grading_type_str = canvas_assignment["grading_type"].as_str().unwrap_or("points");
        let grading_type = GradingType::from(grading_type_str);

        // Parse submission types
        let submission_types = if let Some(types) = canvas_assignment["submission_types"].as_array() {
            types.iter()
                .filter_map(|t| t.as_str())
                .map(SubmissionType::from)
                .collect()
        } else if let Some(types_str) = canvas_assignment["submission_types"].as_str() {
            types_str.split(',')
                .map(str::trim)
                .map(SubmissionType::from)
                .collect()
        } else {
            vec![SubmissionType::None]
        };

        // Parse published status
        let workflow_state = canvas_assignment["workflow_state"].as_str().unwrap_or("unpublished");
        let status = match workflow_state {
            "published" => AssignmentStatus::Published,
            "deleted" => AssignmentStatus::Deleted,
            _ => AssignmentStatus::Unpublished,
        };
        let is_published = status == AssignmentStatus::Published;

        // Parse group category ID
        let group_category_id = canvas_assignment["group_category_id"].as_str()
            .or_else(|| canvas_assignment["group_category_id"].as_i64().map(|id| id.to_string()));

        // Parse assignment group ID
        let assignment_group_id = canvas_assignment["assignment_group_id"].as_str()
            .or_else(|| canvas_assignment["assignment_group_id"].as_i64().map(|id| id.to_string()));

        // Parse peer review settings
        let peer_reviews = canvas_assignment["peer_reviews"].as_bool().unwrap_or(false);
        let automatic_peer_reviews = canvas_assignment["automatic_peer_reviews"].as_bool().unwrap_or(false);
        let peer_review_count = canvas_assignment["peer_review_count"].as_i64().map(|c| c as i32);

        // Parse position
        let position = canvas_assignment["position"].as_i64().map(|p| p as i32);

        // Parse quiz ID
        let quiz_id = canvas_assignment["quiz_id"].as_str()
            .or_else(|| canvas_assignment["quiz_id"].as_i64().map(|id| id.to_string()));

        // Parse discussion topic ID
        let discussion_topic_id = canvas_assignment["discussion_topic"].as_object()
            .and_then(|topic| topic.get("id"))
            .and_then(|id| id.as_str().or_else(|| id.as_i64().map(|id| id.to_string())));

        // Convert the canvas_assignment to a HashMap for metadata
        let metadata = serde_json::to_value(canvas_assignment).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();

        let now = Utc::now();

        Self {
            id,
            title,
            description,
            created_at: now,
            updated_at: now,
            course_id,
            due_date,
            unlock_date,
            lock_date,
            points_possible,
            grading_type,
            submission_types,
            status,
            is_published,
            group_category_id,
            assignment_group_id,
            peer_reviews,
            automatic_peer_reviews,
            peer_review_count,
            canvas_id: Some(canvas_id),
            discourse_id: None,
            quiz_id,
            discussion_topic_id,
            position,
            source_system: Some("canvas".to_string()),
            metadata,
        }
    }

    /// Create an Assignment from a Discourse topic JSON
    pub fn from_discourse_topic(discourse_topic: &serde_json::Value) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let discourse_id = discourse_topic["id"].as_str()
            .or_else(|| discourse_topic["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let title = discourse_topic["title"].as_str().unwrap_or("").to_string();
        let description = discourse_topic["raw"].as_str().map(|s| s.to_string());

        // Parse dates
        let created_at = discourse_topic["created_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let updated_at = discourse_topic["updated_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        // Parse category ID (as course ID)
        let course_id = discourse_topic["category_id"].as_str()
            .or_else(|| discourse_topic["category_id"].as_i64().map(|id| id.to_string()));

        // Convert the discourse_topic to a HashMap for metadata
        let metadata = serde_json::to_value(discourse_topic).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();

        Self {
            id,
            title,
            description,
            created_at,
            updated_at,
            course_id,
            due_date: None,
            unlock_date: None,
            lock_date: None,
            points_possible: None,
            grading_type: GradingType::NotGraded,
            submission_types: vec![SubmissionType::DiscussionTopic],
            status: AssignmentStatus::Published,
            is_published: true,
            group_category_id: None,
            assignment_group_id: None,
            peer_reviews: false,
            automatic_peer_reviews: false,
            peer_review_count: None,
            canvas_id: None,
            discourse_id: Some(discourse_id),
            quiz_id: None,
            discussion_topic_id: Some(discourse_id),
            position: None,
            source_system: Some("discourse".to_string()),
            metadata,
        }
    }

    /// Convert Assignment to Canvas assignment JSON
    pub fn to_canvas_assignment(&self) -> serde_json::Value {
        let submission_types: Vec<String> = self.submission_types.iter()
            .map(|st| st.to_string())
            .collect();

        serde_json::json!({
            "id": self.canvas_id,
            "name": self.title,
            "description": self.description,
            "due_at": self.due_date.map(|dt| dt.to_rfc3339()),
            "unlock_at": self.unlock_date.map(|dt| dt.to_rfc3339()),
            "lock_at": self.lock_date.map(|dt| dt.to_rfc3339()),
            "course_id": self.course_id,
            "points_possible": self.points_possible,
            "grading_type": self.grading_type.to_string(),
            "submission_types": submission_types,
            "workflow_state": self.status.to_string(),
            "published": self.is_published,
            "group_category_id": self.group_category_id,
            "assignment_group_id": self.assignment_group_id,
            "peer_reviews": self.peer_reviews,
            "automatic_peer_reviews": self.automatic_peer_reviews,
            "peer_review_count": self.peer_review_count,
            "position": self.position,
            "quiz_id": self.quiz_id,
            "discussion_topic": self.discussion_topic_id.as_ref().map(|id| {
                serde_json::json!({
                    "id": id
                })
            })
        })
    }

    /// Convert Assignment to Discourse topic JSON
    pub fn to_discourse_topic(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "title": self.title,
            "raw": self.description,
            "created_at": self.created_at.to_rfc3339(),
            "updated_at": self.updated_at.to_rfc3339(),
            "category_id": self.course_id
        })
    }

    /// Publish the assignment
    pub fn publish(&mut self) {
        self.status = AssignmentStatus::Published;
        self.is_published = true;
        self.updated_at = Utc::now();
    }

    /// Unpublish the assignment
    pub fn unpublish(&mut self) {
        self.status = AssignmentStatus::Unpublished;
        self.is_published = false;
        self.updated_at = Utc::now();
    }

    /// Delete the assignment
    pub fn delete(&mut self) {
        self.status = AssignmentStatus::Deleted;
        self.is_published = false;
        self.updated_at = Utc::now();
    }

    /// Check if the assignment is a quiz
    pub fn is_quiz(&self) -> bool {
        self.quiz_id.is_some() || self.submission_types.contains(&SubmissionType::OnlineQuiz)
    }

    /// Check if the assignment is a discussion topic
    pub fn is_discussion_topic(&self) -> bool {
        self.discussion_topic_id.is_some() || self.submission_types.contains(&SubmissionType::DiscussionTopic)
    }

    /// Check if the assignment allows submissions
    pub fn allows_submissions(&self) -> bool {
        !self.submission_types.is_empty() &&
        !self.submission_types.contains(&SubmissionType::None) &&
        !self.submission_types.contains(&SubmissionType::OnPaper) &&
        !self.submission_types.contains(&SubmissionType::Attendance)
    }

    /// Check if the assignment is locked for a specific date
    pub fn is_locked_for_date(&self, date: &DateTime<Utc>) -> bool {
        if let Some(lock_date) = self.lock_date {
            return *date >= lock_date;
        }
        false
    }

    /// Check if the assignment is unlocked for a specific date
    pub fn is_unlocked_for_date(&self, date: &DateTime<Utc>) -> bool {
        if let Some(unlock_date) = self.unlock_date {
            return *date >= unlock_date;
        }
        true
    }

    /// Check if the assignment is available for a specific date
    pub fn is_available_for_date(&self, date: &DateTime<Utc>) -> bool {
        self.is_unlocked_for_date(date) && !self.is_locked_for_date(date)
    }

    /// Check if the assignment is overdue for a specific date
    pub fn is_overdue_for_date(&self, date: &DateTime<Utc>) -> bool {
        if let Some(due_date) = self.due_date {
            return *date > due_date;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_assignment() {
        let assignment = Assignment::new(
            None,
            "Test Assignment".to_string(),
        );

        assert_eq!(assignment.title, "Test Assignment");
        assert_eq!(assignment.status, AssignmentStatus::Unpublished);
        assert_eq!(assignment.is_published, false);
        assert_eq!(assignment.submission_types, vec![SubmissionType::None]);
        assert_eq!(assignment.grading_type, GradingType::Points);
        assert_eq!(assignment.points_possible, Some(0.0));
    }

    #[test]
    fn test_from_canvas_assignment() {
        let canvas_json = serde_json::json!({
            "id": "12345",
            "name": "Canvas Assignment",
            "description": "A test assignment from Canvas",
            "due_at": "2023-12-31T23:59:59Z",
            "unlock_at": "2023-12-01T00:00:00Z",
            "lock_at": "2024-01-07T23:59:59Z",
            "course_id": "67890",
            "points_possible": 100.0,
            "grading_type": "points",
            "submission_types": ["online_text_entry", "online_upload"],
            "workflow_state": "published",
            "published": true,
            "group_category_id": "54321",
            "assignment_group_id": "98765",
            "peer_reviews": true,
            "automatic_peer_reviews": true,
            "peer_review_count": 3,
            "position": 2
        });

        let assignment = Assignment::from_canvas_assignment(&canvas_json);

        assert_eq!(assignment.title, "Canvas Assignment");
        assert_eq!(assignment.description, Some("A test assignment from Canvas".to_string()));
        assert_eq!(assignment.course_id, Some("67890".to_string()));
        assert_eq!(assignment.points_possible, Some(100.0));
        assert_eq!(assignment.grading_type, GradingType::Points);
        assert_eq!(assignment.submission_types.len(), 2);
        assert!(assignment.submission_types.contains(&SubmissionType::OnlineTextEntry));
        assert!(assignment.submission_types.contains(&SubmissionType::OnlineUpload));
        assert_eq!(assignment.status, AssignmentStatus::Published);
        assert_eq!(assignment.is_published, true);
        assert_eq!(assignment.group_category_id, Some("54321".to_string()));
        assert_eq!(assignment.assignment_group_id, Some("98765".to_string()));
        assert_eq!(assignment.peer_reviews, true);
        assert_eq!(assignment.automatic_peer_reviews, true);
        assert_eq!(assignment.peer_review_count, Some(3));
        assert_eq!(assignment.position, Some(2));
        assert_eq!(assignment.canvas_id, Some("12345".to_string()));
        assert_eq!(assignment.source_system, Some("canvas".to_string()));
    }

    #[test]
    fn test_from_discourse_topic() {
        let discourse_json = serde_json::json!({
            "id": "67890",
            "title": "Discourse Topic",
            "raw": "A test topic from Discourse",
            "created_at": "2023-12-01T00:00:00Z",
            "updated_at": "2023-12-01T12:34:56Z",
            "category_id": "54321"
        });

        let assignment = Assignment::from_discourse_topic(&discourse_json);

        assert_eq!(assignment.title, "Discourse Topic");
        assert_eq!(assignment.description, Some("A test topic from Discourse".to_string()));
        assert_eq!(assignment.course_id, Some("54321".to_string()));
        assert_eq!(assignment.submission_types, vec![SubmissionType::DiscussionTopic]);
        assert_eq!(assignment.status, AssignmentStatus::Published);
        assert_eq!(assignment.is_published, true);
        assert_eq!(assignment.discourse_id, Some("67890".to_string()));
        assert_eq!(assignment.discussion_topic_id, Some("67890".to_string()));
        assert_eq!(assignment.source_system, Some("discourse".to_string()));
    }

    #[test]
    fn test_to_canvas_assignment() {
        let mut assignment = Assignment::new(
            Some("abcd1234".to_string()),
            "Test Canvas Assignment".to_string(),
        );

        assignment.canvas_id = Some("54321".to_string());
        assignment.description = Some("A test assignment for Canvas".to_string());
        assignment.course_id = Some("67890".to_string());
        assignment.due_date = Some(Utc::now());
        assignment.points_possible = Some(50.0);
        assignment.grading_type = GradingType::Percentage;
        assignment.submission_types = vec![SubmissionType::OnlineTextEntry, SubmissionType::OnlineUpload];
        assignment.publish();

        let canvas_assignment = assignment.to_canvas_assignment();

        assert_eq!(canvas_assignment["id"], "54321");
        assert_eq!(canvas_assignment["name"], "Test Canvas Assignment");
        assert_eq!(canvas_assignment["description"], "A test assignment for Canvas");
        assert_eq!(canvas_assignment["course_id"], "67890");
        assert_eq!(canvas_assignment["points_possible"], 50.0);
        assert_eq!(canvas_assignment["grading_type"], "percent");
        assert_eq!(canvas_assignment["workflow_state"], "published");
        assert_eq!(canvas_assignment["published"], true);

        let submission_types = canvas_assignment["submission_types"].as_array().unwrap();
        assert_eq!(submission_types.len(), 2);
        assert!(submission_types.contains(&serde_json::json!("online_text_entry")));
        assert!(submission_types.contains(&serde_json::json!("online_upload")));
    }

    #[test]
    fn test_to_discourse_topic() {
        let mut assignment = Assignment::new(
            Some("efgh5678".to_string()),
            "Test Discourse Topic".to_string(),
        );

        assignment.discourse_id = Some("98765".to_string());
        assignment.description = Some("A test topic for Discourse".to_string());
        assignment.course_id = Some("54321".to_string());

        let discourse_topic = assignment.to_discourse_topic();

        assert_eq!(discourse_topic["id"], "98765");
        assert_eq!(discourse_topic["title"], "Test Discourse Topic");
        assert_eq!(discourse_topic["raw"], "A test topic for Discourse");
        assert_eq!(discourse_topic["category_id"], "54321");
    }

    #[test]
    fn test_publish_unpublish_delete() {
        let mut assignment = Assignment::new(
            None,
            "Test Assignment".to_string(),
        );

        // Initially unpublished
        assert_eq!(assignment.status, AssignmentStatus::Unpublished);
        assert_eq!(assignment.is_published, false);

        // Publish
        assignment.publish();
        assert_eq!(assignment.status, AssignmentStatus::Published);
        assert_eq!(assignment.is_published, true);

        // Unpublish
        assignment.unpublish();
        assert_eq!(assignment.status, AssignmentStatus::Unpublished);
        assert_eq!(assignment.is_published, false);

        // Delete
        assignment.delete();
        assert_eq!(assignment.status, AssignmentStatus::Deleted);
        assert_eq!(assignment.is_published, false);
    }

    #[test]
    fn test_is_quiz_and_discussion_topic() {
        let mut assignment = Assignment::new(
            None,
            "Test Assignment".to_string(),
        );

        // Initially not a quiz or discussion topic
        assert_eq!(assignment.is_quiz(), false);
        assert_eq!(assignment.is_discussion_topic(), false);

        // Set as quiz by submission type
        assignment.submission_types = vec![SubmissionType::OnlineQuiz];
        assert_eq!(assignment.is_quiz(), true);
        assert_eq!(assignment.is_discussion_topic(), false);

        // Set as discussion topic by submission type
        assignment.submission_types = vec![SubmissionType::DiscussionTopic];
        assert_eq!(assignment.is_quiz(), false);
        assert_eq!(assignment.is_discussion_topic(), true);

        // Set as quiz by ID
        assignment.submission_types = vec![SubmissionType::OnlineTextEntry];
        assignment.quiz_id = Some("12345".to_string());
        assert_eq!(assignment.is_quiz(), true);
        assert_eq!(assignment.is_discussion_topic(), false);

        // Set as discussion topic by ID
        assignment.quiz_id = None;
        assignment.discussion_topic_id = Some("67890".to_string());
        assert_eq!(assignment.is_quiz(), false);
        assert_eq!(assignment.is_discussion_topic(), true);
    }

    #[test]
    fn test_allows_submissions() {
        let mut assignment = Assignment::new(
            None,
            "Test Assignment".to_string(),
        );

        // Initially doesn't allow submissions
        assert_eq!(assignment.allows_submissions(), false);

        // Set as online text entry
        assignment.submission_types = vec![SubmissionType::OnlineTextEntry];
        assert_eq!(assignment.allows_submissions(), true);

        // Set as on paper
        assignment.submission_types = vec![SubmissionType::OnPaper];
        assert_eq!(assignment.allows_submissions(), false);

        // Set as attendance
        assignment.submission_types = vec![SubmissionType::Attendance];
        assert_eq!(assignment.allows_submissions(), false);

        // Set as multiple types including online
        assignment.submission_types = vec![SubmissionType::OnlineTextEntry, SubmissionType::OnlineUpload];
        assert_eq!(assignment.allows_submissions(), true);

        // Set as multiple types including non-submittable
        assignment.submission_types = vec![SubmissionType::OnlineTextEntry, SubmissionType::None];
        assert_eq!(assignment.allows_submissions(), false);
    }

    #[test]
    fn test_date_checks() {
        let mut assignment = Assignment::new(
            None,
            "Test Assignment".to_string(),
        );

        // Set dates
        let now = Utc::now();
        let yesterday = now - chrono::Duration::days(1);
        let tomorrow = now + chrono::Duration::days(1);
        let next_week = now + chrono::Duration::days(7);

        assignment.due_date = Some(tomorrow);
        assignment.unlock_date = Some(yesterday);
        assignment.lock_date = Some(next_week);

        // Check availability
        assert_eq!(assignment.is_unlocked_for_date(&now), true);
        assert_eq!(assignment.is_locked_for_date(&now), false);
        assert_eq!(assignment.is_available_for_date(&now), true);
        assert_eq!(assignment.is_overdue_for_date(&now), false);

        // Check before unlock date
        let before_unlock = yesterday - chrono::Duration::hours(1);
        assert_eq!(assignment.is_unlocked_for_date(&before_unlock), false);
        assert_eq!(assignment.is_available_for_date(&before_unlock), false);

        // Check after lock date
        let after_lock = next_week + chrono::Duration::hours(1);
        assert_eq!(assignment.is_locked_for_date(&after_lock), true);
        assert_eq!(assignment.is_available_for_date(&after_lock), false);

        // Check after due date
        let after_due = tomorrow + chrono::Duration::hours(1);
        assert_eq!(assignment.is_overdue_for_date(&after_due), true);
    }
}