use crate::test_helpers::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::net::SocketAddr;
use tower::ServiceExt;
use serde_json::json;

#[tokio::test]
async fn test_create_and_get_mapping() {
    // Set up test app
    let (app, _) = setup_test_app().await;

    // 1. Create a new mapping
    let create_req = Request::builder()
        .uri("/api/integration/mappings")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "course_id": 123,
            "category_id": 456
        }).to_string()))
        .unwrap();

    let create_resp = app.oneshot(create_req).await.unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    
    let create_body = hyper::body::to_bytes(create_resp.into_body()).await.unwrap();
    let mapping: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let mapping_id = mapping["id"].as_i64().unwrap();

    // 2. Get mapping by ID
    let get_req = Request::builder()
        .uri(&format!("/api/integration/mappings/{}", mapping_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let get_resp = app.oneshot(get_req).await.unwrap();
    assert_eq!(get_resp.status(), StatusCode::OK);
    
    let get_body = hyper::body::to_bytes(get_resp.into_body()).await.unwrap();
    let retrieved: serde_json::Value = serde_json::from_slice(&get_body).unwrap();
    
    assert_eq!(retrieved["course_id"], 123);
    assert_eq!(retrieved["category_id"], 456);
    assert_eq!(retrieved["sync_enabled"], true);

    // 3. Get mapping by course ID
    let get_by_course_req = Request::builder()
        .uri("/api/integration/mappings/course/123")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let get_by_course_resp = app.oneshot(get_by_course_req).await.unwrap();
    assert_eq!(get_by_course_resp.status(), StatusCode::OK);
    
    // 4. Update the mapping
    let update_req = Request::builder()
        .uri(&format!("/api/integration/mappings/{}", mapping_id))
        .method("PUT")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "sync_enabled": false,
            "sync_topics": true,
            "sync_users": false
        }).to_string()))
        .unwrap();
    
    let update_resp = app.oneshot(update_req).await.unwrap();
    assert_eq!(update_resp.status(), StatusCode::OK);
    
    let update_body = hyper::body::to_bytes(update_resp.into_body()).await.unwrap();
    let updated: serde_json::Value = serde_json::from_slice(&update_body).unwrap();
    
    assert_eq!(updated["sync_enabled"], false);
    assert_eq!(updated["sync_topics"], true);
    assert_eq!(updated["sync_users"], false);
    
    // 5. Delete the mapping
    let delete_req = Request::builder()
        .uri(&format!("/api/integration/mappings/{}", mapping_id))
        .method("DELETE")
        .body(Body::empty())
        .unwrap();
        
    let delete_resp = app.oneshot(delete_req).await.unwrap();
    assert_eq!(delete_resp.status(), StatusCode::NO_CONTENT);
    
    // 6. Verify it's been deleted
    let verify_delete_req = Request::builder()
        .uri(&format!("/api/integration/mappings/{}", mapping_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();
        
    let verify_delete_resp = app.oneshot(verify_delete_req).await.unwrap();
    assert_eq!(verify_delete_resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_generate_sso_token() {
    // Set up test app
    let (app, _) = setup_test_app().await;
    
    // Generate a token
    let token_req = Request::builder()
        .uri("/api/integration/auth/token")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "user_id": "user123",
            "role": "student",
            "canvas_id": "canvas456",
            "discourse_id": "discourse789"
        }).to_string()))
        .unwrap();
        
    let token_resp = app.oneshot(token_req).await.unwrap();
    assert_eq!(token_resp.status(), StatusCode::OK);
    
    let token_body = hyper::body::to_bytes(token_resp.into_body()).await.unwrap();
    let token_json: serde_json::Value = serde_json::from_slice(&token_body).unwrap();
    
    assert!(token_json["token"].as_str().is_some());
}