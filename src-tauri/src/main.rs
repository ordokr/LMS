#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod database;
mod routes; // Add this line to import your routes module
mod middleware;
mod commands; // Integration commands and other command modules
mod core;
mod sync;
mod lms;
mod forum;
mod api;
mod services;
mod db;
mod repositories;
mod cache;
mod tasks;
mod monitoring;
mod search;
mod metrics;
mod error;
// mod analyzers; // Analyzers have been moved to tools/unified-analyzer
mod ai;
mod examples; // Examples for demonstrating features

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

// Add these imports
use crate::db::course_category_repository::CourseCategoryRepository;
use crate::api::integration::{
    create_course_category_mapping,
    get_course_category_mapping,
    get_course_category_mapping_by_canvas_course,
    get_all_course_category_mappings,
    update_course_category_mapping,
    delete_course_category_mapping,
    sync_course_category
};

// Update your main.rs to include blockchain components

use blockchain::{
    HybridChain,
    AdaptiveBatcher,
    DifferentialAnchoring,
    AdaptiveSyncManager,
    ResourceGovernor,
    PerformanceMetrics,
    SyncPriority,
    initialize_blockchain
};

// Add this to your existing imports
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::services::integration::discourse_integration::DiscourseIntegrationService;
use crate::services::integration::sync_service::IntegrationSyncService;
use crate::services::integration::batch_sync::BatchSyncService;
use crate::api::integration_commands::{
    sync_topic,
    sync_all_pending,
    mark_topic_for_sync,
    test_canvas_connectivity,
    test_discourse_connectivity,
};

// Add these imports at the top
use crate::api::integration_settings::{
    get_course_integration_settings,
    update_course_integration_settings,
    connect_course_to_canvas,
    disconnect_course_from_canvas,
    sync_course_with_canvas
};

// Add to your imports
use crate::services::sync_scheduler::SyncScheduler;

// Add these imports to your existing imports
use crate::api::sync_status::{
    get_sync_status,
    clear_sync_errors
};

// Add these imports to your existing imports
use crate::api::sync_history::{
    get_sync_history,
    get_sync_history_stats
};

// Add these imports
use crate::controllers::forum::{
    // Topics controller
    list_topics,
    get_topic,
    create_topic,
    update_topic,
    delete_topic,

    // Posts controller
    get_post,
    create_post,
    update_post,
    delete_post,
    like_post,
    unlike_post,

    // Categories controller
    list_categories,
    get_category,
    create_category,
    update_category,
    delete_category,
    get_category_topics
};

// Add these to your existing imports at the top
use crate::controllers::user_controller;
use crate::controllers::follow_controller;

// Shared state containing the database connection
struct AppState {
    conn: Arc<Mutex<Connection>>,
    db: sqlx::SqlitePool,
    chain: Arc<Mutex<HybridChain>>,
    batcher: Arc<AdaptiveBatcher>,
    sync_manager: Arc<AdaptiveSyncManager>,
    metrics: Arc<PerformanceMetrics>,
    governor: Arc<ResourceGovernor>,
    jwt_secret: Vec<u8>,
}

pub struct AppState {
    db_pool: sqlx::SqlitePool,
    jwt_secret: Vec<u8>,
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

            // Initialize database
            let db = db::init_db().expect("Failed to initialize database");

            // Create the integration services
            let canvas_service = Arc::new(CanvasIntegrationService::new(db.clone()));
            let discourse_service = Arc::new(DiscourseIntegrationService::new(db.clone()));
            let sync_service = Arc::new(IntegrationSyncService::new(
                db.clone(),
                canvas_service.clone(),
                discourse_service.clone(),
            ));

            // Create and start the batch sync service
            let batch_sync_service = BatchSyncService::new(
                db.clone(),
                sync_service.clone(),
                10, // batch size
                60, // interval in seconds
            );

            // Start the batch sync loop in a background task
            let batch_sync_service_arc = Arc::new(batch_sync_service);
            let batch_sync_clone = batch_sync_service_arc.clone();
            rt.spawn(async move {
                if let Err(e) = batch_sync_clone.start_batch_sync_loop().await {
                    eprintln!("Error starting batch sync loop: {}", e);
                }
            });

            // Make services available to the app
            app.manage(db);
            app.manage(canvas_service);
            app.manage(discourse_service);
            app.manage(sync_service);
            app.manage(batch_sync_service_arc);

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
            // Auth commands
            login,
            verify_token,

