use super::models::{Quiz, StudyMode, QuizVisibility};
use super::storage::HybridQuizStore;
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

/// Adaptive learning path model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveLearningPath {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Option<Uuid>,
    pub subject: String,
    pub tags: Vec<String>,
    pub nodes: Vec<LearningPathNode>,
    pub edges: Vec<LearningPathEdge>,
    pub default_study_mode: StudyMode,
    pub default_visibility: QuizVisibility,
    pub is_public: bool,
    pub usage_count: i32,
    pub rating: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Learning path node types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LearningPathNodeType {
    Quiz,
    Assessment,
    Content,
    Checkpoint,
    Start,
    End,
    Custom,
}

/// Learning path node model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathNode {
    pub id: Uuid,
    pub path_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub node_type: LearningPathNodeType,
    pub content_id: Option<Uuid>,
    pub position_x: f32,
    pub position_y: f32,
    pub required_score: Option<f32>,
    pub custom_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Learning path edge condition types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeConditionType {
    Score,
    Completion,
    Time,
    Custom,
}

/// Learning path edge model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathEdge {
    pub id: Uuid,
    pub path_id: Uuid,
    pub source_node_id: Uuid,
    pub target_node_id: Uuid,
    pub condition_type: EdgeConditionType,
    pub condition_value: Option<serde_json::Value>,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User learning path progress model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLearningPathProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub path_id: Uuid,
    pub current_node_id: Uuid,
    pub completed_nodes: Vec<Uuid>,
    pub scores: std::collections::HashMap<String, f32>,
    pub started_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub custom_data: Option<serde_json::Value>,
}

/// Learning path recommendation model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathRecommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub path_id: Uuid,
    pub score: f32,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

/// Adaptive learning path service
pub struct AdaptiveLearningService {
    db_pool: SqlitePool,
    quiz_store: Arc<HybridQuizStore>,
}

impl AdaptiveLearningService {
    pub fn new(db_pool: SqlitePool, quiz_store: Arc<HybridQuizStore>) -> Self {
        Self {
            db_pool,
            quiz_store,
        }
    }

