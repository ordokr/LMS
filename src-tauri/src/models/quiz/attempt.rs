use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct QuizAttempt {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub score: Option<f64>,
    pub status: AttemptStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    InProgress,
    Completed,
    Abandoned,
}

// Implement FromRow for AttemptStatus
impl<'r> sqlx::decode::Decode<'r, sqlx::Sqlite> for AttemptStatus {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as sqlx::decode::Decode<sqlx::Sqlite>>::decode(value)?;
        match value {
            "in_progress" => Ok(AttemptStatus::InProgress),
            "completed" => Ok(AttemptStatus::Completed),
            "abandoned" => Ok(AttemptStatus::Abandoned),
            _ => Err(format!("Unknown attempt status: {}", value).into()),
        }
    }
}

// Implement Type for AttemptStatus
impl sqlx::Type<sqlx::Sqlite> for AttemptStatus {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <&str as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

// Implement Encode for AttemptStatus
impl<'q> sqlx::encode::Encode<'q, sqlx::Sqlite> for AttemptStatus {
    fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        let s = match self {
            AttemptStatus::InProgress => "in_progress",
            AttemptStatus::Completed => "completed",
            AttemptStatus::Abandoned => "abandoned",
        };
        <&str as sqlx::encode::Encode<sqlx::Sqlite>>::encode_by_ref(&s, args)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartAttemptRequest {
    pub quiz_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteAttemptRequest {
    pub attempt_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbandonAttemptRequest {
    pub attempt_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptResult {
    pub attempt: QuizAttempt,
    pub quiz: super::Quiz,
    pub total_questions: i64,
    pub correct_answers: i64,
    pub score_percentage: f64,
    pub passed: bool,
    pub time_taken: Option<i64>, // in seconds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptWithAnswers {
    pub attempt: QuizAttempt,
    pub answers: Vec<super::UserAnswer>,
}
