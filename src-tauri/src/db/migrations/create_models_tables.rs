// Add these table definitions to your existing migration

// Create enrollments table
conn.execute(
    "CREATE TABLE IF NOT EXISTS enrollments (
        id TEXT PRIMARY KEY,
        user_id TEXT NOT NULL,
        course_id TEXT NOT NULL,
        role TEXT NOT NULL,
        state TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        last_activity_at TEXT,
        canvas_enrollment_id TEXT UNIQUE,
        FOREIGN KEY (user_id) REFERENCES users(id),
        FOREIGN KEY (course_id) REFERENCES courses(id),
        UNIQUE(user_id, course_id)
    )"
).await?;

// Create assignments table
conn.execute(
    "CREATE TABLE IF NOT EXISTS assignments (
        id TEXT PRIMARY KEY,
        course_id TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT,
        points_possible REAL NOT NULL,
        due_at TEXT,
        unlock_at TEXT,
        lock_at TEXT,
        submission_types TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        published INTEGER NOT NULL,
        canvas_assignment_id TEXT UNIQUE,
        FOREIGN KEY (course_id) REFERENCES courses(id)
    )"
).await?;

// Create submissions table
conn.execute(
    "CREATE TABLE IF NOT EXISTS submissions (
        id TEXT PRIMARY KEY,
        assignment_id TEXT NOT NULL,
        user_id TEXT NOT NULL,
        submitted_at TEXT NOT NULL,
        grade TEXT,
        score REAL,
        submission_type TEXT NOT NULL,
        body TEXT,
        graded_at TEXT,
        grader_id TEXT,
        canvas_submission_id TEXT UNIQUE,
        FOREIGN KEY (assignment_id) REFERENCES assignments(id),
        FOREIGN KEY (user_id) REFERENCES users(id),
        FOREIGN KEY (grader_id) REFERENCES users(id),
        UNIQUE(assignment_id, user_id)
    )"
).await?;

// Create attachments table
conn.execute(
    "CREATE TABLE IF NOT EXISTS attachments (
        id TEXT PRIMARY KEY,
        filename TEXT NOT NULL,
        content_type TEXT NOT NULL,
        size INTEGER NOT NULL,
        url TEXT NOT NULL,
        user_id TEXT NOT NULL,
        created_at TEXT NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(id)
    )"
).await?;

// Create additional indexes
conn.execute("CREATE INDEX IF NOT EXISTS idx_enrollments_user_id ON enrollments(user_id)").await?;
conn.execute("CREATE INDEX IF NOT EXISTS idx_enrollments_course_id ON enrollments(course_id)").await?;
conn.execute("CREATE INDEX IF NOT EXISTS idx_assignments_course_id ON assignments(course_id)").await?;
conn.execute("CREATE INDEX IF NOT EXISTS idx_submissions_assignment_id ON submissions(assignment_id)").await?;
conn.execute("CREATE INDEX IF NOT EXISTS idx_submissions_user_id ON submissions(user_id)").await?;