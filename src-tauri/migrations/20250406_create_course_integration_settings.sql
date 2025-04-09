CREATE TABLE IF NOT EXISTS course_integration_settings (
    course_id TEXT PRIMARY KEY,
    canvas_course_id TEXT,
    auto_sync_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    sync_frequency_hours INTEGER,
    sync_modules BOOLEAN NOT NULL DEFAULT TRUE,
    sync_assignments BOOLEAN NOT NULL DEFAULT TRUE,
    sync_discussions BOOLEAN NOT NULL DEFAULT TRUE,
    sync_files BOOLEAN NOT NULL DEFAULT TRUE,
    sync_announcements BOOLEAN NOT NULL DEFAULT TRUE,
    last_sync TEXT,
    
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_course_integration_settings_canvas_id ON course_integration_settings(canvas_course_id);