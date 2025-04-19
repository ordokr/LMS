use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Discourse user model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseUser {
    pub id: i64,
    pub username: String,
    pub name: Option<String>,
    pub avatar_template: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub last_emailed_at: Option<DateTime<Utc>>,
    pub trust_level: Option<i32>,
    pub moderator: Option<bool>,
    pub admin: Option<bool>,
    pub title: Option<String>,
    pub badge_count: Option<i32>,
    pub custom_fields: Option<HashMap<String, Value>>,
    pub featured_topic: Option<Value>,
    pub staged: Option<bool>,
    pub email: Option<String>,
    pub secondary_emails: Option<Vec<String>>,
    pub unconfirmed_emails: Option<Vec<String>>,
    pub associated_accounts: Option<Vec<Value>>,
    pub bio_raw: Option<String>,
    pub bio_cooked: Option<String>,
    pub bio_excerpt: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub website_name: Option<String>,
    pub profile_background: Option<String>,
    pub card_background: Option<String>,
    pub timezone: Option<String>,
    pub invited_by: Option<Value>,
    pub groups: Option<Vec<DiscourseGroup>>,
    pub group_users: Option<Vec<DiscourseGroupUser>>,
    pub user_option: Option<DiscourseUserOption>,
}

/// Discourse group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseGroup {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
    pub bio_raw: Option<String>,
    pub bio_cooked: Option<String>,
    pub bio_excerpt: Option<String>,
    pub automatic: Option<bool>,
    pub user_count: Option<i32>,
    pub mentionable_level: Option<i32>,
    pub messageable_level: Option<i32>,
    pub visibility_level: Option<i32>,
    pub primary_group: Option<bool>,
    pub title: Option<String>,
    pub grant_trust_level: Option<i32>,
    pub incoming_email: Option<String>,
    pub has_messages: Option<bool>,
    pub flair_url: Option<String>,
    pub flair_bg_color: Option<String>,
    pub flair_color: Option<String>,
    pub public_admission: Option<bool>,
    pub public_exit: Option<bool>,
    pub allow_membership_requests: Option<bool>,
    pub default_notification_level: Option<i32>,
}

/// Discourse group user model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseGroupUser {
    pub group_id: i64,
    pub user_id: i64,
    pub notification_level: Option<i32>,
    pub owner: Option<bool>,
}

/// Discourse user option model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseUserOption {
    pub user_id: i64,
    pub mailing_list_mode: Option<bool>,
    pub email_digests: Option<bool>,
    pub email_level: Option<i32>,
    pub email_messages_level: Option<i32>,
    pub external_links_in_new_tab: Option<bool>,
    pub dynamic_favicon: Option<bool>,
    pub enable_quoting: Option<bool>,
    pub disable_jump_reply: Option<bool>,
    pub automatically_unpin_topics: Option<bool>,
    pub digest_after_minutes: Option<i32>,
    pub auto_track_topics_after_msecs: Option<i64>,
    pub new_topic_duration_minutes: Option<i32>,
    pub last_redirected_to_top_at: Option<DateTime<Utc>>,
    pub email_previous_replies: Option<i32>,
    pub email_in_reply_to: Option<bool>,
    pub like_notification_frequency: Option<i32>,
    pub include_tl0_in_digests: Option<bool>,
    pub theme_ids: Option<Vec<i64>>,
    pub theme_key_seq: Option<i32>,
    pub allow_private_messages: Option<bool>,
    pub homepage_id: Option<i64>,
    pub hide_profile_and_presence: Option<bool>,
    pub text_size: Option<String>,
    pub text_size_seq: Option<i32>,
    pub title_count_mode: Option<String>,
    pub timezone: Option<String>,
    pub skip_new_user_tips: Option<bool>,
    pub color_scheme_id: Option<i64>,
    pub dark_scheme_id: Option<i64>,
}

/// Discourse category model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseCategory {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub slug: String,
    pub topic_count: Option<i32>,
    pub post_count: Option<i32>,
    pub position: Option<i32>,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub description_excerpt: Option<String>,
    pub topic_url: Option<String>,
    pub read_restricted: Option<bool>,
    pub permission: Option<i32>,
    pub notification_level: Option<i32>,
    pub can_edit: Option<bool>,
    pub topic_template: Option<String>,
    pub has_children: Option<bool>,
    pub sort_order: Option<String>,
    pub sort_ascending: Option<bool>,
    pub show_subcategory_list: Option<bool>,
    pub num_featured_topics: Option<i32>,
    pub default_view: Option<String>,
    pub subcategory_list_style: Option<String>,
    pub default_top_period: Option<String>,
    pub default_list_filter: Option<String>,
    pub minimum_required_tags: Option<i32>,
    pub navigate_to_first_post_after_read: Option<bool>,
    pub custom_fields: Option<HashMap<String, Value>>,
    pub allowed_tags: Option<Vec<String>>,
    pub allowed_tag_groups: Option<Vec<String>>,
    pub allow_global_tags: Option<bool>,
    pub required_tag_groups: Option<Vec<DiscourseRequiredTagGroup>>,
    pub parent_category_id: Option<i64>,
    pub subcategory_ids: Option<Vec<i64>>,
}

/// Discourse required tag group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseRequiredTagGroup {
    pub name: String,
    pub min_count: i32,
}

