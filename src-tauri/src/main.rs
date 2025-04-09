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
mod db;
mod repositories;
mod cache;
mod tasks;
mod monitoring;
mod search;
mod metrics;
mod error;
mod analyzers;
mod ai;

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
            
            // Sync commands
            sync_topic,
            sync_all_pending,
            mark_topic_for_sync,
            test_canvas_connectivity,
            test_discourse_connectivity,
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

// Add these commands to your existing main.rs Tauri setup

mod api;
mod core;
mod db;
mod models;

use api::monitoring_commands::{
    get_system_monitoring_data,
    get_system_monitoring_history,
    save_system_monitoring_snapshot,
    update_system_health,
    record_system_sync_attempt,
    record_system_conflict,
    record_system_conflict_resolution,
    update_system_connection_status,
    cleanup_system_monitoring_data,
};
use core::monitoring::initialize_monitoring;

fn main() {
    // Initialize monitoring system at startup
    if let Err(e) = initialize_monitoring() {
        eprintln!("Failed to initialize monitoring system: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Your existing commands here
            
            // Monitoring commands
            get_system_monitoring_data,
            get_system_monitoring_history,
            save_system_monitoring_snapshot,
            update_system_health,
            record_system_sync_attempt,
            record_system_conflict,
            record_system_conflict_resolution,
            update_system_connection_status,
            cleanup_system_monitoring_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod db;
mod repositories;
mod api;
mod cache;
mod tasks;
mod monitoring;
mod search;
mod metrics;

use std::sync::Arc;
use axum::{Router, Server};
use search::meilisearch::MeiliSearchClient;
use search::embedded::EmbeddedMeilisearch;
use search::setup::setup_meilisearch;
use search::async_init::AsyncSearchInitializer;
use api::search::search_router;
use api::forum::{forum_router, AppState as ForumAppState};
use log::info;
use tokio::sync::oneshot;

const MAX_MEILISEARCH_MEMORY_MB: u64 = 256; // Limit memory usage to 256MB

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
    
    // Initialize embedded Meilisearch without blocking startup
    let (meili_setup_tx, meili_setup_rx) = oneshot::channel();
    
    tokio::spawn(async move {
        match setup_meilisearch(&app_data_dir).await {
            Ok(meili) => {
                let _ = meili_setup_tx.send(Arc::new(meili));
            },
            Err(e) => {
                log::error!("Failed to setup Meilisearch: {}", e);
                let _ = meili_setup_tx.send(Arc::new(EmbeddedMeilisearch::new(
                    std::path::PathBuf::new(), // Empty path
                    app_data_dir.join("meilisearch_data"),
                    7701,
                    None,
                    MAX_MEILISEARCH_MEMORY_MB
                )));
            }
        }
    });
    
    // Initialize cache systems (this doesn't need to wait for search)
    let forum_cache = Arc::new(cache::forum_cache::ForumCache::new());

    // Create forum API state
    let forum_state = Arc::new(ForumAppState {
        pool: pool.clone(),
        cache: forum_cache.clone(),
    });
    
    // Create placeholder search state that will be updated when search is ready
    let default_search_client = Arc::new(MeiliSearchClient::new(
        "http://localhost:7701",
        None,
        pool.clone()
    ));
    
    let search_state = Arc::new(api::search::AppState {
        search_client: default_search_client,
    });
    
    // Create API router
    let app = Router::new()
        .nest("/api/forum", forum_router().with_state(forum_state.clone()))
        .nest("/api/search", search_router().with_state(search_state.clone()));
    
    // Start server in background to allow main thread to continue initialization
    let (server_tx, server_rx) = oneshot::channel();
    
    tokio::spawn(async move {
        // Get address
        let addr = "127.0.0.1:3000".parse().expect("Invalid server address");
        info!("Starting server on {}", addr);
        
        // Create the server
        let server = Server::bind(&addr)
            .serve(app.into_make_service());
            
        // Signal that the server is ready
        let _ = server_tx.send(());
        
        // Start serving
        if let Err(e) = server.await {
            log::error!("Server error: {}", e);
        }
    });
    
    // Wait for server to be ready
    let _ = server_rx.await;
    
    // Now that the server is running, initialize search in background
    tokio::spawn(async move {
        // Wait for Meilisearch setup to complete
        match meili_setup_rx.await {
            Ok(embedded_meili) => {
                info!("Starting Meilisearch with limited resources");
                
                // Create async initializer
                let initializer = Arc::new(AsyncSearchInitializer::new(embedded_meili));
                
                // Start initialization in background
                initializer.start_initialization(pool.clone()).await;
                
                // Wait for initialization to complete or timeout
                let init_result = tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                        info!("Search initialization continuing in background");
                        false
                    },
                    is_ready = initializer.is_ready() => is_ready,
                };
                
                if init_result {
                    info!("Search is ready to use");
                    
                    // Get the client
                    if let Some(client) = initializer.get_client().await {
                        // Replace search client in API state
                        // Note: This would require modifying the search_state to use Arc<RwLock<MeiliSearchClient>>
                    }
                }
            },
            Err(_) => {
                log::error!("Failed to setup Meilisearch");
            }
        }
    });
    
    // Setup graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        info!("Shutting down gracefully");
    };
    
    // Wait for shutdown signal
    shutdown_signal.await;
}

