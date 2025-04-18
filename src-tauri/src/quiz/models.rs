use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub questions: Vec<Question>,
    pub settings: QuizSettings,
    // Adding Quenti-specific fields
    pub author_id: Option<Uuid>,
    pub visibility: QuizVisibility,
    pub tags: Vec<String>,
    pub study_mode: StudyMode,
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
    pub rich_text: Option<String>, // Markdown or HTML content
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
}

impl QuestionContent {
    pub fn render(&self) -> String {
        // In a real implementation, this would render the content based on type
        // For now, just return the text
        self.rich_text.clone().unwrap_or_else(|| self.text.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnswerType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Essay,
    Matching,
    Ordering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub id: Uuid,
    pub text: String,
    pub rich_text: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Answer {
    Choice(Uuid),              // ID of the selected choice
    Text(String),              // Free text answer
    Matching(Vec<(Uuid, Uuid)>), // Pairs of matching items
    Ordering(Vec<Uuid>),       // Ordered list of item IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSettings {
    pub shuffle_questions: bool,
    pub time_limit: Option<i32>,  // in minutes
    pub allow_retries: bool,
    pub show_correct_answers: bool,
    pub passing_score: Option<f32>,
    pub study_mode: StudyMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuizVisibility {
    Public,
    Private,
    SharedWithUsers(Vec<Uuid>),
    Course(Uuid), // Associated with a specific course
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StudyMode {
    Flashcards,
    MultipleChoice,
    Written,
    Mixed,
}

// Spaced repetition data for flashcards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashcardData {
    pub question_id: Uuid,
    pub user_id: Uuid,
    pub ease_factor: f32,      // SM-2 algorithm ease factor
    pub interval: i32,         // Current interval in days
    pub repetitions: i32,      // Number of successful repetitions
    pub due_date: DateTime<Utc>, // Next review date
    pub last_reviewed: DateTime<Utc>,
}

// Quiz attempt/session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub score: Option<f32>,
    pub answers: Vec<QuestionAnswer>,
    pub time_spent: i32, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer {
    pub question_id: Uuid,
    pub answer: Answer,
    pub is_correct: Option<bool>,
    pub time_spent: i32, // in seconds
}

impl Quiz {
    pub fn new(title: String, author_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            questions: Vec::new(),
            settings: QuizSettings {
                shuffle_questions: false,
                time_limit: None,
                allow_retries: true,
                show_correct_answers: true,
                passing_score: None,
                study_mode: StudyMode::MultipleChoice,
            },
            author_id,
            visibility: QuizVisibility::Private,
            tags: Vec::new(),
            study_mode: StudyMode::MultipleChoice,
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
        self.updated_at = Utc::now();
    }

    pub fn remove_question(&mut self, question_id: Uuid) -> Option<Question> {
        let position = self.questions.iter().position(|q| q.id == question_id)?;
        let question = self.questions.remove(position);
        self.updated_at = Utc::now();
        Some(question)
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
        let id = choice.id;
        self.choices.push(choice);
        id
    }

    pub fn set_correct_answer(&mut self, answer: Answer) {
        self.correct_answer = answer;
    }

    pub fn check_answer(&self, answer: &Answer) -> bool {
        match (&self.correct_answer, answer) {
            (Answer::Choice(correct_id), Answer::Choice(answer_id)) => {
                correct_id == answer_id
            },
            (Answer::Text(correct_text), Answer::Text(answer_text)) => {
                // Simple case-insensitive comparison
                // In a real implementation, this would use more sophisticated matching
                correct_text.to_lowercase() == answer_text.to_lowercase()
            },
            (Answer::Matching(correct_pairs), Answer::Matching(answer_pairs)) => {
                if correct_pairs.len() != answer_pairs.len() {
                    return false;
                }

                // Check if all pairs match
                correct_pairs.iter().all(|correct_pair| {
                    answer_pairs.contains(correct_pair)
                })
            },
            (Answer::Ordering(correct_order), Answer::Ordering(answer_order)) => {
                if correct_order.len() != answer_order.len() {
                    return false;
                }

                // Check if the order matches
                correct_order == answer_order
            },
            _ => false, // Mismatched answer types
        }
    }
}
