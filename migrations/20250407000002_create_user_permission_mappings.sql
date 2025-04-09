CREATE TABLE IF NOT EXISTS user_permission_mappings (
    id UUID PRIMARY KEY,
    canvas_user_id TEXT NOT NULL,
    discourse_user_id TEXT NOT NULL,
    canvas_role TEXT NOT NULL,
    discourse_group TEXT NOT NULL,
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(canvas_user_id, discourse_user_id)
);

CREATE INDEX idx_user_permissions_canvas_id ON user_permission_mappings(canvas_user_id);
CREATE INDEX idx_user_permissions_discourse_id ON user_permission_mappings(discourse_user_id);