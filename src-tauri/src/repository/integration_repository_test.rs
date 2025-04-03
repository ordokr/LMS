#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::{
        IntegrationRepository, ForumCategoryRepository, ForumTopicRepository, 
        UserRepository, CourseRepository
    };
    use crate::models::forum::{CreateCategoryRequest, CreateTopicRequest};
    use crate::models::lms::{CreateCourseRequest, CreateModuleRequest, CreateAssignmentRequest};
    use chrono::Utc;
    use rusqlite::Connection;
    use std::sync::Arc;

    fn setup_test_db() -> Arc<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        
        // Initialize schema
        conn.execute_batch(include_str!("../../migrations/20250403000000_forum_integration.sql")).unwrap();
        
        // Create test users
        conn.execute(
            "INSERT INTO users (username, email, display_name, password_hash, role, created_at, updated_at)
             VALUES ('testuser', 'test@example.com', 'Test User', 'hash', 'student', ?, ?)",
            params![Utc::now().to_rfc3339(), Utc::now().to_rfc3339()]
        ).unwrap();
        
        Arc::new(conn)
    }

    #[test]
    fn test_link_course_to_category() {
        let conn = setup_test_db();
        let integration_repo = IntegrationRepository::new(conn.clone());
        let cat_repo = ForumCategoryRepository::new(conn.clone());
        let course_repo = CourseRepository::new(conn.clone());
        
        // Create a test course
        let course_req = CreateCourseRequest {
            title: "Test Course".into(),
            code: "TEST101".into(),
            description: Some("Test Description".into()),
            start_date: Some(Utc::now()),
            end_date: None,
        };
        let course = course_repo.create_course(&course_req, 1).unwrap();
        
        // Create a test category
        let cat_req = CreateCategoryRequest {
            name: "Test Category".into(),
            slug: "test-category".into(),
            description: None,
            parent_id: None,
            course_id: None,
            color: None,
            text_color: None,
        };
        let category = cat_repo.create_category(&cat_req).unwrap();
        
        // Link course to category
        let result = integration_repo.link_course_to_category(course.id, category.id);
        assert!(result.is_ok());
        
        // Verify the link exists
        let fetched_category = integration_repo.get_category_for_course(course.id).unwrap();
        assert!(fetched_category.is_some());
        assert_eq!(fetched_category.unwrap().id, category.id);
    }

    #[test]
    fn test_link_module_to_topic() {
        let conn = setup_test_db();
        let integration_repo = IntegrationRepository::new(conn.clone());
        let topic_repo = ForumTopicRepository::new(conn.clone());
        let cat_repo = ForumCategoryRepository::new(conn.clone());
        let course_repo = CourseRepository::new(conn.clone());
        
        // Create a test course
        let course_req = CreateCourseRequest {
            title: "Test Course".into(),
            code: "TEST101".into(),
            description: Some("Test Description".into()),
            start_date: Some(Utc::now()),
            end_date: None,
        };
        let course = course_repo.create_course(&course_req, 1).unwrap();
        
        // Create a test module
        let module_req = CreateModuleRequest {
            course_id: course.id,
            title: "Test Module".into(),
            description: None,
            order: 1,
        };
        let module = course_repo.create_module(&module_req).unwrap();
        
        // Create a test category
        let cat_req = CreateCategoryRequest {
            name: "Test Category".into(),
            slug: "test-category".into(),
            description: None,
            parent_id: None,
            course_id: Some(course.id),
            color: None,
            text_color: None,
        };
        let category = cat_repo.create_category(&cat_req).unwrap();
        
        // Create a test topic
        let topic_req = CreateTopicRequest {
            title: "Test Topic".into(),
            slug: "test-topic".into(),
            category_id: category.id,
            content: "Test content".into(),
        };
        let topic = topic_repo.create_topic(&topic_req, 1).unwrap();
        
        // Link module to topic
        let result = integration_repo.link_module_to_topic(module.id, topic.id);
        assert!(result.is_ok());
        
        // Verify the link exists
        let fetched_topic = integration_repo.get_topic_for_module(module.id).unwrap();
        assert!(fetched_topic.is_some());
        assert_eq!(fetched_topic.unwrap().id, topic.id);
    }

    #[test]
    fn test_link_assignment_to_topic() {
        let conn = setup_test_db();
        let integration_repo = IntegrationRepository::new(conn.clone());
        let topic_repo = ForumTopicRepository::new(conn.clone());
        let cat_repo = ForumCategoryRepository::new(conn.clone());
        let course_repo = CourseRepository::new(conn.clone());
        
        // Create a test course
        let course_req = CreateCourseRequest {
            title: "Test Course".into(),
            code: "TEST101".into(),
            description: Some("Test Description".into()),
            start_date: Some(Utc::now()),
            end_date: None,
        };
        let course = course_repo.create_course(&course_req, 1).unwrap();
        
        // Create a test module
        let module_req = CreateModuleRequest {
            course_id: course.id,
            title: "Test Module".into(),
            description: None,
            order: 1,
        };
        let module = course_repo.create_module(&module_req).unwrap();
        
        // Create a test assignment
        let assignment_req = CreateAssignmentRequest {
            module_id: module.id,
            title: "Test Assignment".into(),
            description: None,
            points: Some(100),
            due_date: None,
            order: 1,
        };
        let assignment = course_repo.create_assignment(&assignment_req).unwrap();
        
        // Create a test category
        let cat_req = CreateCategoryRequest {
            name: "Test Category".into(),
            slug: "test-category".into(),
            description: None,
            parent_id: None,
            course_id: Some(course.id),
            color: None,
            text_color: None,
        };
        let category = cat_repo.create_category(&cat_req).unwrap();
        
        // Create a test topic
        let topic_req = CreateTopicRequest {
            title: "Test Topic".into(),
            slug: "test-topic".into(),
            category_id: category.id,
            content: "Test content".into(),
        };
        let topic = topic_repo.create_topic(&topic_req, 1).unwrap();
        
        // Link assignment to topic
        let result = integration_repo.link_assignment_to_topic(assignment.id, topic.id);
        assert!(result.is_ok());
        
        // Verify the link exists
        let fetched_topic = integration_repo.get_topic_for_assignment(assignment.id).unwrap();
        assert!(fetched_topic.is_some());
        assert_eq!(fetched_topic.unwrap().id, topic.id);
    }

    #[test]
    fn test_get_recent_course_activity() {
        let conn = setup_test_db();
        let integration_repo = IntegrationRepository::new(conn.clone());
        let topic_repo = ForumTopicRepository::new(conn.clone());
        let cat_repo = ForumCategoryRepository::new(conn.clone());
        let course_repo = CourseRepository::new(conn.clone());
        
        // Create a test course
        let course_req = CreateCourseRequest {
            title: "Test Course".into(),
            code: "TEST101".into(),
            description: Some("Test Description".into()),
            start_date: Some(Utc::now()),
            end_date: None,
        };
        let course = course_repo.create_course(&course_req, 1).unwrap();
        
        // Create a test category
        let cat_req = CreateCategoryRequest {
            name: "Test Category".into(),
            slug: "test-category".into(),
            description: None,
            parent_id: None,
            course_id: Some(course.id),
            color: None,
            text_color: None,
        };
        let category = cat_repo.create_category(&cat_req).unwrap();
        
        // Link course to category
        integration_repo.link_course_to_category(course.id, category.id).unwrap();
        
        // Create test topics
        for i in 1..=5 {
            let topic_req = CreateTopicRequest {
                title: format!("Test Topic {}", i),
                slug: format!("test-topic-{}", i),
                category_id: category.id,
                content: format!("Test content {}", i),
            };
            topic_repo.create_topic(&topic_req, 1).unwrap();
        }
        
        // Get recent activity
        let activity = integration_repo.get_recent_course_activity(course.id, 3).unwrap();
        
        // Verify we get the right number of topics
        assert_eq!(activity.len(), 3);
        
        // Verify topics are sorted by creation date (descending)
        for i in 0..activity.len() - 1 {
            assert!(activity[i].created_at >= activity[i + 1].created_at);
        }
    }
}