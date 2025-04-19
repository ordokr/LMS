use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Grade model - ported from Canvas
/// Represents a grade for an assignment or course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    // Core fields
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub assignment_id: Option<Uuid>,
    pub course_id: Uuid,
    pub score: Option<f64>,
    pub grade: Option<String>,
    pub graded_at: Option<DateTime<Utc>>,
    pub grader_id: Option<Uuid>,
    pub grade_matches_current_submission: bool,
    pub grade_state: GradeState,
    pub excused: bool,
    pub late: bool,
    pub missing: bool,
    pub late_policy_status: Option<String>,
    pub points_deducted: Option<f64>,
    pub seconds_late: Option<i64>,

    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub canvas_assignment_id: Option<String>,
    pub canvas_course_id: Option<String>,
    pub canvas_user_id: Option<String>,
    pub canvas_grader_id: Option<String>,
    pub workflow_state: Option<String>,
    pub posted_at: Option<DateTime<Utc>>,
    pub hidden: bool,
    pub muted: bool,

    // Common fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub comments: Vec<GradeComment>,
    pub rubric_assessments: Vec<RubricAssessment>,
}

/// Grade state enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GradeState {
    Submitted,
    Graded,
    Pending,
    NotSubmitted,
    Excused,
}

/// Grade comment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeComment {
    pub id: Option<Uuid>,
    pub grade_id: Option<Uuid>,
    pub author_id: Uuid,
    pub comment: String,
    pub comment_type: CommentType,
    pub media_comment_id: Option<String>,
    pub media_comment_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canvas_id: Option<String>,
}

/// Comment type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommentType {
    Comment,
    Draft,
    Media,
    Private,
}

/// Rubric assessment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricAssessment {
    pub id: Option<Uuid>,
    pub grade_id: Option<Uuid>,
    pub rubric_id: Uuid,
    pub assessor_id: Option<Uuid>,
    pub artifact_id: Option<Uuid>,
    pub artifact_type: Option<String>,
    pub assessment_type: Option<String>,
    pub data: Vec<RubricAssessmentData>,
    pub score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canvas_id: Option<String>,
}

/// Rubric assessment data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricAssessmentData {
    pub id: Option<Uuid>,
    pub rubric_assessment_id: Option<Uuid>,
    pub criterion_id: String,
    pub rating_id: Option<String>,
    pub points: Option<f64>,
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Grade {
    /// Create a new grade
    pub fn new(user_id: Uuid, course_id: Uuid, assignment_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            user_id,
            assignment_id,
            course_id,
            score: None,
            grade: None,
            graded_at: None,
            grader_id: None,
            grade_matches_current_submission: true,
            grade_state: GradeState::NotSubmitted,
            excused: false,
            late: false,
            missing: false,
            late_policy_status: None,
            points_deducted: None,
            seconds_late: None,
            canvas_id: None,
            canvas_assignment_id: None,
            canvas_course_id: None,
            canvas_user_id: None,
            canvas_grader_id: None,
            workflow_state: None,
            posted_at: None,
            hidden: false,
            muted: false,
            created_at: now,
            updated_at: now,
            comments: Vec::new(),
            rubric_assessments: Vec::new(),
        }
    }

    /// Create a grade from Canvas data
    pub fn from_canvas(
        user_id: Uuid,
        course_id: Uuid,
        assignment_id: Option<Uuid>,
        canvas_id: String,
        canvas_assignment_id: Option<String>,
        canvas_course_id: String,
        canvas_user_id: String,
        score: Option<f64>,
        grade: Option<String>,
        workflow_state: Option<String>,
    ) -> Self {
        let mut new_grade = Self::new(user_id, course_id, assignment_id);
        new_grade.canvas_id = Some(canvas_id);
        new_grade.canvas_assignment_id = canvas_assignment_id;
        new_grade.canvas_course_id = Some(canvas_course_id);
        new_grade.canvas_user_id = Some(canvas_user_id);
        new_grade.score = score;
        new_grade.grade = grade;
        new_grade.workflow_state = workflow_state;

        // Set grade state based on workflow state
        if let Some(state) = &new_grade.workflow_state {
            new_grade.grade_state = match state.as_str() {
                "graded" => GradeState::Graded,
                "submitted" => GradeState::Submitted,
                "pending_review" => GradeState::Pending,
                "unsubmitted" => GradeState::NotSubmitted,
                _ => GradeState::NotSubmitted,
            };
        }

        new_grade
    }

    /// Add a comment to the grade
    pub fn add_comment(&mut self, author_id: Uuid, comment: String, comment_type: CommentType) -> Uuid {
        let now = Utc::now();
        let comment_id = Uuid::new_v4();

        self.comments.push(GradeComment {
            id: Some(comment_id),
            grade_id: self.id,
            author_id,
            comment,
            comment_type,
            media_comment_id: None,
            media_comment_type: None,
            created_at: now,
            updated_at: now,
            canvas_id: None,
        });

        self.updated_at = now;
        comment_id
    }

    /// Add a rubric assessment to the grade
    pub fn add_rubric_assessment(&mut self, rubric_id: Uuid, assessor_id: Option<Uuid>, score: Option<f64>) -> Uuid {
        let now = Utc::now();
        let assessment_id = Uuid::new_v4();

        self.rubric_assessments.push(RubricAssessment {
            id: Some(assessment_id),
            grade_id: self.id,
            rubric_id,
            assessor_id,
            artifact_id: None,
            artifact_type: None,
            assessment_type: None,
            data: Vec::new(),
            score,
            created_at: now,
            updated_at: now,
            canvas_id: None,
        });

        self.updated_at = now;
        assessment_id
    }

    /// Update grade with new score and grade
    pub fn update_grade(&mut self, score: Option<f64>, grade: Option<String>, grader_id: Option<Uuid>) {
        let now = Utc::now();
        self.score = score;
        self.grade = grade;
        self.grader_id = grader_id;
        self.graded_at = Some(now);
        self.grade_state = GradeState::Graded;
        self.updated_at = now;
    }

    /// Excuse the grade
    pub fn excuse(&mut self) {
        let now = Utc::now();
        self.excused = true;
        self.grade_state = GradeState::Excused;
        self.updated_at = now;
    }

    /// Mark the grade as late
    pub fn mark_late(&mut self, seconds_late: i64, points_deducted: Option<f64>) {
        let now = Utc::now();
        self.late = true;
        self.seconds_late = Some(seconds_late);
        self.points_deducted = points_deducted;
        self.late_policy_status = Some("late".to_string());
        self.updated_at = now;
    }

    /// Mark the grade as missing
    pub fn mark_missing(&mut self) {
        let now = Utc::now();
        self.missing = true;
        self.grade_state = GradeState::NotSubmitted;
        self.updated_at = now;
    }
}
