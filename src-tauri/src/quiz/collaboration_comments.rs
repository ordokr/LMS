use super::collaboration::{QuizComment, CollaborationService};
use uuid::Uuid;
use std::error::Error;
use chrono::{DateTime, Utc};

impl CollaborationService {
    /// Add a comment to a quiz
    pub async fn add_comment(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        // Create a new comment
        let comment = QuizComment {
            id: Uuid::new_v4(),
            quiz_id,
            question_id: None,
            user_id,
            parent_id: None,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Store the comment
        self.store_comment(&comment).await?;
        
        Ok(comment)
    }
    
    /// Add a comment to a specific question
    pub async fn add_question_comment(
        &self,
        quiz_id: Uuid,
        question_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        // Create a new comment
        let comment = QuizComment {
            id: Uuid::new_v4(),
            quiz_id,
            question_id: Some(question_id),
            user_id,
            parent_id: None,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Store the comment
        self.store_comment(&comment).await?;
        
        Ok(comment)
    }
    
    /// Add a reply to a comment
    pub async fn add_reply(
        &self,
        parent_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        // Get the parent comment
        let parent = self.get_comment(parent_id).await?;
        
        // Create a new comment
        let comment = QuizComment {
            id: Uuid::new_v4(),
            quiz_id: parent.quiz_id,
            question_id: parent.question_id,
            user_id,
            parent_id: Some(parent_id),
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Store the comment
        self.store_comment(&comment).await?;
        
        Ok(comment)
    }
    
    /// Update a comment
    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        // Get the comment
        let mut comment = self.get_comment(comment_id).await?;
        
        // Check if the user is the author
        if comment.user_id != user_id {
            return Err("Only the author can update a comment".into());
        }
        
        // Update the comment
        comment.content = content;
        comment.updated_at = Utc::now();
        
        // Store the updated comment
        self.store_comment(&comment).await?;
        
        Ok(comment)
    }
    
    /// Delete a comment
    pub async fn delete_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the comment
        let comment = self.get_comment(comment_id).await?;
        
        // Check if the user is the author
        if comment.user_id != user_id {
            // Check if the user is the quiz owner
            let collaborator = self.get_collaborator(comment.quiz_id, user_id).await?;
            if collaborator.role != super::collaboration::CollaborationRole::Owner {
                return Err("Only the author or quiz owner can delete a comment".into());
            }
        }
        
        // Delete the comment
        sqlx::query!(
            r#"
            DELETE FROM quiz_comments
            WHERE id = ?
            "#,
            comment_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Get a comment
    pub async fn get_comment(
        &self,
        comment_id: Uuid,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, quiz_id, question_id, user_id, parent_id, content, created_at, updated_at
            FROM quiz_comments
            WHERE id = ?
            "#,
            comment_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            let comment = QuizComment {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                question_id: row.question_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                user_id: Uuid::parse_str(&row.user_id)?,
                parent_id: row.parent_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                content: row.content,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            Ok(comment)
        } else {
            Err("Comment not found".into())
        }
    }
    
    /// Get all comments for a quiz
    pub async fn get_comments_for_quiz(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<QuizComment>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, question_id, user_id, parent_id, content, created_at, updated_at
            FROM quiz_comments
            WHERE quiz_id = ? AND question_id IS NULL AND parent_id IS NULL
            ORDER BY created_at DESC
            "#,
            quiz_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut comments = Vec::new();
        
        for row in rows {
            let comment = QuizComment {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                question_id: row.question_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                user_id: Uuid::parse_str(&row.user_id)?,
                parent_id: row.parent_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                content: row.content,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            comments.push(comment);
        }
        
        Ok(comments)
    }
    
    /// Get all comments for a question
    pub async fn get_comments_for_question(
        &self,
        question_id: Uuid,
    ) -> Result<Vec<QuizComment>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, question_id, user_id, parent_id, content, created_at, updated_at
            FROM quiz_comments
            WHERE question_id = ? AND parent_id IS NULL
            ORDER BY created_at DESC
            "#,
            question_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut comments = Vec::new();
        
        for row in rows {
            let comment = QuizComment {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                question_id: row.question_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                user_id: Uuid::parse_str(&row.user_id)?,
                parent_id: row.parent_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                content: row.content,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            comments.push(comment);
        }
        
        Ok(comments)
    }
    
    /// Get all replies to a comment
    pub async fn get_replies(
        &self,
        comment_id: Uuid,
    ) -> Result<Vec<QuizComment>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, question_id, user_id, parent_id, content, created_at, updated_at
            FROM quiz_comments
            WHERE parent_id = ?
            ORDER BY created_at ASC
            "#,
            comment_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut comments = Vec::new();
        
        for row in rows {
            let comment = QuizComment {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                question_id: row.question_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                user_id: Uuid::parse_str(&row.user_id)?,
                parent_id: row.parent_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                content: row.content,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            comments.push(comment);
        }
        
        Ok(comments)
    }
    
    /// Store a comment
    async fn store_comment(
        &self,
        comment: &QuizComment,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check if the comment already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_comments
            WHERE id = ?
            "#,
            comment.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing comment
            sqlx::query!(
                r#"
                UPDATE quiz_comments
                SET content = ?, updated_at = ?
                WHERE id = ?
                "#,
                comment.content,
                comment.updated_at,
                comment.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new comment
            sqlx::query!(
                r#"
                INSERT INTO quiz_comments
                (id, quiz_id, question_id, user_id, parent_id, content, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                comment.id.to_string(),
                comment.quiz_id.to_string(),
                comment.question_id.map(|id| id.to_string()),
                comment.user_id.to_string(),
                comment.parent_id.map(|id| id.to_string()),
                comment.content,
                comment.created_at,
                comment.updated_at
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
}
