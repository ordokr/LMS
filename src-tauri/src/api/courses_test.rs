#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::sync::Arc;
    use crate::db::course_repository::{CourseRepository, MockCourseRepository};
    use crate::models::course::{Course, CourseStatus};
    
    #[tokio::test]
    async fn test_get_courses_success() {
        // Create mock repository
        let mut mock_repo = MockCourseRepository::new();
        
        // Set up expected behavior
        let expected_courses = vec![
            Course {
                id: "course1".to_string(),
                title: "Introduction to Rust".to_string(),
                description: "Learn Rust programming".to_string(),
                status: CourseStatus::Active,
                // ... other fields with default values or constructors
            },
            Course {
                id: "course2".to_string(),
                title: "Advanced TypeScript".to_string(),
                description: "TypeScript for experienced developers".to_string(),
                status: CourseStatus::Active,
                // ... other fields with default values or constructors
            }
        ];
        
        mock_repo.expect_get_courses()
            .with(eq(Some(CourseStatus::Active)))
            .times(1)
            .returning(move |_| Ok(expected_courses.clone()));
        
        // Create State with Arc-wrapped mock
        let repo_state = tauri::State::new(Arc::new(mock_repo) as Arc<dyn CourseRepository + Send + Sync>);
        
        // Call the function
        let result = get_courses(Some(CourseStatus::Active), repo_state).await;
        
        // Assert
        assert!(result.is_ok());
        let courses = result.unwrap();
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].title, "Introduction to Rust");
        assert_eq!(courses[1].title, "Advanced TypeScript");
    }

    #[tokio::test]
    async fn test_get_course_found() {
        // Create mock repository
        let mut mock_repo = MockCourseRepository::new();
        
        // Set up expected behavior
        let course_id = "course1";
        let expected_course = Course {
            id: course_id.to_string(),
            title: "Introduction to Rust".to_string(),
            description: "Learn Rust programming".to_string(),
            status: CourseStatus::Active,
            // ... other fields as needed
        };
        
        mock_repo.expect_get_course_by_id()
            .with(eq(course_id))
            .times(1)
            .returning(move |_| Ok(Some(expected_course.clone())));
        
        // Create State with Arc-wrapped mock
        let repo_state = tauri::State::new(Arc::new(mock_repo));
        
        // Call the function
        let result = get_course(course_id.to_string(), repo_state).await;
        
        // Assert
        assert!(result.is_ok());
        let course = result.unwrap();
        assert_eq!(course.id, course_id);
        assert_eq!(course.title, "Introduction to Rust");
    }

    #[tokio::test]
    async fn test_get_course_not_found() {
        // Create mock repository
        let mut mock_repo = MockCourseRepository::new();
        
        // Set up expected behavior
        let course_id = "nonexistent";
        
        mock_repo.expect_get_course_by_id()
            .with(eq(course_id))
            .times(1)
            .returning(|_| Ok(None));
        
        // Create State with Arc-wrapped mock
        let repo_state = tauri::State::new(Arc::new(mock_repo));
        
        // Call the function
        let result = get_course(course_id.to_string(), repo_state).await;
        
        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), format!("Course not found with ID: {}", course_id));
    }
}