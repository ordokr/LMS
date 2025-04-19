use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AnswerOption {
    pub id: String,
    pub question_id: String,
    pub option_text: String,
    pub content: Option<String>, // JSON content with text, rich_text, and media
    pub is_correct: bool,
    pub position: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswerOptionRequest {
    pub question_id: String,
    pub option_text: String,
    pub content: Option<Value>, // JSON content with text, rich_text, and media
    pub is_correct: bool,
    pub position: Option<i64>,
}

impl CreateAnswerOptionRequest {
    pub fn new(question_id: String, option_text: String, is_correct: bool) -> Self {
        Self {
            question_id,
            option_text,
            content: None,
            is_correct,
            position: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAnswerOptionRequest {
    pub option_text: Option<String>,
    pub content: Option<Value>, // JSON content with text, rich_text, and media
    pub is_correct: Option<bool>,
    pub position: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserAnswer {
    pub id: String,
    pub attempt_id: String,
    pub question_id: String,
    pub answer_option_id: Option<String>,
    pub text_answer: Option<String>,
    pub content: Option<String>, // JSON content with text, rich_text, and media
    pub is_correct: Option<bool>,
    pub points_awarded: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitAnswerRequest {
    pub attempt_id: String,
    pub question_id: String,
    pub answer_option_id: Option<String>,
    pub text_answer: Option<String>,
    pub content: Option<Value>, // JSON content with text, rich_text, and media
}
