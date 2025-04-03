use leptos::*;
use wasm_bindgen_test::*;
use crate::app::App;
use crate::tests::utils::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

fn setup_test() {
    // Setup mock API responses
    setup_mock_api(vec![
        // Mock course data
        mock_response("GET", "/api/courses/123", 200, json!({
            "id": 123,
            "title": "Test Course",
            "code": "TEST101",
            "description": "Test description",
            "instructor_id": 1,
            "instructor_name": "Test Instructor"
        })),
        
        // Mock module data
        mock_response("GET", "/api/courses/modules/456", 200, json!({
            "id": 456,
            "course_id": 123,
            "title": "Test Module",
            "description": "Module description",
            "order": 1
        })),
        
        // Mock assignment data
        mock_response("GET", "/api/courses/assignments/789", 200, json!({
            "id": 789,
            "module_id": 456,
            "title": "Test Assignment",
            "description": "Assignment description",
            "points": 100,
            "due_date": null,
            "order": 1
        })),
        
        // Mock forum topics
        mock_response("GET", "/api/courses/0/modules/456/discussion", 404, json!({
            "error": "Not found"
        })),
        
        mock_response("POST", "/api/courses/123/modules/456/discussion", 200, json!({
            "id": 888,
            "title": "Discussion: Test Module",
            "category_id": 1,
            "user_id": 1,
            "post_count": 0,
            "view_count": 0,
            "created_at": "2025-04-03T12:00:00Z",
            "updated_at": "2025-04-03T12:00:00Z",
            "author_name": "Test User",
            "category_name": "Course: Test Course"
        })),
        
        mock_response("GET", "/api/courses/0/modules/456/discussion", 200, json!({
            "id": 888,
            "title": "Discussion: Test Module",
            "category_id": 1,
            "user_id": 1,
            "post_count": 0,
            "view_count": 0,
            "created_at": "2025-04-03T12:00:00Z",
            "updated_at": "2025-04-03T12:00:00Z",
            "author_name": "Test User",
            "category_name": "Course: Test Course"
        })),
        
        // Course forum activity
        mock_response("GET", "/api/courses/123/forum/activity?limit=5", 200, json!([
            {
                "id": 888,
                "title": "Discussion: Test Module",
                "category_id": 1,
                "user_id": 1,
                "post_count": 0,
                "view_count": 0,
                "reply_count": 0,
                "created_at": "2025-04-03T12:00:00Z",
                "updated_at": "2025-04-03T12:00:00Z",
                "author_name": "Test User",
                "category_name": "Course: Test Course"
            }
        ])),
    ]);
}

#[wasm_bindgen_test]
async fn test_module_discussion_integration() {
    // Setup test data and mocks
    setup_test();
    
    // Create test user and login
    login_test_user();
    
    // Render the app
    mount_to_body(|cx| view! { cx, <App /> });
    
    // Navigate to module page
    navigate_to("/courses/modules/456");
    
    // Wait for page to load
    gloo_timers::future::TimeoutFuture::new(100).await;
    
    // Find the module discussion section
    let discussion_section = document().query_selector(".module-discussion").unwrap().unwrap();
    
    // It should show no discussion initially
    let no_discussion = document().query_selector(".no-discussion").unwrap().unwrap();
    assert!(no_discussion.inner_html().contains("no discussion board for this module yet"));
    
    // Click the create discussion button
    let create_button = document().query_selector(".create-discussion").unwrap().unwrap();
    create_button.dyn_ref::<HtmlElement>().unwrap().click();
    
    // Wait for API call
    gloo_timers::future::TimeoutFuture::new(100).await;
    
    // Check that success message appears
    let success_message = document().query_selector(".success-message").unwrap().unwrap();
    assert!(success_message.inner_html().contains("Discussion created successfully"));
    
    // Navigate to course page to see activity
    navigate_to("/courses/123");
    
    // Wait for page to load
    gloo_timers::future::TimeoutFuture::new(100).await;
    
    // Check that forum activity is shown
    let activity_section = document().query_selector(".course-forum-activity").unwrap().unwrap();
    let topic_item = document().query_selector(".topic-item").unwrap().unwrap();
    assert!(topic_item.inner_html().contains("Discussion: Test Module"));
}