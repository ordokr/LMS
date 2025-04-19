use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Topic visibility enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopicVisibility {
    /// Public - visible to everyone
    Public,
    /// Private - visible only to specific users
    Private,
    /// Course - visible only to course members
    Course,
    /// Group - visible only to group members
    Group,
}

impl std::fmt::Display for TopicVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicVisibility::Public => write!(f, "public"),
            TopicVisibility::Private => write!(f, "private"),
            TopicVisibility::Course => write!(f, "course"),
            TopicVisibility::Group => write!(f, "group"),
        }
    }
}

impl From<&str> for TopicVisibility {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "public" => TopicVisibility::Public,
            "private" => TopicVisibility::Private,
            "course" => TopicVisibility::Course,
            "group" => TopicVisibility::Group,
            _ => TopicVisibility::Private,
        }
    }
}

/// Topic status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopicStatus {
    /// Open - active and available for posting
    Open,
    /// Closed - no new posts allowed
    Closed,
    /// Archived - read-only historical record
    Archived,
    /// Deleted - soft-deleted
    Deleted,
}

impl std::fmt::Display for TopicStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicStatus::Open => write!(f, "open"),
            TopicStatus::Closed => write!(f, "closed"),
            TopicStatus::Archived => write!(f, "archived"),
            TopicStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl From<&str> for TopicStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "open" => TopicStatus::Open,
            "closed" => TopicStatus::Closed,
            "archived" => TopicStatus::Archived,
            "deleted" => TopicStatus::Deleted,
            _ => TopicStatus::Open,
        }
    }
}

/// Topic type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopicType {
    /// Regular discussion
    Regular,
    /// Question and answer
    QuestionAnswer,
    /// Linked to assignment
    Assignment,
    /// Announcement
    Announcement,
}

impl std::fmt::Display for TopicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicType::Regular => write!(f, "regular"),
            TopicType::QuestionAnswer => write!(f, "question_answer"),
            TopicType::Assignment => write!(f, "assignment"),
            TopicType::Announcement => write!(f, "announcement"),
        }
    }
}

impl From<&str> for TopicType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "regular" => TopicType::Regular,
            "question_answer" | "question" | "q&a" => TopicType::QuestionAnswer,
            "assignment" => TopicType::Assignment,
            "announcement" => TopicType::Announcement,
            _ => TopicType::Regular,
        }
    }
}

/// Topic model that harmonizes all existing topic/discussion implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    // Core fields
    pub id: String,                           // Primary identifier (UUID)
    pub title: String,                        // Topic title
    pub content: Option<String>,              // Topic content/message
    pub created_at: DateTime<Utc>,            // Creation timestamp
    pub updated_at: DateTime<Utc>,            // Last update timestamp

    // Context relationships
    pub course_id: Option<String>,            // Course ID (if in a course)
    pub category_id: Option<String>,          // Category ID (forum category)
    pub group_id: Option<String>,             // Group ID (if in a group)
    pub author_id: Option<String>,            // Author/creator ID
    pub assignment_id: Option<String>,        // Assignment ID (if linked to assignment)

    // Status and visibility
    pub status: TopicStatus,                  // Topic status
    pub visibility: TopicVisibility,          // Topic visibility
    pub topic_type: TopicType,                // Topic type

    // Flags
    pub is_pinned: bool,                      // Whether the topic is pinned
    pub is_locked: bool,                      // Whether the topic is locked
    pub allow_rating: bool,                   // Whether rating is allowed
    pub require_initial_post: bool,           // Whether viewing requires posting first

    // Dates
    pub posted_at: Option<DateTime<Utc>>,     // When the topic was posted
    pub last_reply_at: Option<DateTime<Utc>>, // When the last reply was made
    pub delayed_post_at: Option<DateTime<Utc>>, // When to post if scheduled

    // Stats
    pub view_count: Option<i32>,              // Number of views
    pub reply_count: Option<i32>,             // Number of replies
    pub participant_count: Option<i32>,       // Number of participants

    // External system IDs
    pub canvas_id: Option<String>,            // Canvas discussion ID
    pub discourse_id: Option<String>,         // Discourse topic ID

    // Additional data
    pub slug: Option<String>,                 // URL-friendly slug
    pub tags: Vec<String>,                    // Topic tags

    // Metadata and extensibility
    pub source_system: Option<String>,        // Source system (canvas, discourse, etc.)
    pub metadata: HashMap<String, serde_json::Value>, // Extensible metadata
}

