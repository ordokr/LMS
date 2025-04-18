use super::QuizEngine;
use super::adaptive_learning::{AdaptiveLearningPath, LearningPathNode, LearningPathEdge, LearningPathNodeType, EdgeConditionType, UserLearningPathProgress, LearningPathRecommendation};
use super::models::{StudyMode, QuizVisibility};
use uuid::Uuid;
use std::error::Error;
use serde_json::Value;

impl QuizEngine {
    // Adaptive Learning methods
    
    /// Create a new adaptive learning path
    pub async fn create_learning_path(
        &self,
        title: String,
        description: Option<String>,
        author_id: Option<Uuid>,
        subject: String,
        tags: Vec<String>,
        default_study_mode: StudyMode,
        default_visibility: QuizVisibility,
        is_public: bool,
    ) -> Result<AdaptiveLearningPath, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.create_path(
                title,
                description,
                author_id,
                subject,
                tags,
                default_study_mode,
                default_visibility,
                is_public,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Add a node to a learning path
    pub async fn add_learning_path_node(
        &self,
        path_id: Uuid,
        title: String,
        description: Option<String>,
        node_type: LearningPathNodeType,
        content_id: Option<Uuid>,
        position_x: f32,
        position_y: f32,
        required_score: Option<f32>,
        custom_data: Option<Value>,
    ) -> Result<LearningPathNode, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.add_node(
                path_id,
                title,
                description,
                node_type,
                content_id,
                position_x,
                position_y,
                required_score,
                custom_data,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Add an edge to a learning path
    pub async fn add_learning_path_edge(
        &self,
        path_id: Uuid,
        source_node_id: Uuid,
        target_node_id: Uuid,
        condition_type: EdgeConditionType,
        condition_value: Option<Value>,
        label: Option<String>,
    ) -> Result<LearningPathEdge, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.add_edge(
                path_id,
                source_node_id,
                target_node_id,
                condition_type,
                condition_value,
                label,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Update a learning path
    pub async fn update_learning_path(
        &self,
        path_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        subject: Option<String>,
        tags: Option<Vec<String>>,
        default_study_mode: Option<StudyMode>,
        default_visibility: Option<QuizVisibility>,
        is_public: Option<bool>,
    ) -> Result<AdaptiveLearningPath, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.update_path(
                path_id,
                title,
                description,
                subject,
                tags,
                default_study_mode,
                default_visibility,
                is_public,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Update a node in a learning path
    pub async fn update_learning_path_node(
        &self,
        node_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        node_type: Option<LearningPathNodeType>,
        content_id: Option<Uuid>,
        position_x: Option<f32>,
        position_y: Option<f32>,
        required_score: Option<f32>,
        custom_data: Option<Value>,
    ) -> Result<LearningPathNode, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.update_node(
                node_id,
                title,
                description,
                node_type,
                content_id,
                position_x,
                position_y,
                required_score,
                custom_data,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Update an edge in a learning path
    pub async fn update_learning_path_edge(
        &self,
        edge_id: Uuid,
        condition_type: Option<EdgeConditionType>,
        condition_value: Option<Value>,
        label: Option<String>,
    ) -> Result<LearningPathEdge, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.update_edge(
                edge_id,
                condition_type,
                condition_value,
                label,
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Delete a learning path
    pub async fn delete_learning_path(
        &self,
        path_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.delete_path(path_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Delete a node from a learning path
    pub async fn delete_learning_path_node(
        &self,
        node_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.delete_node(node_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Delete an edge from a learning path
    pub async fn delete_learning_path_edge(
        &self,
        edge_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.delete_edge(edge_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get a learning path by ID
    pub async fn get_learning_path(
        &self,
        path_id: Uuid,
    ) -> Result<AdaptiveLearningPath, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_path(path_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get all learning paths
    pub async fn get_all_learning_paths(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_all_paths(limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get all public learning paths
    pub async fn get_public_learning_paths(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_public_paths(limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Search learning paths
    pub async fn search_learning_paths(
        &self,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.search_paths(query, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get learning paths by author
    pub async fn get_learning_paths_by_author(
        &self,
        author_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_paths_by_author(author_id, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Start a learning path for a user
    pub async fn start_learning_path(
        &self,
        user_id: Uuid,
        path_id: Uuid,
    ) -> Result<UserLearningPathProgress, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.start_path(user_id, path_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get user progress for a learning path
    pub async fn get_user_learning_path_progress(
        &self,
        user_id: Uuid,
        path_id: Uuid,
    ) -> Result<UserLearningPathProgress, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_user_progress(user_id, path_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get all user progress for a user
    pub async fn get_all_user_learning_path_progress(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserLearningPathProgress>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_all_user_progress(user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Complete a node in a learning path
    pub async fn complete_learning_path_node(
        &self,
        user_id: Uuid,
        path_id: Uuid,
        node_id: Uuid,
        score: Option<f32>,
    ) -> Result<UserLearningPathProgress, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.complete_node(user_id, path_id, node_id, score).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Move to the next node in a learning path
    pub async fn move_to_next_learning_path_node(
        &self,
        user_id: Uuid,
        path_id: Uuid,
    ) -> Result<(UserLearningPathProgress, LearningPathNode), Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.move_to_next_node(user_id, path_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Generate recommendations for a user
    pub async fn generate_learning_path_recommendations(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<LearningPathRecommendation>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.generate_recommendations(user_id, limit).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
    
    /// Get recommendations for a user
    pub async fn get_learning_path_recommendations(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<LearningPathRecommendation>, Box<dyn Error + Send + Sync>> {
        if let Some(adaptive_learning_service) = &self.adaptive_learning_service {
            adaptive_learning_service.get_recommendations(user_id, limit).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Adaptive learning service is not available".into())
        }
    }
}
