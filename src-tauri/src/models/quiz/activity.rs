use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Types of quiz activities that can be tracked
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    QuizStarted,
    QuizCompleted,
    QuizAbandoned,
    QuestionAnswered,
    FlashcardViewed,
    FlashcardFlipped,
    FlashcardRated,
    StudySessionStarted,
    StudySessionEnded,
    ContentViewed,
    Custom(String),
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityType::QuizStarted => write!(f, "quiz_started"),
            ActivityType::QuizCompleted => write!(f, "quiz_completed"),
            ActivityType::QuizAbandoned => write!(f, "quiz_abandoned"),
            ActivityType::QuestionAnswered => write!(f, "question_answered"),
            ActivityType::FlashcardViewed => write!(f, "flashcard_viewed"),
            ActivityType::FlashcardFlipped => write!(f, "flashcard_flipped"),
            ActivityType::FlashcardRated => write!(f, "flashcard_rated"),
            ActivityType::StudySessionStarted => write!(f, "study_session_started"),
            ActivityType::StudySessionEnded => write!(f, "study_session_ended"),
            ActivityType::ContentViewed => write!(f, "content_viewed"),
            ActivityType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Quiz activity model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizActivity {
    pub id: String,
    pub user_id: String,
    pub quiz_id: Option<String>,
    pub question_id: Option<String>,
    pub attempt_id: Option<String>,
    pub activity_type: String, // Stored as string in DB
    pub data: Option<String>,  // JSON data
    pub duration_ms: Option<i64>,
    pub timestamp: String,
    pub created_at: String,
    pub synced: bool,
}

/// Create quiz activity request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuizActivityRequest {
    pub user_id: String,
    pub quiz_id: Option<String>,
    pub question_id: Option<String>,
    pub attempt_id: Option<String>,
    pub activity_type: ActivityType,
    pub data: Option<serde_json::Value>,
    pub duration_ms: Option<i64>,
}

/// Quiz activity summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizActivitySummary {
    pub user_id: String,
    pub quiz_id: Option<String>,
    pub total_activities: i64,
    pub total_duration_ms: i64,
    pub activity_counts: serde_json::Value, // Map of activity types to counts
    pub first_activity_at: String,
    pub last_activity_at: String,
}

/// Quiz activity stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizActivityStats {
    pub total_quizzes_started: i64,
    pub total_quizzes_completed: i64,
    pub total_questions_answered: i64,
    pub total_study_time_ms: i64,
    pub average_quiz_duration_ms: Option<i64>,
    pub average_question_time_ms: Option<i64>,
    pub activity_by_day: serde_json::Value, // Map of days to activity counts
}

impl QuizActivity {
    /// Create a new quiz activity
    pub fn new(
        user_id: String,
        quiz_id: Option<String>,
        question_id: Option<String>,
        attempt_id: Option<String>,
        activity_type: ActivityType,
        data: Option<serde_json::Value>,
        duration_ms: Option<i64>,
    ) -> Self {
        let now = Utc::now().to_rfc3339();
        
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            quiz_id,
            question_id,
            attempt_id,
            activity_type: activity_type.to_string(),
            data: data.map(|d| serde_json::to_string(&d).unwrap_or_default()),
            duration_ms,
            timestamp: now.clone(),
            created_at: now,
            synced: false,
        }
    }
}
