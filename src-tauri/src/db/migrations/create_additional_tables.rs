use sqlx::{Executor, SqliteConnection};
use crate::error::Error;

pub async fn run(conn: &mut SqliteConnection) -> Result<(), Error> {
    // Create modules table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS modules (
            id TEXT PRIMARY KEY,
            course_id TEXT NOT NULL,
            name TEXT NOT NULL,
            position INTEGER NOT NULL,
            unlock_at TEXT,
            require_sequential_progress INTEGER NOT NULL,
            published INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            canvas_module_id TEXT UNIQUE,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        )"
    ).await?;
    
    // Create module_prerequisites table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS module_prerequisites (
            module_id TEXT NOT NULL,
            prerequisite_module_id TEXT NOT NULL,
            PRIMARY KEY (module_id, prerequisite_module_id),
            FOREIGN KEY (module_id) REFERENCES modules(id),
            FOREIGN KEY (prerequisite_module_id) REFERENCES modules(id)
        )"
    ).await?;
    
    // Create module_items table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS module_items (
            id TEXT PRIMARY KEY,
            module_id TEXT NOT NULL,
            title TEXT NOT NULL,
            position INTEGER NOT NULL,
            item_type TEXT NOT NULL,
            content_id TEXT NOT NULL,
            published INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            canvas_module_item_id TEXT UNIQUE,
            FOREIGN KEY (module_id) REFERENCES modules(id)
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
            lock_at TEXT,
            unlock_at TEXT,
            assignment_type INTEGER NOT NULL,
            published INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            position INTEGER NOT NULL,
            canvas_assignment_id TEXT UNIQUE,
            discussion_topic_id TEXT,
            FOREIGN KEY (course_id) REFERENCES courses(id),
            FOREIGN KEY (discussion_topic_id) REFERENCES topics(id)
        )"
    ).await?;
    
    // Create assignment_submission_types table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assignment_submission_types (
            assignment_id TEXT NOT NULL,
            submission_type TEXT NOT NULL,
            PRIMARY KEY (assignment_id, submission_type),
            FOREIGN KEY (assignment_id) REFERENCES assignments(id)
        )"
    ).await?;
    
    // Create submissions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS submissions (
            id TEXT PRIMARY KEY,
            assignment_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            submitted_at TEXT,
            graded_at TEXT,
            score REAL,
            grade TEXT,
            submission_type TEXT,
            body TEXT,
            url TEXT,
            attempt INTEGER NOT NULL,
            status INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            canvas_submission_id TEXT UNIQUE,
            grader_id TEXT,
            FOREIGN KEY (assignment_id) REFERENCES assignments(id),
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (grader_id) REFERENCES users(id)
        )"
    ).await?;
    
    // Create attachments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attachments (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            content_type TEXT NOT NULL,
            size INTEGER NOT NULL,
            url TEXT,
            local_path TEXT,
            created_at TEXT NOT NULL,
            user_id TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    ).await?;
    
    // Create submission_attachments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS submission_attachments (
            submission_id TEXT NOT NULL,
            attachment_id TEXT NOT NULL,
            PRIMARY KEY (submission_id, attachment_id),
            FOREIGN KEY (submission_id) REFERENCES submissions(id),
            FOREIGN KEY (attachment_id) REFERENCES attachments(id)
        )"
    ).await?;
    
    // Create submission_comments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS submission_comments (
            id TEXT PRIMARY KEY,
            submission_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            comment TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (submission_id) REFERENCES submissions(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    ).await?;
    
    // Create comment_attachments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS comment_attachments (
            comment_id TEXT NOT NULL,
            attachment_id TEXT NOT NULL,
            PRIMARY KEY (comment_id, attachment_id),
            FOREIGN KEY (comment_id) REFERENCES submission_comments(id),
            FOREIGN KEY (attachment_id) REFERENCES attachments(id)
        )"
    ).await?;
    
    // Create indexes
    conn.execute("CREATE INDEX IF NOT EXISTS idx_modules_course_id ON modules(course_id)").await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_assignments_course_id ON assignments(course_id)").await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_submissions_assignment_id ON submissions(assignment_id)").await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_submissions_user_id ON submissions(user_id)").await?;
    conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS idx_submissions_assignment_user ON submissions(assignment_id, user_id)").await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_submission_comments_submission_id ON submission_comments(submission_id)").await?;
    
    Ok(())
}