#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc, TimeZone};
    use crate::models::content::Assignment;
    use serde_json::json;

    #[test]
    fn test_assignment_date_handling() {
        // Create an assignment with dates
        let due_date = Utc.with_ymd_and_hms(2025, 5, 15, 23, 59, 59).unwrap();
        let available_from = Utc.with_ymd_and_hms(2025, 5, 1, 0, 0, 0).unwrap();
        let available_until = Utc.with_ymd_and_hms(2025, 5, 20, 23, 59, 59).unwrap();
        let created_at = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2025, 4, 16, 9, 15, 30).unwrap();

        let assignment = Assignment {
            id: Some(12345),
            course_id: Some(54321),
            title: "Test Assignment".to_string(),
            description: Some("Test assignment description".to_string()),
            points_possible: Some(100.0),
            due_date: Some(due_date),
            available_from: Some(available_from),
            available_until: Some(available_until),
            is_published: true,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        };

        // Test serialization/deserialization preserves dates
        let json = serde_json::to_string(&assignment).unwrap();
        let deserialized: Assignment = serde_json::from_str(&json).unwrap();
        
        assert_eq!(
            assignment.due_date.unwrap().timestamp(),
            deserialized.due_date.unwrap().timestamp()
        );
        assert_eq!(
            assignment.available_from.unwrap().timestamp(),
            deserialized.available_from.unwrap().timestamp()
        );
        assert_eq!(
            assignment.available_until.unwrap().timestamp(),
            deserialized.available_until.unwrap().timestamp()
        );
    }

    #[test]
    fn test_assignment_from_canvas_api() {
        let canvas_json = json!({
            "id": 12345,
            "course_id": 54321,
            "name": "Test Assignment",
            "description": "Test assignment description",
            "points_possible": 100.0,
            "due_at": "2025-05-15T23:59:59Z",
            "unlock_at": "2025-05-01T00:00:00Z",
            "lock_at": "2025-05-20T23:59:59Z",
            "created_at": "2025-04-15T10:30:00Z",
            "updated_at": "2025-04-16T09:15:30Z",
            "published": true
        });
        
        let assignment = Assignment::from_canvas_api(&canvas_json).unwrap();
        
        // Verify basic fields
        assert_eq!(assignment.id, Some(12345));
        assert_eq!(assignment.course_id, Some(54321));
        assert_eq!(assignment.title, "Test Assignment");
        assert_eq!(assignment.description, Some("Test assignment description".to_string()));
        assert_eq!(assignment.points_possible, Some(100.0));
        assert_eq!(assignment.is_published, true);
        
        // Verify dates were parsed correctly
        assert_eq!(assignment.due_date.unwrap().year(), 2025);
        assert_eq!(assignment.due_date.unwrap().month(), 5);
        assert_eq!(assignment.due_date.unwrap().day(), 15);
        
        assert_eq!(assignment.available_from.unwrap().year(), 2025);
        assert_eq!(assignment.available_from.unwrap().month(), 5);
        assert_eq!(assignment.available_from.unwrap().day(), 1);
        
        assert_eq!(assignment.available_until.unwrap().year(), 2025);
        assert_eq!(assignment.available_until.unwrap().month(), 5);
        assert_eq!(assignment.available_until.unwrap().day(), 20);
    }

    #[test]
    fn test_assignment_validation() {
        // Valid assignment
        let valid_assignment = Assignment {
            id: Some(1),
            course_id: Some(1),
            title: "Test Assignment".to_string(),
            description: Some("Test description".to_string()),
            points_possible: Some(100.0),
            due_date: Some(Utc.with_ymd_and_hms(2025, 5, 15, 23, 59, 59).unwrap()),
            available_from: Some(Utc.with_ymd_and_hms(2025, 5, 1, 0, 0, 0).unwrap()),
            available_until: Some(Utc.with_ymd_and_hms(2025, 5, 20, 23, 59, 59).unwrap()),
            is_published: true,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        assert!(valid_assignment.validate().is_ok());
        
        // Invalid: Empty title
        let mut invalid_assignment = valid_assignment.clone();
        invalid_assignment.title = "".to_string();
        assert!(invalid_assignment.validate().is_err());
        
        // Invalid: Negative points
        let mut invalid_assignment = valid_assignment.clone();
        invalid_assignment.points_possible = Some(-10.0);
        assert!(invalid_assignment.validate().is_err());
        
        // Invalid: available_until before available_from
        let mut invalid_assignment = valid_assignment.clone();
        invalid_assignment.available_from = Some(Utc.with_ymd_and_hms(2025, 5, 20, 0, 0, 0).unwrap());
        invalid_assignment.available_until = Some(Utc.with_ymd_and_hms(2025, 5, 1, 0, 0, 0).unwrap());
        assert!(invalid_assignment.validate().is_err());
    }
}