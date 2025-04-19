use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct QuizSettings {
    pub quiz_id: String, // Primary key
    pub allow_retakes: bool,
    pub max_attempts: Option<i64>,
    pub show_correct_answers: bool,
    pub show_correct_answers_after_completion: bool,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizSettingsRequest {
    pub quiz_id: String,
    pub allow_retakes: Option<bool>,
    pub max_attempts: Option<i64>,
    pub show_correct_answers: Option<bool>,
    pub show_correct_answers_after_completion: Option<bool>,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuizSettingsRequest {
    pub allow_retakes: Option<bool>,
    pub max_attempts: Option<i64>,
    pub show_correct_answers: Option<bool>,
    pub show_correct_answers_after_completion: Option<bool>,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: Option<bool>,
}
