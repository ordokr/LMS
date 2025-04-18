#[cfg(feature = "standalone-quiz")]
use ordo::quiz::standalone::StandaloneQuizApp;

#[cfg(feature = "standalone-quiz")]
#[tokio::main]
async fn main() -> Result<()> {
    let config = StandaloneConfig {
        storage_path: "./quiz-data".into(),
        enable_sync: false,
        offline_mode: true,
        encryption_key: None,
    };

    let app = StandaloneQuizApp::new(config)?;
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            quiz_commands::create_quiz,
            quiz_commands::add_question,
            quiz_commands::start_session,
            quiz_commands::submit_answer,
        ])
        .manage(app)
        .run(tauri::generate_context!())
        .expect("error running standalone quiz app");
        
    Ok(())
}