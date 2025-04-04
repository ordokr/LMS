# Code Relationship Map
_Generated on 2025-04-04_

## Model Relationships

```mermaid
classDiagram
    class Course {
        pub id: Option<i64>
        pub title: String
        pub description: Option<String>
        pub instructor_id: i64
        pub code: String
        pub start_date: Option<String>
        pub end_date: Option<String>
        pub created_at: Option<String>
        pub updated_at: Option<String>
        pub status: CourseStatus
    }
    class Module {
        pub id: Option<i64>
        pub course_id: i64
        pub title: String
        pub description: Option<String>
        pub position: i32
        pub published: bool
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class Assignment {
        pub id: Option<i64>
        pub course_id: i64
        pub title: String
        pub description: String
        pub due_date: Option<String>
        pub points: Option<i32>
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class Submission {
        pub id: Option<i64>
        pub assignment_id: i64
        pub user_id: i64
        pub content: String
        pub submitted_at: Option<String>
        pub grade: Option<f32>
        pub feedback: Option<String>
    }
    class Enrollment {
        pub id: Option<i64>
        pub course_id: i64
        pub user_id: i64
        pub role: EnrollmentRole
        pub enrollment_state: EnrollmentState
        pub limit_privileges_to_course_section: bool
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class CourseStatus {
        Draft
        Active
        Archived
    }
    class EnrollmentRole {
        Student
        Teacher
        TeachingAssistant
        Designer
        Observer
    }
    class ForumCategory {
        pub id: Option<i64>
        pub name: String
        pub description: Option<String>
        pub slug: String
    }
    class ForumTopic {
        pub id: Option<i64>
        pub title: String
        pub category_id: i64
        pub user_id: i64
        pub created_at: String
    }
    class ForumPost {
        pub id: Option<i64>
        pub topic_id: i64
        pub user_id: i64
        pub content: String
        pub created_at: String
    }
    class ForumUserPreferences {
        pub id: Option<i64>
        pub user_id: i64
        pub email_on_reply: bool
        pub email_on_mention: bool
        pub email_digest: bool
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class ForumTrustLevel {
        pub id: Option<i64>
        pub user_id: i64
        pub trust_level: i32
        pub posts_read: i32
        pub posts_created: i32
        pub updated_at: Option<String>
    }
    class User {
        pub id: Option<i64>
        pub username: String
        pub email: String
        #[serde(skip_serializing)]
    pub password_hash: Option<String>
        pub display_name: Option<String>
        pub avatar_url: Option<String>
        pub bio: Option<String>
        pub role: UserRole
        pub created_at: Option<String>
        pub last_login: Option<String>
    }
    class UserRole {
        Student
        Teacher
        Admin
    }
    class UserProfile {
        user: User
        roles: Vec<UserRole>
        forum_trust_level: Option<i32>
    }
    class LoginRequest {
        email: String
        password: String
    }
    class RegisterRequest {
        name: String
        email: String
        password: String
        password_confirmation: String
        username: String
    }
    class AuthResponse {
        pub token: String
        pub user: UserResponse
    }
    class GreetArgs {
        name: &'a str
    }
    class CreateCourseArgs {
        name: &'a str
        description: &'a str
    }
    class ForumThread {
        pub id: i32
        pub title: String
        pub category: String
        pub created_at: String
    }
    class AuditEntry {
        model: String
        file: String
        source: String
        source_file: String
        percentage: u8
        output: String
    }
    class ModelImplementation {
        struct_found: bool
        method_count: usize
        methods_implemented: Vec<String>
    }
    class DashboardStats {
        pub total_users: i64
        pub active_users: i64
        pub new_users_today: i64
        pub total_topics: i64
        pub new_topics_today: i64
        pub total_posts: i64
        pub new_posts_today: i64
        pub total_page_views: i64
        pub page_views_today: i64
        pub pending_reports_count: Option<i64>
        pub popular_topics: Vec<PopularTopic>
        pub top_contributors: Vec<TopContributor>
    }
    class ActivityItem {
        pub id: i64
        pub user_id: i64
        pub username: String
        pub action_type: String
        pub entity_type: String
        pub entity_id: i64
        pub description: String
        pub created_at: String
    }
    class SystemHealth {
        pub status: String
        pub database_status: String
        pub disk_space: String
        pub memory_usage: String
        pub uptime: String
    }
    class UserListResponse {
        pub users: Vec<AdminUserView>
        pub total_users: i32
        pub total_pages: i32
        pub page: i32
        pub per_page: i32
    }
    class AdminUserView {
        pub id: i64
        pub username: String
        pub email: String
        pub display_name: String
        pub avatar_url: Option<String>
        pub is_admin: bool
        pub trust_level: i32
        pub created_at: String
        pub last_seen_at: Option<String>
        pub is_suspended: bool
        pub suspended_until: Option<String>
        pub topics_count: i32
        pub posts_count: i32
    }
    class SuspendUserPayload {
        pub user_id: i64
        pub days: i32
        pub reason: String
    }
    class UpdateRolePayload {
        pub user_id: i64
        pub is_admin: bool
    }
    class MockIntegrationService {
        module_topics: Arc<RefCell<std::collections::HashMap<i64
        Option<Topic>>>>
    }
    class LoginResponse {
        token: String
        
    }
    class RegisterResponse {
        token: String
        
    }
    class UserData {
        pub id: i64
        pub username: String
        pub email: String
        pub display_name: String
        pub avatar_url: Option<String>
        pub is_admin: bool
        pub trust_level: i32
    }
    class AuthData {
        pub token: String
        pub user: UserData
    }
    class CourseDetail {
        id: i64
        title: String
        description: String
        instructor: String
        image_url: Option<String>
        modules: Vec<Module>
    }
    class Config {
        pub canvas: CanvasConfig
        pub discourse: DiscourseConfig
    }
    class CanvasConfig {
        pub api_url: String
        pub api_token: Option<String>
    }
    class DiscourseConfig {
        pub api_url: String
        pub api_key: Option<String>
        pub api_username: Option<String>
    }
    class ForumConfig {
        #[serde(default = "default_categories")]
    pub categories: Hierarchy<Category>
        pub trust_levels: TrustSystem
        #[serde(default)]
    pub plugins: Vec<PluginConfig>
    }
    class Category {
        pub id: Option<i64>
        pub name: String
        pub slug: String
        pub description: Option<String>
        pub color: Option<String>
        pub text_color: Option<String>
        pub parent_id: Option<i64>
        pub position: i32
        pub created_at: DateTime<Utc>
        pub updated_at: DateTime<Utc>
        pub is_deleted: bool
    }
    class TrustSystem {
    }
    class PluginConfig {
    }
    class Hierarchy {
        
    }
    class Args {
        /// List available courses
    #[clap(long)]
    list_courses: bool
        /// Get a specific course by ID
    #[clap(long)]
    course_id: Option<i64>
        /// List forum topics
    #[clap(long)]
    list_topics: bool
        /// Verbose output
    #[clap(short
        long)]
    verbose: bool
    }
    class ForumSettings {
        pub forum_name: String
        pub forum_description: String
        pub logo_url: Option<String>
        pub favicon_url: Option<String>
        pub primary_color: String
        pub allow_guest_view: bool
        pub require_email_verification: bool
        pub allow_registration: bool
        pub allow_file_uploads: bool
        pub max_upload_size_mb: i32
        pub allowed_file_types: Vec<String>
        pub topics_per_page: i32
        pub posts_per_page: i32
        pub min_chars_per_post: i32
        pub max_chars_per_post: i32
        pub enable_user_signatures: bool
    }
    class ForumSettingsUpdate {
        pub forum_name: String
        pub forum_description: String
        pub logo_url: Option<String>
        pub favicon_url: Option<String>
        pub primary_color: String
        pub allow_guest_viewing: bool
        pub allow_registration: bool
        pub require_email_verification: bool
        pub posts_per_page: i32
        pub topics_per_page: i32
        pub max_topic_title_length: i32
        pub min_post_length: i32
        pub max_post_length: i32
    }
    class ReportedContent {
        pub id: i64
        pub reporter_id: i64
        pub reporter_name: String
        pub content_id: i64
        pub content_type: String
        pub parent_id: Option<i64>
        pub content_title: Option<String>
        pub content_excerpt: Option<String>
        pub reason: String
        pub details: Option<String>
        pub status: ReportStatus
        pub created_at: DateTime<Utc>
        pub resolved_by: Option<i64>
        pub resolver_name: Option<String>
        pub resolved_at: Option<DateTime<Utc>>
        pub decision: Option<ReportDecision>
        pub resolution_note: Option<String>
    }
    class ActivityLog {
        pub id: i64
        pub user_id: i64
        pub username: Option<String>
        pub ip_address: Option<String>
        pub activity_type: ActivityType
        pub target_id: Option<i64>
        pub target_name: Option<String>
        pub details: Option<String>
        pub created_at: DateTime<Utc>
    }
    class ActivityLogPage {
        pub logs: Vec<ActivityLog>
        pub total: usize
        pub page: usize
        pub total_pages: usize
    }
    class PopularTopic {
        pub id: i64
        pub title: String
        pub view_count: i32
        pub author_name: String
        pub category_name: String
    }
    class TopContributor {
        pub id: i64
        pub name: String
        pub avatar_url: Option<String>
        pub created_at: DateTime<Utc>
        pub post_count: i32
        pub topic_count: i32
    }
    class ActivityData {
        pub time_series: Vec<TimeSeriesData>
        pub distribution: Vec<DistributionData>
    }
    class TimeSeriesData {
        pub date: String
        pub posts: i32
        pub topics: i32
        pub users: i32
        pub views: i32
    }
    class DistributionData {
        pub label: String
        pub value: i32
        pub color: String
    }
    class UserManagementPage {
        pub users: Vec<crate::models::user::User>
        pub total: usize
        pub page: usize
        pub total_pages: usize
    }
    class NotificationSettings {
        pub user_id: i64
        pub email_notifications_enabled: bool
        pub browser_notifications_enabled: bool
        pub push_notifications_enabled: bool
        pub notification_types: Vec<NotificationType>
        pub quiet_hours_enabled: bool
        pub quiet_hours_start: Option<String>
        // Format: "HH:MM"
    pub quiet_hours_end: Option<String>
        // Format: "HH:MM"
    pub digest_frequency: DigestFrequency
        pub created_at: DateTime<Utc>
        pub updated_at: DateTime<Utc>
    }
    class UserGroup {
        pub id: i64
        pub name: String
        pub description: Option<String>
        pub color: Option<String>
        pub icon: Option<String>
        pub is_visible: bool
        pub is_public: bool
        pub can_self_assign: bool
        pub member_count: i32
        pub created_at: chrono::DateTime<chrono::Utc>
        pub updated_at: chrono::DateTime<chrono::Utc>
    }
    class UserGroupCreate {
        pub name: String
        pub description: Option<String>
        pub color: Option<String>
        pub icon: Option<String>
        pub is_visible: bool
        pub is_public: bool
        pub can_self_assign: bool
    }
    class UserGroupUpdate {
        pub name: String
        pub description: Option<String>
        pub color: Option<String>
        pub icon: Option<String>
        pub is_visible: bool
        pub is_public: bool
        pub can_self_assign: bool
    }
    class GroupMember {
        pub id: i64
        pub group_id: Option<i64>
        pub user_id: Option<i64>
        pub workflow_state: Option<String>
        pub moderator: Option<bool>
        pub created_at: Option<DateTime<Utc>>
        pub updated_at: Option<DateTime<Utc>>
        pub just_created: Option<bool>
        pub sis_batch_id: Option<i64>
        pub sis_import_id: Option<i64>
    }
    class SiteCustomization {
        pub site_name: String
        pub site_tagline: Option<String>
        pub site_description: Option<String>
        pub site_logo_url: Option<String>
        pub site_favicon_url: Option<String>
        pub primary_color: String
        pub secondary_color: String
        pub success_color: String
        pub info_color: String
        pub warning_color: String
        pub danger_color: String
        pub background_color: String
        pub text_color: String
        pub heading_font: String
        pub body_font: String
        pub code_font: String
        pub base_font_size: i32
        pub border_radius: f64
        pub button_style: String
        pub card_style: String
        pub custom_css: Option<String>
        pub custom_header_html: Option<String>
        pub custom_footer_html: Option<String>
    }
    class ExportOptions {
        pub include_users: bool
        pub include_categories: bool
        pub include_tags: bool
        pub include_topics: bool
        pub include_posts: bool
        pub include_uploads: bool
        pub include_settings: bool
        pub format: String
    }
    class ImportOptions {
        pub merge_users: bool
        pub replace_categories: bool
        pub replace_tags: bool
    }
    class ImportStats {
        pub users_imported: usize
        pub categories_imported: usize
        pub tags_imported: usize
        pub topics_imported: usize
        pub posts_imported: usize
    }
    class BackupInfo {
        pub id: String
        pub filename: String
        pub size: usize
        pub format: String
        pub created_at: chrono::DateTime<chrono::Utc>
    }
    class Setting {
        pub id: i64
        pub name: Option<String>
        pub value: Option<String>
        pub description: Option<String>
        pub created_at: Option<DateTime<Utc>>
        pub updated_at: Option<DateTime<Utc>>
    }
    class ReportStatus {
        Pending
        Resolved
        Dismissed
    }
    class ReportDecision {
        Remove
        Warn
        Approve
    }
    class ActivityType {
        UserLogin
        UserLogout
        UserRegistration
        UserProfileUpdate
        TopicCreated
        TopicUpdated
        TopicDeleted
        PostCreated
        PostUpdated
        PostDeleted
        CategoryCreated
        CategoryUpdated
        CategoryDeleted
        ModeratorAction
        AdminAction
        SystemEvent
        Other
    }
    class Tag {
        pub id: Option<i64>
        pub name: String
        pub description: Option<String>
        pub created_at: DateTime<Utc>
        pub updated_at: DateTime<Utc>
        pub is_deleted: bool
    }
    class TagWithTopics {
        pub tag: Tag
        pub recent_topics: Vec<crate::models::forum::Topic>
    }
    class CreateTagRequest {
        pub name: String
        pub description: Option<String>
        pub color: Option<String>
        pub icon: Option<String>
        pub is_restricted: Option<bool>
    }
    class UpdateTagRequest {
        pub name: Option<String>
        pub description: Option<Option<String>>
        // Option<Option<String>> for null values
    pub color: Option<Option<String>>
        pub icon: Option<Option<String>>
        pub is_restricted: Option<bool>
    }
    class FollowedTag {
        pub tag: Tag
        pub notification_level: String
        
        "normal"
        or "high"
    pub followed_at: chrono::DateTime<chrono::Utc>
    }
    class Topic {
        pub id: Option<i64>
        pub title: String
        pub slug: String
        pub category_id: i64
        pub user_id: i64
        pub views: i32
        pub created_at: DateTime<Utc>
        pub updated_at: DateTime<Utc>
        pub last_posted_at: Option<DateTime<Utc>>
        pub is_closed: bool
        pub is_pinned: bool
        pub is_deleted: bool
    }
    class Post {
        pub id: Option<i64>
        pub topic_id: i64
        pub user_id: i64
        pub content: String
        pub content_html: String
        pub created_at: DateTime<Utc>
        pub updated_at: DateTime<Utc>
        pub is_deleted: bool
    }
    class ForumStats {
        pub total_posts: i64
        pub total_topics: i64
        pub total_users: i64
        pub posts_today: i32
        pub active_users_today: i32
    }
    class CreateTopicRequest {
        pub category_id: i64
        pub title: String
        pub content: String
    }
    class CreatePostRequest {
        pub content: String
        pub parent_id: Option<i64>
    }
    class UpdatePostRequest {
        pub content: String
    }
    class TopicSearchResult {
        pub id: i64
        pub title: String
        pub excerpt: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub author_id: i64
        pub author_name: Option<String>
        pub category_id: i64
        pub category_name: Option<String>
        pub reply_count: Option<i64>
    }
    class PostSearchResult {
        pub id: i64
        pub content: String
        pub excerpt: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub author_id: i64
        pub author_name: Option<String>
        pub topic_id: i64
        pub topic_title: Option<String>
    }
    class UserSearchResult {
        pub id: i64
        pub name: String
        pub avatar_url: Option<String>
        pub bio: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub topic_count: Option<i64>
        pub post_count: Option<i64>
    }
    class TopicCreationRequest {
        pub title: String
        pub content: String
        pub category_id: i64
        pub pinned: Option<bool>
        pub locked: Option<bool>
        pub tags: Option<Vec<String>>
        
    }
    class TopicUpdateRequest {
        pub title: Option<String>
        pub content: Option<String>
        pub category_id: Option<i64>
        pub pinned: Option<bool>
        pub locked: Option<bool>
        pub tags: Option<Vec<String>>
        
    }
    class Group {
        pub id: i64
        pub name: Option<String>
        pub display_name: Option<String>
        pub description: Option<String>
        pub members_count: Option<i32>
        pub mentionable_level: Option<i32>
        pub messageable_level: Option<i32>
        pub visibility_level: Option<i32>
        pub primary_group: Option<bool>
        pub title: Option<String>
        pub grant_trust_level: Option<i32>
        pub automatic: Option<bool>
        pub bio_raw: Option<String>
        pub bio_cooked: Option<String>
        pub public_admission: Option<bool>
        pub public_exit: Option<bool>
        pub allow_membership_requests: Option<bool>
        pub full_name: Option<String>
        pub default_notification_level: Option<i32>
        // Canvas-specific fields
    pub context_type: Option<String>
        pub context_id: Option<i64>
        pub max_membership: Option<i32>
        pub is_public: Option<bool>
        pub join_level: Option<String>
    }
    class Site {
        pub id: i64
        pub name: Option<String>
        pub title: Option<String>
        pub description: Option<String>
        pub categories: Option<Vec<Category>>
        pub notification_types: Option<HashMap<String
        i32>>
        pub post_action_types: Option<Vec<PostActionType>>
        pub group_names: Option<Vec<String>>
        pub trust_levels: Option<HashMap<String
        i32>>
        pub archetypes: Option<Vec<String>>
        pub user_tips: Option<HashMap<String
        bool>>
        pub default_archetype: Option<String>
        pub uncategorized_category_id: Option<i64>
        pub is_readonly: Option<bool>
        pub categories_by_id: Option<HashMap<i64
        Category>>
    }
    class SiteFeatures {
        pub tags_enabled: bool
        pub polls_enabled: bool
        pub reactions_enabled: bool
        pub allow_uncategorized: bool
        pub embed_enabled: bool
        pub private_messaging_enabled: bool
        pub secure_uploads: bool
    }
    class PostActionType {
        pub id: i32
        pub name_key: Option<String>
        pub name: Option<String>
        pub description: Option<String>
        pub is_flag: Option<bool>
        pub icon: Option<String>
        pub position: Option<i32>
        pub score_bonus: Option<f32>
    }
    class UserFieldType {
        pub id: i32
        pub name: String
        pub field_type: String
        pub editable: bool
        pub description: Option<String>
        pub required: bool
        pub show_on_profile: bool
        pub show_on_user_card: bool
        pub position: i32
    }
    class SearchResult {
        pub result_type: String
        
        "post"
        "user"
        "category"
    pub id: i64
        pub title: Option<String>
        pub excerpt: Option<String>
        pub highlight: Option<String>
        // HTML highlighted snippet
    pub url: String
        pub category_id: Option<i64>
        pub category_name: Option<String>
        pub category_color: Option<String>
        pub topic_id: Option<i64>
        pub topic_title: Option<String>
        pub author: Option<UserBasic>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub last_activity_at: Option<chrono::DateTime<chrono::Utc>>
        pub reply_count: Option<i32>
        pub view_count: Option<i32>
        pub tags: Option<Vec<String>>
        pub score: Option<f64>
        
    }
    class GroupMembershipLevel {
        NotMember
        Member
        Owner
    }
    class ModuleItem {
        pub id: Option<i64>
        pub module_id: i64
        pub title: String
        pub item_type: ModuleItemType
        pub content_id: Option<i64>
        pub content_type: Option<String>
        pub page_url: Option<String>
        pub external_url: Option<String>
        pub position: i32
        pub indent_level: i32
        pub published: bool
        pub completion_requirement: Option<CompletionRequirement>
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class CompletionRequirement {
        pub requirement_type: CompletionRequirementType
        pub min_score: Option<f64>
        pub completed: bool
    }
    class AssignmentGroup {
        pub id: i64
        pub course_id: i64
        pub name: String
        pub position: Option<i32>
        pub group_weight: Option<f64>
        pub rules: Option<AssignmentGroupRules>
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class AssignmentGroupRules {
        pub drop_lowest: Option<i32>
        pub drop_highest: Option<i32>
        pub never_drop: Vec<i64>
        
    }
    class CourseCreationRequest {
        pub code: String
        pub name: String
        pub description: Option<String>
        pub start_date: Option<String>
        pub end_date: Option<String>
        pub status: CourseStatus
    }
    class ModuleWithItems {
        pub module: Module
        pub items: Vec<ModuleItem>
    }
    class Page {
        pub id: i64
        pub title: Option<String>
        pub body: Option<String>
        pub url: Option<String>
        pub created_at: Option<DateTime<Utc>>
        pub updated_at: Option<DateTime<Utc>>
        pub editing_roles: Option<String>
        pub published: Option<bool>
        pub hide_from_students: Option<bool>
        pub front_page: Option<bool>
        pub locked_for_user: Option<bool>
        pub lock_explanation: Option<String>
    }
    class SubmissionComment {
        pub id: i64
        pub author_id: Option<i64>
        pub author_name: Option<String>
        pub comment: Option<String>
        pub created_at: Option<DateTime<Utc>>
        pub edited_at: Option<DateTime<Utc>>
    }
    class EnrollmentStatus {
        Active
        Invited
        Completed
        Rejected
    }
    class Notification {
        pub id: i64
        pub subject: Option<String>
        pub message: Option<String>
        pub notification_type: Option<String>
        pub read: Option<bool>
        pub created_at: Option<DateTime<Utc>>
        pub user_id: Option<i64>
        pub workflow_state: Option<String>
        pub context_type: Option<String>
        pub context_id: Option<i64>
        pub url: Option<String>
    }
    class NotificationPreference {
        pub id: i64
        pub user_id: Option<i64>
        pub notification_type: Option<String>
        pub frequency: Option<String>
        
        "daily"
        "weekly"
        "never"
    pub communication_channel_id: Option<i64>
    }
    class NotificationPreferences {
        pub user_id: i64
        pub enable_browser_notifications: bool
        pub enable_email_notifications: bool
        pub mentions_notification: bool
        pub replies_notification: bool
        pub quotes_notification: bool
        pub likes_notification: bool
        pub messages_notification: bool
        pub follows_notification: bool
        pub group_mentions_notification: bool
        pub group_messages_notification: bool
        pub digest_emails: String
        
        "daily"
        "weekly"
    }
    class NotificationSummary {
        pub total_count: i32
        pub unread_count: i32
        pub mention_count: i32
        pub message_count: i32
    }
    class NotificationData {
        pub title: Option<String>
        pub message: String
        pub topic_id: Option<i64>
        pub topic_title: Option<String>
        pub post_id: Option<i64>
        pub post_number: Option<i32>
        pub from_user_id: Option<i64>
        pub from_username: Option<String>
        pub from_user_avatar: Option<String>
        pub category_id: Option<i64>
        pub category_name: Option<String>
        pub category_color: Option<String>
        pub tag_name: Option<String>
        pub badge_id: Option<i64>
        pub badge_name: Option<String>
        pub badge_icon: Option<String>
        // Additional fields that might be specific to certain notification types
    pub reaction_type: Option<String>
        pub group_id: Option<i64>
        pub group_name: Option<String>
        pub old_category_id: Option<i64>
        pub old_category_name: Option<String>
        pub user_count: Option<i32>
    }
    class DigestFrequency {
        Never
        Daily
        Weekly
    }
    class NotificationType {
        Reply
        Mention
        Quote
        PrivateMessage
        GroupMention
        Reaction
        TopicCreated
        AdminNotification
        PostEdited
        TopicMoved
        BadgeAwarded
        WelcomeNotification
        SystemNotification
        TaggedUser
    }
    class SearchRequest {
        pub query: String
        pub filter_type: Option<String>
        
        "posts"
        "users"
        "categories"
    pub filter_categories: Option<Vec<i64>>
        pub filter_tags: Option<Vec<String>>
        pub filter_date_from: Option<chrono::DateTime<chrono::Utc>>
        pub filter_date_to: Option<chrono::DateTime<chrono::Utc>>
        pub filter_user_id: Option<i64>
        pub sort_by: Option<String>
        
        "newest"
        "oldest"
        "most_replies"
    pub page: usize
        pub limit: usize
    }
    class SearchResponse {
        pub results: Vec<SearchResult>
        pub total: usize
        pub page: usize
        pub limit: usize
        pub query: String
        pub filters_applied: SearchFilters
        pub execution_time_ms: u64
    }
    class SearchFilters {
        pub filter_type: Option<String>
        pub filter_categories: Option<Vec<i64>>
        pub filter_tags: Option<Vec<String>>
        pub filter_date_from: Option<chrono::DateTime<chrono::Utc>>
        pub filter_date_to: Option<chrono::DateTime<chrono::Utc>>
        pub filter_user_id: Option<i64>
        pub sort_by: Option<String>
    }
    class SearchSuggestion {
        pub text: String
        pub type_: String
        
        "user"
        "tag"
        "category"
    pub id: Option<i64>
        pub url: String
    }
    class SearchStats {
        pub post_count: usize
        pub topic_count: usize
        pub user_count: usize
        pub indexed_up_to: chrono::DateTime<chrono::Utc>
    }
    class Badge {
        pub id: i64
        pub name: Option<String>
        pub description: Option<String>
        pub badge_type_id: Option<i32>
        pub icon: Option<String>
        pub image_url: Option<String>
        pub slug: Option<String>
        pub multiple_grant: Option<bool>
        pub enabled: Option<bool>
        pub allow_title: Option<bool>
        pub stackable: Option<bool>
        pub show_posts: Option<bool>
        pub system: Option<bool>
        pub long_description: Option<String>
        pub image: Option<String>
    }
    class UserUpdateRequest {
        pub name: Option<String>
        pub avatar_url: Option<String>
        pub bio: Option<String>
        pub website: Option<Option<String>>
        // Option<Option<String>> to handle null values
    pub location: Option<Option<String>>
        
    }
    class UserPreferences {
        pub user_id: i64
        // Interface preferences
    pub theme_preference: String
        
        "light"
        "dark"
    pub homepage_view: String
        
        "top"
        "unread"
        "categories"
    pub posts_per_page: i32
        pub compact_view: bool
        pub highlight_new_content: bool
        pub interface_language: String
        
        "fr"
        etc.
    
    // Email preferences
    pub enable_email_notifications: bool
        pub notify_on_reply: bool
        pub notify_on_mention: bool
        pub notify_on_message: bool
        pub digest_emails: String
        
        "daily"
        "weekly"
    pub mailing_list_mode: bool
        // Privacy preferences
    pub hide_profile: bool
        pub hide_online_status: bool
        pub allow_private_messages: bool
        pub hide_activity: bool
        // Content preferences
    pub auto_track_topics: bool
        pub auto_watch_replied: bool
        pub include_toc: bool
        pub default_code_lang: String
        pub link_previews: bool
        pub embedded_media: bool
        pub created_at: chrono::DateTime<chrono::Utc>
        pub updated_at: chrono::DateTime<chrono::Utc>
    }
    class UserPreferencesUpdate {
        // Interface preferences
    pub theme_preference: String
        pub homepage_view: String
        pub posts_per_page: i32
        pub compact_view: bool
        pub highlight_new_content: bool
        pub interface_language: String
        // Email preferences
    pub enable_email_notifications: bool
        pub notify_on_reply: bool
        pub notify_on_mention: bool
        pub notify_on_message: bool
        pub digest_emails: String
        pub mailing_list_mode: bool
        // Privacy preferences
    pub hide_profile: bool
        pub hide_online_status: bool
        pub allow_private messages: bool
        pub hide_activity: bool
        // Content preferences
    pub auto_track_topics: bool
        pub auto_watch_replied: bool
        pub include_toc: bool
        pub default_code_lang: String
        pub link_previews: bool
        pub embedded_media: bool
    }
    class TopicSubscription {
        pub topic_id: i64
        pub topic_title: String
        pub category_name: Option<String>
        pub category_color: Option<String>
        pub notification_level: String
        
        "tracking"
        "normal"
        "muted"
    pub unread_count: Option<i32>
        pub last_activity_at: chrono::DateTime<chrono::Utc>
    }
    class BookmarkedTopic {
        pub id: i64
        pub user_id: i64
        pub topic_id: i64
        pub post_id: Option<i64>
        pub topic_title: String
        pub category_name: Option<String>
        pub category_color: Option<String>
        pub note: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub updated_at: chrono::DateTime<chrono::Utc>
    }
    class ApiClient {
        client: Client
        base_url: String
    }
    class TopicsResponse {
        topic_list: TopicList
    }
    class TopicList {
        topics: Vec<Topic>
    }
    class CategoriesResponse {
        category_list: CategoryList
    }
    class CategoryList {
        categories: Vec<Category>
    }
    class ApiError {
        Offline
        NetworkError(String)
        ServerError(u16
        String)
        AuthError(String)
        Deserialization(String)
        UnexpectedError(String)
    }
    class TopicSearchResultDto {
        pub id: i64
        pub title: String
        pub excerpt: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub author_id: i64
        pub author_name: Option<String>
        pub category_id: i64
        pub category_name: Option<String>
        pub reply_count: Option<i64>
    }
    class PostSearchResultDto {
        pub id: i64
        pub content: String
        pub excerpt: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub author_id: i64
        pub author_name: Option<String>
        pub topic_id: i64
        pub topic_title: Option<String>
    }
    class UserSearchResultDto {
        pub id: i64
        pub name: String
        pub avatar_url: Option<String>
        pub bio: Option<String>
        pub created_at: chrono::DateTime<chrono::Utc>
        pub topic_count: Option<i64>
        pub post_count: Option<i64>
    }
    class SearchResultDto {
        #[serde(rename = "topic")]
    Topic(TopicSearchResultDto)
        #[serde(rename = "post")]
    Post(PostSearchResultDto)
        #[serde(rename = "user")]
    User(UserSearchResultDto)
    }
    class ForumService {
        client: ApiClient
        db: Arc<Database>
        api: Arc<ApiClient>
        sync_manager: Arc<SyncManager>
    }
    class CrossReference {
        pub id: String
        // Use String to support both server IDs and local IDs
    pub source_type: EntityType
        pub source_id: String
        pub target_type: EntityType
        pub target_id: String
        pub created_at: DateTime<Utc>
        pub updated_at: DateTime<Utc>
        pub metadata: Option<serde_json::Value>
    }
    class ActivityEntry {
        pub id: String
        pub user_id: String
        pub entity_type: EntityType
        pub entity_id: String
        pub action_type: ActionType
        pub created_at: DateTime<Utc>
        pub metadata: Option<serde_json::Value>
    }
    class IntegrationService {
        client: ApiClient
        course_service: CourseService
        forum_service: ForumService
        sync_manager: Option<Arc<SyncManager>>
        
    }
    class EntityType {
        Course
        Module
        Assignment
        Category
        Topic
        Post
    }
    class ActionType {
        Created
        Updated
        Deleted
        Viewed
        Commented
        Submitted
        Graded
    }
    class WebSocketService {
        ws: Option<web_sys::WebSocket>
        connected: RwSignal<bool>
        error: RwSignal<Option<String>>
    }
    class WebSocketMessage {
        Notification(Notification)
        OnlineCount(i32)
        TopicUpdated { topic_id: i64
    }
    class LocalStorage {
        storage: Option<Storage>
    }
    class LocalStorageError {
        WindowUnavailable
        StorageUnavailable
        SerializationError(String)
        DeserializationError(String)
        StorageError(String)
        NotFound
    }
    class SyncQueue {
        operations: VecDeque<SyncOperation>
    }
    class SyncOperation {
        pub id: String
        pub device_id: String
        pub user_id: i64
        pub operation_type: OperationType
        pub entity_type: String
        pub entity_id: Option<String>
        pub payload: Value
        pub timestamp: i64
        pub vector_clock: HashMap<String
        i64>
        pub synced: bool
        pub synced_at: Option<i64>
    }
    class SyncState {
        pub last_sync_timestamp: i64
        // Unix timestamp
    pub entities: HashMap<String
        HashMap<i64
        EntityStatus>>
    }
    class EntityStatus {
        /// Entity exists locally and on server (fully synced)
    Synced
        /// Entity has been created locally but not yet on server
    PendingCreate
        /// Entity exists on server but has local modifications
    PendingUpdate
        /// Entity has been marked for deletion locally but not deleted on server
    PendingDelete
        /// Entity exists only on server (not yet downloaded)
    RemoteOnly
    }
    class JwtClaims {
        pub sub: String
        // User ID
    pub name: String
        // User's name
    pub email: String
        // User's email
    pub roles: Vec<String>
        // User's roles
    pub exp: usize
        
    }
    class SyncClient {
        device_id: String
    }
    class PaginationParams {
        #[serde(default = "default_page")]
    pub page: usize
        #[serde(default = "default_per_page")]
    pub per_page: usize
    }
    class CreateCategoryRequest {
        pub name: String
        pub description: Option<String>
        pub course_id: Option<i64>
        pub parent_id: Option<i64>
        pub color: Option<String>
    }
    class TopicWithPosts {
        #[serde(flatten)]
    pub topic: Topic
        pub posts: Vec<Post>
    }
    class AppError {
        #[error("Authentication error: {0
    }
    class PostsQuery {
        page: Option<usize>
        per_page: Option<usize>
    }
    class ActivityQuery {
        limit: Option<usize>
    }
    class CourseFilter {
        pub status: Option<String>
        pub user_id: Option<i64>
    }
    class EnrollmentRequest {
        pub user_id: i64
        pub role: String
        pub status: Option<String>
    }
    class EnrollmentUpdateRequest {
        pub role: Option<String>
        pub status: Option<String>
    }
    class Claims {
        pub sub: String
        // Subject (user ID)
    pub name: String
        // User's full name
    pub email: String
        // User's email
    pub roles: Vec<String>
        // User roles
    pub exp: usize
        // Expiration time (as UTC timestamp)
    pub iat: usize
        // Issued at (as UTC timestamp)
    pub jti: String
        
    }
    class ProjectAnalysis {
        route_components: Vec<String>
        defined_components: Vec<String>
        missing_components: Vec<String>
        name_mismatches: HashMap<String
        String>
    }
    class AuthService {
        jwt_secret: String
        token_expiration: Duration
    }
    class AppConfig {
        pub database: DatabaseConfig
        pub server: ServerConfig
        pub sync: SyncConfig
    }
    class DatabaseConfig {
        pub connection_string: String
        pub max_connections: u32
        pub sqlite_path: String
    }
    class ServerConfig {
        pub host: String
        pub port: u16
        pub jwt_secret: String
        pub jwt_expiration: u64
        
    }
    class SyncConfig {
        pub enabled: bool
        pub sync_interval: u64
        // in seconds
    pub batch_size: u32
        pub sync_endpoint: String
    }
    class ErrorResponse {
        pub error: String
        pub error_code: String
    }
    class AssignmentRepository {
        db: Pool<Sqlite>
        sync_engine: Arc<SyncEngine>
    }
    class CategoryRepository {
        conn: &'a Connection
    }
    class CourseRepository {
        db: Pool<Sqlite>
    }
    class ForumCategoryRepository {
        pool: Pool<Sqlite>
    }
    class ForumTopicRepository {
        pool: Pool<Sqlite>
    }
    class ModuleRepository {
        db: Pool<Sqlite>
        sync_engine: Arc<SyncEngine>
    }
    class PostRepository {
        conn: &'a Connection
    }
    class TopicRepository {
        conn: &'a Connection
    }
    class UserRepository {
        conn: &'a Connection
    }
    class CourseSettings {
        pub id: Option<i64>
        pub course_id: i64
        pub allow_student_discussion_topics: bool
        pub allow_student_discussion_editing: bool
        pub allow_student_forum_attachments: bool
        pub restrict_student_past_view: bool
        pub restrict_student_future_view: bool
        pub hide_final_grades: bool
        pub hide_distribution_graphs: bool
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class CourseSection {
        pub id: Option<i64>
        pub course_id: i64
        pub name: String
        pub start_date: Option<String>
        pub end_date: Option<String>
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class CourseUser {
        pub id: Option<i64>
        pub course_id: i64
        pub user_id: i64
        pub role: CourseUserRole
        pub section_id: Option<i64>
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class CourseUserRole {
        Student
        Teacher
        TeacherAssistant
        Designer
        Observer
    }
    class ModuleItemType {
        Assignment
        Quiz
        File
        Page
        Discussion
        ExternalUrl
        ExternalTool
        Header
    }
    class CompletionRequirementType {
        MustView
        MustSubmit
        MustContribute
        MinScore
        MarkDone
    }
    class SubmissionFile {
        pub id: Option<i64>
        pub submission_id: i64
        pub filename: String
        pub display_name: String
        pub content_type: String
        pub size: i64
        pub url: String
        pub created_at: Option<String>
    }
    class ContentPage {
        pub id: Option<i64>
        pub course_id: i64
        pub title: String
        pub body: String
        pub published: bool
        pub front_page: bool
        pub url: String
        pub created_at: Option<String>
        pub updated_at: Option<String>
    }
    class CourseVisibility {
        Public
        InstitutionOnly
        CourseMembers
    }
    class HomepageType {
        ActivityStream
        Syllabus
        Modules
        Assignments
        CustomPage
    }
    class GradingType {
        Points
        Percentage
        LetterGrade
        GpaScale
        PassFail
        NotGraded
    }
    class SubmissionType {
        None
        OnlineText
        OnlineUrl
        OnlineUpload
        MediaRecording
        Discussion
        Quiz
        ExternalTool
        NotGraded
    }
    class EnrollmentState {
        Active
        Invited
        Inactive
        Completed
        Rejected
    }
    class AppState {
        conn: Arc<Mutex<Connection>>
    }
    class CourseIdParams {
        course_id: i32
    }
    class ThreadIdParams {
        thread_id: i32
    }
    class SubmissionIdParams {
        submission_id: i32
    }
    class StudentIdParams {
        student_id: i32
    }
    class ForumPostRepository {
        conn: Arc<Connection>
    }
    class IntegrationRepository {
        conn: Arc<Connection>
    }
    class RepositoryError {
        #[error("Database error: {0
    }
    class CreateCategoryPayload {
        pub name: String
        pub slug: String
        pub description: Option<String>
        pub parent_id: Option<i64>
    }
    class UpdateCategoryPayload {
        pub name: Option<String>
        pub description: Option<String>
        pub color: Option<String>
        pub text_color: Option<String>
        pub position: Option<i32>
    }
    class CreatePostPayload {
        pub topic_id: i64
        pub user_id: i64
        pub content: String
    }
    class UpdatePostPayload {
        pub content: String
    }
    class CreateTopicPayload {
        pub title: String
        pub category_id: i64
        pub content: String
        // First post content
    pub user_id: i64
        
    }
    class UpdateTopicPayload {
        pub title: Option<String>
        pub is_closed: Option<bool>
        pub is_pinned: Option<bool>
    }
    class TopicQuery {
        #[serde(default = "default_page")]
    pub page: i64
        #[serde(default = "default_per_page")]
    pub per_page: i64
    }
    class RegisterUserPayload {
        pub username: String
        pub email: String
        pub password: String
        pub display_name: Option<String>
    }
    class LoginPayload {
        pub username: String
        pub password: String
    }
    class UserResponse {
        pub id: i64
        pub username: String
        pub email: String
        pub display_name: String
        pub avatar_url: Option<String>
        pub is_admin: bool
        pub trust_level: i32
    }
    class UpdateUserPayload {
        pub display_name: Option<String>
        pub avatar_url: Option<String>
        pub bio: Option<String>
        pub website: Option<String>
        pub location: Option<String>
    }
    class SyncService {
        engine: Arc<SyncEngine>
        sync_endpoint: String
        sync_interval: Duration
    }
    class ConflictType {
        CreateCreate
        // Two creates for the same entity
    CreateUpdate
        // Create and update for the same entity
    CreateDelete
        // Create and delete for the same entity
    UpdateUpdate
        // Two updates for the same entity
    UpdateDelete
        // Update and delete for the same entity
    DeleteDelete
        
    }
    class ConflictResolution {
        KeepFirst
        // Keep the first operation
    KeepSecond
        // Keep the second operation
    Merge
        // Merge the operations
    KeepBoth
        
    }
    class SyncEngine {
        db: Pool<Sqlite>
        device_id: String
        vector_clock: Arc<Mutex<HashMap<String
        i64>>>
    }
    class SyncBatch {
        pub device_id: String
        pub user_id: i64
        pub operations: Vec<SyncOperation>
        pub timestamp: i64
        pub vector_clock: HashMap<String
        i64>
    }
    class OperationType {
        Create
        Update
        Delete
        Reference
    }
    class FileEntry {
        path: String
        is_directory: bool
        size: u64
        children: Vec<FileEntry>
    }
    Course --> CourseStatus
    Enrollment --> EnrollmentRole
    Enrollment --> EnrollmentState
    User --> UserRole
    UserProfile --> User
    UserProfile --> UserRole
    AuthResponse --> User
    AuthResponse --> UserResponse
    DashboardStats --> PopularTopic
    DashboardStats --> TopContributor
    DashboardStats --> Topic
    UserListResponse --> User
    UserListResponse --> AdminUserView
    MockIntegrationService --> Topic
    AuthData --> User
    AuthData --> UserData
    CourseDetail --> Module
    Config --> CanvasConfig
    Config --> DiscourseConfig
    ForumConfig --> Config
    ForumConfig --> Category
    ForumConfig --> TrustSystem
    ForumConfig --> PluginConfig
    ForumConfig --> Hierarchy
    ReportedContent --> ReportStatus
    ReportedContent --> ReportDecision
    ActivityLog --> ActivityType
    ActivityLogPage --> ActivityLog
    ActivityData --> TimeSeriesData
    ActivityData --> DistributionData
    UserManagementPage --> User
    NotificationSettings --> Notification
    NotificationSettings --> DigestFrequency
    NotificationSettings --> NotificationType
    ActivityType --> User
    ActivityType --> UserProfile
    ActivityType --> Category
    ActivityType --> Topic
    ActivityType --> Post
    TagWithTopics --> Tag
    TagWithTopics --> Topic
    FollowedTag --> Tag
    TopicCreationRequest --> Tag
    TopicUpdateRequest --> Tag
    Site --> Category
    Site --> Post
    Site --> PostActionType
    Site --> ActionType
    SearchResult --> User
    ModuleItem --> Module
    ModuleItem --> CompletionRequirement
    ModuleItem --> ModuleItemType
    CompletionRequirement --> CompletionRequirementType
    AssignmentGroup --> Assignment
    AssignmentGroup --> Group
    AssignmentGroup --> AssignmentGroupRules
    AssignmentGroupRules --> Assignment
    CourseCreationRequest --> Course
    CourseCreationRequest --> CourseStatus
    ModuleWithItems --> Module
    ModuleWithItems --> ModuleItem
    NotificationType --> User
    NotificationType --> Tag
    NotificationType --> Topic
    NotificationType --> Post
    NotificationType --> Group
    NotificationType --> Notification
    NotificationType --> Badge
    SearchResponse --> SearchResult
    SearchResponse --> SearchFilters
    TopicsResponse --> Topic
    TopicsResponse --> TopicList
    TopicList --> Topic
    CategoriesResponse --> Category
    CategoriesResponse --> CategoryList
    CategoryList --> Category
    SearchResultDto --> User
    SearchResultDto --> Topic
    SearchResultDto --> Post
    SearchResultDto --> TopicSearchResult
    SearchResultDto --> PostSearchResult
    SearchResultDto --> UserSearchResult
    SearchResultDto --> SearchResult
    SearchResultDto --> TopicSearchResultDto
    SearchResultDto --> PostSearchResultDto
    SearchResultDto --> UserSearchResultDto
    ForumService --> ApiClient
    CrossReference --> EntityType
    ActivityEntry --> EntityType
    ActivityEntry --> ActionType
    IntegrationService --> Course
    IntegrationService --> ApiClient
    IntegrationService --> ForumService
    EntityType --> Course
    EntityType --> Module
    EntityType --> Assignment
    EntityType --> Category
    EntityType --> Topic
    EntityType --> Post
    WebSocketMessage --> Topic
    WebSocketMessage --> Notification
    SyncQueue --> SyncOperation
    SyncOperation --> OperationType
    SyncState --> EntityStatus
    JwtClaims --> User
    TopicWithPosts --> Topic
    TopicWithPosts --> Post
    Claims --> User
    AppConfig --> Config
    AppConfig --> DatabaseConfig
    AppConfig --> ServerConfig
    AppConfig --> SyncConfig
    AssignmentRepository --> SyncEngine
    ModuleRepository --> SyncEngine
    CourseUser --> Course
    CourseUser --> User
    CourseUser --> UserRole
    CourseUser --> CourseUserRole
    ModuleItemType --> Assignment
    ModuleItemType --> Page
    CourseVisibility --> Course
    HomepageType --> Module
    HomepageType --> Assignment
    HomepageType --> Page
    SyncService --> SyncEngine
    SyncBatch --> SyncOperation
```

