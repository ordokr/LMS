use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde_json::Value;
use crate::models::user::User;

/// Represents a forum category
/// Based on Discourse Category and Canvas eportfolio_category models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub parent_category_id: Option<i64>,
    pub topic_count: Option<i32>,
    pub post_count: Option<i32>,
    pub position: Option<i32>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub read_restricted: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    
    // Canvas-specific fields for eportfolio categories
    pub eportfolio_id: Option<i64>,
}

impl Category {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            description: None,
            slug: None,
            parent_category_id: None,
            topic_count: None,
            post_count: None,
            position: None,
            color: None,
            text_color: None,
            read_restricted: None,
            created_at: None, 
            updated_at: None,
            eportfolio_id: None,
        }
    }
    
    /// Find a category by slug
    pub fn find_by_slug(slug: &str) -> Option<Self> {
        // Implementation would connect to backend service
        None
    }
    
    /// Get topics in this category
    pub fn topics(&self) -> Vec<Topic> {
        // Implementation would connect to backend service
        Vec::new()
    }
    
    /// Get subcategories of this category
    pub fn subcategories(&self) -> Vec<Self> {
        // Implementation would connect to backend service
        Vec::new()
    }
}

/// Represents a discussion topic
/// Based on Canvas's DiscussionTopic model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: Option<String>,
    pub message: Option<String>,
    pub html_url: Option<String>,
    pub posted_at: Option<DateTime<Utc>>,
    pub last_reply_at: Option<DateTime<Utc>>,
    pub require_initial_post: Option<bool>,
    pub discussion_subentry_count: Option<i32>,
    pub assignment_id: Option<i64>,
    pub delayed_post_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub pinned: Option<bool>,
    pub locked: Option<bool>,
    pub author_id: Option<i64>,
    pub author: Option<User>,
    pub category_id: Option<i64>,
    pub views: Option<i32>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    pub can_reply: Option<bool>,
}

impl Topic {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: None,
            message: None,
            html_url: None,
            posted_at: None,
            last_reply_at: None,
            require_initial_post: None,
            discussion_subentry_count: None,
            assignment_id: None,
            delayed_post_at: None,
            status: None,
            pinned: None,
            locked: None,
            author_id: None,
            author: None,
            category_id: None,
            views: None,
            closed: None,
            archived: None,
            can_reply: None,
        }
    }
    
    /// Get all posts for this topic
    pub fn posts(&self) -> Vec<Post> {
        // Implementation would connect to backend service
        Vec::new()
    }
    
    /// Check if the topic is visible to a user
    pub fn visible_to(&self, user: &User) -> bool {
        // Implementation for visibility logic
        true
    }
    
    /// Create a new post in this topic
    pub fn create_post(&self, content: &str, user: &User) -> Result<Post, String> {
        // Implementation for creating posts
        Err("Not implemented".to_string())
    }
}

/// Represents a post/reply in a topic
/// Based on Canvas's PostPolicy model and Discourse's Post model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub topic_id: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub post_number: Option<i32>,
    pub reply_count: Option<i32>,
    pub quote_count: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub off_topic_count: Option<i32>,
    pub like_count: Option<i32>,
    pub incoming_link_count: Option<i32>,
    pub bookmark_count: Option<i32>,
    pub score: Option<f32>,
    pub reads: Option<i32>,
    pub post_type: Option<i32>,
    pub sort_order: Option<i32>,
    pub last_editor_id: Option<i64>,
    pub hidden: Option<bool>,
    pub hidden_reason_id: Option<i64>,
    pub notify_moderators_count: Option<i32>,
    pub spam_count: Option<i32>,
    pub illegal_count: Option<i32>,
    pub inappropriate_count: Option<i32>,
    pub raw: Option<String>,
    pub cooked: Option<String>,
    pub reply_to_post_number: Option<i32>,
    pub policy: Option<String>, // Canvas-specific for post policy
}

impl Post {
    pub fn new() -> Self {
        Self {
            id: 0,
            topic_id: None,
            user_id: None,
            created_at: None,
            updated_at: None,
            post_number: None,
            reply_count: None,
            quote_count: None,
            deleted_at: None,
            off_topic_count: None,
            like_count: None,
            incoming_link_count: None,
            bookmark_count: None,
            score: None,
            reads: None,
            post_type: None,
            sort_order: None,
            last_editor_id: None,
            hidden: None,
            hidden_reason_id: None,
            notify_moderators_count: None,
            spam_count: None,
            illegal_count: None,
            inappropriate_count: None,
            raw: None,
            cooked: None,
            reply_to_post_number: None,
            policy: None,
        }
    }
    
