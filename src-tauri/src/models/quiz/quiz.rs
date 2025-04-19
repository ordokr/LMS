use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub course_id: Option<String>,
    pub author_id: String,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: bool,
    pub show_results: bool,
    pub visibility: Option<QuizVisibility>,
    pub tags: Option<String>, // JSON array of tags
    pub study_mode: Option<StudyMode>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QuizVisibility {
    Private,
    Public,
    Course,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StudyMode {
    MultipleChoice,
    Flashcard,
    Adaptive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizRequest {
    pub title: String,
    pub description: Option<String>,
    pub course_id: Option<String>,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: Option<bool>,
    pub show_results: Option<bool>,
    pub visibility: Option<QuizVisibility>,
    pub tags: Option<Vec<String>>,
    pub study_mode: Option<StudyMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuizRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub course_id: Option<String>,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: Option<bool>,
    pub show_results: Option<bool>,
    pub visibility: Option<QuizVisibility>,
    pub tags: Option<Vec<String>>,
    pub study_mode: Option<StudyMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizSummary {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub question_count: i64,
    pub time_limit: Option<i64>,
    pub author_name: String,
    pub created_at: String,
    pub visibility: Option<QuizVisibility>,
    pub tags: Option<Vec<String>>,
    pub study_mode: Option<StudyMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizWithQuestions {
    pub quiz: Quiz,
    pub questions: Vec<super::QuestionWithAnswers>,
    pub settings: Option<super::QuizSettings>,
}

impl Quiz {
    pub fn new(title: String, author_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: None,
            course_id: None,
            author_id,
            time_limit: None,
            passing_score: None,
            shuffle_questions: false,
            show_results: true,
            visibility: Some(QuizVisibility::Private),
            tags: None,
            study_mode: Some(StudyMode::MultipleChoice),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            deleted_at: None,
        }
    }
}
