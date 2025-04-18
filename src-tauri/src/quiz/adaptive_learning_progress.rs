use super::adaptive_learning::{AdaptiveLearningService, AdaptiveLearningPath, LearningPathNode, LearningPathEdge, LearningPathNodeType, EdgeConditionType, UserLearningPathProgress, LearningPathRecommendation};
use uuid::Uuid;
use std::error::Error;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

impl AdaptiveLearningService {
    /// Start a learning path for a user
    pub async fn start_path(
        &self,
        user_id: Uuid,
        path_id: Uuid,
    ) -> Result<UserLearningPathProgress, Box<dyn Error + Send + Sync>> {
        // Get the path
        let path = self.get_path(path_id).await?;
        
        // Find the start node
        let start_node = path.nodes.iter()
            .find(|n| n.node_type == LearningPathNodeType::Start)
            .ok_or("No start node found in path")?;
        
        // Check if the user already has progress for this path
        let existing_progress = self.get_user_progress(user_id, path_id).await;
        
        if let Ok(progress) = existing_progress {
            // Return the existing progress
            Ok(progress)
        } else {
            // Create new progress
            let progress = UserLearningPathProgress {
                id: Uuid::new_v4(),
                user_id,
                path_id,
                current_node_id: start_node.id,
                completed_nodes: Vec::new(),
                scores: HashMap::new(),
                started_at: Utc::now(),
                last_activity_at: Utc::now(),
                completed_at: None,
                custom_data: None,
            };
            
            // Store the progress
            self.store_progress(&progress).await?;
            
            // Increment the path usage count
            let mut updated_path = path;
            updated_path.usage_count += 1;
            self.store_path(&updated_path).await?;
            
            Ok(progress)
        }
    }
    
    /// Get user progress for a learning path
    pub async fn get_user_progress(
        &self,
        user_id: Uuid,
        path_id: Uuid,
    ) -> Result<UserLearningPathProgress, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, path_id, current_node_id, completed_nodes, scores,
                   started_at, last_activity_at, completed_at, custom_data
            FROM quiz_user_learning_path_progress
            WHERE user_id = ? AND path_id = ?
            "#,
            user_id.to_string(),
            path_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Parse the completed nodes
            let completed_nodes: Vec<Uuid> = serde_json::from_str(&row.completed_nodes)?;
            
            // Parse the scores
            let scores: HashMap<String, f32> = serde_json::from_str(&row.scores)?;
            
            // Parse the custom data if present
            let custom_data = if let Some(data_str) = &row.custom_data {
                Some(serde_json::from_str(data_str)?)
            } else {
                None
            };
            
