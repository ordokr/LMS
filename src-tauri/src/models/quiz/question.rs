use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Question {
    pub id: String,
    pub quiz_id: String,
    pub question_text: String,
    pub content: Option<String>, // JSON content with text, rich_text, and media
    pub question_type: QuestionType,
    pub points: i64,
    pub position: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Matching,
    Essay,
}

// Implement FromRow for QuestionType
impl<'r> sqlx::decode::Decode<'r, sqlx::Sqlite> for QuestionType {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as sqlx::decode::Decode<sqlx::Sqlite>>::decode(value)?;
        match value {
            "multiple_choice" => Ok(QuestionType::MultipleChoice),
            "true_false" => Ok(QuestionType::TrueFalse),
            "short_answer" => Ok(QuestionType::ShortAnswer),
            "matching" => Ok(QuestionType::Matching),
            "essay" => Ok(QuestionType::Essay),
            _ => Err(format!("Unknown question type: {}", value).into()),
        }
    }
}

// Implement Type for QuestionType
impl sqlx::Type<sqlx::Sqlite> for QuestionType {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <&str as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

// Implement Encode for QuestionType
impl<'q> sqlx::encode::Encode<'q, sqlx::Sqlite> for QuestionType {
    fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        let s = match self {
            QuestionType::MultipleChoice => "multiple_choice",
            QuestionType::TrueFalse => "true_false",
            QuestionType::ShortAnswer => "short_answer",
            QuestionType::Matching => "matching",
            QuestionType::Essay => "essay",
        };
        <&str as sqlx::encode::Encode<sqlx::Sqlite>>::encode_by_ref(&s, args)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestionRequest {
    pub quiz_id: String,
    pub question_text: String,
    pub content: Option<Value>, // JSON content with text, rich_text, and media
    pub question_type: QuestionType,
    pub points: Option<i64>,
    pub position: Option<i64>,
    pub answer_options: Option<Vec<super::CreateAnswerOptionRequest>>,
}

impl CreateQuestionRequest {
    pub fn new(quiz_id: String, question_text: String, question_type: QuestionType) -> Self {
        Self {
            quiz_id,
            question_text,
            content: None,
            question_type,
            points: Some(1),
            position: None,
            answer_options: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestionRequest {
    pub question_text: Option<String>,
    pub content: Option<Value>, // JSON content with text, rich_text, and media
    pub question_type: Option<QuestionType>,
    pub points: Option<i64>,
    pub position: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionWithAnswers {
    pub question: Question,
    pub answer_options: Vec<super::AnswerOption>,
}
