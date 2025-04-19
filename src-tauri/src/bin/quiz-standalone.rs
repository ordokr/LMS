use std::sync::Arc;
use sqlx::SqlitePool;
use anyhow::Result;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use std::path::{Path, PathBuf};
use std::fs;
use lms_lib::app_state::AppState;
use lms_lib::modules::quiz::services::QuizService;
use lms_lib::modules::quiz::commands as quiz_commands;
use lms_lib::models::quiz::ActivityType;

struct StandaloneConfig {
    storage_path: PathBuf,
    enable_sync: bool,
    offline_mode: bool,
    encryption_key: Option<String>,
}

struct StandaloneQuizApp {
    quiz_service: Arc<QuizService>,
    config: StandaloneConfig,
}

impl StandaloneQuizApp {
    async fn new(config: StandaloneConfig) -> Result<Self> {
        // Initialize database connection
        let db_url = format!("sqlite:{}/ordo_quiz.db?mode=rwc", config.storage_path.display());

        info!("Connecting to database: {}", db_url);
        let db_pool = SqlitePool::connect(&db_url).await?;

        // Initialize quiz database
        info!("Initializing quiz database...");
        match lms_lib::database::init_quiz_db::init_quiz_db(&db_pool).await {
            Ok(_) => info!("Quiz database initialized successfully"),
            Err(e) => error!("Failed to initialize quiz database: {}", e),
        }

        // Create test data if needed
        match lms_lib::database::init_quiz_db::check_quiz_tables(&db_pool).await {
            Ok(true) => {
                info!("Quiz tables exist, creating test data if needed...");
                if let Err(e) = lms_lib::database::init_quiz_db::create_test_data(&db_pool).await {
                    error!("Failed to create test data: {}", e);
                }
            },
            Ok(false) => error!("Quiz tables do not exist"),
            Err(e) => error!("Failed to check quiz tables: {}", e),
        }

        // Create app state
        let app_state = AppState::new(db_pool, "ordo_quiz_secret_key".as_bytes().to_vec(), config.storage_path.clone())
            .with_quiz_repository();

        // Initialize quiz service
        let app_state = match app_state.with_quiz_service().await {
            Ok(state) => state,
            Err(e) => {
                error!("Failed to initialize quiz service: {}", e);
                return Err(e);
            }
        };

        let quiz_service = app_state.get_quiz_service()?;

        // Check for sync file from main app
        let main_app_sync_path = config.storage_path.join("main_app_sync.json");
        if main_app_sync_path.exists() {
            info!("Found sync file from main app, syncing...");
            match quiz_service.sync_with_main_app(&main_app_sync_path).await {
                Ok(_) => info!("Synced with main app successfully"),
                Err(e) => error!("Failed to sync with main app: {}", e),
            }
        }

        // Track application start activity
        match quiz_service.track_activity("system", None, ActivityType::StudySessionStarted, None, None).await {
            Ok(_) => info!("Tracked study session start"),
            Err(e) => error!("Failed to track study session start: {}", e),
        }

        Ok(Self {
            quiz_service: Arc::new(quiz_service),
            config,
        })
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Ordo Quiz...");

        // Track application end activity
        match self.quiz_service.track_activity("system", None, ActivityType::StudySessionEnded, None, None).await {
            Ok(_) => info!("Tracked study session end"),
            Err(e) => error!("Failed to track study session end: {}", e),
        }

        // Export sync data before shutdown
        let sync_dir = self.config.storage_path.join("sync");
        if !sync_dir.exists() {
            fs::create_dir_all(&sync_dir)?;
        }

        let export_path = sync_dir.join("standalone_sync.json");
        match self.quiz_service.shutdown().await {
            Ok(_) => info!("Quiz service shut down successfully"),
            Err(e) => error!("Failed to shut down quiz service: {}", e),
        }

        info!("Exported sync data to {}", export_path.display());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting Ordo Quiz...");

    // Create data directory
    let storage_path = PathBuf::from("./quiz-data");
    if !storage_path.exists() {
        fs::create_dir_all(&storage_path)?;
    }

    // Create sync directory
    let sync_dir = storage_path.join("sync");
    if !sync_dir.exists() {
        fs::create_dir_all(&sync_dir)?;
    }

    let config = StandaloneConfig {
        storage_path,
        enable_sync: true,
        offline_mode: false,
        encryption_key: None,
    };

    let app = StandaloneQuizApp::new(config).await?;

    // Register a shutdown handler
    let app_clone = app.clone();
    let shutdown_handler = move || {
        let app = app_clone.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            if let Err(e) = app.shutdown().await {
                error!("Error during shutdown: {}", e);
            }
        });
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            quiz_commands::get_quizzes,
            quiz_commands::get_quiz,
            quiz_commands::create_quiz,
            quiz_commands::update_quiz,
            quiz_commands::delete_quiz,
            quiz_commands::get_questions,
            quiz_commands::get_question_with_answers,
            quiz_commands::create_question,
            quiz_commands::update_question,
            quiz_commands::delete_question,
            quiz_commands::start_quiz_attempt,
            quiz_commands::complete_quiz_attempt,
            quiz_commands::abandon_quiz_attempt,
            quiz_commands::get_quiz_settings,
            quiz_commands::create_quiz_settings,
            quiz_commands::update_quiz_settings,
            quiz_commands::launch_quiz_module,
        ])
        .manage(app)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                // Prevent the window from closing immediately
                api.prevent_close();

                // Get the window
                let window = event.window().clone();

                // Run the shutdown handler
                shutdown_handler();

                // Close the window
                window.close().unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error running standalone quiz app");

    Ok(())
}