            // User profile commands
            user_controller::get_user_profile,
            user_controller::update_user_profile,
            user_controller::get_user_activities,
            user_controller::record_user_activity,
            user_controller::get_user_topics,
            user_controller::get_user_posts,
            user_controller::get_user_id_by_username,

            // Notification commands
            user_controller::get_user_notifications,
            user_controller::mark_notification_read,
            user_controller::mark_all_notifications_read,
            user_controller::get_unread_notification_count,
            user_controller::create_notification,
            user_controller::get_user_notification_summaries,

            // Integration notification commands
            api::notification_commands::get_notifications,
            api::notification_commands::get_unread_notifications,
            api::notification_commands::get_unread_notification_count,
            api::notification_commands::mark_notification_read,
            api::notification_commands::mark_all_notifications_read,
            api::notification_commands::dismiss_notification,
            api::notification_commands::dismiss_all_notifications,

            // Follow commands
            follow_controller::follow_user,
            follow_controller::unfollow_user,
            follow_controller::get_following,
            follow_controller::get_followers,
            follow_controller::check_follows_user,
            follow_controller::subscribe_to_topic,
            follow_controller::subscribe_to_category,
            follow_controller::get_topic_subscription,

            // Forum commands
            list_topics,
            get_topic,
            create_topic,
            update_topic,
            delete_topic,

            get_post,
            create_post,
            update_post,
            delete_post,
            like_post,
            unlike_post,

            list_categories,
            get_category,
            create_category,
            update_category,
            delete_category,
            get_category_topics,
              // Integration commands
            get_course_integration_settings,
            update_course_integration_settings,
            connect_course_to_canvas,
            disconnect_course_from_canvas,
            sync_course_with_canvas,
              // Course Category Mapping
            create_course_category_mapping,
            get_course_category_mapping,
            get_course_category_mapping_by_canvas_course,
            get_all_course_category_mappings,
            update_course_category_mapping,
            delete_course_category_mapping,
            sync_course_category,
            update_course_category_sync_direction,

            // Discourse Integration commands
            commands::get_discourse_integration_status,
            commands::get_discourse_topics,
            commands::get_discourse_categories,
            commands::get_discourse_sync_history,
            commands::sync_all_discourse_topics,
            commands::sync_discourse_topic,
            commands::setup_discourse_integration,
            commands::open_url,

            // Sync commands
            sync_topic,
            sync_all_pending,
            mark_topic_for_sync,
            test_canvas_connectivity,
            test_discourse_connectivity,
            get_sync_conflicts,
            resolve_sync_conflict,
            get_sync_status,
            clear_sync_errors,
            get_sync_history,
            get_sync_history_stats,

            // Discussion topic commands
            list_topics,
            get_topic,
            create_topic,
            update_topic,
            delete_topic,
            get_topics_by_category,
            get_topics_by_author,
            search_topics,
            get_recent_topics,
            get_active_topics,

            // Quiz flashcard commands
            quiz::commands::rate_flashcard,
            quiz::commands::create_flashcard_session,
            quiz::commands::get_flashcard_stats,

            // Quiz analytics commands
            quiz::commands::get_user_stats,
            quiz::commands::get_quiz_analytics,
            quiz::commands::generate_user_report,
            quiz::commands::generate_quiz_report,

            // Quiz export/import commands
            quiz::commands::export_quiz,
            quiz::commands::export_quiz_to_file,
            quiz::commands::export_quiz_with_options,
            quiz::commands::import_quiz_from_file,
            quiz::commands::import_quiz,

            // Quiz course integration commands
            quiz::commands::add_quiz_to_course,
            quiz::commands::remove_quiz_from_course,
            quiz::commands::update_quiz_course_mapping,
            quiz::commands::get_quizzes_for_course,
            quiz::commands::get_courses_for_quiz,
            quiz::commands::get_quiz_with_context,
            quiz::commands::get_student_quizzes,
            quiz::commands::assign_quiz_to_student,

            // Quiz notification commands
            quiz::commands::get_quiz_notifications,
            quiz::commands::get_unread_notification_count,
            quiz::commands::mark_notification_as_read,
            quiz::commands::mark_all_notifications_as_read,
            quiz::commands::delete_notification,
            quiz::commands::delete_all_notifications,
            quiz::commands::check_quiz_notifications,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// The main entry point for the application binary
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!(event = "application_start", version = env!("CARGO_PKG_VERSION"));

