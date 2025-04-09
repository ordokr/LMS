use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::user::user::User;
use uuid::Uuid;
use chrono::Utc;

pub struct CanvasConverter;

impl CanvasConverter {
    /// Convert a Canvas discussion topic to our unified Topic model
    pub fn topic_from_canvas(canvas_topic: &canvas_lms::DiscussionTopic) -> Topic {
        let mut topic = Topic {
            id: Uuid::new_v4(),
            title: canvas_topic.title.clone(),
            author_id: Uuid::nil(), // Will need to be resolved
            category_id: None,
            course_id: canvas_topic.course_id.as_ref().map(|id| {
                // Convert Canvas course ID to our UUID format
                // This is a placeholder - you'll need proper conversion
                Uuid::parse_str(id).unwrap_or(Uuid::nil())
            }),
            content: canvas_topic.message.clone().unwrap_or_default(),
            pinned: canvas_topic.pinned.unwrap_or(false),
            locked: canvas_topic.locked.unwrap_or(false),
            created_at: canvas_topic.posted_at.unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            last_post_at: canvas_topic.last_reply_at,
            views: 0, // Canvas doesn't provide view count
            post_count: 0, // Will be populated separately
            is_announcement: canvas_topic.discussion_type == Some("announcement".to_string()),
            allow_rating: false,
            assignment_id: None, // Will need to be resolved
            tags: Vec::new(),
            canvas_topic_id: Some(canvas_topic.id.to_string()),
            discourse_topic_id: None,
            sync_status: crate::models::forum::topic::SyncStatus::SyncedWithCanvas,
        };
        
        // Handle assignment if present
        if let Some(assignment_id) = &canvas_topic.assignment_id {
            // Convert Canvas assignment ID to our UUID format
            // This is a placeholder - you'll need proper conversion
            topic.assignment_id = Some(Uuid::parse_str(assignment_id).unwrap_or(Uuid::nil()));
        }
        
        topic
    }
    
    /// Convert a Canvas discussion entry to our unified Post model
    pub fn post_from_canvas(canvas_entry: &canvas_lms::DiscussionEntry, topic_id: Uuid) -> Post {
        Post {
            id: Uuid::new_v4(),
            topic_id,
            author_id: Uuid::nil(), // Will need to be resolved
            parent_id: canvas_entry.parent_id.as_ref().map(|_| Uuid::nil()), // Will need to be resolved
            content: canvas_entry.message.clone(),
            html_content: None,
            created_at: canvas_entry.created_at,
            updated_at: canvas_entry.updated_at,
            likes: 0, // Canvas doesn't provide like count directly
            is_solution: false,
            score: None,
            read_status: canvas_entry.read_state == Some("read".to_string()),
            attachment_ids: Vec::new(), // Will need to be resolved
            canvas_entry_id: Some(canvas_entry.id.to_string()),
            discourse_post_id: None,
            sync_status: crate::models::forum::post::SyncStatus::SyncedWithCanvas,
        }
    }
}

pub struct DiscourseConverter;

impl DiscourseConverter {
    /// Convert a Discourse topic to our unified Topic model
    pub fn topic_from_discourse(discourse_topic: &discourse::Topic) -> Topic {
        Topic {
            id: Uuid::new_v4(),
            title: discourse_topic.title.clone(),
            author_id: Uuid::nil(), // Will need to be resolved
            category_id: discourse_topic.category_id.map(|_| Uuid::nil()), // Will need to be resolved
            course_id: None,
            content: "".to_string(), // Discourse topics don't contain content directly
            pinned: discourse_topic.pinned.unwrap_or(false),
            locked: discourse_topic.closed.unwrap_or(false),
            created_at: discourse_topic.created_at,
            updated_at: discourse_topic.last_posted_at.unwrap_or_else(Utc::now),
            last_post_at: discourse_topic.last_posted_at,
            views: discourse_topic.views.unwrap_or(0),
            post_count: discourse_topic.posts_count.unwrap_or(0),
            is_announcement: false,
            allow_rating: true,
            assignment_id: None,
            tags: discourse_topic.tags.clone().unwrap_or_default(),
            canvas_topic_id: None,
            discourse_topic_id: Some(discourse_topic.id),
            sync_status: crate::models::forum::topic::SyncStatus::SyncedWithDiscourse,
        }
    }
    
    /// Convert a Discourse post to our unified Post model
    pub fn post_from_discourse(discourse_post: &discourse::Post, topic_id: Uuid) -> Post {
        Post {
            id: Uuid::new_v4(),
            topic_id,
            author_id: Uuid::nil(), // Will need to be resolved
            parent_id: discourse_post.reply_to_post_number.map(|_| Uuid::nil()), // Will need to be resolved
            content: discourse_post.raw.clone(),
            html_content: Some(discourse_post.cooked.clone()),
            created_at: discourse_post.created_at,
            updated_at: discourse_post.updated_at.unwrap_or_else(Utc::now),
            likes: discourse_post.like_count.unwrap_or(0),
            is_solution: false, // Need to check if this is available in Discourse
            score: None,
            read_status: true, // Discourse doesn't track this per-post
            attachment_ids: Vec::new(), // Will need to be resolved
            canvas_entry_id: None,
            discourse_post_id: Some(discourse_post.id),
            sync_status: crate::models::forum::post::SyncStatus::SyncedWithDiscourse,
        }
    }
}