use std::path::PathBuf;
use std::process::Command;
use anyhow::{Result, anyhow};
use tracing::{info, error};
use std::env;

/// Launch the Ordo Quiz app standalone
pub fn launch_quenti_standalone() -> Result<()> {
    info!("Launching Ordo Quiz standalone app");

    // Get the current executable's directory
    let current_exe = env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or_else(|| anyhow!("Failed to get executable directory"))?;

    // Determine the path to the Quenti standalone binary
    // First, check if it's in the same directory
    let mut quenti_path = exe_dir.join("quiz-standalone");

    // Add extension on Windows
    if cfg!(target_os = "windows") {
        quenti_path.set_extension("exe");
    }

    // If not found in the same directory, check in a quenti subdirectory
    if !quenti_path.exists() {
        quenti_path = exe_dir.join("quenti").join("quiz-standalone");
        if cfg!(target_os = "windows") {
            quenti_path.set_extension("exe");
        }
    }

    // If still not found, check in the parent directory
    if !quenti_path.exists() {
        if let Some(parent_dir) = exe_dir.parent() {
            quenti_path = parent_dir.join("quiz-standalone");
            if cfg!(target_os = "windows") {
                quenti_path.set_extension("exe");
            }
        }
    }

    // If still not found, check in the source directory (for development)
    if !quenti_path.exists() {
        quenti_path = PathBuf::from("target\\release\\quiz-standalone");
        if cfg!(target_os = "windows") {
            quenti_path.set_extension("exe");
        }
    }

    // Final fallback - try to find it in the PATH
    if !quenti_path.exists() {
        quenti_path = PathBuf::from("quiz-standalone");
        if cfg!(target_os = "windows") {
            quenti_path.set_extension("exe");
        }
    }

    info!("Attempting to launch Ordo Quiz from: {}", quenti_path.display());

    // Launch the Ordo Quiz app
    let status = Command::new(&quenti_path)
        .spawn()
        .map_err(|e| {
            error!("Failed to launch Ordo Quiz: {}", e);
            anyhow!("Failed to launch Ordo Quiz: {}", e)
        })?;

    info!("Ordo Quiz launched successfully: {:?}", status);

    Ok(())
}