    // Initialize database connection pool
    let db_url = "sqlite:educonnect.db?mode=rwc&cache=shared";
    let db_pool = SqlitePoolOptions::new()
        .max_connections(16) // Adjust based on expected load
        .connect(db_url)
        .await?;

    // Apply optimizations
    optimize_db_connection(&db_pool).await?;

    // Initialize blockchain components with optimized async configuration
    let hybrid_chain = HybridChain::new(Some("chain_config.toml")).await?;
    let chain = Arc::new(Mutex::new(hybrid_chain));

    // Spawn CPU-intensive blockchain operations on dedicated threads
    let chain_clone = Arc::clone(&chain);
    tokio::task::spawn_blocking(move || {
        // Initialize blockchain cryptography (CPU-intensive)
        let crypto = BlockchainCrypto::new();
        // Store in thread-local storage or return for further use
    });

    // Spawn I/O-bound batch processing on the async runtime
    let chain_clone = Arc::clone(&chain);
    let db_clone = db_pool.clone();
    tokio::spawn(async move {
        let batch_processor = BatchProcessor::new(db_clone, chain_clone);
        batch_processor.start_batching().await;
    });

    // Initialize Tauri app with optimized state management
    tauri::Builder::default()
        .manage(AppState {
            chain: Arc::clone(&chain),
            db: db_pool,
        })
        .invoke_handler(tauri::generate_handler![
            // Command handlers
        ])
        .run(tauri::generate_context!())?;

    Ok(())
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

    // Optimize database connection
    optimize_db_connection(&db_pool).await.expect("Failed to optimize database connection");

