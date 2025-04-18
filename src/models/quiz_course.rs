use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents the relationship between a quiz and a course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizCourseMapping {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub course_id: Uuid,
    pub module_id: Option<Uuid>,
    pub section_id: Option<Uuid>,
    pub position: i32,
    pub is_required: bool,
    pub passing_score: Option<f32>,
    pub due_date: Option<DateTime<Utc>>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub max_attempts: Option<i32>,
    pub time_limit: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz assignment status for a student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizAssignmentStatus {
    NotStarted,
    InProgress,
    Completed,
    Overdue,
}

/// Quiz assignment for a student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAssignment {
    pub id: Uuid,
    pub mapping_id: Uuid,
    pub student_id: Uuid,
    pub status: QuizAssignmentStatus,
    pub attempts: i32,
    pub best_score: Option<f32>,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl QuizCourseMapping {
    pub fn new(quiz_id: Uuid, course_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            course_id,
            module_id: None,
            section_id: None,
            position: 0,
            is_required: true,
            passing_score: Some(70.0),
            due_date: None,
            available_from: None,
            available_until: None,
            max_attempts: None,
            time_limit: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl QuizAssignment {
    pub fn new(mapping_id: Uuid, student_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            mapping_id,
            student_id,
            status: QuizAssignmentStatus::NotStarted,
            attempts: 0,
            best_score: None,
            last_attempt_at: None,
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
