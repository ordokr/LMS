use super::templates::{TemplateService, QuizTemplate, TemplateCategory};
use uuid::Uuid;
use std::error::Error;
use chrono::Utc;

impl TemplateService {
    /// Store a quiz template
    pub async fn store_template(
        &self,
        template: &QuizTemplate,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Convert the category to a string
        let category_str = match template.category {
            TemplateCategory::Education => "Education",
            TemplateCategory::Business => "Business",
            TemplateCategory::Science => "Science",
            TemplateCategory::Technology => "Technology",
            TemplateCategory::Language => "Language",
            TemplateCategory::Arts => "Arts",
            TemplateCategory::Health => "Health",
            TemplateCategory::Custom => "Custom",
        };
        
        // Convert the tags to a JSON string
        let tags_json = serde_json::to_string(&template.tags)?;
        
        // Convert the study mode to a string
        let study_mode_str = template.default_study_mode.to_string();
        
        // Convert the visibility to a string
        let visibility_str = template.default_visibility.to_string();
        
        // Check if the template already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_templates
            WHERE id = ?
            "#,
            template.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing template
            sqlx::query!(
                r#"
                UPDATE quiz_templates
                SET title = ?, description = ?, author_id = ?, category = ?, tags = ?,
                    default_study_mode = ?, default_visibility = ?, is_public = ?,
                    usage_count = ?, rating = ?, updated_at = ?
                WHERE id = ?
                "#,
                template.title,
                template.description,
                template.author_id.map(|id| id.to_string()),
                category_str,
                tags_json,
                study_mode_str,
                visibility_str,
                template.is_public as i32,
                template.usage_count,
                template.rating,
                template.updated_at,
                template.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new template
            sqlx::query!(
                r#"
                INSERT INTO quiz_templates
                (id, title, description, author_id, category, tags, default_study_mode,
                 default_visibility, is_public, usage_count, rating, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                template.id.to_string(),
                template.title,
                template.description,
                template.author_id.map(|id| id.to_string()),
                category_str,
                tags_json,
                study_mode_str,
                visibility_str,
                template.is_public as i32,
                template.usage_count,
                template.rating,
                template.created_at,
                template.updated_at
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        // Store the question templates
        for question_template in &template.question_templates {
            // Convert the answer type to a string
            let answer_type_str = question_template.answer_type.to_string();
            
            // Convert the example answers to a JSON string
            let examples_json = serde_json::to_string(&question_template.example_answers)?;
            
            // Check if the question template already exists
            let existing = sqlx::query!(
                r#"
                SELECT id FROM quiz_question_templates
                WHERE id = ?
                "#,
                question_template.id.to_string()
            )
            .fetch_optional(&self.db_pool)
            .await?;
            
            if existing.is_some() {
                // Update existing question template
                sqlx::query!(
                    r#"
                    UPDATE quiz_question_templates
                    SET text = ?, description = ?, answer_type = ?, placeholder_text = ?,
                        example_answers = ?, updated_at = ?
                    WHERE id = ?
                    "#,
                    question_template.text,
                    question_template.description,
                    answer_type_str,
                    question_template.placeholder_text,
                    examples_json,
                    question_template.updated_at,
                    question_template.id.to_string()
                )
                .execute(&self.db_pool)
                .await?;
            } else {
                // Insert new question template
                sqlx::query!(
                    r#"
                    INSERT INTO quiz_question_templates
                    (id, template_id, text, description, answer_type, placeholder_text,
                     example_answers, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    question_template.id.to_string(),
                    question_template.template_id.to_string(),
                    question_template.text,
                    question_template.description,
                    answer_type_str,
                    question_template.placeholder_text,
                    examples_json,
                    question_template.created_at,
                    question_template.updated_at
                )
                .execute(&self.db_pool)
                .await?;
            }
        }
        
        Ok(())
    }
}
