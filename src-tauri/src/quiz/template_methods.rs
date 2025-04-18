use super::QuizEngine;
use super::templates::{QuizTemplate, QuestionTemplate, TemplateCategory, TemplateRating};
use super::models::{Quiz, StudyMode, QuizVisibility, AnswerType};
use uuid::Uuid;
use std::error::Error;

impl QuizEngine {
    // Template methods
    
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
        if let Some(template_service) = &self.template_service {
            template_service.create_template(
                title,
                description,
                author_id,
                category,
                tags,
                default_study_mode,
                default_visibility,
                is_public,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
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
        if let Some(template_service) = &self.template_service {
            template_service.create_template_from_quiz(
                quiz_id,
                title,
                description,
                author_id,
                category,
                tags,
                is_public,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
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
        if let Some(template_service) = &self.template_service {
            template_service.add_question_template(
                template_id,
                text,
                description,
                answer_type,
                placeholder_text,
                example_answers,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
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
        if let Some(template_service) = &self.template_service {
            template_service.update_template(
                template_id,
                title,
                description,
                category,
                tags,
                default_study_mode,
                default_visibility,
                is_public,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Delete a quiz template
    pub async fn delete_template(
        &self,
        template_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.delete_template(template_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Get a quiz template by ID
    pub async fn get_template(
        &self,
        template_id: Uuid,
    ) -> Result<QuizTemplate, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.get_template(template_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Get all public templates
    pub async fn get_public_templates(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizTemplate>, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.get_public_templates(limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Search templates by title, description, or tags
    pub async fn search_templates(
        &self,
        query: &str,
        category: Option<TemplateCategory>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizTemplate>, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.search_templates(query, category, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Get templates by author
    pub async fn get_templates_by_author(
        &self,
        author_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizTemplate>, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.get_templates_by_author(author_id, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Create a quiz from a template
    pub async fn create_quiz_from_template(
        &self,
        template_id: Uuid,
        title: String,
        description: Option<String>,
        author_id: Option<Uuid>,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.create_quiz_from_template(template_id, title, description, author_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Rate a template
    pub async fn rate_template(
        &self,
        template_id: Uuid,
        user_id: Uuid,
        rating: f32,
        comment: Option<String>,
    ) -> Result<TemplateRating, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.rate_template(template_id, user_id, rating, comment).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Get a user's rating for a template
    pub async fn get_user_template_rating(
        &self,
        template_id: Uuid,
        user_id: Uuid,
    ) -> Result<TemplateRating, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.get_user_rating(template_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Get all ratings for a template
    pub async fn get_template_ratings(
        &self,
        template_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<TemplateRating>, Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.get_template_ratings(template_id, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
    
    /// Delete a rating
    pub async fn delete_template_rating(
        &self,
        rating_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(template_service) = &self.template_service {
            template_service.delete_rating(rating_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Template service is not available".into())
        }
    }
}
