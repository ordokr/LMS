use super::adaptive_learning::{AdaptiveLearningService, AdaptiveLearningPath, UserLearningPathProgress, LearningPathRecommendation};
use uuid::Uuid;
use std::error::Error;
use chrono::Utc;

impl AdaptiveLearningService {
    /// Store a learning path
    pub async fn store_path(
        &self,
        path: &AdaptiveLearningPath,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Convert the tags to a JSON string
        let tags_json = serde_json::to_string(&path.tags)?;
        
        // Convert the nodes to a JSON string
        let nodes_json = serde_json::to_string(&path.nodes)?;
        
        // Convert the edges to a JSON string
        let edges_json = serde_json::to_string(&path.edges)?;
        
        // Convert the study mode to a string
        let study_mode_str = path.default_study_mode.to_string();
        
        // Convert the visibility to a string
        let visibility_str = path.default_visibility.to_string();
        
        // Check if the path already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_adaptive_learning_paths
            WHERE id = ?
            "#,
            path.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing path
            sqlx::query!(
                r#"
                UPDATE quiz_adaptive_learning_paths
                SET title = ?, description = ?, author_id = ?, subject = ?, tags = ?,
                    default_study_mode = ?, default_visibility = ?, is_public = ?,
                    usage_count = ?, rating = ?, updated_at = ?, nodes = ?, edges = ?
                WHERE id = ?
                "#,
                path.title,
                path.description,
                path.author_id.map(|id| id.to_string()),
                path.subject,
                tags_json,
                study_mode_str,
                visibility_str,
                path.is_public as i32,
                path.usage_count,
                path.rating,
                path.updated_at,
                nodes_json,
                edges_json,
                path.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new path
            sqlx::query!(
                r#"
                INSERT INTO quiz_adaptive_learning_paths
                (id, title, description, author_id, subject, tags, default_study_mode,
                 default_visibility, is_public, usage_count, rating, created_at, updated_at,
                 nodes, edges)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                path.id.to_string(),
                path.title,
                path.description,
                path.author_id.map(|id| id.to_string()),
                path.subject,
                tags_json,
                study_mode_str,
                visibility_str,
                path.is_public as i32,
                path.usage_count,
                path.rating,
                path.created_at,
                path.updated_at,
                nodes_json,
                edges_json
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
    
    /// Store user learning path progress
    pub async fn store_progress(
        &self,
        progress: &UserLearningPathProgress,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Convert the completed nodes to a JSON string
        let completed_nodes_json = serde_json::to_string(&progress.completed_nodes)?;
        
        // Convert the scores to a JSON string
        let scores_json = serde_json::to_string(&progress.scores)?;
        
        // Convert the custom data to a JSON string if present
        let custom_data_json = if let Some(data) = &progress.custom_data {
            Some(serde_json::to_string(data)?)
        } else {
            None
        };
        
        // Check if the progress already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_user_learning_path_progress
            WHERE id = ?
            "#,
            progress.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing progress
            sqlx::query!(
                r#"
                UPDATE quiz_user_learning_path_progress
                SET current_node_id = ?, completed_nodes = ?, scores = ?,
                    last_activity_at = ?, completed_at = ?, custom_data = ?
                WHERE id = ?
                "#,
                progress.current_node_id.to_string(),
                completed_nodes_json,
                scores_json,
                progress.last_activity_at,
                progress.completed_at,
                custom_data_json,
                progress.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new progress
            sqlx::query!(
                r#"
                INSERT INTO quiz_user_learning_path_progress
                (id, user_id, path_id, current_node_id, completed_nodes, scores,
                 started_at, last_activity_at, completed_at, custom_data)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                progress.id.to_string(),
                progress.user_id.to_string(),
                progress.path_id.to_string(),
                progress.current_node_id.to_string(),
                completed_nodes_json,
                scores_json,
                progress.started_at,
                progress.last_activity_at,
                progress.completed_at,
                custom_data_json
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
    
    /// Store a learning path recommendation
    pub async fn store_recommendation(
        &self,
        recommendation: &LearningPathRecommendation,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check if the recommendation already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_learning_path_recommendations
            WHERE id = ?
            "#,
            recommendation.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing recommendation
            sqlx::query!(
                r#"
                UPDATE quiz_learning_path_recommendations
                SET score = ?, reason = ?
                WHERE id = ?
                "#,
                recommendation.score,
                recommendation.reason,
                recommendation.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new recommendation
            sqlx::query!(
                r#"
                INSERT INTO quiz_learning_path_recommendations
                (id, user_id, path_id, score, reason, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                recommendation.id.to_string(),
                recommendation.user_id.to_string(),
                recommendation.path_id.to_string(),
                recommendation.score,
                recommendation.reason,
                recommendation.created_at
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
}
