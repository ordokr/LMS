#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use tauri::Manager;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use std::path::{Path, PathBuf};
use std::fs;

use lms_lib::app_state::AppState;
use lms_lib::modules::quiz::services::QuizService;
use lms_lib::models::quiz::ActivityType;
use lms_lib::modules::quiz::analytics::{
    dashboard::AnalyticsDashboard,
    charts::ChartService,
    reports::ReportService
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting Ordo Quiz Analytics Dashboard...");

    // Initialize database connection
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ordo_quiz.db?mode=rwc".to_string());
    
    info!("Connecting to database: {}", db_url);
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // Initialize quiz database
    info!("Initializing quiz database...");
    match lms_lib::database::init_quiz_db::init_quiz_db(&db_pool).await {
        Ok(_) => info!("Quiz database initialized successfully"),
        Err(e) => error!("Failed to initialize quiz database: {}", e),
    }

    // Create data directory
    let data_dir = PathBuf::from("ordo_quiz_data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    
    // Create app state
    let app_state = AppState::new(db_pool.clone(), "ordo_quiz_secret_key".as_bytes().to_vec(), data_dir.clone())
        .with_quiz_repository();
    
    // Initialize quiz service
    let app_state = match app_state.with_quiz_service().await {
        Ok(state) => Arc::new(state),
        Err(e) => {
            error!("Failed to initialize quiz service: {}", e);
            return Err(e.into());
        }
    };

    // Initialize analytics services
    let dashboard = AnalyticsDashboard::new(db_pool.clone());
    let chart_service = ChartService::new(db_pool.clone());
    let report_service = ReportService::new(db_pool.clone());

    // Build Tauri application
    tauri::Builder::default()
        .setup(|app| {
            // Make services available to the app
            app.manage(app_state);
            app.manage(dashboard);
            app.manage(chart_service);
            app.manage(report_service);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Quiz commands
            lms_lib::modules::quiz::commands::get_quizzes,
            lms_lib::modules::quiz::commands::get_quiz,
            
            // Activity commands
            lms_lib::modules::quiz::commands::track_quiz_started,
            lms_lib::modules::quiz::commands::track_quiz_completed,
            lms_lib::modules::quiz::commands::track_quiz_abandoned,
            lms_lib::modules::quiz::commands::track_question_answered,
            lms_lib::modules::quiz::commands::track_flashcard_activity,
            lms_lib::modules::quiz::commands::get_user_activity_summary,
            lms_lib::modules::quiz::commands::get_quiz_activity_summary,
            lms_lib::modules::quiz::commands::get_activity_stats,
            
            // Analytics commands
            lms_lib::modules::quiz::commands::get_dashboard_data,
            lms_lib::modules::quiz::commands::get_activity_by_day_chart,
            lms_lib::modules::quiz::commands::get_activity_by_type_chart,
            lms_lib::modules::quiz::commands::get_quiz_completion_chart,
            lms_lib::modules::quiz::commands::get_time_distribution_chart,
            lms_lib::modules::quiz::commands::generate_user_activity_report,
            lms_lib::modules::quiz::commands::generate_quiz_performance_report,
            lms_lib::modules::quiz::commands::generate_question_analysis_report,
            lms_lib::modules::quiz::commands::generate_time_analysis_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
