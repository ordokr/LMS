use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discussion {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub content: String,
    pub topic_id: Option<String>,
    pub status: DiscussionStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionCreate {
    pub course_id: String,
    pub title: String,
    pub content: String,
    pub topic_id: Option<String>,
    pub status: Option<DiscussionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscussionStatus {
    Open,
    Locked,
    Archived,
    Pinned,
}

impl ToString for DiscussionStatus {
    fn to_string(&self) -> String {
        match self {
            DiscussionStatus::Open => "open".to_string(),
            DiscussionStatus::Locked => "locked".to_string(),
            DiscussionStatus::Archived => "archived".to_string(),
            DiscussionStatus::Pinned => "pinned".to_string(),
        }
    }
}