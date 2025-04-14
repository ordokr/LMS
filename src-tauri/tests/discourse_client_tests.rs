use crate::api::discourse::DiscourseClient;
use crate::models::discussion_mapping::{DiscourseTopic, DiscoursePost};
use mockito::{mock, Matcher};
use tokio;

#[tokio::test]
async fn test_get_topic() {
    let _mock = mock("GET", "/t/123.json")
        .with_status(200)
        .with_body(r#"{
            "id": "123",
            "title": "Test Topic",
            "category_id": "1",
            "created_at": "2025-04-13T00:00:00Z",
            "updated_at": "2025-04-13T01:00:00Z"
        }"#)
        .create();

    let client = DiscourseClient::new(mockito::server_url(), "api_key", "api_username");
    let topic = client.get_topic("123").await.unwrap();

    assert_eq!(topic.id, "123");
    assert_eq!(topic.title, "Test Topic");
    assert_eq!(topic.category_id, "1");
}

#[tokio::test]
async fn test_create_topic() {
    let _mock = mock("POST", "/posts.json")
        .match_body(Matcher::PartialJson(r#"{
            "title": "New Topic",
            "raw": "This is a new topic."
        }"#))
        .with_status(200)
        .with_body(r#"{
            "id": "124",
            "title": "New Topic",
            "category_id": "1",
            "created_at": "2025-04-13T00:00:00Z",
            "updated_at": "2025-04-13T01:00:00Z"
        }"#)
        .create();

    let client = DiscourseClient::new(mockito::server_url(), "api_key", "api_username");
    let topic = client.create_topic("New Topic", "This is a new topic.", None).await.unwrap();

    assert_eq!(topic.id, "124");
    assert_eq!(topic.title, "New Topic");
}

#[tokio::test]
async fn test_get_categories() {
    let _mock = mock("GET", "/categories.json")
        .with_status(200)
        .with_body(r#"[
            {"id": "1", "name": "General"},
            {"id": "2", "name": "Announcements"}
        ]"#)
        .create();

    let client = DiscourseClient::new(mockito::server_url(), "api_key", "api_username");
    let categories = client.get_categories().await.unwrap();

    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].id, "1");
    assert_eq!(categories[0].name, "General");
}
