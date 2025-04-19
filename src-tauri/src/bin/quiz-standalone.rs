use std::sync::Arc;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::io::Write;
use tauri::{Manager, State};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;
use crate::protocol_handler::handle_tauri_protocol;

// App state for the Ordo Quiz standalone app
#[derive(Debug)]
pub struct AppState {
    pub db_pool: SqlitePool,
    pub app_data_dir: PathBuf,
    pub is_online: bool,
    pub sync_pending_count: usize,
}

// Configuration for the Ordo Quiz standalone app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizConfig {
    pub storage_path: String,
    pub enable_sync: bool,
    pub offline_mode: bool,
    pub sync_url: Option<String>,
    pub encryption_key: Option<String>,
}

impl Default for QuizConfig {
    fn default() -> Self {
        Self {
            storage_path: "./ordo_quiz_data".to_string(),
            enable_sync: true,
            offline_mode: false,
            sync_url: Some("http://localhost:3000/api/quiz/sync".to_string()),
            encryption_key: None,
        }
    }
}

// Commands for the Ordo Quiz standalone app
#[tauri::command]
async fn get_sync_status(state: State<'_, AppState>) -> Result<(bool, usize), String> {
    Ok((state.is_online, state.sync_pending_count))
}

#[tauri::command]
async fn toggle_offline_mode(state: State<'_, AppState>) -> Result<bool, String> {
    // Toggle offline mode and update app state
    let mut is_online = state.is_online;
    is_online = !is_online;

    // Update config file
    if let Ok(mut config) = load_config().await {
        config.offline_mode = !is_online;
        if let Err(e) = save_config(&config).await {
            return Err(format!("Failed to save config: {}", e));
        }
    }

    Ok(is_online)
}

#[tauri::command]
async fn sync_now(state: State<'_, AppState>) -> Result<String, String> {
    if !state.is_online {
        return Err("Cannot sync while offline".to_string());
    }

    // Get sync directory
    let sync_dir = state.app_data_dir.join("sync");
    if !sync_dir.exists() {
        return Ok("No pending items to sync".to_string());
    }

    // Count pending items
    let count = match count_pending_sync_items(&sync_dir) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to count pending items: {}", e)),
    };

    if count == 0 {
        return Ok("No pending items to sync".to_string());
    }

    // In a real implementation, this would process each pending item
    // and send it to the server

    Ok(format!("Syncing {} items...", count))
}

#[tauri::command]
async fn get_quizzes(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    // Query the database for quizzes
    let result = sqlx::query("SELECT * FROM quizzes LIMIT 100")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| format!("Failed to fetch quizzes: {}", e))?;

    // Convert to JSON
    let quizzes: Vec<serde_json::Value> = result
        .iter()
        .map(|row| {
            let id: String = row.get("id");
            let title: String = row.get("title");
            let description: Option<String> = row.get("description");
            let time_limit: i32 = row.get("time_limit");
            let passing_score: i32 = row.get("passing_score");

            serde_json::json!({
                "id": id,
                "title": title,
                "description": description.unwrap_or_default(),
                "timeLimit": time_limit,
                "passingScore": passing_score
            })
        })
        .collect();

    Ok(quizzes)
}

