use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub is_first_post: bool,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(
        topic_id: Uuid,
        author_id: Uuid,
        content: String,
        is_first_post: bool,
        parent_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            topic_id,
            author_id,
            content,
            is_first_post,
            parent_id,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.html_content = markdown_to_html(&content);
        self.updated_at = Utc::now();
        self.edited = true;
    }
    
    pub fn mark_as_solution(&mut self, is_solution: bool) {
        self.is_solution = is_solution;
        self.updated_at = Utc::now();
    }
}

fn markdown_to_html(markdown: &str) -> String {
    // Simple implementation - in a real app you'd use a proper Markdown processor
    // For now, just do very basic conversion
    let mut html = markdown.replace("\n\n", "</p><p>");
    html = html.replace("\n", "<br>");
    format!("<p>{}</p>", html)
}