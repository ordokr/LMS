use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

use lms::{
    AppState,
    auth::jwt::Claims,
    controllers::{
        auth_controller::{LoginRequest, LoginResponse},
        course_controller::CourseResponse,
    },
    db::{
        user_repository::UserRepository,
        course_repository::CourseRepository,
        category_repository::CategoryRepository,
        topic_repository::TopicRepository,
        post_repository::PostRepository,
        assignment_repository::AssignmentRepository,
    },
    models::{
        user::User,
        course::Course,
        category::Category,
        topic::Topic,
        assignment::Assignment,
    },
    services::{
        course_category_mapper::CourseCategoryMapper,
        assignment_topic_mapper::AssignmentTopicMapper,
    },
    setup_app,
};

async fn setup_test_app() -> (axum::Router, PgPool) {
    // Use test database URL
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/lms_test".to_string());
    
    // Set up test JWT secret
    std::env::set_var("JWT_SECRET", "test_jwt_secret_key");
    
    // Create test database pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations on test database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    // Clear test database
    sqlx::query!("TRUNCATE TABLE users, courses, categories, topics, posts, assignments CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate tables");
    
    // Create app state with test database
    let user_repo = UserRepository::new(pool.clone());
    let course_repo = CourseRepository::new(pool.clone());
    let category_repo = CategoryRepository::new(pool.clone());
    let topic_repo = TopicRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool.clone());
    let assignment_repo = AssignmentRepository::new(pool.clone());
    
    let app_state = Arc::new(AppState {
        db: user_repo,
        course_repo,
        category_repo,
        topic_repo,
        post_repo,
        assignment_repo,
    });
    
    // Set up app with routes
    let app = setup_app(app_state);
    
    (app, pool)
}

async fn create_test_user(pool: &PgPool, instructor: bool) -> User {
    let role = if instructor { "instructor" } else { "student" };
    
    let user = User::new(
        format!("test_user_{}", Uuid::new_v4().to_string().split('-').next().unwrap()),
        format!("test{}@example.com", Uuid::new_v4().to_string().split('-').next().unwrap()),
        "password_hash_for_testing", // In a real test you'd use proper hashing
        role.to_string(),
        format!("canvas_{}", Uuid::new_v4().to_string()),
    );
    
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, canvas_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        user.id,
        user.username,
        user.email,
        user.password_hash,
        user.role,
        user.canvas_id,
        user.created_at,
        user.updated_at
    )
    .execute(pool)
    .await
    .expect("Failed to insert test user");
    
    user
}

