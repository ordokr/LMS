-- Add sync_direction column to course_categories table
ALTER TABLE course_categories ADD COLUMN IF NOT EXISTS sync_direction TEXT NOT NULL DEFAULT 'Bidirectional';

-- Create index for sync_direction
CREATE INDEX IF NOT EXISTS idx_course_categories_sync_direction ON course_categories(sync_direction);