    /// Like this post
    pub fn like(&self, user: &User) -> Result<bool, String> {
        // Implementation for liking posts
        Ok(true)
    }
    
    /// Flag this post
    pub fn flag(&self, user: &User, flag_type: &str, message: Option<&str>) -> Result<bool, String> {
        // Implementation for flagging posts
        Ok(true)
    }
    
    /// Get the user who created this post
    pub fn user(&self) -> Option<User> {
        // Implementation would connect to backend service
        None
    }
    
    /// Get topic this post belongs to
    pub fn topic(&self) -> Option<Topic> {
        // Implementation would connect to backend service
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumStats {
    pub total_posts: i64,
    pub total_topics: i64,
    pub total_users: i64,
    pub posts_today: i32,
    pub active_users_today: i32,
}

// Request/response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub category_id: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub topic_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub content: String,
}

// Add these new types to your models/forum.rs

/// Search result enum to represent different types of search results
#[derive(Debug, Clone)]
pub enum SearchResult {
    Topic(TopicSearchResult),
    Post(PostSearchResult),
    User(UserSearchResult),
}

#[derive(Debug, Clone)]
pub struct TopicSearchResult {
    pub id: i64,
    pub title: String,
    pub excerpt: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub reply_count: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PostSearchResult {
    pub id: i64,
    pub content: String,
    pub excerpt: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub topic_id: i64,
    pub topic_title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserSearchResult {
    pub id: i64,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub topic_count: Option<i64>,
    pub post_count: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicCreationRequest {
    pub title: String,
    pub content: String,
    pub category_id: i64,
    pub pinned: Option<bool>,
    pub locked: Option<bool>,
    pub tags: Option<Vec<String>>, // Tag names
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicUpdateRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i64>,
    pub pinned: Option<bool>,
    pub locked: Option<bool>,
    pub tags: Option<Vec<String>>, // Tag names
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub members_count: Option<i32>,
    pub mentionable_level: Option<i32>,
    pub messageable_level: Option<i32>,
    pub visibility_level: Option<i32>,
    pub primary_group: Option<bool>,
    pub title: Option<String>,
    pub grant_trust_level: Option<i32>,
    pub automatic: Option<bool>,
    pub bio_raw: Option<String>,
    pub bio_cooked: Option<String>,
    pub public_admission: Option<bool>,
    pub public_exit: Option<bool>,
    pub allow_membership_requests: Option<bool>,
    pub full_name: Option<String>,
    pub default_notification_level: Option<i32>,
    
    // Canvas-specific fields
    pub context_type: Option<String>,
    pub context_id: Option<i64>,
    pub max_membership: Option<i32>,
    pub is_public: Option<bool>,
    pub join_level: Option<String>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            display_name: None,
            description: None,
            members_count: None,
            mentionable_level: None,
            messageable_level: None,
            visibility_level: None,
            primary_group: None,
            title: None,
            grant_trust_level: None,
            automatic: None,
            bio_raw: None,
            bio_cooked: None,
            public_admission: None,
            public_exit: None,
            allow_membership_requests: None,
            full_name: None,
            default_notification_level: None,
            context_type: None,
            context_id: None,
            max_membership: None,
            is_public: None,
            join_level: None,
        }
    }
    
    /// Find a group by name
    pub fn find_by_name(name: &str) -> Result<Self, String> {
        // Implementation would connect to backend service
        Err("Not implemented".to_string())
    }
    
    /// Get the group's members
    pub fn members(&self) -> Vec<User> {
        // Implementation would connect to backend service
        Vec::new()
    }
    
    /// Add a user to this group
    pub fn add_user(&self, user: &User) -> Result<bool, String> {
        // Implementation would connect to backend service
        Ok(true)
    }
    
    /// Remove a user from this group
    pub fn remove_user(&self, user: &User) -> Result<bool, String> {
        // Implementation would connect to backend service
        Ok(true)
    }
}

/// Represents a Discourse Site configuration
/// Based on Discourse's Site model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: i64,
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub categories: Option<Vec<Category>>,
    pub notification_types: Option<HashMap<String, i32>>,
    pub post_action_types: Option<Vec<PostActionType>>,
    pub group_names: Option<Vec<String>>,
    pub trust_levels: Option<HashMap<String, i32>>,
    pub archetypes: Option<Vec<String>>,
    pub user_tips: Option<HashMap<String, bool>>,
    pub default_archetype: Option<String>,
    pub uncategorized_category_id: Option<i64>,
    pub is_readonly: Option<bool>,
    pub categories_by_id: Option<HashMap<i64, Category>>,
}

impl Site {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            title: None,
            description: None,
            categories: None,
            notification_types: None,
            post_action_types: None,
            group_names: None,
            trust_levels: None,
            archetypes: None,
            user_tips: None,
            default_archetype: None,
            uncategorized_category_id: None,
            is_readonly: None,
            categories_by_id: None,
        }
    }

    /// Creates a singleton instance with the site data
    pub fn create_current(site_data: HashMap<String, Value>) -> Self {
        let mut site = Self::new();
        
        if let Some(name) = site_data.get("name").and_then(|v| v.as_str()) {
            site.name = Some(name.to_string());
        }
        
        if let Some(title) = site_data.get("title").and_then(|v| v.as_str()) {
            site.title = Some(title.to_string());
        }
        
        if let Some(is_readonly) = site_data.get("isReadOnly").and_then(|v| v.as_bool()) {
            site.is_readonly = Some(is_readonly);
        }
        
        site
    }
    
    /// Get categories sorted by count
    pub fn categories_by_count(&self) -> Vec<&Category> {
        match &self.categories {
            Some(categories) => {
                let mut sorted = categories.iter().collect::<Vec<_>>();
                sorted.sort_by(|a, b| b.topic_count.unwrap_or(0).cmp(&a.topic_count.unwrap_or(0)));
                sorted
            }
            None => Vec::new(),
        }
    }
    
    /// Get categories organized by parent ID
    pub fn categories_by_parent_id(&self) -> HashMap<Option<i64>, Vec<&Category>> {
        let mut result: HashMap<Option<i64>, Vec<&Category>> = HashMap::new();
        
        if let Some(categories) = &self.categories {
            for category in categories {
                result.entry(category.parent_category_id).or_insert_with(Vec::new).push(category);
            }
        }
        
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteFeatures {
    pub tags_enabled: bool,
    pub polls_enabled: bool,
    pub reactions_enabled: bool,
    pub allow_uncategorized: bool,
    pub embed_enabled: bool,
    pub private_messaging_enabled: bool,
    pub secure_uploads: bool,
}

/// Represents a type of action that can be taken on a post (like, flag, etc.)
/// Based on Discourse's PostActionType model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostActionType {
    pub id: i32,
    pub name_key: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_flag: Option<bool>,
    pub icon: Option<String>,
    pub position: Option<i32>,
    pub score_bonus: Option<f32>,
}