/// Discourse topic model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTopic {
    pub id: i64,
    pub title: String,
    pub fancy_title: Option<String>,
    pub slug: String,
    pub posts_count: Option<i32>,
    pub reply_count: Option<i32>,
    pub highest_post_number: Option<i32>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub bumped: Option<bool>,
    pub bumped_at: Option<DateTime<Utc>>,
    pub archetype: Option<String>,
    pub unseen: Option<bool>,
    pub pinned: Option<bool>,
    pub unpinned: Option<bool>,
    pub visible: Option<bool>,
    pub closed: Option<bool>,
    pub archived: Option<bool>,
    pub bookmarked: Option<bool>,
    pub liked: Option<bool>,
    pub views: Option<i32>,
    pub like_count: Option<i32>,
    pub has_summary: Option<bool>,
    pub last_poster_username: Option<String>,
    pub category_id: i64,
    pub pinned_globally: Option<bool>,
    pub featured_link: Option<String>,
    pub posters: Option<Vec<DiscoursePoster>>,
    pub participants: Option<Vec<DiscourseParticipant>>,
    pub tags: Option<Vec<String>>,
    pub tags_descriptions: Option<HashMap<String, String>>,
    pub user_id: i64,
    pub draft: Option<String>,
    pub draft_key: Option<String>,
    pub draft_sequence: Option<i32>,
    pub posted: Option<bool>,
    pub unpinned_at: Option<DateTime<Utc>>,
    pub pinned_at: Option<DateTime<Utc>>,
    pub pinned_until: Option<DateTime<Utc>>,
    pub details: Option<DiscourseTopicDetails>,
    pub current_post_number: Option<i32>,
    pub chunk_size: Option<i32>,
    pub message_bus_last_id: Option<i64>,
    pub first_post: Option<DiscoursePost>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Discourse poster model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoursePoster {
    pub extras: Option<String>,
    pub description: Option<String>,
    pub user_id: i64,
    pub primary_group_id: Option<i64>,
}

/// Discourse participant model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseParticipant {
    pub id: i64,
    pub username: String,
    pub name: Option<String>,
    pub avatar_template: Option<String>,
    pub post_count: Option<i32>,
    pub primary_group_name: Option<String>,
    pub primary_group_flair_url: Option<String>,
    pub primary_group_flair_color: Option<String>,
    pub primary_group_flair_bg_color: Option<String>,
}

/// Discourse topic details model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTopicDetails {
    pub created_by: Option<DiscourseUser>,
    pub last_poster: Option<DiscourseUser>,
    pub participants: Option<Vec<DiscourseParticipant>>,
    pub notification_level: Option<i32>,
    pub can_move_posts: Option<bool>,
    pub can_edit: Option<bool>,
    pub can_delete: Option<bool>,
    pub can_recover: Option<bool>,
    pub can_remove_allowed_users: Option<bool>,
    pub can_invite_to: Option<bool>,
    pub can_invite_via_email: Option<bool>,
    pub can_create_post: Option<bool>,
    pub can_reply_as_new_topic: Option<bool>,
    pub can_flag_topic: Option<bool>,
}

/// Discourse post model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoursePost {
    pub id: i64,
    pub name: Option<String>,
    pub username: String,
    pub avatar_template: Option<String>,
    pub created_at: DateTime<Utc>,
    pub cooked: String,
    pub post_number: i32,
    pub post_type: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
    pub reply_count: Option<i32>,
    pub reply_to_post_number: Option<i32>,
    pub quote_count: Option<i32>,
    pub incoming_link_count: Option<i32>,
    pub reads: Option<i32>,
    pub readers_count: Option<i32>,
    pub score: Option<f64>,
    pub yours: Option<bool>,
    pub topic_id: i64,
    pub topic_slug: Option<String>,
    pub display_username: Option<String>,
    pub primary_group_name: Option<String>,
    pub primary_group_flair_url: Option<String>,
    pub primary_group_flair_bg_color: Option<String>,
    pub primary_group_flair_color: Option<String>,
    pub version: Option<i32>,
    pub can_edit: Option<bool>,
    pub can_delete: Option<bool>,
    pub can_recover: Option<bool>,
    pub can_wiki: Option<bool>,
    pub read: Option<bool>,
    pub user_title: Option<String>,
    pub actions_summary: Option<Vec<DiscourseActionSummary>>,
    pub moderator: Option<bool>,
    pub admin: Option<bool>,
    pub staff: Option<bool>,
    pub user_id: i64,
    pub hidden: bool,
    pub trust_level: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub user_deleted: Option<bool>,
    pub edit_reason: Option<String>,
    pub can_view_edit_history: Option<bool>,
    pub wiki: Option<bool>,
    pub reviewable_id: Option<i64>,
    pub reviewable_score_count: Option<i32>,
    pub reviewable_score_pending_count: Option<i32>,
    pub raw: String,
    pub like_count: Option<i32>,
    pub has_flags: bool,
}

/// Discourse action summary model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseActionSummary {
    pub id: i32,
    pub can_act: bool,
    pub count: Option<i32>,
    pub hidden: Option<bool>,
    pub can_defer_flags: Option<bool>,
}

/// Discourse tag model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTag {
    pub id: String,
    pub name: String,
    pub topic_count: i32,
    pub staff: Option<bool>,
    pub tag_group_id: Option<i64>,
}

/// Discourse tag group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTagGroup {
    pub id: String,
    pub name: String,
    pub tag_names: Vec<String>,
    pub parent_tag_name: Option<String>,
    pub one_per_topic: bool,
    pub description: Option<String>,
}
