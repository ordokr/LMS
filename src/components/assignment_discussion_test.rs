#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use leptos::*;
use crate::components::assignment_discussion::AssignmentDiscussion;
use crate::services::integration_service::IntegrationService;
use crate::models::forum::Topic;
use std::sync::Arc;
use std::cell::RefCell;

wasm_bindgen_test_configure!(run_in_browser);

struct MockIntegrationService {
    assignment_topics: Arc<RefCell<std::collections::HashMap<i64, Option<Topic>>>>,
}

impl MockIntegrationService {
    fn new() -> Self {
        Self {
            assignment_topics: Arc::new(RefCell::new(std::collections::HashMap::new())),
        }
    }
    
    fn with_assignment_topic(mut self, assignment_id: i64, topic: Option<Topic>) -> Self {
        self.assignment_topics.borrow_mut().insert(assignment_id, topic);
        self
    }
    
    fn get_assignment_topic(&self, assignment_id: i64) -> Result<Option<Topic>, String> {
        match self.assignment_topics.borrow().get(&assignment_id) {
            Some(topic) => Ok(topic.clone()),
            None => Ok(None),
        }
    }
    
    fn create_assignment_discussion(&self, _course_id: i64, assignment_id: i64) -> Result<Topic, String> {
        let topic = Topic {
            id: 999,
            title: format!("Discussion for assignment {}", assignment_id),
            slug: format!("discussion-for-assignment-{}", assignment_id),
            category_id: 1,
            user_id: 1,
            pinned: false,
            locked: false,
            post_count: 0,
            view_count: 0,
            has_solution: false,
            last_post_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            author_name: "Test User".into(),
            category_name: "Test Category".into(),
            reply_count: 0,
            excerpt: None,
        };
        
        self.assignment_topics.borrow_mut().insert(assignment_id, Some(topic.clone()));
        Ok(topic)
    }
}

#[wasm_bindgen_test]
fn test_assignment_discussion_no_topic() {
    create_scope_immediate(|cx| {
        // Create mock services
        let mock_integration = MockIntegrationService::new().with_assignment_topic(42, None);
        provide_context(cx, mock_integration);
        
        // Render the component
        let view = view! { cx,
            <AssignmentDiscussion course_id=1 assignment_id=42 />
        };
        
        // Mount it to the document
        mount_to_body(cx, view);
        
        // Check that the component renders correctly
        let component = document().query_selector(".assignment-discussion").unwrap().unwrap();
        assert!(component.inner_html().contains("There is no discussion board for this assignment yet"));
        
        // Check that the create button is present
        let create_button = document().query_selector(".create-discussion").unwrap().unwrap();
        assert!(create_button.inner_html().contains("Create Discussion Board"));
    });
}

#[wasm_bindgen_test]
fn test_assignment_discussion_with_topic() {
    create_scope_immediate(|cx| {
        // Create a mock topic with solution
        let topic = Topic {
            id: 123,
            title: "Test Assignment Discussion".into(),
            slug: "test-assignment-discussion".into(),
            category_id: 1,
            user_id: 1,
            pinned: false,
            locked: false,
            post_count: 5,
            view_count: 10,
            has_solution: true,
            last_post_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            author_name: "Test User".into(),
            category_name: "Test Category".into(),
            reply_count: 4,
            excerpt: None,
        };
        
        // Create mock services
        let mock_integration = MockIntegrationService::new().with_assignment_topic(42, Some(topic));
        provide_context(cx, mock_integration);
        
        // Render the component
        let view = view! { cx,
            <AssignmentDiscussion course_id=1 assignment_id=42 />
        };
        
        // Mount it to the document
        mount_to_body(cx, view);
        
        // Check that the component renders correctly
        let component = document().query_selector(".assignment-discussion").unwrap().unwrap();
        
        // Check that the topic title is displayed
        let topic_title = document().query_selector(".discussion-topic h4").unwrap().unwrap();
        assert_eq!(topic_title.text_content().unwrap(), "Test Assignment Discussion");
        
        // Check that the solution indicator is displayed
        let has_solution = document().query_selector(".has-solution").unwrap();
        assert!(has_solution.is_some());
        
        // Check that view discussion button is present
        let view_button = document().query_selector(".view-discussion").unwrap().unwrap();
        assert_eq!(view_button.get_attribute("href").unwrap(), "/forum/topics/123");
        
        // Check that ask question button is present
        let ask_button = document().query_selector(".ask-question").unwrap().unwrap();
        assert_eq!(ask_button.get_attribute("href").unwrap(), "/forum/topics/123/new");
    });
}