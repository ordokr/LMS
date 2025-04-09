-- Optimize frequently accessed fields
CREATE INDEX IF NOT EXISTS idx_topics_category_id ON topics(category_id);
CREATE INDEX IF NOT EXISTS idx_topics_user_id ON topics(user_id);
CREATE INDEX IF NOT EXISTS idx_topics_created_at ON topics(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_topics_slug ON topics(slug);

-- For search functionality
CREATE INDEX IF NOT EXISTS idx_topics_title_search ON topics(title COLLATE NOCASE);

-- Full-text search for content when needed
CREATE VIRTUAL TABLE IF NOT EXISTS topics_fts USING fts5(
    title,
    content,
    content='topics',
    content_rowid='id'
);

-- Trigger to keep FTS index updated
CREATE TRIGGER IF NOT EXISTS topics_ai AFTER INSERT ON topics BEGIN
    INSERT INTO topics_fts(rowid, title, content) 
    VALUES (new.id, new.title, new.content);
END;

CREATE TRIGGER IF NOT EXISTS topics_au AFTER UPDATE ON topics BEGIN
    INSERT INTO topics_fts(topics_fts, rowid, title, content) 
    VALUES('delete', old.id, old.title, old.content);
    INSERT INTO topics_fts(rowid, title, content) 
    VALUES (new.id, new.title, new.content);
END;

CREATE TRIGGER IF NOT EXISTS topics_ad AFTER DELETE ON topics BEGIN
    INSERT INTO topics_fts(topics_fts, rowid, title, content) 
    VALUES('delete', old.id, old.title, old.content);
END;