impl PostActionType {
    pub fn new() -> Self {
        Self {
            id: 0,
            name_key: None,
            name: None,
            description: None,
            is_flag: None,
            icon: None,
            position: None,
            score_bonus: None,
        }
    }
    
    /// Returns all types that are flags
    pub fn flag_types() -> Vec<Self> {
        // Implementation would typically fetch from a service
        vec![
            Self {
                id: 4,
                name_key: Some("inappropriate".to_string()),
                name: Some("Inappropriate".to_string()),
                description: Some("This post contains content that a reasonable person would consider offensive".to_string()),
                is_flag: Some(true),
                icon: Some("flag".to_string()),
                position: Some(1),
                score_bonus: Some(0.0),
            },
            Self {
                id: 8,
                name_key: Some("spam".to_string()),
                name: Some("Spam".to_string()),
                description: Some("This post is an advertisement. It is not useful or relevant".to_string()),
                is_flag: Some(true),
                icon: Some("flag".to_string()),
                position: Some(2),
                score_bonus: Some(0.0),
            }
        ]
    }
    
    /// Find a specific type by ID
    pub fn find_by_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Self {
                id: 1,
                name_key: Some("like".to_string()),
                name: Some("Like".to_string()),
                description: Some("I like this post".to_string()),
                is_flag: Some(false),
                icon: Some("heart".to_string()),
                position: Some(1),
                score_bonus: Some(1.0),
            }),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFieldType {
    pub id: i32,
    pub name: String,
    pub field_type: String,
    pub editable: bool,
    pub description: Option<String>,
    pub required: bool,
    pub show_on_profile: bool,
    pub show_on_user_card: bool,
    pub position: i32,
}

// Add a new member to an enum that might already exist in your code:
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupMembershipLevel {
    NotMember,
    Member,
    Owner,
}