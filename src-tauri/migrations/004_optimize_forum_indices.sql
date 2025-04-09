-- Add covering indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_topics_category_updated 
ON topics(category_id, pinned DESC, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_topics_user 
ON topics(user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_posts_topic_created 
ON posts(topic_id, created_at ASC);

-- Add index for text search
CREATE INDEX IF NOT EXISTS idx_topics_title_trgm 
ON topics(title);

-- Add indexes for foreign key relationships
CREATE INDEX IF NOT EXISTS idx_posts_user 
ON posts(user_id);

-- Add index for notification queries
CREATE INDEX IF NOT EXISTS idx_notifications_user_read 
ON notifications(user_id, read, created_at DESC);

-- Optimize table structure by adding statistics
ANALYZE topics;
ANALYZE posts;
ANALYZE categories;
ANALYZE users;

-- Add efficient pagination support with keyset pagination
-- Avoid OFFSET-based pagination which is inefficient for large tables
CREATE VIEW topics_for_pagination AS
SELECT 
    t.id, 
    t.title, 
    t.slug,
    t.pinned,
    t.updated_at,
    t.category_id,
    (SELECT COUNT(*) FROM posts p WHERE p.topic_id = t.id) as post_count
FROM 
    topics t
ORDER BY 
    t.pinned DESC, 
    t.updated_at DESC;