CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    parent_id UUID REFERENCES categories(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    course_id UUID REFERENCES courses(id),
    position INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_categories_parent_id ON categories(parent_id);
CREATE INDEX idx_categories_course_id ON categories(course_id);

-- After categories table is created, add foreign key to courses
ALTER TABLE courses 
ADD CONSTRAINT fk_courses_category_id 
FOREIGN KEY (category_id) REFERENCES categories(id);