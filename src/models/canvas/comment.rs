use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Comment model - ported from Canvas and Discourse
/// Represents a comment on various entities like assignments, discussions, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    // Core fields
    pub id: Option<Uuid>,
    pub author_id: Uuid,
    pub content: String,
    pub html_content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Target entity information
    pub target_type: CommentTargetType,
    pub target_id: Uuid,
    
    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub canvas_author_id: Option<String>,
    pub media_comment_id: Option<String>,
    pub media_comment_type: Option<String>,
    pub attachment_ids: Vec<String>,
    
    // Discourse-specific fields
    pub discourse_id: Option<i64>,
    pub discourse_author_id: Option<i64>,
    pub likes: i32,
    pub hidden: bool,
    pub deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_id: Option<Uuid>,
    
    // Common fields
    pub parent_id: Option<Uuid>,
    pub edited: bool,
    pub edited_at: Option<DateTime<Utc>>,
    pub flagged: bool,
    pub read: bool,
}

/// Comment target type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommentTargetType {
    Assignment,
    Discussion,
    DiscussionEntry,
    Submission,
    Announcement,
    Conversation,
    File,
    Profile,
    Other(String),
}

impl Comment {
    /// Create a new comment
    pub fn new(author_id: Uuid, content: String, target_type: CommentTargetType, target_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            author_id,
            content,
            html_content: None,
            created_at: now,
            updated_at: now,
            target_type,
            target_id,
            canvas_id: None,
            canvas_author_id: None,
            media_comment_id: None,
            media_comment_type: None,
            attachment_ids: Vec::new(),
            discourse_id: None,
            discourse_author_id: None,
            likes: 0,
            hidden: false,
            deleted: false,
            deleted_at: None,
            deleted_by_id: None,
            parent_id: None,
            edited: false,
            edited_at: None,
            flagged: false,
            read: false,
        }
    }
    
    /// Create a comment from Canvas data
    pub fn from_canvas(
        author_id: Uuid,
        content: String,
        target_type: CommentTargetType,
        target_id: Uuid,
        canvas_id: String,
        canvas_author_id: String,
        created_at: DateTime<Utc>,
        media_comment_id: Option<String>,
        media_comment_type: Option<String>,
        attachment_ids: Vec<String>,
    ) -> Self {
        let mut comment = Self::new(author_id, content, target_type, target_id);
        comment.canvas_id = Some(canvas_id);
        comment.canvas_author_id = Some(canvas_author_id);
        comment.created_at = created_at;
        comment.updated_at = created_at;
        comment.media_comment_id = media_comment_id;
        comment.media_comment_type = media_comment_type;
        comment.attachment_ids = attachment_ids;
        comment
    }
    
    /// Create a comment from Discourse data
    pub fn from_discourse(
        author_id: Uuid,
        content: String,
        html_content: String,
        target_type: CommentTargetType,
        target_id: Uuid,
        discourse_id: i64,
        discourse_author_id: i64,
        created_at: DateTime<Utc>,
        likes: i32,
    ) -> Self {
        let mut comment = Self::new(author_id, content, target_type, target_id);
        comment.html_content = Some(html_content);
        comment.discourse_id = Some(discourse_id);
        comment.discourse_author_id = Some(discourse_author_id);
        comment.created_at = created_at;
        comment.updated_at = created_at;
        comment.likes = likes;
        comment
    }
    
    /// Update comment content
    pub fn update_content(&mut self, content: String, html_content: Option<String>) {
        let now = Utc::now();
        self.content = content;
        self.html_content = html_content;
        self.updated_at = now;
        self.edited = true;
        self.edited_at = Some(now);
    }
    
    /// Add a like to the comment
    pub fn add_like(&mut self) {
        self.likes += 1;
        self.updated_at = Utc::now();
    }
    
    /// Remove a like from the comment
    pub fn remove_like(&mut self) {
        if self.likes > 0 {
            self.likes -= 1;
            self.updated_at = Utc::now();
        }
    }
    
    /// Mark the comment as deleted
    pub fn mark_deleted(&mut self, deleted_by_id: Option<Uuid>) {
        let now = Utc::now();
        self.deleted = true;
        self.deleted_at = Some(now);
        self.deleted_by_id = deleted_by_id;
        self.updated_at = now;
    }
    
    /// Mark the comment as hidden
    pub fn mark_hidden(&mut self) {
        self.hidden = true;
        self.updated_at = Utc::now();
    }
    
    /// Mark the comment as flagged
    pub fn mark_flagged(&mut self) {
        self.flagged = true;
        self.updated_at = Utc::now();
    }
    
    /// Mark the comment as read
    pub fn mark_read(&mut self) {
        self.read = true;
        self.updated_at = Utc::now();
    }
    
    /// Add an attachment to the comment
    pub fn add_attachment(&mut self, attachment_id: String) {
        self.attachment_ids.push(attachment_id);
        self.updated_at = Utc::now();
    }
    
    /// Set the parent comment
    pub fn set_parent(&mut self, parent_id: Uuid) {
        self.parent_id = Some(parent_id);
        self.updated_at = Utc::now();
    }
}
