use rusqlite::Connection;
use crate::Course;

pub fn create_course(name: String, description: Option<String>) -> Result<String, String> {
    // TODO: Implement database interaction
    Ok("Course created successfully".to_string())
}

pub fn get_courses() -> Result<Vec<Course>, String> {
    // TODO: Implement database query
    Ok(vec![
        Course { id: 1, name: "Intro to Rust".to_string(), description: Some("Learn the basics".to_string()) },
        Course { id: 2, name: "Web Development with Axum".to_string(), description: None },
    ])
}