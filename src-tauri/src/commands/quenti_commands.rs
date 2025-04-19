use crate::launchers::launch_quenti_standalone;

/// Launch the Ordo Quiz app standalone
#[tauri::command]
pub fn launch_quenti_app() -> Result<String, String> {
    match launch_quenti_standalone() {
        Ok(_) => Ok("Ordo Quiz app launched successfully".to_string()),
        Err(e) => Err(format!("Failed to launch Ordo Quiz app: {}", e)),
    }
}
