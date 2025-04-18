use super::templates::{TemplateService, TemplateRating};
use uuid::Uuid;
use std::error::Error;
use chrono::Utc;

impl TemplateService {
    /// Rate a template
    pub async fn rate_template(
        &self,
        template_id: Uuid,
        user_id: Uuid,
        rating: f32,
        comment: Option<String>,
    ) -> Result<TemplateRating, Box<dyn Error + Send + Sync>> {
        // Check if the user has already rated this template
        let existing_rating = self.get_user_rating(template_id, user_id).await;
        
        if let Ok(mut existing) = existing_rating {
            // Update the existing rating
            existing.rating = rating;
            existing.comment = comment;
            existing.updated_at = Utc::now();
            
            // Store the rating
            self.store_rating(&existing).await?;
            
            // Update the template's average rating
            self.update_template_rating(template_id).await?;
            
            Ok(existing)
        } else {
            // Create a new rating
            let new_rating = TemplateRating {
                id: Uuid::new_v4(),
                template_id,
                user_id,
                rating,
                comment,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            // Store the rating
            self.store_rating(&new_rating).await?;
            
            // Update the template's average rating
            self.update_template_rating(template_id).await?;
            
            Ok(new_rating)
        }
    }
    
    /// Get a user's rating for a template
    pub async fn get_user_rating(
        &self,
        template_id: Uuid,
        user_id: Uuid,
    ) -> Result<TemplateRating, Box<dyn Error + Send + Sync>> {
        // Get the rating
        let row = sqlx::query!(
            r#"
            SELECT id, template_id, user_id, rating, comment, created_at, updated_at
            FROM quiz_template_ratings
            WHERE template_id = ? AND user_id = ?
            "#,
            template_id.to_string(),
            user_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Create the rating
            let rating = TemplateRating {
                id: Uuid::parse_str(&row.id)?,
                template_id: Uuid::parse_str(&row.template_id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                rating: row.rating,
                comment: row.comment,
                created_at: row.created_at.parse()?,
                updated_at: row.updated_at.parse()?,
            };
            
            Ok(rating)
        } else {
            Err("Rating not found".into())
        }
    }
    
    /// Get all ratings for a template
    pub async fn get_template_ratings(
        &self,
        template_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<TemplateRating>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        // Get the ratings
        let rows = sqlx::query!(
            r#"
            SELECT id, template_id, user_id, rating, comment, created_at, updated_at
            FROM quiz_template_ratings
            WHERE template_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            template_id.to_string(),
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut ratings = Vec::new();
        
        for row in rows {
            // Create the rating
            let rating = TemplateRating {
                id: Uuid::parse_str(&row.id)?,
                template_id: Uuid::parse_str(&row.template_id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                rating: row.rating,
                comment: row.comment,
                created_at: row.created_at.parse()?,
                updated_at: row.updated_at.parse()?,
            };
            
            ratings.push(rating);
        }
        
        Ok(ratings)
    }
    
    /// Delete a rating
    pub async fn delete_rating(
        &self,
        rating_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the rating
        let row = sqlx::query!(
            r#"
            SELECT template_id, user_id
            FROM quiz_template_ratings
            WHERE id = ?
            "#,
            rating_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Check if the user is the author of the rating
            let rating_user_id = Uuid::parse_str(&row.user_id)?;
            
            if rating_user_id != user_id {
                return Err("Only the author can delete a rating".into());
            }
            
            // Delete the rating
            sqlx::query!(
                r#"
                DELETE FROM quiz_template_ratings
                WHERE id = ?
                "#,
                rating_id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
            
            // Update the template's average rating
            let template_id = Uuid::parse_str(&row.template_id)?;
            self.update_template_rating(template_id).await?;
            
            Ok(())
        } else {
            Err("Rating not found".into())
        }
    }
    
    /// Store a rating
    async fn store_rating(
        &self,
        rating: &TemplateRating,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check if the rating already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_template_ratings
            WHERE id = ?
            "#,
            rating.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing rating
            sqlx::query!(
                r#"
                UPDATE quiz_template_ratings
                SET rating = ?, comment = ?, updated_at = ?
                WHERE id = ?
                "#,
                rating.rating,
                rating.comment,
                rating.updated_at,
                rating.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new rating
            sqlx::query!(
                r#"
                INSERT INTO quiz_template_ratings
                (id, template_id, user_id, rating, comment, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
                rating.id.to_string(),
                rating.template_id.to_string(),
                rating.user_id.to_string(),
                rating.rating,
                rating.comment,
                rating.created_at,
                rating.updated_at
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
    
    /// Update a template's average rating
    async fn update_template_rating(
        &self,
        template_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Calculate the average rating
        let row = sqlx::query!(
            r#"
            SELECT AVG(rating) as avg_rating
            FROM quiz_template_ratings
            WHERE template_id = ?
            "#,
            template_id.to_string()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Update the template
        sqlx::query!(
            r#"
            UPDATE quiz_templates
            SET rating = ?, updated_at = ?
            WHERE id = ?
            "#,
            row.avg_rating,
            Utc::now(),
            template_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
}
