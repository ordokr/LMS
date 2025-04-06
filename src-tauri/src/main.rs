#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod database;
mod routes; // Add this line to import your routes module
mod middleware;
mod core;
mod sync;
mod lms;
mod forum;
mod api;
mod services;

use axum::{
    routing::{get, post},
    Router,
    extract::{Query, Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    middleware::{self, from_fn_with_state},
    Extension,
};
use std::{net::SocketAddr, sync::Arc};
use serde::Deserialize;
use rusqlite::Connection;
use tokio::sync::Mutex;
use tauri::Manager;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import from lms_lib for our handlers to use
use lms_lib::{
    Course, ForumPost, ForumThread, Assignment, Submission, Grade, CourseProgress, StudentPerformance,
    create_course, get_courses,
    create_forum_thread, get_forum_threads,
    create_forum_post, get_forum_posts,
    create_assignment, get_assignments,
    create_submission, get_submissions,
    create_grade, get_grade,
    create_course_progress, get_course_progress,
    create_student_performance, get_student_performance,
};

// Update imports to include the new models
use models::{User, Category, Topic, Post, Tag};

use routes::{
    list_categories, get_category, create_category, update_category, delete_category,
    // Import other routes as you implement them
};

// Add these imports to your existing imports
use crate::api::auth::{login, verify_token};

// Shared state containing the database connection
struct AppState {
    conn: Arc<Mutex<Connection>>,
}

// --- Query Parameter Structs ---
#[derive(Deserialize)]
struct CourseIdParams {
    course_id: i32,
}

#[derive(Deserialize)]
struct ThreadIdParams {
    thread_id: i32,
}

#[derive(Deserialize)]
struct SubmissionIdParams {
    submission_id: i32,
}

#[derive(Deserialize)]
struct StudentIdParams {
    student_id: i32,
}

// --- Axum Handler Functions ---

// Basic root handler
async fn root() -> &'static str {
    "Hello, World!"
}

