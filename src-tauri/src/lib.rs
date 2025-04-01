// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod database;

#[tauri::command]
fn create_course(name: String, description: Option<String>) -> Result<String, String> {
    let conn = database::establish_connection().map_err(|e| e.to_string())?;
    database::create_course(&conn, &name, description.as_deref())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumPost {
    pub id: i32,
    pub thread_id: i32,
    pub author_id: i32,
    pub content: String,
    pub created_at: String, // Using String for simplicity, consider a proper DateTime type
}


#[tauri::command]
fn create_forum_thread(title: String, category: String) -> Result<String, String> {
    let conn = database::establish_connection().map_err(|e| e.to_string())?;
    let created_at = chrono::Utc::now().to_rfc3339();
    let sql = format!(
        "INSERT INTO forum_threads (title, category, created_at) VALUES ('{}', '{}', '{}')",
        title, category, created_at
    );
    database::execute_sql(&conn, &sql).map(|_| "Forum thread created successfully".to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_forum_threads() -> Result<Vec<ForumThread>, String> {
    let conn = database::establish_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, title, category, created_at FROM forum_threads").map_err(|e| e.to_string())?;
    let mut rows = stmt.query_map([], |row| {
        Ok(ForumThread {
            id: row.get(0)?,
            title: row.get(1)?,
            category: row.get(2)?,
            created_at: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut threads = Vec::new();
    while let Some(result) = rows.next() {
        let thread = result.map_err(|e| e.to_string())?;
        threads.push(thread);
    }

    Ok(threads)
}

#[tauri::command]
fn create_forum_post(thread_id: i32, author_id: i32, content: String) -> Result<String, String> {
    let conn = database::establish_connection().map_err(|e| e.to_string())?;
    let created_at = chrono::Utc::now().to_rfc3339();
    let sql = format!(
        "INSERT INTO forum_posts (thread_id, author_id, content, created_at) VALUES ({}, {}, '{}', '{}')",
        thread_id, author_id, content, created_at
    );
    database::execute_sql(&conn, &sql).map(|_| "Forum post created successfully".to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_forum_posts(thread_id: i32) -> Result<Vec<ForumPost>, String> {
    let conn = database::establish_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, thread_id, author_id, content, created_at FROM forum_posts WHERE thread_id = ?").map_err(|e| e.to_string())?;
    let mut rows = stmt.query_map([thread_id], |row| {
        Ok(ForumPost {
            id: row.get(0)?,
            thread_id: row.get(1)?,
            author_id: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut posts = Vec::new();
    while let Some(result) = rows.next() {
        let post = result.map_err(|e| e.to_string())?;
        posts.push(post);
    }

    Ok(posts)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumThread {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub created_at: String, // Using String for simplicity, consider a proper DateTime type
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Assignment {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub description: String,
    pub due_date: String, // Using String for simplicity, consider a proper DateTime type
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Submission {
    pub id: i32,
    pub assignment_id: i32,
    pub student_id: i32,
    pub submission_date: String, // Using String for simplicity, consider a proper DateTime type
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Grade {
    pub id: i32,
    pub submission_id: i32,
    pub grader_id: i32,
    pub grade: f32,
    pub feedback: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CourseProgress {
    pub id: i32,
    pub course_id: i32,
    pub student_id: i32,
    pub completed_modules: i32,
    pub total_modules: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StudentPerformance {
    pub id: i32,
    pub student_id: i32,
    pub course_id: i32,
    pub average_grade: f32,
    pub time_spent: i32, // in minutes
}

use tauri::AppHandle;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Do nothing
}

pub fn setup(app_handle: &AppHandle) {
    // Access the app handle here
    println!("App handle: {:?}", app_handle);
}