mod db;
mod repositories;
mod api;
mod cache;
mod tasks;
mod monitoring;
mod search;

use std::sync::Arc;
use tauri::{
    AppHandle, Manager, State, Window,
    async_runtime::{self, JoinHandle}, 
};
use serde::{Serialize, Deserialize};
use log::{info, error};
use crate::search::manager::SEARCH_MANAGER;

// Define application state
struct AppState {
    db_pool: Arc<sqlx::SqlitePool>,
    search_handle: Arc<tokio::sync::Mutex<Option<JoinHandle<()>>>>,
    jwt_secret: Vec<u8>,
}

// Commands for Tauri app
#[tauri::command]
async fn start_search(app_handle: AppHandle, state: State<'_, AppState>) -> Result<bool, String> {
    info!("Starting search service...");
    
    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    
    // Initialize search in background
    let pool = state.db_pool.clone();
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    // Create a background task that won't block the UI
    let handle = async_runtime::spawn(async move {
        let result = SEARCH_MANAGER.initialize_if_needed(&app_data_dir, pool).await;
        let _ = tx.send(result);
    });
    
    // Store handle
    let mut search_handle = state.search_handle.lock().await;
    *search_handle = Some(handle);
    
    // Wait for result
    match rx.await {
        Ok(result) => Ok(result),
        Err(_) => Err("Failed to initialize search".to_string()),
    }
}

#[tauri::command]
async fn get_search_status() -> Result<search::manager::SearchStatus, String> {
    Ok(SEARCH_MANAGER.get_status().await)
}

#[tauri::command]
async fn sync_search_data(window: Window) -> Result<usize, String> {
    let rx = search::initialization::sync_search_data_in_background().await;
    
    // Wait for result but don't block UI
    tokio::spawn(async move {
        match rx.await {
            Ok(Ok(count)) => {
                let _ = window.emit("search:sync-complete", count);
            }
            Ok(Err(e)) | Err(e) => {
                let _ = window.emit("search:sync-error", e.to_string());
            }
        }
    });
    
    Ok(0) // Return immediately
}

#[derive(Serialize, Deserialize)]
struct SearchSettings {
    enabled: bool,
    memory_limit_mb: usize,
}

#[tauri::command]
async fn get_search_settings(app_handle: AppHandle) -> Result<SearchSettings, String> {
    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let settings_path = app_data_dir.join("search_settings.json");
    
    // Read settings file if it exists
    if settings_path.exists() {
        let contents = tokio::fs::read_to_string(&settings_path)
            .await
            .map_err(|e| e.to_string())?;
            
        serde_json::from_str::<SearchSettings>(&contents)
            .map_err(|e| e.to_string())
    } else {
        // Default settings
        Ok(SearchSettings {
            enabled: true,
            memory_limit_mb: 128,
        })
    }
}

