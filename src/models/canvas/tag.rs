use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Tag model - ported from Discourse
/// Represents a tag that can be applied to topics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    // Core fields
    pub id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub topic_count: i32,
    pub target_tag_id: Option<Uuid>,
    pub parent_tag_id: Option<Uuid>,
    
    // Discourse-specific fields
    pub discourse_id: Option<i64>,
    pub discourse_tag_group_id: Option<i64>,
    
    // Common fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tag group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagGroup {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub one_per_topic: bool,
    pub tags: Vec<Uuid>,
    pub discourse_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    /// Create a new tag
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        let slug = name.to_lowercase().replace(" ", "-");
        Self {
            id: Some(Uuid::new_v4()),
            name,
            slug,
            description: None,
            topic_count: 0,
            target_tag_id: None,
            parent_tag_id: None,
            discourse_id: None,
            discourse_tag_group_id: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Create a tag from Discourse data
    pub fn from_discourse(
        name: String,
        discourse_id: i64,
        discourse_tag_group_id: Option<i64>,
        topic_count: i32,
        description: Option<String>,
    ) -> Self {
        let mut tag = Self::new(name);
        tag.discourse_id = Some(discourse_id);
        tag.discourse_tag_group_id = discourse_tag_group_id;
        tag.topic_count = topic_count;
        tag.description = description;
        tag
    }
    
    /// Update tag with new data
    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        topic_count: Option<i32>,
    ) {
        let now = Utc::now();
        if let Some(name) = name {
            self.name = name;
            self.slug = self.name.to_lowercase().replace(" ", "-");
        }
        if description.is_some() {
            self.description = description;
        }
        if let Some(count) = topic_count {
            self.topic_count = count;
        }
        self.updated_at = now;
    }
    
    /// Set the parent tag
    pub fn set_parent(&mut self, parent_tag_id: Uuid) {
        self.parent_tag_id = Some(parent_tag_id);
        self.updated_at = Utc::now();
    }
    
    /// Set the target tag (for synonyms)
    pub fn set_target(&mut self, target_tag_id: Uuid) {
        self.target_tag_id = Some(target_tag_id);
        self.updated_at = Utc::now();
    }
    
    /// Increment the topic count
    pub fn increment_topic_count(&mut self) {
        self.topic_count += 1;
        self.updated_at = Utc::now();
    }
    
    /// Decrement the topic count
    pub fn decrement_topic_count(&mut self) {
        if self.topic_count > 0 {
            self.topic_count -= 1;
            self.updated_at = Utc::now();
        }
    }
}

impl TagGroup {
    /// Create a new tag group
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            name,
            description: None,
            one_per_topic: false,
            tags: Vec::new(),
            discourse_id: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Create a tag group from Discourse data
    pub fn from_discourse(
        name: String,
        discourse_id: i64,
        description: Option<String>,
        one_per_topic: bool,
    ) -> Self {
        let mut tag_group = Self::new(name);
        tag_group.discourse_id = Some(discourse_id);
        tag_group.description = description;
        tag_group.one_per_topic = one_per_topic;
        tag_group
    }
    
    /// Add a tag to the group
    pub fn add_tag(&mut self, tag_id: Uuid) {
        if !self.tags.contains(&tag_id) {
            self.tags.push(tag_id);
            self.updated_at = Utc::now();
        }
    }
    
    /// Remove a tag from the group
    pub fn remove_tag(&mut self, tag_id: Uuid) {
        if let Some(index) = self.tags.iter().position(|id| *id == tag_id) {
            self.tags.remove(index);
            self.updated_at = Utc::now();
        }
    }
    
    /// Update tag group with new data
    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        one_per_topic: Option<bool>,
    ) {
        let now = Utc::now();
        if let Some(name) = name {
            self.name = name;
        }
        if description.is_some() {
            self.description = description;
        }
        if let Some(one_per_topic) = one_per_topic {
            self.one_per_topic = one_per_topic;
        }
        self.updated_at = now;
    }
}