## Module Structure

```mermaid
flowchart TD
    M1[shared\lib]
    M2[shared\models\course]
    M3[shared\models\forum]
    M4[shared\models\mod]
    M5[shared\models\user]
    M6[shared\src\lib]
    M7[shared\src\models\course]
    M8[shared\src\models\forum]
    M9[shared\src\models\mod]
    M10[shared\src\models\user]
    M11[src\app]
    M12[src\bin\update_audit]
    M13[src\components\admin\categories]
    M14[src\components\admin\dashboard]
    M15[src\components\admin\layout]
    M16[src\components\admin\mod]
    M17[src\components\admin\notification_settings]
    M18[src\components\admin\users]
    M19[src\components\assignment_discussion]
    M20[src\components\assignment_discussions]
    M21[src\components\assignment_discussion_test]
    M22[src\components\auth\login]
    M23[src\components\auth\mod]
    M24[src\components\auth\register]
    M25[src\components\auth]
    M26[src\components\categories]
    M27[src\components\common\mod]
    M28[src\components\common\pagination]
    M29[src\components\courses\course_detail]
    M30[src\components\courses\course_list]
    M31[src\components\courses\mod]
    M32[src\components\course_forum_activity]
    M33[src\components\dashboard]
    M34[src\components\forum\admin\activity_log]
    M35[src\components\forum\admin\admin_layout]
    M36[src\components\forum\admin\category_management]
    M37[src\components\forum\admin\dashboard]
    M38[src\components\forum\admin\forum_settings]
    M39[src\components\forum\admin\import_export]
    M40[src\components\forum\admin\mod]
    M41[src\components\forum\admin\moderation_queue]
    M42[src\components\forum\admin\reported_content]
    M43[src\components\forum\admin\site_customization]
    M44[src\components\forum\admin\user_groups]
    M45[src\components\forum\admin\user_management]
    M46[src\components\forum\all_notifications]
    M47[src\components\forum\categories]
    M48[src\components\forum\categories_list]
    M49[src\components\forum\category_detail]
    M50[src\components\forum\category_form]
    M51[src\components\forum\forum_home]
    M52[src\components\forum\forum_nav]
    M53[src\components\forum\forum_search]
    M54[src\components\forum\forum_threads]
    M55[src\components\forum\group_management]
    M56[src\components\forum\mod]
    M57[src\components\forum\notifications\mod]
    M58[src\components\forum\notifications\notifications_list]
    M59[src\components\forum\notifications\notifications_page]
    M60[src\components\forum\notifications\notification_center]
    M61[src\components\forum\notifications\notification_dropdown]
    M62[src\components\forum\notifications\notification_preferences]
    M63[src\components\forum\notification_indicator]
    M64[src\components\forum\profile_edit]
    M65[src\components\forum\rich_editor]
    M66[src\components\forum\search_bar]
    M67[src\components\forum\tag_analytics]
    M68[src\components\forum\tag_browser]
    M69[src\components\forum\tag_cloud]
    M70[src\components\forum\tag_detail]
    M71[src\components\forum\tag_feed]
    M72[src\components\forum\tag_filter]
    M73[src\components\forum\tag_following]
    M74[src\components\forum\tag_management]
    M75[src\components\forum\tag_selector]
    M76[src\components\forum\threads]
    M77[src\components\forum\thread_detail]
    M78[src\components\forum\topics\bookmark_button]
    M79[src\components\forum\topics\subscription_button]
    M80[src\components\forum\topics\topic_detail]
    M81[src\components\forum\topic_form]
    M82[src\components\forum\user\mod]
    M83[src\components\forum\user\preferences]
    M84[src\components\forum\user\profile]
    M85[src\components\forum\user\subscriptions]
    M86[src\components\forum\user_profile]
    M87[src\components\forum_activity_widget]
    M88[src\components\home]
    M89[src\components\layout\app_layout]
    M90[src\components\layout\footer]
    M91[src\components\layout\header]
    M92[src\components\layout\mod]
    M93[src\components\layout\sidebar]
    M94[src\components\layout]
    M95[src\components\lms\assignments]
    M96[src\components\lms\courses]
    M97[src\components\lms\mod]
    M98[src\components\lms\modules]
    M99[src\components\lms\module_items]
    M100[src\components\mod]
    M101[src\components\module_discussion]
    M102[src\components\module_discussions]
    M103[src\components\module_discussion_test]
    M104[src\components\posts]
    M105[src\components\shared\activity_stream]
    M106[src\components\shared\course_forum_linker]
    M107[src\components\shared\error_display]
    M108[src\components\shared\integration_dashboard]
    M109[src\components\shared\mod]
    M110[src\components\shared\offline_indicator]
    M111[src\components\sync_status]
    M112[src\components\topics]
    M113[src\config]
    M114[src\core\forum]
    M115[src\features\dashboard\dashboard_view]
    M116[src\features\dashboard\mod]
    M117[src\features\mod]
    M118[src\main]
    M119[src\models\admin]
    M120[src\models\forum\tag]
    M121[src\models\forum]
    M122[src\models\lms\tests]
    M123[src\models\lms]
    M124[src\models\mod]
    M125[src\models\notification]
    M126[src\models\search]
    M127[src\models\user]
    M128[src\pages\assignment_detail]
    M129[src\pages\course_detail]
    M130[src\pages\course_forum]
    M131[src\pages\module_detail]
    M132[src\services\admin]
    M133[src\services\api\tests]
    M134[src\services\api]
    M135[src\services\errors]
    M136[src\services\forum]
    M137[src\services\forum_service]
    M138[src\services\integration_service]
    M139[src\services\lms_service]
    M140[src\services\mod]
    M141[src\services\notification]
    M142[src\services\search]
    M143[src\services\user]
    M144[src\services\websocket]
    M145[src\storage\local_storage]
    M146[src\storage\mod]
    M147[src\sync\mod]
    M148[src\sync\sync_queue]
    M149[src\sync\sync_state]
    M150[src\utils\api_client]
    M151[src\utils\auth]
    M152[src\utils\errors]
    M153[src\utils\formatting]
    M154[src\utils\mod]
    M155[src\utils\offline]
    M156[src\utils\sync]
    M157[src-tauri\build]
    M158[src-tauri\src\api\auth]
    M159[src-tauri\src\api\forum]
    M160[src-tauri\src\api\forum_posts]
    M161[src-tauri\src\api\integration]
    M162[src-tauri\src\api\integration_test]
    M163[src-tauri\src\api\lms\assignments]
    M164[src-tauri\src\api\lms\courses]
    M165[src-tauri\src\api\lms\mod]
    M166[src-tauri\src\api\lms\modules]
    M167[src-tauri\src\api\mod]
    M168[src-tauri\src\api\sync]
    M169[src-tauri\src\auth]
    M170[src-tauri\src\bin\analyze_project]
    M171[src-tauri\src\core\auth]
    M172[src-tauri\src\core\config]
    M173[src-tauri\src\core\errors]
    M174[src-tauri\src\core\mod]
    M175[src-tauri\src\database\course]
    M176[src-tauri\src\database\forum]
    M177[src-tauri\src\database\mod]
    M178[src-tauri\src\database\repositories\assignment]
    M179[src-tauri\src\database\repositories\category_repository]
    M180[src-tauri\src\database\repositories\course]
    M181[src-tauri\src\database\repositories\forum]
    M182[src-tauri\src\database\repositories\mod]
    M183[src-tauri\src\database\repositories\module]
    M184[src-tauri\src\database\repositories\post_repository]
    M185[src-tauri\src\database\repositories\topic_repository]
    M186[src-tauri\src\database\repositories\user]
    M187[src-tauri\src\database\repositories\user_repository]
    M188[src-tauri\src\database\repositories]
    M189[src-tauri\src\database\schema\mod]
    M190[src-tauri\src\database\schema]
    M191[src-tauri\src\forum\categories]
    M192[src-tauri\src\forum\mod]
    M193[src-tauri\src\forum\topics]
    M194[src-tauri\src\lib]
    M195[src-tauri\src\lms\models\course]
    M196[src-tauri\src\lms\models\mod]
    M197[src-tauri\src\lms\models\module]
    M198[src-tauri\src\lms\models]
    M199[src-tauri\src\main]
    M200[src-tauri\src\models\category]
    M201[src-tauri\src\models\course]
    M202[src-tauri\src\models\mod]
    M203[src-tauri\src\models\post]
    M204[src-tauri\src\models\tag]
    M205[src-tauri\src\models\topic]
    M206[src-tauri\src\models\user]
    M207[src-tauri\src\repositories\forum_category_repository]
    M208[src-tauri\src\repositories\forum_post_repository]
    M209[src-tauri\src\repositories\forum_topic_repository]
    M210[src-tauri\src\repository\forum_post_repository]
    M211[src-tauri\src\repository\integration_repository]
    M212[src-tauri\src\repository\integration_repository_test]
    M213[src-tauri\src\repository\mod]
    M214[src-tauri\src\routes\categories]
    M215[src-tauri\src\routes\mod]
    M216[src-tauri\src\routes\posts]
    M217[src-tauri\src\routes\topics]
    M218[src-tauri\src\routes\users]
    M219[src-tauri\src\services\mod]
    M220[src-tauri\src\services\sync]
    M221[src-tauri\src\sync\conflicts]
    M222[src-tauri\src\sync\engine]
    M223[src-tauri\src\sync\mod]
    M224[src-tauri\src\sync\operations]
    M225[src-tauri\src\utils\index_project]
    M226[tests\integration\forum_integration_test]
    M227[tools\index_project]
    M228[tools\update_audit]
```

