use super::models::{Quiz, Question, Answer, AnswerType, StudyMode, QuizVisibility};
use super::storage::HybridQuizStore;
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

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

/// Template rating model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRating {
    pub id: Uuid,
    pub template_id: Uuid,
    pub user_id: Uuid,
    pub rating: f32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz template service
pub struct TemplateService {
    db_pool: SqlitePool,
    quiz_store: Arc<HybridQuizStore>,
}

impl TemplateService {
    pub fn new(db_pool: SqlitePool, quiz_store: Arc<HybridQuizStore>) -> Self {
        Self {
            db_pool,
            quiz_store,
        }
    }

    /// Create a new quiz template
    pub async fn create_template(
        &self,
        title: String,
        description: Option<String>,
        author_id: Option<Uuid>,
        category: TemplateCategory,
        tags: Vec<String>,
        default_study_mode: StudyMode,
        default_visibility: QuizVisibility,
        is_public: bool,
    ) -> Result<QuizTemplate, Box<dyn Error + Send + Sync>> {
        // Create a new template
        let template = QuizTemplate {
            id: Uuid::new_v4(),
            title,
            description,
            author_id,
            category,
            tags,
            question_templates: Vec::new(),
            default_study_mode,
            default_visibility,
            is_public,
            usage_count: 0,
            rating: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store the template
        self.store_template(&template).await?;

        Ok(template)
    }

    /// Create a template from an existing quiz
    pub async fn create_template_from_quiz(
        &self,
        quiz_id: Uuid,
        title: String,
        description: Option<String>,
        author_id: Option<Uuid>,
        category: TemplateCategory,
        tags: Vec<String>,
        is_public: bool,
    ) -> Result<QuizTemplate, Box<dyn Error + Send + Sync>> {
        // Get the quiz
        let quiz = self.quiz_store.get_quiz(quiz_id).await?;

        // Create a new template
        let mut template = QuizTemplate {
            id: Uuid::new_v4(),
            title,
            description,
            author_id,
            category,
            tags,
            question_templates: Vec::new(),
            default_study_mode: quiz.study_mode,
            default_visibility: quiz.visibility,
            is_public,
            usage_count: 0,
            rating: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Convert questions to question templates
        for question in &quiz.questions {
            let question_template = QuestionTemplate {
                id: Uuid::new_v4(),
                template_id: template.id,
                text: question.text.clone(),
                description: question.description.clone(),
                answer_type: question.answer_type.clone(),
                placeholder_text: None,
                example_answers: question.answers.iter().map(|a| a.text.clone()).collect(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            template.question_templates.push(question_template);
        }

        // Store the template
        self.store_template(&template).await?;

        Ok(template)
    }

    /// Add a question template to a quiz template
    pub async fn add_question_template(
        &self,
        template_id: Uuid,
        text: String,
        description: Option<String>,
        answer_type: AnswerType,
        placeholder_text: Option<String>,
        example_answers: Vec<String>,
    ) -> Result<QuestionTemplate, Box<dyn Error + Send + Sync>> {
        // Get the template
        let mut template = self.get_template(template_id).await?;

        // Create a new question template
        let question_template = QuestionTemplate {
            id: Uuid::new_v4(),
            template_id,
            text,
            description,
            answer_type,
            placeholder_text,
            example_answers,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Add the question template to the template
        template.question_templates.push(question_template.clone());
        template.updated_at = Utc::now();

        // Store the template
        self.store_template(&template).await?;

        Ok(question_template)
    }

    /// Update a quiz template
    pub async fn update_template(
        &self,
        template_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        category: Option<TemplateCategory>,
        tags: Option<Vec<String>>,
        default_study_mode: Option<StudyMode>,
        default_visibility: Option<QuizVisibility>,
        is_public: Option<bool>,
    ) -> Result<QuizTemplate, Box<dyn Error + Send + Sync>> {
        // Get the template
        let mut template = self.get_template(template_id).await?;

        // Update the template
        if let Some(title) = title {
            template.title = title;
        }

        if let Some(description) = description {
            template.description = Some(description);
        }

        if let Some(category) = category {
            template.category = category;
        }

        if let Some(tags) = tags {
            template.tags = tags;
        }

        if let Some(default_study_mode) = default_study_mode {
            template.default_study_mode = default_study_mode;
        }

        if let Some(default_visibility) = default_visibility {
            template.default_visibility = default_visibility;
        }

        if let Some(is_public) = is_public {
            template.is_public = is_public;
        }

        template.updated_at = Utc::now();

        // Store the template
        self.store_template(&template).await?;

        Ok(template)
    }

    /// Delete a quiz template
    pub async fn delete_template(
        &self,
        template_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Delete the template
        sqlx::query!(
            r#"
            DELETE FROM quiz_templates
            WHERE id = ?
            "#,
            template_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        // Delete the question templates
        sqlx::query!(
            r#"
            DELETE FROM quiz_question_templates
            WHERE template_id = ?
            "#,
            template_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        // Delete the ratings
        sqlx::query!(
            r#"
            DELETE FROM quiz_template_ratings
            WHERE template_id = ?
            "#,
            template_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
