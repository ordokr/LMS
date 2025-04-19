use std::path::PathBuf;
use std::process::Command;
use anyhow::{Result, anyhow};
use tracing::{info, error};
use std::env;

/// Launch the Ordo Quiz app standalone
pub fn launch_ordo_quiz_standalone() -> Result<()> {
    info!("Launching Ordo Quiz standalone app");
    
    // Get the current executable's directory
    let current_exe = env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or_else(|| anyhow!("Failed to get executable directory"))?;
    
    // Determine the path to the Ordo Quiz standalone binary
    // First, check if it's in the same directory
    let mut quiz_path = exe_dir.join("quiz-standalone");
    
    // Add extension on Windows
    if cfg!(target_os = "windows") {
        quiz_path.set_extension("exe");
    }
    
    // If not found in the same directory, check in a quiz subdirectory
    if !quiz_path.exists() {
        quiz_path = exe_dir.join("quiz").join("quiz-standalone");
        if cfg!(target_os = "windows") {
            quiz_path.set_extension("exe");
        }
    }
    
    // If still not found, check in the parent directory
    if !quiz_path.exists() {
        if let Some(parent_dir) = exe_dir.parent() {
            quiz_path = parent_dir.join("quiz-standalone");
            if cfg!(target_os = "windows") {
                quiz_path.set_extension("exe");
            }
        }
    }
    
    // If still not found, check in the source directory (for development)
    if !quiz_path.exists() {
        quiz_path = PathBuf::from("target\\release\\quiz-standalone");
        if cfg!(target_os = "windows") {
            quiz_path.set_extension("exe");
        }
    }
    
    // Final fallback - try to find it in the PATH
    if !quiz_path.exists() {
        quiz_path = PathBuf::from("quiz-standalone");
        if cfg!(target_os = "windows") {
            quiz_path.set_extension("exe");
        }
    }
    
    info!("Attempting to launch Ordo Quiz from: {}", quiz_path.display());
    
    // Launch the Ordo Quiz app
    let status = Command::new(&quiz_path)
        .spawn()
        .map_err(|e| {
            error!("Failed to launch Ordo Quiz: {}", e);
            anyhow!("Failed to launch Ordo Quiz: {}", e)
        })?;
    
    info!("Ordo Quiz launched successfully: {:?}", status);
    
    Ok(())
}