async fn create_test_course(pool: &PgPool, instructor_id: Uuid) -> Course {
    let course = Course::new(
        format!("canvas_course_{}", Uuid::new_v4().to_string().split('-').next().unwrap()),
        format!("Test Course {}", Uuid::new_v4().to_string().split('-').next().unwrap()),
        format!("TEST-{}", Uuid::new_v4().to_string().split('-').next().unwrap().to_uppercase()),
        Some("Test course description.".to_string()),
        instructor_id,
        None,
        None,
    );
    
    sqlx::query!(
        r#"
        INSERT INTO courses (
            id, canvas_id, name, code, description, instructor_id, 
            start_date, end_date, created_at, updated_at, category_id, is_published
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        course.id,
        course.canvas_id,
        course.name,
        course.code,
        course.description,
        course.instructor_id,
        course.start_date,
        course.end_date,
        course.created_at,
        course.updated_at,
        course.category_id,
        course.is_published
    )
    .execute(pool)
    .await
    .expect("Failed to insert test course");
    
    course
}

async fn create_test_category(pool: &PgPool, course_id: Option<Uuid>) -> Category {
    let category = Category::new(
        format!("Test Category {}", Uuid::new_v4().to_string().split('-').next().unwrap()),
        Some("Test category description".to_string()),
        None,
        course_id,
        0,
    );
    
    sqlx::query!(
        r#"
        INSERT INTO categories (
            id, name, slug, description, parent_id, 
            created_at, updated_at, course_id, position
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        category.id,
        category.name,
        category.slug,
        category.description,
        category.parent_id,
        category.created_at,
        category.updated_at,
        category.course_id,
        category.position
    )
    .execute(pool)
    .await
    .expect("Failed to insert test category");
    
    category
}

async fn create_test_assignment(pool: &PgPool, course_id: Uuid) -> Assignment {
    let assignment = Assignment::new(
        format!("Test Assignment {}", Uuid::new_v4().to_string().split('-').next().unwrap()),
        course_id,
        Some("Test assignment description".to_string()),
        None,
        Some(100.0),
    );
    
    sqlx::query!(
        r#"
        INSERT INTO assignments (
            id, title, course_id, description, due_date, 
            points_possible, topic_id, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        assignment.id,
        assignment.title,
        assignment.course_id,
        assignment.description,
        assignment.due_date,
        assignment.points_possible,
        assignment.topic_id,
        assignment.created_at,
        assignment.updated_at
    )
    .execute(pool)
    .await
    .expect("Failed to insert test assignment");
    
    assignment
}

#[tokio::test]
async fn test_assignment_topic_integration() {
    // Set up test app
    let (app, pool) = setup_test_app().await;
    
    // Create instructor and student
    let instructor = create_test_user(&pool, true).await;
    
    // Create course, category, and assignment
    let course = create_test_course(&pool, instructor.id).await;
    let category = create_test_category(&pool, Some(course.id)).await;
    let assignment = create_test_assignment(&pool, course.id).await;
    
    // Set up repositories
    let assignment_repo = AssignmentRepository::new(pool.clone());
    let topic_repo = TopicRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool.clone());
    
    // Create topic service
    let topic_service = lms::services::topic_service::TopicService::new(
        topic_repo.clone(),
        post_repo.clone(),
    );
    
    // Create assignment-topic mapper
    let assignment_topic_mapper = AssignmentTopicMapper::new(
        assignment_repo,
        topic_repo,
        post_repo,
        topic_service,
    );
    
    // Test create_topic_from_assignment
    let (topic, post) = assignment_topic_mapper
        .create_topic_from_assignment(
            &assignment.id,
            category.id,
            instructor.id,
        )
        .await
        .expect("Should successfully create topic from assignment");
    
    // Verify topic properties
    assert_eq!(topic.title, assignment.title);
    assert_eq!(topic.category_id, category.id);
    
    // Verify post content
    assert_eq!(post.content, assignment.description.unwrap());
    assert_eq!(post.author_id, instructor.id);
    
    // Test get_topic_for_assignment
    let result = assignment_topic_mapper
        .get_topic_for_assignment(&assignment.id)
        .await
        .expect("Should successfully get topic for assignment");
    
    assert!(result.is_some());
    let (fetched_topic, posts) = result.unwrap();
    assert_eq!(fetched_topic.id, topic.id);
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].id, post.id);
    
    // Test get_assignment_for_topic
    let fetched_assignment = assignment_topic_mapper
        .get_assignment_for_topic(&topic.id)
        .await
        .expect("Should successfully get assignment for topic")
        .expect("Assignment should exist");
    
    assert_eq!(fetched_assignment.id, assignment.id);
    
    // Test update_or_create_topic_for_assignment
    // First, let's update the assignment
    sqlx::query!(
        r#"
        UPDATE assignments
        SET title = $1, description = $2
        WHERE id = $3
        "#,
        "Updated Assignment Title",
        "Updated assignment description",
        assignment.id
    )
    .execute(&pool)
    .await
    .expect("Failed to update assignment");
    
    let (updated_topic, updated_post) = assignment_topic_mapper
        .update_or_create_topic_for_assignment(
            &assignment.id,
            category.id,
            instructor.id,
        )
        .await
        .expect("Should successfully update topic for assignment");
    
    // Verify updated topic properties
    assert_eq!(updated_topic.id, topic.id); // Same topic ID
    assert_eq!(updated_topic.title, "Updated Assignment Title");
    
    // Verify updated post content
    assert_eq!(updated_post.id, post.id); // Same post ID
    assert_eq!(updated_post.content, "Updated assignment description");
    
    // Test unlink_topic_from_assignment
    assignment_topic_mapper
        .unlink_topic_from_assignment(&assignment.id)
        .await
        .expect("Should successfully unlink topic from assignment");
    
    let result = assignment_topic_mapper
        .get_topic_for_assignment(&assignment.id)
        .await
        .expect("Should successfully get topic for assignment");
    
    assert!(result.is_none());
    
    // Make sure topic still exists (just unlinked)
    let topic_still_exists = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM topics WHERE id = $1) as "exists!"
        "#,
        topic.id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check if topic exists")
    .exists;
    
    assert!(topic_still_exists);
}

#[tokio::test]
async fn test_assignment_topic_api_endpoints() {
    // Set up test app
    let (app, pool) = setup_test_app().await;
    
    // Create instructor
    let instructor = create_test_user(&pool, true).await;
    
    // Create course, category, and assignment
    let course = create_test_course(&pool, instructor.id).await;
    let category = create_test_category(&pool, Some(course.id)).await;
    let assignment = create_test_assignment(&pool, course.id).await;
    
    // Generate JWT token for instructor
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = lms::auth::jwt::generate_token(
        &instructor.id.to_string(),
        &instructor.role,
        &instructor.canvas_id,
    ).expect("Failed to generate token");
    
    // Test creating a topic from assignment
    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/assignments/{}/topic", assignment.id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "category_id": category.id
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(create_response.status(), StatusCode::OK);
    
    // Parse response
    let body = create_response.into_body().collect().await.unwrap().to_bytes();
    let response_json: Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json.get("topic").is_some());
    assert!(response_json.get("post").is_some());
    
    let topic_id = response_json["topic"]["id"].as_str().unwrap();
    
    // Test getting the topic for an assignment
    let get_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/assignments/{}/topic", assignment.id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(get_response.status(), StatusCode::OK);
    
    // Parse response
    let body = get_response.into_body().collect().await.unwrap().to_bytes();
    let response_json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["topic"]["id"].as_str().unwrap(), topic_id);
    
    // Test unlinking the topic from the assignment
    let unlink_response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/assignments/{}/topic", assignment.id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(unlink_response.status(), StatusCode::NO_CONTENT);
    
    // Verify topic is unlinked
    let get_response_after_unlink = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/assignments/{}/topic", assignment.id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(get_response_after_unlink.status(), StatusCode::NOT_FOUND);
}
