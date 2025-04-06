CREATE TABLE IF NOT EXISTS topics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    category_id UUID NOT NULL REFERENCES categories(id),
    author_id UUID NOT NULL REFERENCES users(id),
    pinned BOOLEAN NOT NULL DEFAULT false,
    closed BOOLEAN NOT NULL DEFAULT false,
    post_count INTEGER NOT NULL DEFAULT 0,
    view_count INTEGER NOT NULL DEFAULT 0,
    assignment_id UUID REFERENCES assignments(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_topics_category_id ON topics(category_id);
CREATE INDEX idx_topics_assignment_id ON topics(assignment_id);
CREATE INDEX idx_topics_slug ON topics(slug);

-- Update assignments table to reference topics
ALTER TABLE assignments
ADD CONSTRAINT fk_assignments_topic_id
FOREIGN KEY (topic_id) REFERENCES topics(id);