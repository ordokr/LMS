#[cfg(test)]
mod tests {
    use crate::api::course::{get_course, get_courses, create_course, update_course, delete_course};
    use crate::models::course::{Course, CourseStatus};
    use chrono::{Utc, TimeZone};
    use tauri::State;
    use serde_json::{json, Value};
    
    // Mock AppState for testing
    struct MockAppState {
        // Add test database connection, etc.
    }
    
    #[test]
    async fn test_get_course() {
        // Set up test state
        let state = MockAppState {};
        
        // Test fetching a course
        let result = get_course(State::new(state), 1).await;
        
        // Verify result
        assert!(result.is_ok());
        let course = result.unwrap();
        assert_eq!(course.id, Some(1));
        
        // Verify date handling
        assert!(course.created_at.is_some());
        assert!(course.updated_at.is_some());
    }
    
    #[test]
    async fn test_create_course_with_dates() {
        // Set up test state
        let state = MockAppState {};
        
        // Create a new course with dates
        let course_data = json!({
            "name": "New Test Course",
            "code": "NEW101",
            "description": "Test course created via API",
            "instructor_id": 1,
            "start_date": "2025-05-01T00:00:00Z",
            "end_date": "2025-08-31T23:59:59Z",
            "status": "Active"
        });
        
        // Test creating a course
        let result = create_course(State::new(state), course_data).await;
        
        // Verify result
        assert!(result.is_ok());
        let created_course = result.unwrap();
        
        // Verify basic data
        assert_eq!(created_course.name, "New Test Course");
        assert_eq!(created_course.code, "NEW101");
        
        // Verify date parsing
        assert!(created_course.start_date.is_some());
        assert_eq!(created_course.start_date.unwrap().year(), 2025);
        assert_eq!(created_course.start_date.unwrap().month(), 5);
        assert_eq!(created_course.start_date.unwrap().day(), 1);
        
        assert!(created_course.end_date.is_some());
        assert_eq!(created_course.end_date.unwrap().year(), 2025);
        assert_eq!(created_course.end_date.unwrap().month(), 8);
        assert_eq!(created_course.end_date.unwrap().day(), 31);
    }
    
    // Additional tests for update, delete, etc.
}