#[tauri::command]
async fn create_quiz(state: State<'_, AppState>, quiz_data: serde_json::Value) -> Result<serde_json::Value, String> {
    // Extract quiz data
    let title = quiz_data["title"].as_str().ok_or("Missing title")?.to_string();
    let description = quiz_data["description"].as_str().unwrap_or("").to_string();
    let time_limit = quiz_data["timeLimit"].as_i64().unwrap_or(600) as i32;
    let passing_score = quiz_data["passingScore"].as_i64().unwrap_or(70) as i32;
    let shuffle_questions = quiz_data["shuffleQuestions"].as_bool().unwrap_or(false);

    // Generate a new UUID
    let id = uuid::Uuid::new_v4().to_string();

    // Insert into database
    sqlx::query(
        "INSERT INTO quizzes (id, title, description, time_limit, passing_score, shuffle_questions, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
    )
    .bind(&id)
    .bind(&title)
    .bind(&description)
    .bind(time_limit)
    .bind(passing_score)
    .bind(shuffle_questions)
    .execute(&state.db_pool)
    .await
    .map_err(|e| format!("Failed to create quiz: {}", e))?;

    // If offline, create a sync item
    if !state.is_online {
        let sync_dir = state.app_data_dir.join("sync");
        if !sync_dir.exists() {
            std::fs::create_dir_all(&sync_dir).map_err(|e| format!("Failed to create sync directory: {}", e))?;
        }

        let sync_item = serde_json::json!({
            "type": "create_quiz",
            "data": {
                "id": id,
                "title": title,
                "description": description,
                "time_limit": time_limit,
                "passing_score": passing_score,
                "shuffle_questions": shuffle_questions
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let sync_file = sync_dir.join(format!("{}_create_quiz.json", id));
        std::fs::write(
            &sync_file,
            serde_json::to_string_pretty(&sync_item).map_err(|e| format!("Failed to serialize sync item: {}", e))?
        ).map_err(|e| format!("Failed to write sync file: {}", e))?;
    }

    // Return the created quiz
    Ok(serde_json::json!({
        "id": id,
        "title": title,
        "description": description,
        "timeLimit": time_limit,
        "passingScore": passing_score,
        "shuffleQuestions": shuffle_questions
    }))
}

#[tauri::command]
async fn get_quiz_by_id(state: State<'_, AppState>, quiz_id: String) -> Result<serde_json::Value, String> {
    // Query the database for the quiz
    let result = sqlx::query("SELECT * FROM quizzes WHERE id = ?")
        .bind(&quiz_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| format!("Failed to fetch quiz: {}", e))?;

    // Check if quiz exists
    let row = result.ok_or(format!("Quiz not found: {}", quiz_id))?;

    // Convert to JSON
    let id: String = row.get("id");
    let title: String = row.get("title");
    let description: Option<String> = row.get("description");
    let time_limit: i32 = row.get("time_limit");
    let passing_score: i32 = row.get("passing_score");
    let shuffle_questions: bool = row.get("shuffle_questions");

    // Get questions for this quiz
    let questions = get_quiz_questions(state, quiz_id.clone()).await?;

    Ok(serde_json::json!({
        "id": id,
        "title": title,
        "description": description.unwrap_or_default(),
        "timeLimit": time_limit,
        "passingScore": passing_score,
        "shuffleQuestions": shuffle_questions,
        "questions": questions
    }))
}

#[tauri::command]
async fn get_quiz_questions(state: State<'_, AppState>, quiz_id: String) -> Result<Vec<serde_json::Value>, String> {
    // Query the database for questions
    let result = sqlx::query("SELECT * FROM questions WHERE quiz_id = ? ORDER BY position ASC")
        .bind(&quiz_id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| format!("Failed to fetch questions: {}", e))?;

    // Convert to JSON
    let mut questions = Vec::new();

    for row in result {
        let question_id: String = row.get("id");
        let question_text: String = row.get("question_text");
        let question_type: String = row.get("question_type");
        let points: i32 = row.get("points");

        // Get answers for this question
        let answers = get_question_answers(state.clone(), question_id.clone()).await?;

        questions.push(serde_json::json!({
            "id": question_id,
            "quizId": quiz_id,
            "questionText": question_text,
            "questionType": question_type,
            "points": points,
            "answers": answers
        }));
    }

    Ok(questions)
}

#[tauri::command]
async fn get_question_answers(state: State<'_, AppState>, question_id: String) -> Result<Vec<serde_json::Value>, String> {
    // Query the database for answers
    let result = sqlx::query("SELECT * FROM answers WHERE question_id = ? ORDER BY position ASC")
        .bind(&question_id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| format!("Failed to fetch answers: {}", e))?;

    // Convert to JSON
    let answers: Vec<serde_json::Value> = result
        .iter()
        .map(|row| {
            let id: String = row.get("id");
            let option_text: String = row.get("option_text");
            let is_correct: bool = row.get("is_correct");

            serde_json::json!({
                "id": id,
                "questionId": question_id,
                "optionText": option_text,
                "isCorrect": is_correct
            })
        })
        .collect();

    Ok(answers)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    println!("Starting Ordo Quiz Module...");

    // Load or create configuration
    let config = load_or_create_config().await?;

    // Create data directory
    let data_dir = PathBuf::from(&config.storage_path);
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    // Create migrations directory
    let migrations_dir = data_dir.join("migrations");
    if !migrations_dir.exists() {
        fs::create_dir_all(&migrations_dir)?;
    }

    // Create migration file if it doesn't exist
    let migration_path = migrations_dir.join("20240421_ordo_quiz_schema.sql");
    if !migration_path.exists() {
        let migration_sql = include_str!("../../migrations/20240421_ordo_quiz_schema.sql");
        fs::write(&migration_path, migration_sql)?;
        println!("Created migration file: {:?}", migration_path);
    }

    // Initialize database connection
    let db_path = data_dir.join("ordo_quiz.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());
    println!("Connecting to database: {}", db_url);
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Apply migration
    let migration_sql = fs::read_to_string(&migration_path)?;
    sqlx::query(&migration_sql).execute(&db_pool).await?;
    println!("Applied migration to database");

    // Create test data
    let test_data_sql = include_str!("../../migrations/test_data.sql");
    sqlx::query(test_data_sql).execute(&db_pool).await?;
    println!("Inserted test data into database");

    // Create sync directory for offline operations
    let sync_dir = data_dir.join("sync");
    if !sync_dir.exists() {
        fs::create_dir_all(&sync_dir)?;
    }

    // Initialize app state
    let app_state = AppState {
        db_pool,
        app_data_dir: data_dir,
        is_online: !config.offline_mode,
        sync_pending_count: count_pending_sync_items(&sync_dir)?,
    };

    // Launch the Tauri application
    println!("Launching Ordo Quiz UI...");
    tauri::Builder::default()
        .manage(app_state)
        .register_uri_scheme_protocol("tauri", handle_tauri_protocol)
        .invoke_handler(tauri::generate_handler![
            get_sync_status,
            toggle_offline_mode,
            sync_now,
            get_quizzes,
            create_quiz,
            get_quiz_by_id,
            get_quiz_questions,
            get_question_answers,
            crate::quiz::taking::start_quiz,
            crate::quiz::taking::get_current_question,
            crate::quiz::taking::submit_answer,
            crate::quiz::taking::navigate_to_question,
            crate::quiz::taking::complete_quiz,
            crate::quiz::taking::get_quiz_results,
            crate::quiz::ui_controller::initialize_ui,
            crate::quiz::ui_controller::create_quiz_ui,
            crate::quiz::ui_controller::start_quiz_ui,
            crate::quiz::ui_controller::toggle_offline_mode_ui,
            crate::quiz::ui_controller::sync_now_ui,
            crate::quiz::ui_controller::update_ui,
            crate::quiz::ui_controller::handle_button_click,
            crate::quiz::ui_controller::handle_form_submit,
            crate::quiz::taking_controller::start_quiz_taking,
            crate::quiz::taking_controller::select_quiz_option,
            crate::quiz::taking_controller::navigate_quiz_question,
            crate::quiz::taking_controller::complete_quiz_taking
        ])
        .setup(|app| {
            // Set up network status monitoring
            let app_handle = app.handle();
            let state = app.state::<AppState>();

            // Initialize the UI controller
            if let Some(window) = app.get_window("main") {
                // Initialize the UI
                tauri::async_runtime::block_on(async {
                    let _ = crate::quiz::ui_controller::initialize_ui(app_handle.clone(), window.clone(), state.clone()).await;
                });

                // Open devtools in debug mode
                #[cfg(debug_assertions)]
                {
                    window.open_devtools();
                }
            }

            // Start network monitoring in a background thread
            let ui_controller = state.get_ui_controller();
            std::thread::spawn(move || {
                let mut last_status = true; // Assume online initially

                loop {
                    // Simulate network check every 5 seconds
                    std::thread::sleep(std::time::Duration::from_secs(5));

                    // This is just a placeholder - in a real app, you would actually check connectivity
                    let is_online = true; // Simulate always online for now

                    // If status changed, update the UI controller
                    if is_online != last_status {
                        last_status = is_online;

                        // Update the UI controller
                        if let Ok(controller) = ui_controller.lock() {
                            controller.update_state(|state| {
                                state.is_online = is_online;
                            });
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running Ordo Quiz app");

    Ok(())
}

// Load or create configuration file
async fn load_or_create_config() -> Result<QuizConfig, Box<dyn std::error::Error>> {
    let config_dir = if let Some(app_dir) = tauri::api::path::app_dir() {
        app_dir
    } else {
        PathBuf::from("./ordo_quiz_data")
    };

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.json");

    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path)?;
        let config: QuizConfig = serde_json::from_str(&config_str)?;
        Ok(config)
    } else {
        let config = QuizConfig::default();
        let config_str = serde_json::to_string_pretty(&config)?;
        let mut file = fs::File::create(&config_path)?;
        file.write_all(config_str.as_bytes())?;
        Ok(config)
    }
}

// Load configuration file
async fn load_config() -> Result<QuizConfig, Box<dyn std::error::Error>> {
    let config_dir = if let Some(app_dir) = tauri::api::path::app_dir() {
        app_dir
    } else {
        PathBuf::from("./ordo_quiz_data")
    };

    let config_path = config_dir.join("config.json");

    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path)?;
        let config: QuizConfig = serde_json::from_str(&config_str)?;
        Ok(config)
    } else {
        Err("Config file not found".into())
    }
}

// Save configuration file
async fn save_config(config: &QuizConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = if let Some(app_dir) = tauri::api::path::app_dir() {
        app_dir
    } else {
        PathBuf::from("./ordo_quiz_data")
    };

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.json");
    let config_str = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(&config_path)?;
    file.write_all(config_str.as_bytes())?;
    Ok(())
}

// Count pending sync items
fn count_pending_sync_items(sync_dir: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    if !sync_dir.exists() {
        return Ok(0);
    }

    let count = fs::read_dir(sync_dir)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "json")
        })
        .count();

    Ok(count)
}

// Monitor network status
fn monitor_network_status(app_handle: tauri::AppHandle) {
    // In a real implementation, this would periodically check network connectivity
    // and update the app state accordingly
    loop {
        // Simulate network check every 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));

        // This is just a placeholder - in a real app, you would actually check connectivity
        let is_online = true; // Simulate always online for now

        // Update app state
        if let Some(state) = app_handle.try_state::<AppState>() {
            // In a real implementation, you would update the state here
            // This is just a placeholder to show the concept
        }
    }
}