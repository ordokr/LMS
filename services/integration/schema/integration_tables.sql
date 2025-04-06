-- Canvas-Discourse integration database schema

-- Course to Category mapping
CREATE TABLE course_category_mappings (
  id SERIAL PRIMARY KEY,
  canvas_course_id INTEGER NOT NULL,
  discourse_category_id INTEGER NOT NULL,
  sync_enabled BOOLEAN DEFAULT TRUE,
  last_sync_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(canvas_course_id)
);

-- Discussion Topic to Discourse Topic mapping
CREATE TABLE discussion_topic_mappings (
  id SERIAL PRIMARY KEY,
  canvas_discussion_id INTEGER NOT NULL,
  discourse_topic_id INTEGER NOT NULL,
  status VARCHAR(20) DEFAULT 'active',
  last_sync_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(canvas_discussion_id)
);

-- User mapping between Canvas and Discourse
CREATE TABLE user_mappings (
  id SERIAL PRIMARY KEY,
  canvas_user_id INTEGER NOT NULL,
  discourse_user_id INTEGER NOT NULL,
  status VARCHAR(20) DEFAULT 'active',
  last_sync_at TIMESTAMP,
  deleted_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(canvas_user_id)
);

-- Integration logs for auditing and troubleshooting
CREATE TABLE integration_logs (
  id SERIAL PRIMARY KEY,
  correlation_id VARCHAR(100) NOT NULL,
  event_type VARCHAR(50) NOT NULL,
  event_details JSONB,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Integration settings
CREATE TABLE integration_settings (
  id SERIAL PRIMARY KEY,
  setting_key VARCHAR(100) NOT NULL,
  setting_value TEXT,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(setting_key)
);

-- Create indexes
CREATE INDEX idx_course_mapping_course_id ON course_category_mappings(canvas_course_id);
CREATE INDEX idx_course_mapping_category_id ON course_category_mappings(discourse_category_id);
CREATE INDEX idx_topic_mapping_discussion_id ON discussion_topic_mappings(canvas_discussion_id);
CREATE INDEX idx_topic_mapping_topic_id ON discussion_topic_mappings(discourse_topic_id);
CREATE INDEX idx_user_mapping_canvas_id ON user_mappings(canvas_user_id);
CREATE INDEX idx_user_mapping_discourse_id ON user_mappings(discourse_user_id);
CREATE INDEX idx_logs_correlation ON integration_logs(correlation_id);
CREATE INDEX idx_logs_event_type ON integration_logs(event_type);
CREATE INDEX idx_logs_created_at ON integration_logs(created_at);