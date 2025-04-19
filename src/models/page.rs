use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Page model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Unique identifier
    pub id: Uuid,
    /// Page title
    pub title: String,
    /// Page content
    pub content: String,
    /// Course ID
    pub course_id: Uuid,
    /// Published
    pub published: bool,
    /// Front page
    pub front_page: bool,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Canvas ID
    pub canvas_id: Option<String>,
    /// Discourse topic ID
    pub discourse_topic_id: Option<String>,
}

impl Page {
    /// Create a new page
    pub fn new(
        title: String,
        content: String,
        course_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            course_id,
            published: false,
            front_page: false,
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
    
    /// Set front page
    pub fn with_front_page(mut self, front_page: bool) -> Self {
        self.front_page = front_page;
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
    
    /// Update content
    pub fn update_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.updated_at = Utc::now();
    }
    
    /// Update title
    pub fn update_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.updated_at = Utc::now();
    }
}
