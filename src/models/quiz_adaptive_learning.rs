use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::quiz::{StudyMode, QuizVisibility};

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

impl AdaptiveLearningPath {
    pub fn new(
        title: String,
        subject: String,
        default_study_mode: StudyMode,
        default_visibility: QuizVisibility,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            author_id: None,
            subject,
            tags: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            default_study_mode,
            default_visibility,
            is_public: false,
            usage_count: 0,
            rating: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_author(mut self, author_id: Uuid) -> Self {
        self.author_id = Some(author_id);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn add_node(&mut self, node: LearningPathNode) {
        self.nodes.push(node);
        self.updated_at = Utc::now();
    }
    
    pub fn add_edge(&mut self, edge: LearningPathEdge) {
        self.edges.push(edge);
        self.updated_at = Utc::now();
    }
    
    pub fn make_public(&mut self) {
        self.is_public = true;
        self.updated_at = Utc::now();
    }
    
    pub fn make_private(&mut self) {
        self.is_public = false;
        self.updated_at = Utc::now();
    }
    
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
        self.updated_at = Utc::now();
    }
    
    pub fn update_rating(&mut self, new_rating: f32) {
        if let Some(current_rating) = self.rating {
            // Simple average for now
            self.rating = Some((current_rating + new_rating) / 2.0);
        } else {
            self.rating = Some(new_rating);
        }
        self.updated_at = Utc::now();
    }
}

impl LearningPathNode {
    pub fn new(
        path_id: Uuid,
        title: String,
        node_type: LearningPathNodeType,
        position_x: f32,
        position_y: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            path_id,
            title,
            description: None,
            node_type,
            content_id: None,
            position_x,
            position_y,
            required_score: None,
            custom_data: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_content(mut self, content_id: Uuid) -> Self {
        self.content_id = Some(content_id);
        self
    }
    
    pub fn with_required_score(mut self, score: f32) -> Self {
        self.required_score = Some(score);
        self
    }
    
    pub fn with_custom_data(mut self, data: serde_json::Value) -> Self {
        self.custom_data = Some(data);
        self
    }
}

impl LearningPathEdge {
    pub fn new(
        path_id: Uuid,
        source_node_id: Uuid,
        target_node_id: Uuid,
        condition_type: EdgeConditionType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            path_id,
            source_node_id,
            target_node_id,
            condition_type,
            condition_value: None,
            label: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_condition_value(mut self, value: serde_json::Value) -> Self {
        self.condition_value = Some(value);
        self
    }
    
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

impl UserLearningPathProgress {
    pub fn new(
        user_id: Uuid,
        path_id: Uuid,
        start_node_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            path_id,
            current_node_id: start_node_id,
            completed_nodes: Vec::new(),
            scores: std::collections::HashMap::new(),
            started_at: Utc::now(),
            last_activity_at: Utc::now(),
            completed_at: None,
            custom_data: None,
        }
    }
    
    pub fn complete_node(&mut self, node_id: Uuid) {
        if !self.completed_nodes.contains(&node_id) {
            self.completed_nodes.push(node_id);
        }
        self.last_activity_at = Utc::now();
    }
    
    pub fn set_score(&mut self, node_id: String, score: f32) {
        self.scores.insert(node_id, score);
        self.last_activity_at = Utc::now();
    }
    
    pub fn move_to_node(&mut self, node_id: Uuid) {
        self.current_node_id = node_id;
        self.last_activity_at = Utc::now();
    }
    
    pub fn complete_path(&mut self) {
        self.completed_at = Some(Utc::now());
        self.last_activity_at = Utc::now();
    }
    
    pub fn with_custom_data(mut self, data: serde_json::Value) -> Self {
        self.custom_data = Some(data);
        self
    }
}
