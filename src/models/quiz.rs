use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::models::quiz_question_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub questions: Vec<Question>,
    pub study_mode: StudyMode,
    pub visibility: QuizVisibility,
    pub settings: QuizSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub content: QuestionContent,
    pub answer_type: AnswerType,
    pub choices: Vec<Choice>,
    pub correct_answer: Answer,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionContent {
    pub text: String,
    pub rich_text: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
    pub drag_drop_content: Option<DragDropContent>,
    pub hotspot_content: Option<HotspotContent>,
    pub drawing_content: Option<DrawingContent>,
    pub code_execution_content: Option<CodeExecutionContent>,
    pub math_equation_content: Option<MathEquationContent>,
    pub timeline_content: Option<TimelineContent>,
    pub diagram_labeling_content: Option<DiagramLabelingContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub id: Uuid,
    pub text: String,
    pub rich_text: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Answer {
    Choice(Uuid),
    MultipleChoice(Vec<Uuid>),
    Text(String),
    DragDrop(HashMap<String, String>),
    Hotspot(Vec<String>),
    Ordering(Vec<Uuid>),
    Drawing(String),
    CodeExecution(CodeExecutionAnswer),
    MathEquation(String),
    Timeline(Vec<TimelineEvent>),
    DiagramLabeling(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnswerType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Essay,
    Matching,
    Ordering,
    DragDrop,
    Hotspot,
    Drawing,
    CodeExecution,
    MathEquation,
    Timeline,
    DiagramLabeling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudyMode {
    Flashcards,
    MultipleChoice,
    Written,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizVisibility {
    Public,
    Private,
    Unlisted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSettings {
    pub shuffle_questions: bool,
    pub shuffle_choices: bool,
    pub show_feedback: bool,
    pub time_limit: Option<i32>,
    pub passing_score: Option<f32>,
    pub max_attempts: Option<i32>,
}

// Drag and Drop Question Types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragDropContent {
    pub items: Vec<DragDropItem>,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragDropItem {
    pub id: String,
    pub text: String,
    pub image_url: Option<String>,
}

// Hotspot Question Types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotspotContent {
    pub image_url: String,
    pub hotspots: Vec<Hotspot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub id: String,
    pub shape: HotspotShape,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HotspotShape {
    Rectangle { x: f32, y: f32, width: f32, height: f32 },
    Circle { center_x: f32, center_y: f32, radius: f32 },
    Polygon { points: Vec<Point> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Quiz Session Types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSession {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub answers: HashMap<Uuid, Answer>,
    pub score: Option<f32>,
    pub time_spent: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub score: Option<f32>,
    pub time_spent: i32,
    pub answers: Vec<QuestionAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer {
    pub question_id: Uuid,
    pub answer: Answer,
    pub is_correct: Option<bool>,
    pub time_spent: i32,
}

// Flashcard Data

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashcardData {
    pub question_id: Uuid,
    pub user_id: Uuid,
    pub ease_factor: f32,
    pub interval: i32,
    pub repetitions: i32,
    pub due_date: DateTime<Utc>,
    pub last_reviewed: DateTime<Utc>,
}

impl Quiz {
    pub fn new(title: String, author_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            author_id,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            questions: Vec::new(),
            study_mode: StudyMode::MultipleChoice,
            visibility: QuizVisibility::Private,
            settings: QuizSettings {
                shuffle_questions: false,
                shuffle_choices: false,
                show_feedback: true,
                time_limit: None,
                passing_score: None,
                max_attempts: None,
            },
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
        self.updated_at = Some(Utc::now());
    }
}

impl Question {
    pub fn new(quiz_id: Uuid, content: QuestionContent, answer_type: AnswerType) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            content,
            answer_type,
            choices: Vec::new(),
            correct_answer: Answer::Text(String::new()),
            explanation: None,
        }
    }

    pub fn add_choice(&mut self, text: String) -> Uuid {
        let choice = Choice {
            id: Uuid::new_v4(),
            text,
            rich_text: None,
            image_url: None,
        };

        let choice_id = choice.id;
        self.choices.push(choice);

        choice_id
    }

    pub fn set_correct_answer(&mut self, answer: Answer) {
        self.correct_answer = answer;
    }
}