#[tauri::command]
async fn set_search_settings(
    app_handle: AppHandle,
    settings: SearchSettings,
) -> Result<(), String> {
    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let settings_path = app_data_dir.join("search_settings.json");
    
    // Create parent directory if needed
    if let Some(parent) = settings_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    // Write settings to file
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| e.to_string())?;
        
    tokio::fs::write(&settings_path, json)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    // Initialize logger
    env_logger::init();
    
    tauri::Builder::default()
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app.path_resolver().app_data_dir().unwrap();
            let db_path = app_data_dir.join("educonnect.db").to_string_lossy().to_string();
            
            // Initialize database
            let rt = async_runtime::handle();
            let pool = rt.block_on(async {
                match db::setup_database(&db_path).await {
                    Ok(p) => Arc::new(p),
                    Err(e) => {
                        error!("Failed to initialize database: {:?}", e);
                        panic!("Database initialization failed");
                    }
                }
            });
            
            // Initialize JWT secret
            let jwt_secret = generate_or_load_jwt_secret(app_data_dir);
            
            // Register app state
            app.manage(AppState {
                db_pool: pool,
                search_handle: Arc::new(tokio::sync::Mutex::new(None)),
                jwt_secret,
            });
            
            // Setup window close handler
            let app_handle = app.handle();
            app.listen_global("tauri://close-requested", move |_| {
                let app = app_handle.clone();
                // Clean shutdown in background to avoid blocking UI
                async_runtime::spawn(async move {
                    info!("Application shutdown initiated, stopping search service...");
                    
                    // Stop search service
                    if let Err(e) = SEARCH_MANAGER.shutdown().await {
                        error!("Failed to stop search service: {}", e);
                    }
                    
                    async_runtime::sleep(std::time::Duration::from_millis(500)).await;
                    app.exit(0);
                });
            });
            
            Ok(())
        })
        .manage(Arc::new(SqliteCourseRepository::new(pool.clone())) as Arc<dyn CourseRepository + Send + Sync>)
        .manage(Arc::new(SqliteAssignmentRepository::new(pool.clone())) as Arc<dyn AssignmentRepository + Send + Sync>)
        .manage(Arc::new(SqliteUserRepository::new(pool.clone())) as Arc<dyn UserRepository + Send + Sync>)
        .manage(Arc::new(SqliteIntegrationRepository::new(pool.clone())) as Arc<dyn IntegrationRepository + Send + Sync>)
        .manage(Arc::new(SqliteDiscussionRepository::new(pool.clone())) as Arc<dyn DiscussionRepository + Send + Sync>)
        .manage(Arc::new(SqliteNotificationRepository::new(pool.clone())) as Arc<dyn NotificationRepository + Send + Sync>)
        .manage(Arc::new(SqliteModuleRepository::new(pool.clone())) as Arc<dyn ModuleRepository + Send + Sync>)
        .invoke_handler(tauri::generate_handler![
            start_search,
            get_search_status,
            sync_search_data,
            get_search_settings,
            set_search_settings,
            api::courses::get_courses,
            api::courses::get_course,
            api::courses::create_course,
            api::courses::update_course,
            api::courses::delete_course,
            api::auth::login_user,
            api::auth::register_user,
            api::auth::get_current_user,
            api::integration::create_course_category_mapping,
            api::integration::get_course_category_mapping,
            api::integration::sync_course_category,
            api::assignments::get_assignments,
            api::assignments::get_assignment,
            api::assignments::create_assignment,
            api::assignments::update_assignment,
            api::assignments::delete_assignment,
            api::submissions::get_submissions,
            api::submissions::get_submission,
            api::submissions::create_submission,
            api::submissions::update_submission,
            api::discussions::get_discussions,
            api::discussions::get_discussion,
            api::discussions::create_discussion,
            api::discussions::update_discussion,
            api::discussions::sync_discussion,
            api::discussions::delete_discussion,
            api::users::get_user_profile,
            api::users::update_user_profile,
            api::users::get_user_preferences,
            api::users::update_user_preferences,
            api::users::get_user_integration_settings,
            api::users::update_user_integration_settings,
            api::notifications::get_notifications,
            api::notifications::get_unread_notification_count,
            api::notifications::create_notification,
            api::notifications::mark_notifications_as_read,
            api::notifications::mark_all_notifications_as_read,
            api::notifications::delete_notification,
            api::modules::get_modules,
            api::modules::get_module,
            api::modules::create_module,
            api::modules::update_module,
            api::modules::delete_module,
            api::modules::reorder_modules,
            api::modules::get_module_items,
            api::modules::get_module_item,
            api::modules::create_module_item,
            api::modules::update_module_item,
            api::modules::delete_module_item,
            api::modules::reorder_module_items,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}

// Generate or load JWT secret
fn generate_or_load_jwt_secret(app_data_dir: std::path::PathBuf) -> Vec<u8> {
    use std::fs;
    use rand::RngCore;
    
    let secret_path = app_data_dir.join("jwt_secret.key");
    
    if secret_path.exists() {
        // Load existing secret
        match fs::read(&secret_path) {
            Ok(secret) if !secret.is_empty() => {
                info!("Loaded existing JWT secret");
                return secret;
            },
            _ => {
                warn!("Invalid JWT secret file, generating new one");
            }
        }
    }
    
    // Generate new secret
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    
    // Ensure directory exists
    if let Some(parent) = secret_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    // Save secret
    if let Err(e) = fs::write(&secret_path, &secret) {
        error!("Failed to save JWT secret: {}", e);
    } else {
        info!("Generated and saved new JWT secret");
    }
    
    secret.to_vec()
}

mod api {
    pub mod auth;
    pub mod courses;
    pub mod assignments;
    pub mod submissions;
    pub mod users;
    pub mod discussions;
    pub mod integration;
    pub mod notifications;
    pub mod modules; // Add this line
}

// ...existing code...

fn main() {
    // ...existing code...
    
    let module_repo = Arc::new(db::module_repository::SqliteModuleRepository::new(pool.clone()));

    // ...existing code...
    
    tauri::Builder::default()
        // ...existing code...
        .manage(module_repo)
        // ...existing code...
        .invoke_handler(tauri::generate_handler![
            // ...existing commands...
            api::modules::get_modules,
            api::modules::get_module,
            api::modules::create_module,
            api::modules::update_module,
            api::modules::delete_module,
            api::modules::reorder_modules,
            api::modules::get_module_items,
            api::modules::get_module_item,
            api::modules::create_module_item,
            api::modules::update_module_item,
            api::modules::delete_module_item,
            api::modules::reorder_module_items,
        ])
        // ...existing code...
}

use db::DB;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Connect to database
    let db = DB::connect("sqlite:lms.db").await?;
    
    // Initialize database tables
    db.initialize_tables().await?;
    
    // Setup Canvas integration
    let canvas_service = services::integration::canvas_integration::CanvasIntegrationService::new(
        db.clone(),
        std::env::var("CANVAS_API_URL").unwrap_or_else(|_| "https://canvas.example.com".to_string()),
        std::env::var("CANVAS_API_TOKEN").unwrap_or_default(),
    );
    
    // Setup Discourse integration
    let discourse_service = services::integration::discourse_integration::DiscourseIntegrationService::new(
        db.clone(),
        std::env::var("DISCOURSE_API_URL").unwrap_or_else(|_| "https://discourse.example.com".to_string()),
        std::env::var("DISCOURSE_API_KEY").unwrap_or_default(),
        std::env::var("DISCOURSE_API_USERNAME").unwrap_or_else(|_| "system".to_string()),
    );
    
    // Setup sync service
    let sync_service = services::integration::sync_service::IntegrationSyncService::new(
        db.clone(),
        canvas_service,
        discourse_service,
    );
    
    // Add sync scheduler
    let sync_scheduler = SyncScheduler::new(db.clone(), canvas_service.clone());
    sync_scheduler.start().await.expect("Failed to start sync scheduler");
    
    // Start the web server (this would be implemented in a real app)
    println!("LMS server initialized!");
    
    Ok(())
}

