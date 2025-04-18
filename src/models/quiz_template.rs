use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::quiz::{Question, AnswerType, StudyMode, QuizVisibility};

/// Quiz template category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateCategory {
    Education,
    Business,
    Science,
    Technology,
    Language,
    Arts,
    Health,
    Custom,
}

/// Quiz template model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizTemplate {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Option<Uuid>,
    pub category: TemplateCategory,
    pub tags: Vec<String>,
    pub question_templates: Vec<QuestionTemplate>,
    pub default_study_mode: StudyMode,
    pub default_visibility: QuizVisibility,
    pub is_public: bool,
    pub usage_count: i32,
    pub rating: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Question template model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionTemplate {
    pub id: Uuid,
    pub template_id: Uuid,
    pub text: String,
    pub description: Option<String>,
    pub answer_type: AnswerType,
    pub placeholder_text: Option<String>,
    pub example_answers: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl QuizTemplate {
    pub fn new(
        title: String,
        category: TemplateCategory,
        default_study_mode: StudyMode,
        default_visibility: QuizVisibility,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            author_id: None,
            category,
            tags: Vec::new(),
            question_templates: Vec::new(),
            default_study_mode,
            default_visibility,
            is_public: false,
            usage_count: 0,
            rating: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_author(mut self, author_id: Uuid) -> Self {
        self.author_id = Some(author_id);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn add_question_template(&mut self, question_template: QuestionTemplate) {
        self.question_templates.push(question_template);
        self.updated_at = Utc::now();
    }
    
    pub fn make_public(&mut self) {
        self.is_public = true;
        self.updated_at = Utc::now();
    }
    
    pub fn make_private(&mut self) {
        self.is_public = false;
        self.updated_at = Utc::now();
    }
    
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
        self.updated_at = Utc::now();
    }
    
    pub fn update_rating(&mut self, new_rating: f32) {
        if let Some(current_rating) = self.rating {
            // Simple average for now
            self.rating = Some((current_rating + new_rating) / 2.0);
        } else {
            self.rating = Some(new_rating);
        }
        self.updated_at = Utc::now();
    }
}

impl QuestionTemplate {
    pub fn new(
        template_id: Uuid,
        text: String,
        answer_type: AnswerType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            template_id,
            text,
            description: None,
            answer_type,
            placeholder_text: None,
            example_answers: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder_text = Some(placeholder);
        self
    }
    
    pub fn with_example_answers(mut self, examples: Vec<String>) -> Self {
        self.example_answers = examples;
        self
    }
}
