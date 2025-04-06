// Add this to your existing schema file

table! {
    discussion_mappings (id) {
        id -> Text,
        canvas_discussion_id -> Text,
        discourse_topic_id -> Text,
        course_category_id -> Text,
        title -> Text,
        last_sync -> Timestamp,
        sync_enabled -> Bool,
        sync_posts -> Bool,
        created_at -> Timestamp,
    }
}