#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use leptos::*;
use crate::components::module_discussion::ModuleDiscussion;
use crate::services::integration_service::IntegrationService;
use crate::services::forum_service::ForumService;
use crate::services::course_service::CourseService;
use crate::models::forum::Topic;
use std::sync::Arc;
use std::cell::RefCell;

wasm_bindgen_test_configure!(run_in_browser);

struct MockIntegrationService {
    module_topics: Arc<RefCell<std::collections::HashMap<i64, Option<Topic>>>>,
}

impl MockIntegrationService {
    fn new() -> Self {
        Self {
            module_topics: Arc::new(RefCell::new(std::collections::HashMap::new())),
        }
    }
    
    fn with_module_topic(mut self, module_id: i64, topic: Option<Topic>) -> Self {
        self.module_topics.borrow_mut().insert(module_id, topic);
        self
    }
    
    fn get_module_topic(&self, module_id: i64) -> Result<Option<Topic>, String> {
        match self.module_topics.borrow().get(&module_id) {
            Some(topic) => Ok(topic.clone()),
            None => Ok(None),
        }
    }
    
    fn create_module_discussion(&self, _course_id: i64, module_id: i64) -> Result<Topic, String> {
        let topic = Topic {
            id: 999,
            title: format!("Discussion for module {}", module_id),
            slug: format!("discussion-for-module-{}", module_id),
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
        
        self.module_topics.borrow_mut().insert(module_id, Some(topic.clone()));
        Ok(topic)
    }
}

#[wasm_bindgen_test]
fn test_module_discussion_no_topic() {
    create_scope_immediate(|cx| {
        // Create mock services
        let mock_integration = MockIntegrationService::new().with_module_topic(42, None);
        provide_context(cx, mock_integration);
        
        // Render the component
        let view = view! { cx,
            <ModuleDiscussion course_id=1 module_id=42 />
        };
        
        // Mount it to the document
        mount_to_body(cx, view);
        
        // Check that the component renders correctly
        let component = document().query_selector(".module-discussion").unwrap().unwrap();
        assert!(component.inner_html().contains("There is no discussion board for this module yet"));
        
        // Check that the create button is present
        let create_button = document().query_selector(".create-discussion").unwrap().unwrap();
        assert!(create_button.inner_html().contains("Create Discussion Board"));
    });
}

#[wasm_bindgen_test]
fn test_module_discussion_with_topic() {
    create_scope_immediate(|cx| {
        // Create a mock topic
        let topic = Topic {
            id: 123,
            title: "Test Module Discussion".into(),
            slug: "test-module-discussion".into(),
            category_id: 1,
            user_id: 1,
            pinned: false,
            locked: false,
            post_count: 5,
            view_count: 10,
            has_solution: false,
            last_post_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            author_name: "Test User".into(),
            category_name: "Test Category".into(),
            reply_count: 4,
            excerpt: None,
        };
        
        // Create mock services
        let mock_integration = MockIntegrationService::new().with_module_topic(42, Some(topic));
        provide_context(cx, mock_integration);
        
        // Render the component
        let view = view! { cx,
            <ModuleDiscussion course_id=1 module_id=42 />
        };
        
        // Mount it to the document
        mount_to_body(cx, view);
        
        // Check that the component renders correctly
        let component = document().query_selector(".module-discussion").unwrap().unwrap();
        
        // Check that the topic title is displayed
        let topic_title = document().query_selector(".discussion-topic h4").unwrap().unwrap();
        assert_eq!(topic_title.text_content().unwrap(), "Test Module Discussion");
        
        // Check that the post count is displayed
        let post_count = document().query_selector(".post-count").unwrap().unwrap();
        assert!(post_count.inner_html().contains("5 posts"));
        
        // Check that view discussion button is present
        let view_button = document().query_selector(".view-discussion").unwrap().unwrap();
        assert_eq!(view_button.get_attribute("href").unwrap(), "/forum/topics/123");
    });
}

#[wasm_bindgen_test]
async fn test_module_discussion_create() {
    create_scope_immediate(|cx| {
        // Create mock services
        let mock_integration = MockIntegrationService::new().with_module_topic(42, None);
        provide_context(cx, mock_integration);
        
        // Render the component
        let view = view! { cx,
            <ModuleDiscussion course_id=1 module_id=42 />
        };
        
        // Mount it to the document
        mount_to_body(cx, view);
        
        // Get the create button
        let create_button = document().query_selector(".create-discussion").unwrap().unwrap();
        
        // Click the button to create a discussion
        create_button.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        
        // Wait for the action to complete
        wasm_bindgen_futures::spawn_local(async {
            // Use a small delay to let the UI update
            gloo_timers::future::TimeoutFuture::new(100).await;
            
            // Check that success message is displayed
            let success = document().query_selector(".success-message").unwrap();
            assert!(success.is_some());
            
            // Check that the view button has the right URL
            let view_button = document().query_selector(".success-message .view-discussion").unwrap().unwrap();
            assert_eq!(view_button.get_attribute("href").unwrap(), "/forum/topics/999");
        });
    });
}