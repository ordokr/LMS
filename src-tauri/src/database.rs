use rusqlite::{Connection, Result};

pub fn establish_connection() -> Result<Connection, String> {
    let db_path = "lms.db";
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Create index if it doesn't exist
    let index_sql = "CREATE INDEX IF NOT EXISTS idx_courses_name ON courses (name);";
    execute_sql(&conn, index_sql).map_err(|e| e.to_string())?;

    // Create assignments table if it doesn't exist
    let create_assignments_table_sql = "
        CREATE TABLE IF NOT EXISTS assignments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            course_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            due_date TEXT NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        );
    ";
    execute_sql(&conn, create_assignments_table_sql).map_err(|e| e.to_string())?;

    // Create submissions table if it doesn't exist
    let create_submissions_table_sql = "
        CREATE TABLE IF NOT EXISTS submissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            assignment_id INTEGER NOT NULL,
            student_id INTEGER NOT NULL,
            submission_date TEXT NOT NULL,
            content TEXT NOT NULL,
            FOREIGN KEY (assignment_id) REFERENCES assignments(id)
        );
    ";
    execute_sql(&conn, create_submissions_table_sql).map_err(|e| e.to_string())?;

     // Create grades table if it doesn't exist
    let create_grades_table_sql = "
        CREATE TABLE IF NOT EXISTS grades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            submission_id INTEGER NOT NULL,
            grader_id INTEGER NOT NULL,
            grade REAL NOT NULL,
            feedback TEXT NOT NULL,
            FOREIGN KEY (submission_id) REFERENCES submissions(id)
        );
    ";
    execute_sql(&conn, create_grades_table_sql).map_err(|e| e.to_string())?;

    // Create course_progress table if it doesn't exist
    let create_course_progress_table_sql = "
        CREATE TABLE IF NOT EXISTS course_progress (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            course_id INTEGER NOT NULL,
            student_id INTEGER NOT NULL,
            completed_modules INTEGER NOT NULL,
            total_modules INTEGER NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        );
    ";
    execute_sql(&conn, create_course_progress_table_sql).map_err(|e| e.to_string())?;

    // Create student_performance table if it doesn't exist
    let create_student_performance_table_sql = "
        CREATE TABLE IF NOT EXISTS student_performance (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            student_id INTEGER NOT NULL,
            course_id INTEGER NOT NULL,
            average_grade REAL NOT NULL,
            time_spent INTEGER NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        );
    ";
    execute_sql(&conn, create_student_performance_table_sql).map_err(|e| e.to_string())?;

    // Create forum_threads table if it doesn't exist
    let create_threads_table_sql = "
        CREATE TABLE IF NOT EXISTS forum_threads (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            category TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
    ";
    execute_sql(&conn, create_threads_table_sql).map_err(|e| e.to_string())?;

    // Create forum_posts table if it doesn't exist
    let create_posts_table_sql = "
        CREATE TABLE IF NOT EXISTS forum_posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            thread_id INTEGER NOT NULL,
            author_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (thread_id) REFERENCES forum_threads(id)
        );
    ";
    execute_sql(&conn, create_posts_table_sql).map_err(|e| e.to_string())?;

    Ok(conn)
}

pub struct Course {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

pub fn create_course(conn: &Connection, name: &str, description: Option<&str>) -> Result<String, String> {
    let execute_result = conn.execute(
        "INSERT INTO courses (name, description) VALUES (?1, ?2)",
        (name, description),
    );

    match execute_result {
        Ok(_) => Ok("Course created successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn execute_sql(conn: &Connection, sql: &str) -> Result<usize, String> {
    let execute_result = conn.execute(sql, ());

    match execute_result {
        Ok(rows_affected) => Ok(rows_affected),
        Err(e) => Err(e.to_string()),
    }
}
