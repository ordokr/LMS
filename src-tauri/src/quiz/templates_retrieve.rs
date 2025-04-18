use super::templates::{TemplateService, QuizTemplate, QuestionTemplate, TemplateCategory, TemplateRating};
use super::models::{Quiz, Question, Answer, AnswerType};
use uuid::Uuid;
use std::error::Error;
use chrono::{DateTime, Utc};

impl TemplateService {
    /// Get a quiz template by ID
    pub async fn get_template(
        &self,
        template_id: Uuid,
    ) -> Result<QuizTemplate, Box<dyn Error + Send + Sync>> {
        // Get the template
        let row = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, category, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at
            FROM quiz_templates
            WHERE id = ?
            "#,
            template_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the category
            let category = match row.category.as_str() {
                "Education" => TemplateCategory::Education,
                "Business" => TemplateCategory::Business,
                "Science" => TemplateCategory::Science,
                "Technology" => TemplateCategory::Technology,
                "Language" => TemplateCategory::Language,
                "Arts" => TemplateCategory::Arts,
                "Health" => TemplateCategory::Health,
                _ => TemplateCategory::Custom,
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Create the template
            let mut template = QuizTemplate {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                category,
                tags,
                question_templates: Vec::new(),
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            // Get the question templates
            let question_templates = self.get_question_templates(template_id).await?;
            template.question_templates = question_templates;
            
            Ok(template)
        } else {
            Err("Template not found".into())
        }
    }
    
    /// Get all question templates for a quiz template
    async fn get_question_templates(
        &self,
        template_id: Uuid,
    ) -> Result<Vec<QuestionTemplate>, Box<dyn Error + Send + Sync>> {
        // Get the question templates
        let rows = sqlx::query!(
            r#"
            SELECT id, template_id, text, description, answer_type, placeholder_text, 
                   example_answers, created_at, updated_at
            FROM quiz_question_templates
            WHERE template_id = ?
            "#,
            template_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut question_templates = Vec::new();
        
        for row in rows {
            // Parse the example answers
            let example_answers: Vec<String> = if let Some(examples_str) = &row.example_answers {
                serde_json::from_str(examples_str)?
            } else {
                Vec::new()
            };
            
            // Parse the answer type
            let answer_type = row.answer_type.parse()?;
            
            // Create the question template
            let question_template = QuestionTemplate {
                id: Uuid::parse_str(&row.id)?,
                template_id: Uuid::parse_str(&row.template_id)?,
                text: row.text,
                description: row.description,
                answer_type,
                placeholder_text: row.placeholder_text,
                example_answers,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            question_templates.push(question_template);
        }
        
        Ok(question_templates)
    }
    
    /// Get all public templates
    pub async fn get_public_templates(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizTemplate>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        // Get the templates
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, category, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at
            FROM quiz_templates
            WHERE is_public = 1
            ORDER BY usage_count DESC, rating DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut templates = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the category
            let category = match row.category.as_str() {
                "Education" => TemplateCategory::Education,
                "Business" => TemplateCategory::Business,
                "Science" => TemplateCategory::Science,
                "Technology" => TemplateCategory::Technology,
                "Language" => TemplateCategory::Language,
                "Arts" => TemplateCategory::Arts,
                "Health" => TemplateCategory::Health,
                _ => TemplateCategory::Custom,
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Create the template
            let template = QuizTemplate {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                category,
                tags,
                question_templates: Vec::new(), // We don't load question templates for listing
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            templates.push(template);
        }
        
        Ok(templates)
    }
    
    /// Search templates by title, description, or tags
    pub async fn search_templates(
        &self,
        query: &str,
        category: Option<TemplateCategory>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizTemplate>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        // Prepare the search query
        let search_query = format!("%{}%", query);
        
        // Get the templates
        let rows = if let Some(category) = category {
            let category_str = match category {
                TemplateCategory::Education => "Education",
                TemplateCategory::Business => "Business",
                TemplateCategory::Science => "Science",
                TemplateCategory::Technology => "Technology",
                TemplateCategory::Language => "Language",
                TemplateCategory::Arts => "Arts",
                TemplateCategory::Health => "Health",
                TemplateCategory::Custom => "Custom",
            };
            
            sqlx::query!(
                r#"
                SELECT id, title, description, author_id, category, tags, default_study_mode, 
                       default_visibility, is_public, usage_count, rating, created_at, updated_at
                FROM quiz_templates
                WHERE is_public = 1
                AND category = ?
                AND (title LIKE ? OR description LIKE ? OR tags LIKE ?)
                ORDER BY usage_count DESC, rating DESC
                LIMIT ? OFFSET ?
                "#,
                category_str,
                search_query,
                search_query,
                search_query,
                limit,
                offset
            )
            .fetch_all(&self.db_pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                SELECT id, title, description, author_id, category, tags, default_study_mode, 
                       default_visibility, is_public, usage_count, rating, created_at, updated_at
                FROM quiz_templates
                WHERE is_public = 1
                AND (title LIKE ? OR description LIKE ? OR tags LIKE ?)
                ORDER BY usage_count DESC, rating DESC
                LIMIT ? OFFSET ?
                "#,
                search_query,
                search_query,
                search_query,
                limit,
                offset
            )
            .fetch_all(&self.db_pool)
            .await?
        };
        
        let mut templates = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the category
            let category = match row.category.as_str() {
                "Education" => TemplateCategory::Education,
                "Business" => TemplateCategory::Business,
                "Science" => TemplateCategory::Science,
                "Technology" => TemplateCategory::Technology,
                "Language" => TemplateCategory::Language,
                "Arts" => TemplateCategory::Arts,
                "Health" => TemplateCategory::Health,
                _ => TemplateCategory::Custom,
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Create the template
            let template = QuizTemplate {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                category,
                tags,
                question_templates: Vec::new(), // We don't load question templates for listing
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            templates.push(template);
        }
        
        Ok(templates)
    }
    
    /// Get templates by author
    pub async fn get_templates_by_author(
        &self,
        author_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizTemplate>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        // Get the templates
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, category, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at
            FROM quiz_templates
            WHERE author_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            author_id.to_string(),
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut templates = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the category
            let category = match row.category.as_str() {
                "Education" => TemplateCategory::Education,
                "Business" => TemplateCategory::Business,
                "Science" => TemplateCategory::Science,
                "Technology" => TemplateCategory::Technology,
                "Language" => TemplateCategory::Language,
                "Arts" => TemplateCategory::Arts,
                "Health" => TemplateCategory::Health,
                _ => TemplateCategory::Custom,
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Create the template
            let template = QuizTemplate {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                category,
                tags,
                question_templates: Vec::new(), // We don't load question templates for listing
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            templates.push(template);
        }
        
        Ok(templates)
    }
    
    /// Create a quiz from a template
    pub async fn create_quiz_from_template(
        &self,
        template_id: Uuid,
        title: String,
        description: Option<String>,
        author_id: Option<Uuid>,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        // Get the template
        let template = self.get_template(template_id).await?;
        
        // Create a new quiz
        let quiz_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Create the quiz
        let mut quiz = Quiz {
            id: quiz_id,
            title,
            description,
            author_id,
            created_at: Some(now),
            updated_at: Some(now),
            questions: Vec::new(),
            study_mode: template.default_study_mode,
            visibility: template.default_visibility,
            settings: Default::default(),
        };
        
        // Convert question templates to questions
        for (index, question_template) in template.question_templates.iter().enumerate() {
            let question_id = Uuid::new_v4();
            
            // Create the question
            let mut question = Question {
                id: question_id,
                quiz_id,
                text: question_template.text.clone(),
                description: question_template.description.clone(),
                answer_type: question_template.answer_type.clone(),
                answers: Vec::new(),
                position: index as i32,
                points: 1,
                content: Default::default(),
                created_at: Some(now),
                updated_at: Some(now),
            };
            
            // Create example answers if available
            if !question_template.example_answers.is_empty() {
                for (answer_index, answer_text) in question_template.example_answers.iter().enumerate() {
                    let answer = Answer {
                        id: Uuid::new_v4(),
                        question_id,
                        text: answer_text.clone(),
                        is_correct: answer_index == 0, // First answer is correct by default
                        position: answer_index as i32,
                        created_at: Some(now),
                        updated_at: Some(now),
                    };
                    
                    question.answers.push(answer);
                }
            } else {
                // Create a default answer for multiple choice
                if question.answer_type == AnswerType::MultipleChoice {
                    for i in 0..4 {
                        let answer = Answer {
                            id: Uuid::new_v4(),
                            question_id,
                            text: format!("Answer {}", i + 1),
                            is_correct: i == 0, // First answer is correct by default
                            position: i,
                            created_at: Some(now),
                            updated_at: Some(now),
                        };
                        
                        question.answers.push(answer);
                    }
                }
            }
            
            quiz.questions.push(question);
        }
        
        // Store the quiz
        self.quiz_store.save_quiz(&quiz).await?;
        
        // Increment the template usage count
        self.increment_template_usage(template_id).await?;
        
        Ok(quiz)
    }
    
    /// Increment the usage count of a template
    async fn increment_template_usage(
        &self,
        template_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Update the template
        sqlx::query!(
            r#"
            UPDATE quiz_templates
            SET usage_count = usage_count + 1, updated_at = ?
            WHERE id = ?
            "#,
            Utc::now(),
            template_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
}