// Helper to convert Result<T, String> to Axum response for GET requests
fn handle_get_result<T: serde::Serialize>(result: Result<T, String>) -> Response {
    match result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Helper for creation/update responses (POST/PUT etc.)
fn handle_post_result(result: Result<String, String>) -> Response {
    match result {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Course Handlers
async fn handle_create_course(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Course>
) -> impl IntoResponse {
    // Log the attempt
    println!("Attempting to create course: {} - {}", payload.name, payload.description);
    
    // Pass the database connection to the lib function
    let conn = state.conn.lock().await;
    
    // Use a more direct approach to database interaction
    match database::create_course(&conn, &payload.name, &payload.description) {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_courses() -> impl IntoResponse {
    match get_courses().await {
        Ok(courses) => (StatusCode::OK, Json(courses)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Forum Thread Handlers
async fn handle_create_forum_thread(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ForumThread>
) -> impl IntoResponse {
    println!("Attempting to create forum thread: {} - {}", payload.title, payload.category);
    
    let conn = state.conn.lock().await;
    
    match database::create_forum_thread(&conn, &payload.title, &payload.category) {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_forum_threads(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    
    match database::get_forum_threads(&conn) {
        Ok(threads) => (StatusCode::OK, Json(threads)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Forum Post Handlers
async fn handle_create_forum_post(Json(payload): Json<ForumPost>) -> impl IntoResponse {
    match create_forum_post(payload.thread_id, payload.author_id, payload.content).await {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_forum_posts(Query(params): Query<ThreadIdParams>) -> impl IntoResponse {
    match get_forum_posts(params.thread_id).await {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Assignment Handlers
async fn handle_create_assignment(Json(payload): Json<Assignment>) -> impl IntoResponse {
    match create_assignment(payload.course_id, payload.title, payload.description, payload.due_date).await {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_assignments(Query(params): Query<CourseIdParams>) -> impl IntoResponse {
    match get_assignments(params.course_id).await {
        Ok(assignments) => (StatusCode::OK, Json(assignments)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Submission Handlers
async fn handle_create_submission(Json(payload): Json<Submission>) -> impl IntoResponse {
    match create_submission(payload.assignment_id, payload.student_id, payload.content).await {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_submissions(Query(params): Query<CourseIdParams>) -> impl IntoResponse {
    match get_submissions(params.course_id).await {
        Ok(submissions) => (StatusCode::OK, Json(submissions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Grade Handlers
async fn handle_create_grade(Json(payload): Json<Grade>) -> impl IntoResponse {
    match create_grade(payload.submission_id, payload.grader_id, payload.grade, payload.feedback).await {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_grade(Query(params): Query<SubmissionIdParams>) -> impl IntoResponse {
    match get_grade(params.submission_id).await {
        Ok(grade) => (StatusCode::OK, Json(grade)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Course Progress Handlers
async fn handle_create_course_progress(Json(payload): Json<CourseProgress>) -> impl IntoResponse {
    match create_course_progress(payload.course_id, payload.student_id, payload.completed_modules, payload.total_modules).await {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_course_progress(Query(params): Query<CourseIdParams>, Query(student_params): Query<StudentIdParams>) -> impl IntoResponse {
    match get_course_progress(params.course_id, student_params.student_id).await {
        Ok(progress) => (StatusCode::OK, Json(progress)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Student Performance Handlers
async fn handle_create_student_performance(Json(payload): Json<StudentPerformance>) -> impl IntoResponse {
    match create_student_performance(payload.student_id, payload.course_id, payload.average_grade, payload.time_spent).await {
        Ok(message) => (StatusCode::CREATED, message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn handle_get_student_performance(Query(params): Query<CourseIdParams>, Query(student_params): Query<StudentIdParams>) -> impl IntoResponse {
    match get_student_performance(params.course_id, student_params.student_id).await {
        Ok(performance) => (StatusCode::OK, Json(performance)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// Tauri setup and run functions
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Create a Tokio runtime for our Axum server
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            
            // Spawn the server within the runtime
            rt.spawn(async {
                let app_router = create_api_router(app_state.clone());

                let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
                println!("Axum server listening on {}", addr);
                axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app_router.into_make_service())
                    .await
                    .unwrap();
            });
            
            // Store the runtime in the app state to keep it alive
            app.manage(rt);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // ... your existing commands
            login,
            verify_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// The main entry point for the application binary
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::load();
    
    // Initialize database
    let db_pool = init_db(&config.database.sqlite_path)
        .await
        .expect("Failed to initialize database");
    
    // Set up repositories
    let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
    let forum_category_repo = Arc::new(ForumCategoryRepository::new(db_pool.clone()));
    let forum_topic_repo = Arc::new(ForumTopicRepository::new(db_pool.clone()));
    let course_repo = Arc::new(CourseRepository::new(db_pool.clone()));
    let module_repo = Arc::new(ModuleRepository::new(db_pool.clone()));
    let assignment_repo = Arc::new(AssignmentRepository::new(db_pool.clone()));
    
    // Set up sync engine
    let sync_engine = Arc::new(SyncEngine::new(db_pool.clone()));
    
    // Initialize sync engine
    sync_engine.initialize().await.expect("Failed to initialize sync engine");
    
    // Set up sync service
    let sync_service = SyncService::new(
        sync_engine.clone(),
        config.sync.sync_endpoint.clone(),
        config.sync.sync_interval,
    );
    
    // Set up authentication service
    let auth_service = Arc::new(AuthService::new(
        config.server.jwt_secret.clone(),
        config.server.jwt_expiration,
    ));
    
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = create_router(
            user_repo.clone(),
            auth_service.clone(),
            forum_category_repo.clone(),
            forum_topic_repo.clone(),
            course_repo.clone(),
            module_repo.clone(),
            assignment_repo.clone(),
            sync_engine.clone(),
        )
        .layer(cors)
        .layer(Extension(auth_service))
        .layer(Extension(sync_engine))
        .layer(Extension(db_pool));
    
    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    tracing::info!("Listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown");
}

fn create_api_router(app_state: Arc<AppState>) -> Router {
    // Routes that require authentication
    let protected_routes = Router::new()
        // Protected routes that any authenticated user can access
        .route("/auth/me", get(get_current_user))
        .route("/auth/profile", put(update_user_profile))
        .route("/posts", post(create_post))
        .route("/topics", post(create_topic))
        .layer(middleware::from_fn(middleware::require_auth));
    
    // Routes that require admin privileges
    let admin_routes = Router::new()
        .route("/categories", post(create_category))
        .route("/categories/:id", put(update_category).delete(delete_category))
        .layer(middleware::from_fn(middleware::require_admin));
    
    // Public routes
    Router::new()
        .route("/", get(root))
        
        // Auth endpoints
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        
        // Public endpoints
        .route("/categories", get(list_categories))
        .route("/categories/:id", get(get_category))
        .route("/topics/:id", get(get_topic))
        .route("/categories/:id/topics", get(list_topics))
        .route("/topics/:id/posts", get(list_topic_posts))
        .route("/posts/:id", get(get_post))
        
        // Merge in our protected routes
        .merge(protected_routes)
        .merge(admin_routes)
        
        .with_state(app_state)
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

[dependencies]
# ...existing dependencies...
walkdir = "2.3"
serde = { version = "1.0", features = ["derive"] }
