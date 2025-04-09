#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc, TimeZone};
    use crate::models::course::{Course, CourseStatus};
    use serde_json::json;

    #[test]
    fn test_course_date_handling() {
        // Create a course with dates
        let start_date = Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2025, 8, 31, 23, 59, 59).unwrap();
        let created_at = Utc.with_ymd_and_hms(2025, 3, 15, 14, 30, 45).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2025, 3, 16, 9, 15, 30).unwrap();

        let course = Course {
            id: Some(12345),
            name: "Test Course".to_string(),
            code: "TEST101".to_string(),
            description: Some("Test course description".to_string()),
            instructor_id: Some(67890),
            start_date: Some(start_date),
            end_date: Some(end_date),
            status: CourseStatus::Active,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        };

        // Test serialization/deserialization preserves dates
        let json = serde_json::to_string(&course).unwrap();
        let deserialized: Course = serde_json::from_str(&json).unwrap();
        
        assert_eq!(
            course.start_date.unwrap().timestamp(),
            deserialized.start_date.unwrap().timestamp()
        );
        assert_eq!(
            course.end_date.unwrap().timestamp(),
            deserialized.end_date.unwrap().timestamp()
        );
        assert_eq!(
            course.created_at.unwrap().timestamp(),
            deserialized.created_at.unwrap().timestamp()
        );
        assert_eq!(
            course.updated_at.unwrap().timestamp(),
            deserialized.updated_at.unwrap().timestamp()
        );
    }

    #[test]
    fn test_course_from_canvas_api() {
        let canvas_json = json!({
            "id": 12345,
            "name": "Test Course",
            "course_code": "TEST101",
            "description": "Test course description",
            "account_id": 67890,
            "start_at": "2025-04-01T00:00:00Z",
            "end_at": "2025-08-31T23:59:59Z",
            "created_at": "2025-03-15T14:30:45Z",
            "updated_at": "2025-03-16T09:15:30Z",
            "workflow_state": "available"
        });
        
        let course = Course::from_canvas_api(&canvas_json).unwrap();
        
        // Verify basic fields
        assert_eq!(course.id, Some(12345));
        assert_eq!(course.name, "Test Course");
        assert_eq!(course.code, "TEST101");
        assert_eq!(course.description, Some("Test course description".to_string()));
        assert_eq!(course.instructor_id, Some(67890));
        assert_eq!(course.status, CourseStatus::Active);
        
        // Verify dates were parsed correctly
        assert_eq!(course.start_date.unwrap().year(), 2025);
        assert_eq!(course.start_date.unwrap().month(), 4);
        assert_eq!(course.start_date.unwrap().day(), 1);
        
        assert_eq!(course.end_date.unwrap().year(), 2025);
        assert_eq!(course.end_date.unwrap().month(), 8);
        assert_eq!(course.end_date.unwrap().day(), 31);
        
        assert_eq!(course.created_at.unwrap().year(), 2025);
        assert_eq!(course.created_at.unwrap().month(), 3);
        assert_eq!(course.created_at.unwrap().day(), 15);
    }

    #[test]
    fn test_course_validation() {
        // Valid course
        let valid_course = Course {
            id: Some(1),
            name: "Test Course".to_string(),
            code: "TEST101".to_string(),
            description: Some("Test description".to_string()),
            instructor_id: Some(1),
            start_date: Some(Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap()),
            end_date: Some(Utc.with_ymd_and_hms(2025, 8, 31, 0, 0, 0).unwrap()),
            status: CourseStatus::Active,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert!(valid_course.validate().is_ok());
        
        // Invalid: Empty name
        let mut invalid_course = valid_course.clone();
        invalid_course.name = "".to_string();
        assert!(invalid_course.validate().is_err());
        
        // Invalid: Empty code
        let mut invalid_course = valid_course.clone();
        invalid_course.code = "".to_string();
        assert!(invalid_course.validate().is_err());
        
        // Invalid: end_date before start_date
        let mut invalid_course = valid_course.clone();
        invalid_course.start_date = Some(Utc.with_ymd_and_hms(2025, 8, 31, 0, 0, 0).unwrap());
        invalid_course.end_date = Some(Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap());
        assert!(invalid_course.validate().is_err());
    }
}