impl Topic {
    /// Create a new Topic with default values
    pub fn new(
        id: Option<String>,
        title: String,
        content: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        Self {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
            course_id: None,
            category_id: None,
            group_id: None,
            author_id: None,
            assignment_id: None,
            status: TopicStatus::Open,
            visibility: TopicVisibility::Private,
            topic_type: TopicType::Regular,
            is_pinned: false,
            is_locked: false,
            allow_rating: false,
            require_initial_post: false,
            posted_at: Some(now),
            last_reply_at: None,
            delayed_post_at: None,
            view_count: Some(0),
            reply_count: Some(0),
            participant_count: Some(0),
            canvas_id: None,
            discourse_id: None,
            slug: Some(Self::generate_slug(&title)),
            tags: Vec::new(),
            source_system: None,
            metadata: HashMap::new(),
        }
    }

    /// Generate a URL-friendly slug from a title
    fn generate_slug(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .replace(' ', "-")
    }

    /// Create a Topic from a Canvas discussion JSON
    pub fn from_canvas_discussion(canvas_discussion: &serde_json::Value) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let canvas_id = canvas_discussion["id"].as_str()
            .or_else(|| canvas_discussion["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let title = canvas_discussion["title"].as_str().unwrap_or("").to_string();
        let content = canvas_discussion["message"].as_str().map(|s| s.to_string());

        // Parse dates
        let created_at = canvas_discussion["created_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let updated_at = canvas_discussion["updated_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let posted_at = canvas_discussion["posted_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let last_reply_at = canvas_discussion["last_reply_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let delayed_post_at = canvas_discussion["delayed_post_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Parse course ID
        let context_type = canvas_discussion["context_type"].as_str().unwrap_or("");
        let context_id = canvas_discussion["context_id"].as_str()
            .or_else(|| canvas_discussion["context_id"].as_i64().map(|id| id.to_string()));

        let (course_id, group_id) = match context_type {
            "Course" => (context_id, None),
            "Group" => (None, context_id),
            _ => (None, None),
        };

        // Parse assignment ID
        let assignment_id = canvas_discussion["assignment_id"].as_str()
            .or_else(|| canvas_discussion["assignment_id"].as_i64().map(|id| id.to_string()));

        // Parse author ID
        let author_id = canvas_discussion["user_id"].as_str()
            .or_else(|| canvas_discussion["user_id"].as_i64().map(|id| id.to_string()));

        // Parse status and flags
        let workflow_state = canvas_discussion["workflow_state"].as_str().unwrap_or("active");
        let status = match workflow_state {
            "active" => TopicStatus::Open,
            "locked" => TopicStatus::Closed,
            "deleted" => TopicStatus::Deleted,
            _ => TopicStatus::Open,
        };

        let is_locked = canvas_discussion["locked"].as_bool().unwrap_or(false);
        let is_pinned = canvas_discussion["pinned"].as_bool().unwrap_or(false);
        let allow_rating = canvas_discussion["allow_rating"].as_bool().unwrap_or(false);
        let require_initial_post = canvas_discussion["require_initial_post"].as_bool().unwrap_or(false);

        // Parse topic type
        let discussion_type = canvas_discussion["discussion_type"].as_str().unwrap_or("side_comment");
        let topic_type = if assignment_id.is_some() {
            TopicType::Assignment
        } else if discussion_type == "threaded" {
            TopicType::QuestionAnswer
        } else {
            TopicType::Regular
        };

        // Parse stats
        let view_count = None; // Canvas doesn't track views
        let reply_count = canvas_discussion["discussion_subentry_count"].as_i64().map(|c| c as i32);
        let participant_count = None; // Canvas doesn't track participants

        // Convert the canvas_discussion to a HashMap for metadata
        let metadata = serde_json::to_value(canvas_discussion).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();

        Self {
            id,
            title,
            content,
            created_at,
            updated_at,
            course_id,
            category_id: None,
            group_id,
            author_id,
            assignment_id,
            status,
            visibility: TopicVisibility::Course,
            topic_type,
            is_pinned,
            is_locked,
            allow_rating,
            require_initial_post,
            posted_at,
            last_reply_at,
            delayed_post_at,
            view_count,
            reply_count,
            participant_count,
            canvas_id: Some(canvas_id),
            discourse_id: None,
            slug: Some(Self::generate_slug(&title)),
            tags: Vec::new(),
            source_system: Some("canvas".to_string()),
            metadata,
        }
    }

    /// Create a Topic from a Discourse topic JSON
    pub fn from_discourse_topic(discourse_topic: &serde_json::Value) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let discourse_id = discourse_topic["id"].as_str()
            .or_else(|| discourse_topic["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let title = discourse_topic["title"].as_str().unwrap_or("").to_string();
        let content = discourse_topic["raw"].as_str().map(|s| s.to_string())
            .or_else(|| discourse_topic["cooked"].as_str().map(|s| s.to_string()));

        // Parse dates
        let created_at = discourse_topic["created_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let updated_at = discourse_topic["updated_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let posted_at = discourse_topic["created_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let last_reply_at = discourse_topic["last_posted_at"].as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Parse category ID
        let category_id = discourse_topic["category_id"].as_str()
            .or_else(|| discourse_topic["category_id"].as_i64().map(|id| id.to_string()));

        // Parse author ID
        let author_id = discourse_topic["user_id"].as_str()
            .or_else(|| discourse_topic["user_id"].as_i64().map(|id| id.to_string()));

        // Parse status and flags
        let is_closed = discourse_topic["closed"].as_bool().unwrap_or(false);
        let is_archived = discourse_topic["archived"].as_bool().unwrap_or(false);
        let is_deleted = discourse_topic["deleted_at"].is_null() == false;

        let status = if is_deleted {
            TopicStatus::Deleted
        } else if is_archived {
            TopicStatus::Archived
        } else if is_closed {
            TopicStatus::Closed
        } else {
            TopicStatus::Open
        };

        let is_pinned = discourse_topic["pinned"].as_bool().unwrap_or(false) ||
                       discourse_topic["pinned_globally"].as_bool().unwrap_or(false);

        // Parse stats
        let view_count = discourse_topic["views"].as_i64().map(|c| c as i32);
        let reply_count = discourse_topic["posts_count"].as_i64().map(|c| c as i32)
            .map(|c| if c > 0 { c - 1 } else { 0 }); // Subtract 1 for the original post
        let participant_count = discourse_topic["participant_count"].as_i64().map(|c| c as i32);

        // Parse tags
        let tags = if let Some(tags_array) = discourse_topic["tags"].as_array() {
            tags_array.iter()
                .filter_map(|t| t.as_str())
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };

        // Parse slug
        let slug = discourse_topic["slug"].as_str().map(|s| s.to_string());

        // Convert the discourse_topic to a HashMap for metadata
        let metadata = serde_json::to_value(discourse_topic).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();

        Self {
            id,
            title,
            content,
            created_at,
            updated_at,
            course_id: None,
            category_id,
            group_id: None,
            author_id,
            assignment_id: None,
            status,
            visibility: TopicVisibility::Public,
            topic_type: TopicType::Regular,
            is_pinned,
            is_locked: is_closed,
            allow_rating: false,
            require_initial_post: false,
            posted_at,
            last_reply_at,
            delayed_post_at: None,
            view_count,
            reply_count,
            participant_count,
            canvas_id: None,
            discourse_id: Some(discourse_id),
            slug,
            tags,
            source_system: Some("discourse".to_string()),
            metadata,
        }
    }

    /// Convert Topic to Canvas discussion JSON
    pub fn to_canvas_discussion(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.canvas_id,
            "title": self.title,
            "message": self.content,
            "created_at": self.created_at.to_rfc3339(),
            "updated_at": self.updated_at.to_rfc3339(),
            "posted_at": self.posted_at.map(|dt| dt.to_rfc3339()),
            "delayed_post_at": self.delayed_post_at.map(|dt| dt.to_rfc3339()),
            "last_reply_at": self.last_reply_at.map(|dt| dt.to_rfc3339()),
            "require_initial_post": self.require_initial_post,
            "discussion_subentry_count": self.reply_count,
            "read_state": "read",
            "user_id": self.author_id,
            "context_id": self.course_id.clone().or(self.group_id.clone()),
            "context_type": if self.course_id.is_some() { "Course" } else if self.group_id.is_some() { "Group" } else { "Course" },
            "pinned": self.is_pinned,
            "locked": self.is_locked,
            "allow_rating": self.allow_rating,
            "only_graders_can_rate": false,
            "assignment_id": self.assignment_id,
            "workflow_state": match self.status {
                TopicStatus::Open => "active",
                TopicStatus::Closed => "locked",
                TopicStatus::Archived => "locked",
                TopicStatus::Deleted => "deleted"
            },
            "discussion_type": match self.topic_type {
                TopicType::QuestionAnswer => "threaded",
                _ => "side_comment"
            }
        })
    }

    /// Convert Topic to Discourse topic JSON
    pub fn to_discourse_topic(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "title": self.title,
            "raw": self.content,
            "created_at": self.created_at.to_rfc3339(),
            "updated_at": self.updated_at.to_rfc3339(),
            "last_posted_at": self.last_reply_at.map(|dt| dt.to_rfc3339()),
            "category_id": self.category_id,
            "user_id": self.author_id,
            "pinned": self.is_pinned,
            "closed": self.is_locked || self.status == TopicStatus::Closed,
            "archived": self.status == TopicStatus::Archived,
            "deleted_at": if self.status == TopicStatus::Deleted { Some(self.updated_at.to_rfc3339()) } else { None },
            "views": self.view_count,
            "posts_count": self.reply_count.map(|c| c + 1), // Add 1 for the original post
            "participant_count": self.participant_count,
            "slug": self.slug.clone().unwrap_or_else(|| Self::generate_slug(&self.title)),
            "tags": self.tags
        })
    }

    /// Open the topic
    pub fn open(&mut self) {
        self.status = TopicStatus::Open;
        self.is_locked = false;
        self.updated_at = Utc::now();
    }

    /// Close the topic
    pub fn close(&mut self) {
        self.status = TopicStatus::Closed;
        self.is_locked = true;
        self.updated_at = Utc::now();
    }

    /// Archive the topic
    pub fn archive(&mut self) {
        self.status = TopicStatus::Archived;
        self.is_locked = true;
        self.updated_at = Utc::now();
    }

    /// Delete the topic
    pub fn delete(&mut self) {
        self.status = TopicStatus::Deleted;
        self.updated_at = Utc::now();
    }

    /// Pin the topic
    pub fn pin(&mut self) {
        self.is_pinned = true;
        self.updated_at = Utc::now();
    }

    /// Unpin the topic
    pub fn unpin(&mut self) {
        self.is_pinned = false;
        self.updated_at = Utc::now();
    }

    /// Add a tag to the topic
    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
            self.updated_at = Utc::now();
        }
    }

    /// Remove a tag from the topic
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    /// Check if the topic is available for viewing
    pub fn is_available(&self) -> bool {
        self.status != TopicStatus::Deleted
    }

    /// Check if the topic is open for posting
    pub fn is_open_for_posting(&self) -> bool {
        self.status == TopicStatus::Open && !self.is_locked
    }

    /// Check if the topic is linked to an assignment
    pub fn is_assignment(&self) -> bool {
        self.assignment_id.is_some() || self.topic_type == TopicType::Assignment
    }

    /// Increment the view count
    pub fn increment_view_count(&mut self) {
        self.view_count = Some(self.view_count.unwrap_or(0) + 1);
    }

    /// Increment the reply count
    pub fn increment_reply_count(&mut self) {
        self.reply_count = Some(self.reply_count.unwrap_or(0) + 1);
        self.last_reply_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Update the last reply timestamp
    pub fn update_last_reply(&mut self) {
        self.last_reply_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_topic() {
        let topic = Topic::new(
            None,
            "Test Topic".to_string(),
            Some("This is a test topic".to_string()),
        );

        assert_eq!(topic.title, "Test Topic");
        assert_eq!(topic.content, Some("This is a test topic".to_string()));
        assert_eq!(topic.status, TopicStatus::Open);
        assert_eq!(topic.visibility, TopicVisibility::Private);
        assert_eq!(topic.topic_type, TopicType::Regular);
        assert_eq!(topic.is_pinned, false);
        assert_eq!(topic.is_locked, false);
        assert_eq!(topic.allow_rating, false);
        assert_eq!(topic.require_initial_post, false);
        assert_eq!(topic.slug, Some("test-topic".to_string()));
        assert_eq!(topic.tags.len(), 0);
    }

    #[test]
    fn test_from_canvas_discussion() {
        let canvas_json = serde_json::json!({
            "id": "12345",
            "title": "Canvas Discussion",
            "message": "A test discussion from Canvas",
            "created_at": "2023-12-01T00:00:00Z",
            "updated_at": "2023-12-01T12:34:56Z",
            "posted_at": "2023-12-01T00:00:00Z",
            "last_reply_at": "2023-12-02T12:00:00Z",
            "delayed_post_at": "2023-11-30T00:00:00Z",
            "context_id": "67890",
            "context_type": "Course",
            "user_id": "54321",
            "pinned": true,
            "locked": false,
            "allow_rating": true,
            "require_initial_post": true,
            "discussion_type": "threaded",
            "discussion_subentry_count": 10,
            "workflow_state": "active"
        });

        let topic = Topic::from_canvas_discussion(&canvas_json);

        assert_eq!(topic.title, "Canvas Discussion");
        assert_eq!(topic.content, Some("A test discussion from Canvas".to_string()));
        assert_eq!(topic.course_id, Some("67890".to_string()));
        assert_eq!(topic.author_id, Some("54321".to_string()));
        assert_eq!(topic.status, TopicStatus::Open);
        assert_eq!(topic.visibility, TopicVisibility::Course);
        assert_eq!(topic.topic_type, TopicType::QuestionAnswer);
        assert_eq!(topic.is_pinned, true);
        assert_eq!(topic.is_locked, false);
        assert_eq!(topic.allow_rating, true);
        assert_eq!(topic.require_initial_post, true);
        assert_eq!(topic.reply_count, Some(10));
        assert_eq!(topic.canvas_id, Some("12345".to_string()));
        assert_eq!(topic.source_system, Some("canvas".to_string()));
    }

    #[test]
    fn test_from_discourse_topic() {
        let discourse_json = serde_json::json!({
            "id": "67890",
            "title": "Discourse Topic",
            "raw": "A test topic from Discourse",
            "created_at": "2023-12-01T00:00:00Z",
            "updated_at": "2023-12-01T12:34:56Z",
            "last_posted_at": "2023-12-02T12:00:00Z",
            "category_id": "54321",
            "user_id": "12345",
            "pinned": true,
            "closed": false,
            "archived": false,
            "views": 100,
            "posts_count": 11,
            "participant_count": 5,
            "slug": "discourse-topic",
            "tags": ["test", "discourse"]
        });

        let topic = Topic::from_discourse_topic(&discourse_json);

        assert_eq!(topic.title, "Discourse Topic");
        assert_eq!(topic.content, Some("A test topic from Discourse".to_string()));
        assert_eq!(topic.category_id, Some("54321".to_string()));
        assert_eq!(topic.author_id, Some("12345".to_string()));
        assert_eq!(topic.status, TopicStatus::Open);
        assert_eq!(topic.visibility, TopicVisibility::Public);
        assert_eq!(topic.topic_type, TopicType::Regular);
        assert_eq!(topic.is_pinned, true);
        assert_eq!(topic.is_locked, false);
        assert_eq!(topic.view_count, Some(100));
        assert_eq!(topic.reply_count, Some(10)); // posts_count - 1
        assert_eq!(topic.participant_count, Some(5));
        assert_eq!(topic.discourse_id, Some("67890".to_string()));
        assert_eq!(topic.slug, Some("discourse-topic".to_string()));
        assert_eq!(topic.tags, vec!["test".to_string(), "discourse".to_string()]);
        assert_eq!(topic.source_system, Some("discourse".to_string()));
    }

    #[test]
    fn test_to_canvas_discussion() {
        let mut topic = Topic::new(
            Some("abcd1234".to_string()),
            "Test Canvas Discussion".to_string(),
            Some("A test discussion for Canvas".to_string()),
        );

        topic.canvas_id = Some("54321".to_string());
        topic.course_id = Some("67890".to_string());
        topic.author_id = Some("12345".to_string());
        topic.is_pinned = true;
        topic.allow_rating = true;
        topic.require_initial_post = true;
        topic.topic_type = TopicType::QuestionAnswer;
        topic.reply_count = Some(5);

        let canvas_discussion = topic.to_canvas_discussion();

        assert_eq!(canvas_discussion["id"], "54321");
        assert_eq!(canvas_discussion["title"], "Test Canvas Discussion");
        assert_eq!(canvas_discussion["message"], "A test discussion for Canvas");
        assert_eq!(canvas_discussion["context_id"], "67890");
        assert_eq!(canvas_discussion["context_type"], "Course");
        assert_eq!(canvas_discussion["user_id"], "12345");
        assert_eq!(canvas_discussion["pinned"], true);
        assert_eq!(canvas_discussion["allow_rating"], true);
        assert_eq!(canvas_discussion["require_initial_post"], true);
        assert_eq!(canvas_discussion["discussion_subentry_count"], 5);
        assert_eq!(canvas_discussion["discussion_type"], "threaded");
        assert_eq!(canvas_discussion["workflow_state"], "active");
    }

    #[test]
    fn test_to_discourse_topic() {
        let mut topic = Topic::new(
            Some("efgh5678".to_string()),
            "Test Discourse Topic".to_string(),
            Some("A test topic for Discourse".to_string()),
        );

        topic.discourse_id = Some("98765".to_string());
        topic.category_id = Some("54321".to_string());
        topic.author_id = Some("12345".to_string());
        topic.is_pinned = true;
        topic.view_count = Some(50);
        topic.reply_count = Some(10);
        topic.participant_count = Some(5);
        topic.slug = Some("test-discourse-topic".to_string());
        topic.tags = vec!["test".to_string(), "discourse".to_string()];

        let discourse_topic = topic.to_discourse_topic();

        assert_eq!(discourse_topic["id"], "98765");
        assert_eq!(discourse_topic["title"], "Test Discourse Topic");
        assert_eq!(discourse_topic["raw"], "A test topic for Discourse");
        assert_eq!(discourse_topic["category_id"], "54321");
        assert_eq!(discourse_topic["user_id"], "12345");
        assert_eq!(discourse_topic["pinned"], true);
        assert_eq!(discourse_topic["closed"], false);
        assert_eq!(discourse_topic["archived"], false);
        assert_eq!(discourse_topic["views"], 50);
        assert_eq!(discourse_topic["posts_count"], 11); // reply_count + 1
        assert_eq!(discourse_topic["participant_count"], 5);
        assert_eq!(discourse_topic["slug"], "test-discourse-topic");

        let tags = discourse_topic["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&serde_json::json!("test")));
        assert!(tags.contains(&serde_json::json!("discourse")));
    }

    #[test]
    fn test_topic_status_changes() {
        let mut topic = Topic::new(
            None,
            "Test Topic".to_string(),
            Some("This is a test topic".to_string()),
        );

        // Initially open
        assert_eq!(topic.status, TopicStatus::Open);
        assert_eq!(topic.is_locked, false);
        assert!(topic.is_open_for_posting());

        // Close
        topic.close();
        assert_eq!(topic.status, TopicStatus::Closed);
        assert_eq!(topic.is_locked, true);
        assert!(!topic.is_open_for_posting());

        // Open
        topic.open();
        assert_eq!(topic.status, TopicStatus::Open);
        assert_eq!(topic.is_locked, false);
        assert!(topic.is_open_for_posting());

        // Archive
        topic.archive();
        assert_eq!(topic.status, TopicStatus::Archived);
        assert_eq!(topic.is_locked, true);
        assert!(!topic.is_open_for_posting());

        // Delete
        topic.delete();
        assert_eq!(topic.status, TopicStatus::Deleted);
        assert!(!topic.is_available());
        assert!(!topic.is_open_for_posting());
    }

    #[test]
    fn test_topic_pin_unpin() {
        let mut topic = Topic::new(
            None,
            "Test Topic".to_string(),
            Some("This is a test topic".to_string()),
        );

        // Initially not pinned
        assert_eq!(topic.is_pinned, false);

        // Pin
        topic.pin();
        assert_eq!(topic.is_pinned, true);

        // Unpin
        topic.unpin();
        assert_eq!(topic.is_pinned, false);
    }

    #[test]
    fn test_topic_tags() {
        let mut topic = Topic::new(
            None,
            "Test Topic".to_string(),
            Some("This is a test topic".to_string()),
        );

        // Initially no tags
        assert_eq!(topic.tags.len(), 0);

        // Add tags
        topic.add_tag("test");
        topic.add_tag("topic");
        assert_eq!(topic.tags.len(), 2);
        assert!(topic.tags.contains(&"test".to_string()));
        assert!(topic.tags.contains(&"topic".to_string()));

        // Add duplicate tag (should be ignored)
        topic.add_tag("test");
        assert_eq!(topic.tags.len(), 2);

        // Remove tag
        topic.remove_tag("test");
        assert_eq!(topic.tags.len(), 1);
        assert!(topic.tags.contains(&"topic".to_string()));
        assert!(!topic.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_topic_counters() {
        let mut topic = Topic::new(
            None,
            "Test Topic".to_string(),
            Some("This is a test topic".to_string()),
        );

        // Initially zero counts
        assert_eq!(topic.view_count, Some(0));
        assert_eq!(topic.reply_count, Some(0));

        // Increment view count
        topic.increment_view_count();
        topic.increment_view_count();
        assert_eq!(topic.view_count, Some(2));

        // Increment reply count
        topic.increment_reply_count();
        assert_eq!(topic.reply_count, Some(1));
        assert!(topic.last_reply_at.is_some());
    }
}