-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (parent_id) REFERENCES categories (id) ON DELETE SET NULL
);

CREATE UNIQUE INDEX idx_categories_slug ON categories (slug);
CREATE INDEX idx_categories_parent ON categories (parent_id);

-- Topics table
CREATE TABLE IF NOT EXISTS topics (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    category_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    closed BOOLEAN NOT NULL DEFAULT 0,
    pinned BOOLEAN NOT NULL DEFAULT 0,
    pinned_globally BOOLEAN NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT 1,
    deleted_at TEXT,
    views INTEGER NOT NULL DEFAULT 0,
    posts_count INTEGER NOT NULL DEFAULT 0,
    like_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    bumped_at TEXT NOT NULL,
    last_posted_at TEXT,
    highest_post_number INTEGER NOT NULL DEFAULT 0,
    excerpt TEXT,
    
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_topics_slug ON topics (slug);
CREATE INDEX idx_topics_category ON topics (category_id);
CREATE INDEX idx_topics_user ON topics (user_id);
CREATE INDEX idx_topics_bumped ON topics (bumped_at DESC);
CREATE INDEX idx_topics_created ON topics (created_at DESC);
CREATE INDEX idx_topics_visible_deleted ON topics (visible, deleted_at);

-- Posts table
CREATE TABLE IF NOT EXISTS posts (
    id TEXT PRIMARY KEY,
    topic_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    post_number INTEGER NOT NULL,
    raw TEXT NOT NULL,
    cooked TEXT NOT NULL, -- HTML version
    reply_to_post_id TEXT,
    deleted_at TEXT,
    like_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (reply_to_post_id) REFERENCES posts (id) ON DELETE SET NULL
);

CREATE UNIQUE INDEX idx_posts_topic_number ON posts (topic_id, post_number);
CREATE INDEX idx_posts_user ON posts (user_id);
CREATE INDEX idx_posts_created ON posts (created_at);
CREATE INDEX idx_posts_reply_to ON posts (reply_to_post_id);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_tags_name ON tags (name);

-- Topic Tags junction table
CREATE TABLE IF NOT EXISTS topic_tags (
    topic_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    
    PRIMARY KEY (topic_id, tag_id),
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Post Likes junction table
CREATE TABLE IF NOT EXISTS post_likes (
    id TEXT PRIMARY KEY,
    post_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    
    UNIQUE (post_id, user_id),
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_post_likes_post ON post_likes (post_id);
CREATE INDEX idx_post_likes_user ON post_likes (user_id);