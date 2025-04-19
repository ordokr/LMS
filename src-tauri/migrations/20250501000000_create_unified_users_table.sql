-- Create unified users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    avatar_url TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_seen_at TEXT,
    
    -- Role and permission fields
    roles TEXT NOT NULL, -- JSON array
    trust_level INTEGER,
    is_admin INTEGER NOT NULL DEFAULT 0,
    is_moderator INTEGER NOT NULL DEFAULT 0,
    
    -- External system IDs
    canvas_id TEXT,
    discourse_id TEXT,
    sis_id TEXT,
    lti_id TEXT,
    
    -- Profile fields
    bio TEXT,
    location TEXT,
    website TEXT,
    timezone TEXT,
    
    -- Canvas-specific fields
    sortable_name TEXT,
    short_name TEXT,
    
    -- Discourse-specific fields
    post_count INTEGER,
    
    -- Metadata and extensibility
    source_system TEXT,
    metadata TEXT, -- JSON object
    
    -- Constraints
    UNIQUE(email),
    UNIQUE(username),
    UNIQUE(canvas_id),
    UNIQUE(discourse_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_canvas_id ON users(canvas_id);
CREATE INDEX IF NOT EXISTS idx_users_discourse_id ON users(discourse_id);
CREATE INDEX IF NOT EXISTS idx_users_source_system ON users(source_system);
