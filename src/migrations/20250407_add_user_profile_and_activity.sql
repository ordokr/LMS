-- User profiles table
CREATE TABLE IF NOT EXISTS user_profiles (
    user_id TEXT PRIMARY KEY,
    bio TEXT,
    website TEXT,
    location TEXT,
    title TEXT,
    tag_line TEXT,
    profile_views INTEGER NOT NULL DEFAULT 0,
    trust_level INTEGER NOT NULL DEFAULT 0,
    is_moderator BOOLEAN NOT NULL DEFAULT 0,
    is_admin BOOLEAN NOT NULL DEFAULT 0,
    last_seen_at TEXT,
    created_topics_count INTEGER NOT NULL DEFAULT 0,
    posts_count INTEGER NOT NULL DEFAULT 0,
    likes_given INTEGER NOT NULL DEFAULT 0,
    likes_received INTEGER NOT NULL DEFAULT 0,
    featured_topic_id TEXT,
    
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (featured_topic_id) REFERENCES topics (id) ON DELETE SET NULL
);

-- User activity table
CREATE TABLE IF NOT EXISTS user_activities (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    activity_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    target_type TEXT NOT NULL,
    data TEXT,  -- JSON
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_user_activities_user_id ON user_activities (user_id);
CREATE INDEX idx_user_activities_created_at ON user_activities (created_at DESC);

-- Notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY,
    notification_type TEXT NOT NULL,
    user_id TEXT NOT NULL,
    actor_id TEXT,
    target_id TEXT NOT NULL,
    target_type TEXT NOT NULL,
    data TEXT,  -- JSON
    read BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (actor_id) REFERENCES users (id) ON DELETE SET NULL
);

CREATE INDEX idx_notifications_user_id ON notifications (user_id);
CREATE INDEX idx_notifications_read ON notifications (user_id, read);
CREATE INDEX idx_notifications_created_at ON notifications (created_at DESC);

-- User follows table
CREATE TABLE IF NOT EXISTS user_follows (
    id TEXT PRIMARY KEY,
    follower_id TEXT NOT NULL,
    following_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    
    UNIQUE (follower_id, following_id),
    FOREIGN KEY (follower_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (following_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_user_follows_follower ON user_follows (follower_id);
CREATE INDEX idx_user_follows_following ON user_follows (following_id);

-- Topic subscriptions table
CREATE TABLE IF NOT EXISTS topic_subscriptions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    topic_id TEXT NOT NULL,
    subscription_level TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    UNIQUE (user_id, topic_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE
);

CREATE INDEX idx_topic_subscriptions_user ON topic_subscriptions (user_id);
CREATE INDEX idx_topic_subscriptions_topic ON topic_subscriptions (topic_id);

-- Category subscriptions table
CREATE TABLE IF NOT EXISTS category_subscriptions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    subscription_level TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    UNIQUE (user_id, category_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);

CREATE INDEX idx_category_subscriptions_user ON category_subscriptions (user_id);
CREATE INDEX idx_category_subscriptions_category ON category_subscriptions (category_id);