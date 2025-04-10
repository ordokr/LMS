// src/models/discussion.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unified Discussion model
/// Maps between Canvas Discussion and Discourse Topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discussion {
    // Core fields
    pub id: Option<String>,
    pub title: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub creator_id: Option<String>,
    
    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub course_id: Option<String>,
    pub pinned: bool,
    pub locked: bool,
    pub allow_rating: bool,
    pub only_graders_can_rate: bool,
    
    // Discourse-specific fields
    pub discourse_id: Option<String>,
    pub category_id: Option<String>,
    pub slug: String,
    pub views: i32,
    pub posts_count: i32,
    pub closed: bool,
    pub archived: bool,
    pub tags: Vec<String>,
    
    // Integration fields
    pub last_sync: Option<DateTime<Utc>>,
    pub source_system: String,
}

impl Discussion {
    /// Create a new Discussion instance
    pub fn new(
        id: Option<String>,
        title: Option<String>,
        message: Option<String>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        creator_id: Option<String>,
        canvas_id: Option<String>,
        course_id: Option<String>,
        pinned: Option<bool>,
        locked: Option<bool>,
        allow_rating: Option<bool>,
        only_graders_can_rate: Option<bool>,
        discourse_id: Option<String>,
        category_id: Option<String>,
        slug: Option<String>,
        views: Option<i32>,
        posts_count: Option<i32>,
        closed: Option<bool>,
        archived: Option<bool>,
        tags: Option<Vec<String>>,
        last_sync: Option<DateTime<Utc>>,
        source_system: Option<String>,
    ) -> Self {
        let title_str = title.unwrap_or_default();
        let now = Utc::now();
        
        Discussion {
            id,
            title: title_str.clone(),
            message: message.unwrap_or_default(),
            created_at: created_at.unwrap_or(now),
            updated_at: updated_at.unwrap_or(now),
            creator_id,
            canvas_id,
            course_id,
            pinned: pinned.unwrap_or(false),
            locked: locked.unwrap_or(false),
            allow_rating: allow_rating.unwrap_or(false),
            only_graders_can_rate: only_graders_can_rate.unwrap_or(false),
            discourse_id,
            category_id,
            slug: slug.unwrap_or_else(|| Self::generate_slug(&title_str)),
            views: views.unwrap_or(0),
            posts_count: posts_count.unwrap_or(0),
            closed: closed.unwrap_or(false),
            archived: archived.unwrap_or(false),
            tags: tags.unwrap_or_default(),
            last_sync,
            source_system: source_system.unwrap_or_else(|| "canvas".to_string()),
        }
    }
    
    /// Generate a slug from title
    fn generate_slug(title: &str) -> String {
        if title.is_empty() {
            return String::new();
        }
        
        title.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .replace("--", "-")
    }

    /// Create a Discussion from Canvas data
    pub fn from_canvas(canvas_data: &serde_json::Value) -> Self {
        let id = canvas_data["id"].as_str().map(String::from);
        let title = canvas_data["title"].as_str().map(String::from);
        
        let message = canvas_data["message"].as_str()
            .or_else(|| canvas_data["body"].as_str())
            .map(String::from);
            
        let created_at = canvas_data["created_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let updated_at = canvas_data["updated_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let creator_id = canvas_data["user_id"].as_str().map(String::from);
        let course_id = canvas_data["course_id"].as_str().map(String::from);
        let pinned = canvas_data["pinned"].as_bool();
        let locked = canvas_data["locked"].as_bool();
        let allow_rating = canvas_data["allow_rating"].as_bool();
        let only_graders_can_rate = canvas_data["only_graders_can_rate"].as_bool();
        
        Self::new(
            None,
            title,
            message,
            created_at,
            updated_at,
            creator_id,
            id,
            course_id,
            pinned,
            locked,
            allow_rating,
            only_graders_can_rate,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("canvas".to_string()),
        )
    }
    
    /// Create a Discussion from Discourse data
    pub fn from_discourse(discourse_data: &serde_json::Value) -> Self {
        let id = discourse_data["id"].as_str().map(String::from);
        let title = discourse_data["title"].as_str().map(String::from);
        
        let message = discourse_data["raw"].as_str()
            .or_else(|| discourse_data["cooked"].as_str())
            .map(String::from);
            
        let created_at = discourse_data["created_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let updated_at = discourse_data["updated_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let creator_id = discourse_data["creator_id"].as_str()
            .or_else(|| discourse_data["user_id"].as_str())
            .map(String::from);
            
        let category_id = discourse_data["category_id"].as_str().map(String::from);
        let slug = discourse_data["slug"].as_str().map(String::from);
        let views = discourse_data["views"].as_i64().map(|v| v as i32);
        let posts_count = discourse_data["posts_count"].as_i64().map(|v| v as i32);
        let closed = discourse_data["closed"].as_bool();
        let archived = discourse_data["archived"].as_bool();
        
        let tags = discourse_data["tags"].as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            });
        
        Self::new(
            None,
            title,
            message,
            created_at,
            updated_at,
            creator_id,
            None,
            None,
            None,
            None,
            None,
            None,
            id,
            category_id,
            slug,
            views,
            posts_count,
            closed,
            archived,
            tags,
            None,
            Some("discourse".to_string()),
        )
    }
    
    /// Convert to Canvas format
    pub fn to_canvas(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.canvas_id,
            "title": self.title,
            "message": self.message,
            "user_id": self.creator_id,
            "course_id": self.course_id,
            "pinned": self.pinned,
            "locked": self.locked,
            "allow_rating": self.allow_rating,
            "only_graders_can_rate": self.only_graders_can_rate,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
    
    /// Convert to Discourse format
    pub fn to_discourse(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "title": self.title,
            "raw": self.message,
            "category_id": self.category_id,
            "slug": self.slug,
            "closed": self.closed,
            "archived": self.archived,
            "tags": self.tags,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
}
