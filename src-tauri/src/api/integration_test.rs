#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::test_utils::{setup_test_app, create_test_user, login_test_user};
    use crate::models::forum::{Category, Topic};
    use crate::models::lms::{CreateCourseRequest, CreateModuleRequest, CreateAssignmentRequest};
    use axum::http::{Request, StatusCode};
    use axum_test::{TestServer, TestServerConfig};
    use chrono::Utc;
    use serde_json::json;

    #[tokio::test]
    async fn test_course_category_creation() {
        // Setup test server
        let app = setup_test_app().await;
        
        // Create and login a test user
        let user = create_test_user(&app, "admin").await;
        let token = login_test_user(&app, &user.username).await;
        
        // Create a test course
        let course_req = json!({
            "title": "Test Course",
            "code": "TEST101",
            "description": "Test Description",
        });
        
        let response = app
            .post("/api/courses")
            .header("Authorization", format!("Bearer {}", token))
            .json(&course_req)
            .send()
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let course: serde_json::Value = response.json().await;
        let course_id = course["id"].as_i64().unwrap();
        
        // Test get_or_create_course_category
        let response = app
            .post(&format!("/api/courses/{}/category", course_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let category: Category = response.json().await;
        
        // Verify category exists and links to course
        assert!(category.course_id.is_some());
        assert_eq!(category.course_id.unwrap(), course_id);
        
        // Test getting the category
        let response = app
            .get(&format!("/api/courses/{}/category", course_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let fetched_category: Category = response.json().await;
        assert_eq!(fetched_category.id, category.id);
    }

    #[tokio::test]
    async fn test_module_discussion() {
        // Setup test server
        let app = setup_test_app().await;
        
        // Create and login a test user
        let user = create_test_user(&app, "admin").await;
        let token = login_test_user(&app, &user.username).await;
        
        // Create a test course
        let course_req = json!({
            "title": "Test Course",
            "code": "TEST101",
            "description": "Test Description",
        });
        
        let response = app
            .post("/api/courses")
            .header("Authorization", format!("Bearer {}", token))
            .json(&course_req)
            .send()
            .await;
        
        let course: serde_json::Value = response.json().await;
        let course_id = course["id"].as_i64().unwrap();
        
        // Create a test module
        let module_req = json!({
            "course_id": course_id,
            "title": "Test Module",
            "order": 1,
        });
        
        let response = app
            .post("/api/courses/modules")
            .header("Authorization", format!("Bearer {}", token))
            .json(&module_req)
            .send()
            .await;
            
        let module: serde_json::Value = response.json().await;
        let module_id = module["id"].as_i64().unwrap();
        
        // Test creating a module discussion
        let response = app
            .post(&format!("/api/courses/{}/modules/{}/discussion", course_id, module_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let topic: Topic = response.json().await;
        
        // Test getting the module topic
        let response = app
            .get(&format!("/api/courses/0/modules/{}/discussion", module_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let fetched_topic: Topic = response.json().await;
        assert_eq!(fetched_topic.id, topic.id);
    }

    #[tokio::test]
    async fn test_assignment_discussion() {
        // Setup test server
        let app = setup_test_app().await;
        
        // Create and login a test user
        let user = create_test_user(&app, "admin").await;
        let token = login_test_user(&app, &user.username).await;
        
        // Create a test course with module and assignment
        let course_req = json!({
            "title": "Test Course",
            "code": "TEST101",
            "description": "Test Description",
        });
        
        let response = app
            .post("/api/courses")
            .header("Authorization", format!("Bearer {}", token))
            .json(&course_req)
            .send()
            .await;
        
        let course: serde_json::Value = response.json().await;
        let course_id = course["id"].as_i64().unwrap();
        
        // Create a test module
        let module_req = json!({
            "course_id": course_id,
            "title": "Test Module",
            "order": 1,
        });
        
        let response = app
            .post("/api/courses/modules")
            .header("Authorization", format!("Bearer {}", token))
            .json(&module_req)
            .send()
            .await;
            
        let module: serde_json::Value = response.json().await;
        let module_id = module["id"].as_i64().unwrap();
        
        // Create a test assignment
        let assignment_req = json!({
            "module_id": module_id,
            "title": "Test Assignment",
            "points": 100,
            "order": 1,
        });
        
        let response = app
            .post("/api/courses/assignments")
            .header("Authorization", format!("Bearer {}", token))
            .json(&assignment_req)
            .send()
            .await;
            
        let assignment: serde_json::Value = response.json().await;
        let assignment_id = assignment["id"].as_i64().unwrap();
        
        // Test creating an assignment discussion
        let response = app
            .post(&format!("/api/courses/{}/assignments/{}/discussion", course_id, assignment_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let topic: Topic = response.json().await;
        
        // Test getting the assignment topic
        let response = app
            .get(&format!("/api/courses/0/assignments/{}/discussion", assignment_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let fetched_topic: Topic = response.json().await;
        assert_eq!(fetched_topic.id, topic.id);
    }

    #[tokio::test]
    async fn test_course_forum_activity() {
        // Setup test server
        let app = setup_test_app().await;
        
        // Create and login a test user
        let user = create_test_user(&app, "admin").await;
        let token = login_test_user(&app, &user.username).await;
        
        // Create a test course
        let course_req = json!({
            "title": "Test Course",
            "code": "TEST101",
            "description": "Test Description",
        });
        
        let response = app
            .post("/api/courses")
            .header("Authorization", format!("Bearer {}", token))
            .json(&course_req)
            .send()
            .await;
        
        let course: serde_json::Value = response.json().await;
        let course_id = course["id"].as_i64().unwrap();
        
        // Create a course category
        let response = app
            .post(&format!("/api/courses/{}/category", course_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        let category: Category = response.json().await;
        
        // Create a few topics in this category
        for i in 1..=3 {
            let topic_req = json!({
                "title": format!("Test Topic {}", i),
                "category_id": category.id,
                "content": format!("Test content {}", i),
            });
            
            app.post("/api/forum/topics")
                .header("Authorization", format!("Bearer {}", token))
                .json(&topic_req)
                .send()
                .await;
        }
        
        // Test getting course forum activity
        let response = app
            .get(&format!("/api/courses/{}/forum/activity?limit=2", course_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
            
        assert_eq!(response.status_code(), StatusCode::OK);
        let topics: Vec<Topic> = response.json().await;
        
        // Verify we get the right number of topics
        assert_eq!(topics.len(), 2);
        
        // Verify topics are from the right category
        for topic in &topics {
            assert_eq!(topic.category_id, category.id);
        }
    }
}