    // Set up repositories
    let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
    let forum_category_repo = Arc::new(ForumCategoryRepository::new(db_pool.clone()));
    let forum_topic_repo = Arc::new(ForumTopicRepository::new(db_pool.clone()));
    let course_repo = Arc::new(CourseRepository::new(db_pool.clone()));
    let module_repo = Arc::new(ModuleRepository::new(db_pool.clone()));
    let assignment_repo = Arc::new(AssignmentRepository::new(db_pool.clone()));
    let course_category_repo = CourseCategoryRepository::new(db_pool.clone());

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

fn main() {
    // Configure tokio for optimal thread utilization
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_io()
        .enable_time()
        .build()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        // Initialize database
        let db_path = "educonnect.db";
        let pool = db::setup_database(db_path)
            .await
            .expect("Failed to initialize database");

        // Initialize cache
        let forum_cache = Arc::new(ForumCache::new());

        // Create application state
        let app_state = Arc::new(AppState {
            pool,
            cache: forum_cache,
        });

        // Create API router
        let app = Router::new()
            .nest("/api/forum", forum_router())
            .with_state(app_state);

        // Start server
        let addr = "0.0.0.0:3000";
        println!("Starting server on {}", addr);

        Server::bind(&addr.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .manage(pool)
        .manage(course_category_repo)
        .invoke_handler(tauri::generate_handler![
            // ...existing handlers...
            create_course_category_mapping,
            get_course_category_mapping,
            get_course_category_mapping_by_canvas_course,
            get_all_course_category_mappings,
            update_course_category_mapping,
            delete_course_category_mapping,
            sync_course_category,
            api::courses::get_courses,
            api::courses::get_course,
            api::courses::create_course,
            api::courses::update_course,
            api::courses::delete_course,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Add to database initialization section
async fn optimize_db_connection(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA journal_mode = WAL;").execute(pool).await?;
    sqlx::query("PRAGMA synchronous = NORMAL;").execute(pool).await?;
    sqlx::query("PRAGMA cache_size = -64000;").execute(pool).await?; // 64MB cache
    sqlx::query("PRAGMA foreign_keys = ON;").execute(pool).await?;
    Ok(())
}

[dependencies]
# ...existing dependencies...
walkdir = "2.3"
serde = { version = "1.0", features = ["derive"] }
use std::sync::Arc;
use axum::{Router, Server};
use tokio::runtime::Runtime;
use cache::forum_cache::ForumCache;
use api::forum::{forum_router, AppState};
use tasks::queue::TaskQueue;
use tasks::handlers::TaskHandlers;
use monitoring::metrics::METRICS;
use monitoring::memory::MemoryMonitor;
use db::optimize::configure_memory_limits;
use log::info;

fn main() {
    // Initialize logging
    env_logger::init();

    // Configure tokio for optimal thread utilization
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_io()
        .enable_time()
        .build()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        info!("Starting optimized forum application");

        // Initialize database with optimizations
        let db_path = "educonnect.db";
        let pool = match METRICS.measure_async("db_setup", || db::setup_database(db_path)).await {
            Ok(p) => p,
            Err(e) => {
                log::error!("Failed to initialize database: {:?}", e);
                std::process::exit(1);
            }
        };

        // Apply memory optimizations
        if let Err(e) = configure_memory_limits(&pool).await {
            log::warn!("Failed to configure memory limits: {:?}", e);
        }

        // Initialize advanced cache systems
        let forum_cache = Arc::new(ForumCache::new());
        info!("Cache systems initialized");

        // Start memory monitoring
        let memory_monitor = MemoryMonitor::new(pool.clone(), 50); // 50MB threshold
        memory_monitor.start_monitoring();
        info!("Memory monitoring started");

        // Initialize task queue with workers
        let task_queue = Arc::new(TaskQueue::new(4)); // 4 worker threads
        let task_handlers = TaskHandlers::new(pool.clone());
        task_queue.start_workers(task_handlers.get_handler()).await;
        info!("Background task system initialized with 4 workers");

        // Create application state
        let app_state = Arc::new(AppState {
            pool: pool.clone(),
            cache: forum_cache,
            task_queue: task_queue.clone(),
        });

        // Create API router
        let app = Router::new()
            .nest("/api/forum", forum_router())
            .with_state(app_state);

        // Start server
        let addr = "0.0.0.0:3000";
        info!("Starting server on {}", addr);

        Server::bind(&addr.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

mod db;
mod repositories;
mod api;
mod cache;
mod tasks;
mod monitoring;
mod search;

use std::sync::Arc;
use axum::{Router, Server};
use dotenv::dotenv;
use search::meilisearch::MeiliSearchClient;
use api::search::search_router;
use api::forum::{forum_router, AppState as ForumAppState};
use log::info;

#[tokio::main]
async fn main() {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    // Initialize database
    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "educonnect.db".to_string());
    let pool = match db::setup_database(&db_path).await {
        Ok(p) => Arc::new(p),
        Err(e) => {
            log::error!("Failed to initialize database: {:?}", e);
            std::process::exit(1);
        }
    };

    // Initialize Meilisearch client
    let meili_host = std::env::var("MEILI_HOST").unwrap_or_else(|_| "http://localhost:7700".to_string());
    let meili_key = std::env::var("MEILI_MASTER_KEY").ok();

    let search_client = Arc::new(MeiliSearchClient::new(
        &meili_host,
        meili_key.as_deref(),
        pool.clone(),
    ));

    // Initialize Meilisearch indexes
    if let Err(e) = search_client.initialize().await {
        log::warn!("Failed to initialize Meilisearch: {}", e);
        log::info!("Continuing without Meilisearch - search functionality will be limited");
    } else {
        // Start background sync if Meilisearch is available
        if search_client.health_check().await {
            info!("Meilisearch is healthy, starting initial sync");
            if let Err(e) = search_client.sync_data(true).await {
                log::warn!("Initial Meilisearch sync failed: {}", e);
            }

            // Start background sync task
            search_client.clone().start_background_sync();
        }
    }

    // Initialize cache systems
    let forum_cache = Arc::new(cache::forum_cache::ForumCache::new());

    // Create app states
    let forum_state = Arc::new(ForumAppState {
        pool: pool.clone(),
        cache: forum_cache.clone(),
    });

    let search_state = Arc::new(api::search::AppState {
        search_client: search_client.clone(),
    });

    // Create API router
    let app = Router::new()
        .nest("/api/forum", forum_router().with_state(forum_state))
        .nest("/api/search", search_router().with_state(search_state));

    // Start server
    let addr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .expect("Invalid server address");

    info!("Starting server on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod db;
mod repositories;
mod api;
mod cache;
mod tasks;
mod monitoring;
mod search;

use std::sync::Arc;
use axum::{Router, Server};
use search::meilisearch::MeiliSearchClient;
use search::embedded::EmbeddedMeilisearch;
use search::setup::setup_meilisearch;
use api::search::search_router;
use api::forum::{forum_router, AppState as ForumAppState};
use log::info;

#[tokio::main]
async fn main() {
    // Initialize environment
    env_logger::init();

    // Setup app data directory
    let app_data_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .expect("Failed to get app data directory");

    // Initialize database
    let db_path = app_data_dir.join("educonnect.db").to_string_lossy().to_string();
    let pool = match db::setup_database(&db_path).await {
        Ok(p) => Arc::new(p),
        Err(e) => {
            log::error!("Failed to initialize database: {:?}", e);
            std::process::exit(1);
        }
    };

    // Initialize embedded Meilisearch
    let embedded_meili = match setup_meilisearch(&app_data_dir).await {
        Ok(meili) => Arc::new(meili),
        Err(e) => {
            log::error!("Failed to setup Meilisearch: {}", e);
            std::process::exit(1);
        }
    };

    // Start embedded Meilisearch
    let meili_config = match embedded_meili.start().await {
        Ok(config) => config,
        Err(e) => {
            log::warn!("Failed to start embedded Meilisearch: {}", e);
            log::info!("Continuing without Meilisearch - search functionality will be limited");

            // Create default config with no server
            search::embedded::MeilisearchConfig {
                host: "http://127.0.0.1:7701".to_string(),
                api_key: None,
            }
        }
    };

    // Initialize Meilisearch client
    let search_client = Arc::new(MeiliSearchClient::new(
        &meili_config.host,
        meili_config.api_key.as_deref(),
        pool.clone(),
    ));

    // Initialize Meilisearch indexes
    if let Err(e) = search_client.initialize().await {
        log::warn!("Failed to initialize Meilisearch: {}", e);
        log::info!("Search functionality will be limited");
    } else {
        // Start background sync if Meilisearch is available
        if search_client.health_check().await {
            info!("Meilisearch is healthy, starting initial sync");
            if let Err(e) = search_client.sync_data(true).await {
                log::warn!("Initial Meilisearch sync failed: {}", e);
            }

            // Start background sync task
            search_client.clone().start_background_sync();
        }
    }

    // Initialize cache systems
    let forum_cache = Arc::new(cache::forum_cache::ForumCache::new());

    // Create app states
    let forum_state = Arc::new(ForumAppState {
        pool: pool.clone(),
        cache: forum_cache.clone(),
    });

    let search_state = Arc::new(api::search::AppState {
        search_client: search_client.clone(),
    });

    // Create API router
    let app = Router::new()
        .nest("/api/forum", forum_router().with_state(forum_state))
        .nest("/api/search", search_router().with_state(search_state));

    // Start server
    let addr = "127.0.0.1:3000".parse().expect("Invalid server address");
    info!("Starting server on {}", addr);

    // Ensure Meilisearch is properly stopped when the app exits
    let embedded_meili_clone = embedded_meili.clone();
    ctrlc::set_handler(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = embedded_meili_clone.stop().await {
                log::error!("Error stopping Meilisearch: {}", e);
            }
        });
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Add to your existing main.rs

use std::sync::Arc;
use tauri::State;
use sqlx::sqlite::SqlitePoolOptions;
use crate::repositories::unified::UserRepository;
use crate::services::canvas::CanvasClient;
use crate::services::discourse::DiscourseClient;
use crate::services::sync::UserSyncService;

// Updated AppState struct
pub struct AppState {
    db_pool: sqlx::Pool<sqlx::Sqlite>,
    user_repository: Arc<UserRepository>,
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
    user_sync_service: Arc<UserSyncService>,
    is_online: std::sync::atomic::AtomicBool,
}

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Connect to SQLite database
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:data.db")
        .await
        .expect("Failed to connect to SQLite database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    // Initialize repositories
    let user_repository = Arc::new(UserRepository::new(db_pool.clone()));

    // Initialize clients
    let canvas_client = Arc::new(CanvasClient::new(
        "https://canvas.example.com/api/v1",
        "your_canvas_api_key",
    ));

    let discourse_client = Arc::new(DiscourseClient::new(
        "https://discourse.example.com",
        "your_discourse_api_key",
        "system",
    ));

    // Initialize services
    let user_sync_service = Arc::new(UserSyncService::new(
        user_repository.clone(),
        canvas_client.clone(),
        discourse_client.clone(),
    ));

    // Build Tauri application
    tauri::Builder::default()
        .manage(AppState {
            db_pool,
            user_repository,
            canvas_client,
            discourse_client,
            user_sync_service,
            is_online: std::sync::atomic::AtomicBool::new(true),
        })
        .invoke_handler(tauri::generate_handler![
            commands::unified_model_commands::get_user,
            commands::unified_model_commands::sync_user,
            commands::unified_model_commands::create_user,
            commands::unified_model_commands::update_user,
            commands::unified_model_commands::delete_user,
            // Add more command registrations here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