    /// Create a new adaptive learning path
    pub async fn create_path(
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
        // Create a new path
        let path = AdaptiveLearningPath {
            id: Uuid::new_v4(),
            title,
            description,
            author_id,
            subject,
            tags,
            nodes: Vec::new(),
            edges: Vec::new(),
            default_study_mode,
            default_visibility,
            is_public,
            usage_count: 0,
            rating: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store the path
        self.store_path(&path).await?;

        Ok(path)
    }

    /// Add a node to a learning path
    pub async fn add_node(
        &self,
        path_id: Uuid,
        title: String,
        description: Option<String>,
        node_type: LearningPathNodeType,
        content_id: Option<Uuid>,
        position_x: f32,
        position_y: f32,
        required_score: Option<f32>,
        custom_data: Option<serde_json::Value>,
    ) -> Result<LearningPathNode, Box<dyn Error + Send + Sync>> {
        // Get the path
        let mut path = self.get_path(path_id).await?;

        // Create a new node
        let node = LearningPathNode {
            id: Uuid::new_v4(),
            path_id,
            title,
            description,
            node_type,
            content_id,
            position_x,
            position_y,
            required_score,
            custom_data,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Add the node to the path
        path.nodes.push(node.clone());
        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(node)
    }

    /// Add an edge to a learning path
    pub async fn add_edge(
        &self,
        path_id: Uuid,
        source_node_id: Uuid,
        target_node_id: Uuid,
        condition_type: EdgeConditionType,
        condition_value: Option<serde_json::Value>,
        label: Option<String>,
    ) -> Result<LearningPathEdge, Box<dyn Error + Send + Sync>> {
        // Get the path
        let mut path = self.get_path(path_id).await?;

        // Check if the source and target nodes exist
        let source_exists = path.nodes.iter().any(|n| n.id == source_node_id);
        let target_exists = path.nodes.iter().any(|n| n.id == target_node_id);

        if !source_exists || !target_exists {
            return Err("Source or target node does not exist".into());
        }

        // Create a new edge
        let edge = LearningPathEdge {
            id: Uuid::new_v4(),
            path_id,
            source_node_id,
            target_node_id,
            condition_type,
            condition_value,
            label,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Add the edge to the path
        path.edges.push(edge.clone());
        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(edge)
    }

    /// Update a learning path
    pub async fn update_path(
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
        // Get the path
        let mut path = self.get_path(path_id).await?;

        // Update the path
        if let Some(title) = title {
            path.title = title;
        }

        if let Some(description) = description {
            path.description = Some(description);
        }

        if let Some(subject) = subject {
            path.subject = subject;
        }

        if let Some(tags) = tags {
            path.tags = tags;
        }

        if let Some(default_study_mode) = default_study_mode {
            path.default_study_mode = default_study_mode;
        }

        if let Some(default_visibility) = default_visibility {
            path.default_visibility = default_visibility;
        }

        if let Some(is_public) = is_public {
            path.is_public = is_public;
        }

        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(path)
    }

    /// Update a node in a learning path
    pub async fn update_node(
        &self,
        node_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        node_type: Option<LearningPathNodeType>,
        content_id: Option<Uuid>,
        position_x: Option<f32>,
        position_y: Option<f32>,
        required_score: Option<f32>,
        custom_data: Option<serde_json::Value>,
    ) -> Result<LearningPathNode, Box<dyn Error + Send + Sync>> {
        // Get the node's path
        let node = self.get_node(node_id).await?;
        let mut path = self.get_path(node.path_id).await?;

        // Find the node in the path
        let node_index = path.nodes.iter().position(|n| n.id == node_id)
            .ok_or("Node not found in path")?;

        // Update the node
        if let Some(title) = title {
            path.nodes[node_index].title = title;
        }

        if let Some(description) = description {
            path.nodes[node_index].description = Some(description);
        }

        if let Some(node_type) = node_type {
            path.nodes[node_index].node_type = node_type;
        }

        path.nodes[node_index].content_id = content_id.or(path.nodes[node_index].content_id);

        if let Some(position_x) = position_x {
            path.nodes[node_index].position_x = position_x;
        }

        if let Some(position_y) = position_y {
            path.nodes[node_index].position_y = position_y;
        }

        path.nodes[node_index].required_score = required_score.or(path.nodes[node_index].required_score);
        path.nodes[node_index].custom_data = custom_data.or(path.nodes[node_index].custom_data.clone());
        path.nodes[node_index].updated_at = Utc::now();
        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(path.nodes[node_index].clone())
    }

    /// Update an edge in a learning path
    pub async fn update_edge(
        &self,
        edge_id: Uuid,
        condition_type: Option<EdgeConditionType>,
        condition_value: Option<serde_json::Value>,
        label: Option<String>,
    ) -> Result<LearningPathEdge, Box<dyn Error + Send + Sync>> {
        // Get the edge's path
        let edge = self.get_edge(edge_id).await?;
        let mut path = self.get_path(edge.path_id).await?;

        // Find the edge in the path
        let edge_index = path.edges.iter().position(|e| e.id == edge_id)
            .ok_or("Edge not found in path")?;

        // Update the edge
        if let Some(condition_type) = condition_type {
            path.edges[edge_index].condition_type = condition_type;
        }

        path.edges[edge_index].condition_value = condition_value.or(path.edges[edge_index].condition_value.clone());
        path.edges[edge_index].label = label.or(path.edges[edge_index].label.clone());
        path.edges[edge_index].updated_at = Utc::now();
        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(path.edges[edge_index].clone())
    }

    /// Delete a learning path
    pub async fn delete_path(
        &self,
        path_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Delete the path
        sqlx::query!(
            r#"
            DELETE FROM quiz_adaptive_learning_paths
            WHERE id = ?
            "#,
            path_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        // Delete all user progress for this path
        sqlx::query!(
            r#"
            DELETE FROM quiz_user_learning_path_progress
            WHERE path_id = ?
            "#,
            path_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        // Delete all recommendations for this path
        sqlx::query!(
            r#"
            DELETE FROM quiz_learning_path_recommendations
            WHERE path_id = ?
            "#,
            path_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Delete a node from a learning path
    pub async fn delete_node(
        &self,
        node_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the node's path
        let node = self.get_node(node_id).await?;
        let mut path = self.get_path(node.path_id).await?;

        // Remove all edges connected to this node
        path.edges.retain(|e| e.source_node_id != node_id && e.target_node_id != node_id);

        // Remove the node
        path.nodes.retain(|n| n.id != node_id);
        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(())
    }

    /// Delete an edge from a learning path
    pub async fn delete_edge(
        &self,
        edge_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the edge's path
        let edge = self.get_edge(edge_id).await?;
        let mut path = self.get_path(edge.path_id).await?;

        // Remove the edge
        path.edges.retain(|e| e.id != edge_id);
        path.updated_at = Utc::now();

        // Store the path
        self.store_path(&path).await?;

        Ok(())
    }
}
