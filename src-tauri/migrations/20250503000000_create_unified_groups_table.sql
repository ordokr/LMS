-- Create unified groups table
CREATE TABLE IF NOT EXISTS groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    context_id TEXT,
    context_type TEXT,
    group_category_id TEXT,
    join_level TEXT NOT NULL,
    max_membership INTEGER,
    is_public INTEGER NOT NULL DEFAULT 0,
    canvas_id TEXT,
    discourse_id TEXT,
    full_name TEXT,
    visibility_level INTEGER,
    mentionable_level INTEGER,
    messageable_level INTEGER,
    automatic INTEGER NOT NULL DEFAULT 0,
    sis_source_id TEXT,
    storage_quota INTEGER,
    default_view TEXT,
    source_system TEXT,
    metadata TEXT, -- JSON object
    
    -- Constraints
    UNIQUE(name, context_id, context_type),
    UNIQUE(canvas_id),
    UNIQUE(discourse_id)
);

-- Create group memberships table
CREATE TABLE IF NOT EXISTS group_memberships (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    status TEXT NOT NULL,
    is_moderator INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    -- Constraints
    UNIQUE(group_id, user_id),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_groups_name ON groups(name);
CREATE INDEX IF NOT EXISTS idx_groups_context ON groups(context_id, context_type);
CREATE INDEX IF NOT EXISTS idx_groups_category ON groups(group_category_id);
CREATE INDEX IF NOT EXISTS idx_groups_canvas_id ON groups(canvas_id);
CREATE INDEX IF NOT EXISTS idx_groups_discourse_id ON groups(discourse_id);
CREATE INDEX IF NOT EXISTS idx_groups_source_system ON groups(source_system);

CREATE INDEX IF NOT EXISTS idx_group_memberships_group ON group_memberships(group_id);
CREATE INDEX IF NOT EXISTS idx_group_memberships_user ON group_memberships(user_id);
CREATE INDEX IF NOT EXISTS idx_group_memberships_status ON group_memberships(status);
