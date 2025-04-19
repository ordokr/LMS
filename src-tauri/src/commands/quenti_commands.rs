use crate::launchers::launch_quenti_standalone;

/// Launch the Quenti app standalone
#[tauri::command]
pub fn launch_quenti_app() -> Result<String, String> {
    match launch_quenti_standalone() {
        Ok(_) => Ok("Quenti app launched successfully".to_string()),
        Err(e) => Err(format!("Failed to launch Quenti app: {}", e)),
    }
}
