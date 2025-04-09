use std::path::{Path, PathBuf};
use reqwest::Client;
use tokio::fs::{File, create_dir_all};
use tokio::io::AsyncWriteExt;
use std::process::Command;
use log::{info, debug};
use crate::monitoring::system_info;

use super::embedded::EmbeddedMeilisearch;

pub async fn setup_meilisearch(app_data_dir: &Path) -> Result<EmbeddedMeilisearch, String> {
    let binary_path = get_meilisearch_path(app_data_dir).await?;
    
    // Setup data directory
    let data_dir = app_data_dir.join("meilisearch_data");
    if !data_dir.exists() {
        create_dir_all(&data_dir)
            .await
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }
    
    // Generate a random master key if one doesn't exist
    let key_path = app_data_dir.join("meilisearch_key");
    let master_key = if key_path.exists() {
        // Read existing key
        let key = tokio::fs::read_to_string(&key_path)
            .await
            .map_err(|e| format!("Failed to read master key: {}", e))?;
        Some(key)
    } else {
        // Generate new key
        let key = generate_random_key();
        tokio::fs::write(&key_path, &key)
            .await
            .map_err(|e| format!("Failed to save master key: {}", e))?;
        Some(key)
    };
    
    // Determine optimal memory settings based on system resources
    let system_memory = system_info::get_system_memory_mb();
    let available_cpus = system_info::get_available_cpus();
    
    // Calculate limits based on system resources:
    // - For memory: use 10% of system RAM, minimum 64MB, maximum 512MB
    // - For index size: use 5% of system RAM, minimum 32MB, maximum 256MB
    let max_memory_mb = if let Some(mem) = system_memory {
        std::cmp::max(64, std::cmp::min(512, mem / 10))
    } else {
        128 // Default if we can't determine system memory
    };
    
    let max_index_size_mb = if let Some(mem) = system_memory {
        std::cmp::max(32, std::cmp::min(256, mem / 20))
    } else {
        64 // Default if we can't determine system memory
    };
    
    debug!(
        "Configuring Meilisearch with {}MB RAM limit, {}MB index size limit (system has {}MB RAM, {} CPUs)",
        max_memory_mb,
        max_index_size_mb,
        system_memory.unwrap_or(0),
        available_cpus
    );
    
    // Choose port based on available ports
    let port = find_available_port(7700, 7800).unwrap_or(7701);
    
    // Create embedded Meilisearch instance with optimal settings
    let meilisearch = EmbeddedMeilisearch::new(
        binary_path, 
        data_dir,
        port,
        master_key,
    ).with_memory_limits(max_memory_mb, max_index_size_mb);
    
    Ok(meilisearch)
}

/// Get the path to the Meilisearch binary, downloading it if necessary
async fn get_meilisearch_path(app_data_dir: &Path) -> Result<PathBuf, String> {
    let os_info = os_info();
    let version = "v1.1.1"; // Specify the version you want to use
    
    let binary_name = match os_info {
        ("windows", _) => "meilisearch.exe",
        _ => "meilisearch",
    };
    
    let binary_path = app_data_dir.join("bin").join(binary_name);
    
    // Check if binary exists and don't download again
    if binary_path.exists() {
        return Ok(binary_path);
    }
    
    // Create bin directory
    create_dir_all(app_data_dir.join("bin"))
        .await
        .map_err(|e| format!("Failed to create bin directory: {}", e))?;
    
    // Download Meilisearch
    info!("Downloading Meilisearch {}...", version);
    let download_url = get_download_url(version, os_info)?;
    
    // Download with progress tracking
    download_with_progress(&download_url, &binary_path).await?;
    
    // Make executable on Unix systems
    if os_info.0 != "windows" {
        let status = Command::new("chmod")
            .arg("+x")
            .arg(&binary_path)
            .status()
            .map_err(|e| format!("Failed to make binary executable: {}", e))?;
            
        if !status.success() {
            return Err("Failed to make binary executable".into());
        }
    }
    
    Ok(binary_path)
}

// Function to download with progress tracking
async fn download_with_progress(url: &str, path: &Path) -> Result<(), String> {
    // Create parent directories
    if let Some(parent) = path.parent() {
        create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }
    
    // Prepare client with optimized settings
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("EduConnect-LMS")
        .gzip(true)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Start download
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {}", e))?;
        
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }
    
    // Get total size if available
    let total_size = response
        .content_length()
        .unwrap_or(0);
    
    let mut file = File::create(path)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;
        
    let mut downloaded = 0;
    let mut stream = response.bytes_stream();
    
    // Process chunks
    use futures_util::StreamExt;
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| format!("Error while downloading file: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Error while writing to file: {}", e))?;
            
        downloaded += chunk.len() as u64;
        
        // Log progress every 10%
        if total_size > 0 {
            let percent = (downloaded as f64 / total_size as f64 * 100.0) as u64;
            if percent % 10 == 0 {
                debug!("Download progress: {}% ({}/{} bytes)", percent, downloaded, total_size);
            }
        }
    }
    
    debug!("Download complete: {} bytes", downloaded);
    
    Ok(())
}

fn get_download_url(version: &str, (os, arch): (&str, &str)) -> Result<String, String> {
    match (os, arch) {
        ("windows", _) => Ok(format!(
            "https://github.com/meilisearch/meilisearch/releases/download/{}/meilisearch-windows-amd64.exe",
            version
        )),
        ("macos", "x86_64") => Ok(format!(
            "https://github.com/meilisearch/meilisearch/releases/download/{}/meilisearch-macos-amd64",
            version
        )),
        ("macos", "aarch64") => Ok(format!(
            "https://github.com/meilisearch/meilisearch/releases/download/{}/meilisearch-macos-arm64",
            version
        )),
        ("linux", "x86_64") => Ok(format!(
            "https://github.com/meilisearch/meilisearch/releases/download/{}/meilisearch-linux-amd64",
            version
        )),
        ("linux", "aarch64") => Ok(format!(
            "https://github.com/meilisearch/meilisearch/releases/download/{}/meilisearch-linux-arm64",
            version
        )),
        _ => Err(format!("Unsupported platform: {}/{}", os, arch)),
    }
}

async fn download_file(url: &str, path: &Path) -> Result<(), String> {
    // Create parent directories
    if let Some(parent) = path.parent() {
        create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }
    
    // Download file
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {}", e))?;
        
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to get response bytes: {}", e))?;
        
    // Write to file
    let mut file = File::create(path)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;
        
    file.write_all(&bytes)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;
        
    Ok(())
}

fn os_info() -> (&'static str, &'static str) {
    #[cfg(target_os = "windows")]
    return ("windows", "x86_64");
    
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return ("macos", "x86_64");
    
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return ("macos", "aarch64");
    
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return ("linux", "x86_64");
    
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return ("linux", "aarch64");
    
    ("unknown", "unknown")
}

fn generate_random_key() -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

// Find an available port in range
fn find_available_port(start: u16, end: u16) -> Option<u16> {
    for port in start..=end {
        if port_is_available(port) {
            return Some(port);
        }
    }
    None
}

// Check if port is available
fn port_is_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}