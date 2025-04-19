use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Announcement model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// Unique identifier
    pub id: Uuid,
    /// Announcement title
    pub title: String,
    /// Announcement message
    pub message: String,
    /// Course ID
    pub course_id: Uuid,
    /// Author ID
    pub author_id: Uuid,
    /// Published
    pub published: bool,
    /// Delayed post at
    pub delayed_post_at: Option<DateTime<Utc>>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Canvas ID
    pub canvas_id: Option<String>,
    /// Discourse topic ID
    pub discourse_topic_id: Option<String>,
}

impl Announcement {
    /// Create a new announcement
    pub fn new(
        title: String,
        message: String,
        course_id: Uuid,
        author_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            message,
            course_id,
            author_id,
            published: false,
            delayed_post_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canvas_id: None,
            discourse_topic_id: None,
        }
    }
    
    /// Set published
    pub fn with_published(mut self, published: bool) -> Self {
        self.published = published;
        self
    }
    
    /// Set delayed post at
    pub fn with_delayed_post_at(mut self, delayed_post_at: DateTime<Utc>) -> Self {
        self.delayed_post_at = Some(delayed_post_at);
        self
    }
    
    /// Set Canvas ID
    pub fn with_canvas_id(mut self, canvas_id: &str) -> Self {
        self.canvas_id = Some(canvas_id.to_string());
        self
    }
    
    /// Set Discourse topic ID
    pub fn with_discourse_topic_id(mut self, discourse_topic_id: &str) -> Self {
        self.discourse_topic_id = Some(discourse_topic_id.to_string());
        self
    }
    
    /// Update message
    pub fn update_message(&mut self, message: &str) {
        self.message = message.to_string();
        self.updated_at = Utc::now();
    }
    
    /// Update title
    pub fn update_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.updated_at = Utc::now();
    }
    
    /// Check if the announcement is delayed
    pub fn is_delayed(&self) -> bool {
        if let Some(delayed_post_at) = self.delayed_post_at {
            delayed_post_at > Utc::now()
        } else {
            false
        }
    }
    
    /// Check if the announcement is visible
    pub fn is_visible(&self) -> bool {
        self.published && !self.is_delayed()
    }
}
