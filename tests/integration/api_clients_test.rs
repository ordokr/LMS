use std::env;
use mockito::{mock, server_url};
use serde_json::json;

use lms::clients::canvas::CanvasClient;
use lms::clients::discourse::DiscourseClient;
use lms::utils::logger::create_logger;

#[tokio::test]
async fn test_canvas_client_fetch_courses() {
    let mock_server = mockito::Server::new();
    
    // Setup mock response
    let _m = mock_server.mock("GET", "/api/v1/courses")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[
            {"id": 1, "name": "Introduction to Computer Science", "course_code": "CS101"},
            {"id": 2, "name": "Advanced Programming", "course_code": "CS201"}
        ]"#)
        .create();

    // Create client with mock server URL
    let logger = create_logger("canvas_test");
    let canvas_client = CanvasClient::new(
        &mock_server.url(), 
        "test_token",
        logger
    );

    // Test the API call
    let courses = canvas_client.fetch_courses().await.unwrap();
    
    // Verify response
    assert_eq!(courses.len(), 2);
    assert_eq!(courses[0].id, 1);
    assert_eq!(courses[0].name, "Introduction to Computer Science");
    assert_eq!(courses[1].id, 2);
    assert_eq!(courses[1].name, "Advanced Programming");
}

#[tokio::test]
async fn test_discourse_client_fetch_categories() {
    let mock_server = mockito::Server::new();
    
    // Setup mock response
    let _m = mock_server.mock("GET", "/categories.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "category_list": {
                "categories": [
                    {"id": 1, "name": "General", "slug": "general"},
                    {"id": 2, "name": "Course Discussions", "slug": "course-discussions"}
                ]
            }
        }"#)
        .create();

    // Create client with mock server URL
    let logger = create_logger("discourse_test");
    let discourse_client = DiscourseClient::new(
        &mock_server.url(), 
        "test_api_key",
        "test_username",
        logger
    );

    // Test the API call
    let categories = discourse_client.fetch_categories().await.unwrap();
    
    // Verify response
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].id, 1);
    assert_eq!(categories[0].name, "General");
    assert_eq!(categories[1].id, 2);
    assert_eq!(categories[1].name, "Course Discussions");
}

#[tokio::test]
async fn test_error_handling_canvas() {
    let mock_server = mockito::Server::new();
    
    // Setup mock response with error
    let _m = mock_server.mock("GET", "/api/v1/courses")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"errors": [{"message": "Invalid access token"}]}"#)
        .create();

    // Create client with mock server URL
    let logger = create_logger("canvas_test");
    let canvas_client = CanvasClient::new(
        &mock_server.url(), 
        "invalid_token",
        logger
    );

    // Test the API call
    let result = canvas_client.fetch_courses().await;
    
    // Verify error handling
    assert!(result.is_err());
    match result {
        Err(e) => assert!(e.to_string().contains("Invalid access token")),
        _ => panic!("Expected an error"),
    }
}

#[tokio::test]
async fn test_error_handling_discourse() {
    let mock_server = mockito::Server::new();
    
    // Setup mock response with error
    let _m = mock_server.mock("GET", "/categories.json")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body(r#"{"errors": ["You are not authorized to perform this action"]}"#)
        .create();

    // Create client with mock server URL
    let logger = create_logger("discourse_test");
    let discourse_client = DiscourseClient::new(
        &mock_server.url(), 
        "invalid_api_key",
        "test_username",
        logger
    );

    // Test the API call
    let result = discourse_client.fetch_categories().await;
    
    // Verify error handling
    assert!(result.is_err());
    match result {
        Err(e) => assert!(e.to_string().contains("not authorized")),
        _ => panic!("Expected an error"),
    }
}