            // Create the progress
            let progress = UserLearningPathProgress {
                id: Uuid::parse_str(&row.id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                path_id: Uuid::parse_str(&row.path_id)?,
                current_node_id: Uuid::parse_str(&row.current_node_id)?,
                completed_nodes,
                scores,
                started_at: row.started_at.parse::<DateTime<Utc>>()?,
                last_activity_at: row.last_activity_at.parse::<DateTime<Utc>>()?,
                completed_at: row.completed_at.map(|dt| dt.parse::<DateTime<Utc>>()).transpose()?,
                custom_data,
            };
            
            Ok(progress)
        } else {
            Err("User progress not found".into())
        }
    }
    
    /// Get all user progress for a user
    pub async fn get_all_user_progress(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserLearningPathProgress>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, path_id, current_node_id, completed_nodes, scores,
                   started_at, last_activity_at, completed_at, custom_data
            FROM quiz_user_learning_path_progress
            WHERE user_id = ?
            ORDER BY last_activity_at DESC
            "#,
            user_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut progress_list = Vec::new();
        
        for row in rows {
            // Parse the completed nodes
            let completed_nodes: Vec<Uuid> = serde_json::from_str(&row.completed_nodes)?;
            
            // Parse the scores
            let scores: HashMap<String, f32> = serde_json::from_str(&row.scores)?;
            
            // Parse the custom data if present
            let custom_data = if let Some(data_str) = &row.custom_data {
                Some(serde_json::from_str(data_str)?)
            } else {
                None
            };
            
            // Create the progress
            let progress = UserLearningPathProgress {
                id: Uuid::parse_str(&row.id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                path_id: Uuid::parse_str(&row.path_id)?,
                current_node_id: Uuid::parse_str(&row.current_node_id)?,
                completed_nodes,
                scores,
                started_at: row.started_at.parse::<DateTime<Utc>>()?,
                last_activity_at: row.last_activity_at.parse::<DateTime<Utc>>()?,
                completed_at: row.completed_at.map(|dt| dt.parse::<DateTime<Utc>>()).transpose()?,
                custom_data,
            };
            
            progress_list.push(progress);
        }
        
        Ok(progress_list)
    }
    
    /// Complete a node in a learning path
    pub async fn complete_node(
        &self,
        user_id: Uuid,
        path_id: Uuid,
        node_id: Uuid,
        score: Option<f32>,
    ) -> Result<UserLearningPathProgress, Box<dyn Error + Send + Sync>> {
        // Get the user's progress
        let mut progress = self.get_user_progress(user_id, path_id).await?;
        
        // Get the path
        let path = self.get_path(path_id).await?;
        
        // Check if the node exists in the path
        let node = path.nodes.iter()
            .find(|n| n.id == node_id)
            .ok_or("Node not found in path")?;
        
        // Add the node to completed nodes if not already there
        if !progress.completed_nodes.contains(&node_id) {
            progress.completed_nodes.push(node_id);
        }
        
        // Update the score if provided
        if let Some(score_value) = score {
            progress.scores.insert(node_id.to_string(), score_value);
        }
        
        // Update the last activity time
        progress.last_activity_at = Utc::now();
        
        // Store the updated progress
        self.store_progress(&progress).await?;
        
        Ok(progress)
    }
    
    /// Move to the next node in a learning path
    pub async fn move_to_next_node(
        &self,
        user_id: Uuid,
        path_id: Uuid,
    ) -> Result<(UserLearningPathProgress, LearningPathNode), Box<dyn Error + Send + Sync>> {
        // Get the user's progress
        let mut progress = self.get_user_progress(user_id, path_id).await?;
        
        // Get the path
        let path = self.get_path(path_id).await?;
        
        // Get the current node
        let current_node = path.nodes.iter()
            .find(|n| n.id == progress.current_node_id)
            .ok_or("Current node not found in path")?;
        
        // Find all outgoing edges from the current node
        let outgoing_edges: Vec<&LearningPathEdge> = path.edges.iter()
            .filter(|e| e.source_node_id == current_node.id)
            .collect();
        
        if outgoing_edges.is_empty() {
            return Err("No outgoing edges from current node".into());
        }
        
        // Find the best edge to follow based on conditions
        let next_edge = self.find_best_edge(&outgoing_edges, &progress)?;
        
        // Find the target node
        let next_node = path.nodes.iter()
            .find(|n| n.id == next_edge.target_node_id)
            .ok_or("Target node not found in path")?;
        
        // Update the current node
        progress.current_node_id = next_node.id;
        
        // Check if this is the end node
        if next_node.node_type == LearningPathNodeType::End {
            progress.completed_at = Some(Utc::now());
        }
        
        // Update the last activity time
        progress.last_activity_at = Utc::now();
        
        // Store the updated progress
        self.store_progress(&progress).await?;
        
        Ok((progress, next_node.clone()))
    }
    
    /// Find the best edge to follow based on conditions
    fn find_best_edge<'a>(
        &self,
        edges: &[&'a LearningPathEdge],
        progress: &UserLearningPathProgress,
    ) -> Result<&'a LearningPathEdge, Box<dyn Error + Send + Sync>> {
        // If there's only one edge, return it
        if edges.len() == 1 {
            return Ok(edges[0]);
        }
        
        // Check each edge's condition
        for edge in edges {
            match edge.condition_type {
                EdgeConditionType::Completion => {
                    // Check if the source node is completed
                    if progress.completed_nodes.contains(&edge.source_node_id) {
                        return Ok(edge);
                    }
                },
                EdgeConditionType::Score => {
                    // Check if the score meets the condition
                    if let Some(condition_value) = &edge.condition_value {
                        if let Some(required_score) = condition_value.get("min_score").and_then(|s| s.as_f64()) {
                            if let Some(score) = progress.scores.get(&edge.source_node_id.to_string()) {
                                if *score >= required_score as f32 {
                                    return Ok(edge);
                                }
                            }
                        }
                    }
                },
                EdgeConditionType::Time => {
                    // Time-based conditions not implemented yet
                    // Default to allowing this edge
                    return Ok(edge);
                },
                EdgeConditionType::Custom => {
                    // Custom conditions not implemented yet
                    // Default to allowing this edge
                    return Ok(edge);
                },
            }
        }
        
        // If no edge matches the conditions, return the first one as a fallback
        Ok(edges[0])
    }
    
    /// Generate recommendations for a user
    pub async fn generate_recommendations(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<LearningPathRecommendation>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(5);
        
        // Get all public paths
        let paths = self.get_public_paths(None, None).await?;
        
        // Get all user progress
        let user_progress = self.get_all_user_progress(user_id).await?;
        
        // Create a set of path IDs the user has already started
        let started_path_ids: std::collections::HashSet<Uuid> = user_progress.iter()
            .map(|p| p.path_id)
            .collect();
        
        // Filter out paths the user has already started
        let new_paths: Vec<&AdaptiveLearningPath> = paths.iter()
            .filter(|p| !started_path_ids.contains(&p.id))
            .collect();
        
        if new_paths.is_empty() {
            return Ok(Vec::new());
        }
        
        // Simple recommendation algorithm: recommend paths with highest usage and rating
        let mut recommendations = Vec::new();
        
        for path in new_paths {
            // Calculate a score based on usage count and rating
            let usage_score = path.usage_count as f32 / 100.0; // Normalize usage
            let rating_score = path.rating.unwrap_or(0.0);
            
            // Combined score (70% rating, 30% usage)
            let score = (rating_score * 0.7) + (usage_score * 0.3);
            
            // Create a recommendation
            let recommendation = LearningPathRecommendation {
                id: Uuid::new_v4(),
                user_id,
                path_id: path.id,
                score,
                reason: format!("Popular path with {} users", path.usage_count),
                created_at: Utc::now(),
            };
            
            recommendations.push(recommendation);
        }
        
        // Sort by score (highest first) and limit
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        recommendations.truncate(limit as usize);
        
        // Store the recommendations
        for recommendation in &recommendations {
            self.store_recommendation(recommendation).await?;
        }
        
        Ok(recommendations)
    }
    
    /// Get recommendations for a user
    pub async fn get_recommendations(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<LearningPathRecommendation>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(5);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, path_id, score, reason, created_at
            FROM quiz_learning_path_recommendations
            WHERE user_id = ?
            ORDER BY score DESC
            LIMIT ?
            "#,
            user_id.to_string(),
            limit
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut recommendations = Vec::new();
        
        for row in rows {
            // Create the recommendation
            let recommendation = LearningPathRecommendation {
                id: Uuid::parse_str(&row.id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                path_id: Uuid::parse_str(&row.path_id)?,
                score: row.score,
                reason: row.reason,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
            };
            
            recommendations.push(recommendation);
        }
        
        // If no recommendations found, generate new ones
        if recommendations.is_empty() {
            recommendations = self.generate_recommendations(user_id, limit).await?;
        }
        
        Ok(recommendations)
    }
}