// Inside your main function or Tauri setup
fn main() {
    // Initialize database
    let db = DB::connect("sqlite:lms.db").await.unwrap();
    db.initialize_tables().await.unwrap();
    
    // Setup Canvas integration
    let canvas_service = Arc::new(CanvasIntegrationService::new(
        db.clone(),
        std::env::var("CANVAS_API_URL").unwrap_or_else(|_| "https://canvas.example.com".to_string()),
        std::env::var("CANVAS_API_TOKEN").unwrap_or_default(),
    ));
    
    // Setup Discourse integration
    let discourse_service = Arc::new(DiscourseIntegrationService::new(
        db.clone(),
        std::env::var("DISCOURSE_API_URL").unwrap_or_else(|_| "https://discourse.example.com".to_string()),
        std::env::var("DISCOURSE_API_KEY").unwrap_or_default(),
        std::env::var("DISCOURSE_API_USERNAME").unwrap_or_else(|_| "system".to_string()),
    ));
    
    // Add sync scheduler
    let sync_scheduler = SyncScheduler::new(db.clone(), canvas_service.clone());
    sync_scheduler.start().await.expect("Failed to start sync scheduler");
    
    tauri::Builder::default()
        .manage(db)
        .manage(canvas_service)
        .manage(discourse_service)
        .manage(sync_scheduler)
        .invoke_handler(tauri::generate_handler![
            // Your existing commands
            
            // Integration commands
            sync_topic,
            sync_all_pending,
            mark_topic_for_sync,
            test_canvas_connectivity,
            test_discourse_connectivity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
