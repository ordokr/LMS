use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error, warn, debug};
use serde::{Serialize, Deserialize};
use tokio::time::Duration;
use std::io::Write;

use crate::metrics::ResourceMonitor;

#[derive(Debug, Serialize, Deserialize)]
pub struct MeiliSearchConfig {
    pub host: String,
    pub api_key: Option<String>,
}

// Meilisearch monitoring and control
struct MeiliProcess {
    child: Child,
    startup_time: std::time::Instant,
    max_memory_mb: u64,
    memory_monitor: Arc<ResourceMonitor>,
}

pub struct EmbeddedMeilisearch {
    binary_path: PathBuf,
    data_path: PathBuf,
    port: u16,
    master_key: Option<String>,
    process: Mutex<Option<MeiliProcess>>,
    config: MeiliSearchConfig,
    // Performance control settings
    max_memory_mb: u64,
    throttle_indexing: bool,
}

impl EmbeddedMeilisearch {
    pub fn new(
        binary_path: PathBuf, 
        data_path: PathBuf,
        port: u16,
        master_key: Option<String>,
        max_memory_mb: u64,
    ) -> Self {
        Self {
            binary_path,
            data_path,
            port,
            master_key: master_key.clone(),
            process: Mutex::new(None),
            config: MeiliSearchConfig {
                host: format!("http://127.0.0.1:{}", port),
                api_key: master_key,
            },
            max_memory_mb,
            throttle_indexing: true,
        }
    }
    
    /// Start the embedded Meilisearch server with resource constraints
    pub async fn start(&self) -> Result<MeiliSearchConfig, String> {
        // Create data directory if it doesn't exist
        if !self.data_path.exists() {
            std::fs::create_dir_all(&self.data_path)
                .map_err(|e| format!("Failed to create data directory: {}", e))?;
        }
        
        // Check if already running
        let mut process_guard = self.process.lock().await;
        if process_guard.is_some() {
            return Ok(self.config.clone());
        }
        
        // Prepare optimized command
        let mut cmd = Command::new(&self.binary_path);
        
        // Configure for low resource usage
        cmd.arg("--db-path").arg(&self.data_path)
           .arg("--http-addr").arg(format!("127.0.0.1:{}", self.port))
           .arg("--no-analytics")
           .arg("--env").arg("production"); // Use production mode for better performance
        
        // Set resource limits
        cmd.arg("--max-indexing-memory").arg(format!("{}", self.max_memory_mb / 2)); // Half for indexing
        cmd.arg("--max-indexing-threads").arg("1"); // Limit threads to avoid CPU contention
        
        if self.throttle_indexing {
            cmd.arg("--indexer-options").arg("max_concurrent_tasks=1");
        }
        
        if let Some(key) = &self.master_key {
            cmd.arg("--master-key").arg(key);
        }
        
        // Configure IO redirection
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        // Start the process
        info!("Starting embedded Meilisearch server on port {}", self.port);
        match cmd.spawn() {
            Ok(mut child) => {
                // Set up process monitoring
                let memory_monitor = Arc::new(ResourceMonitor::new(child.id().unwrap_or(0)));
                
                // Set up stdout/stderr handling
                if let Some(stdout) = child.stdout.take() {
                    let stdout_reader = std::io::BufReader::new(stdout);
                    tokio::spawn(Self::process_output(stdout_reader, "stdout"));
                }
                
                if let Some(stderr) = child.stderr.take() {
                    let stderr_reader = std::io::BufReader::new(stderr);
                    tokio::spawn(Self::process_output(stderr_reader, "stderr"));
                }
                
                let meili_process = MeiliProcess {
                    child,
                    startup_time: std::time::Instant::now(),
                    max_memory_mb: self.max_memory_mb,
                    memory_monitor: memory_monitor.clone(),
                };
                
                *process_guard = Some(meili_process);
                
                // Release the lock before waiting
                drop(process_guard);
                
                // Set up memory monitoring
                tokio::spawn({
                    let process_mutex = self.process.clone();
                    let memory_monitor = memory_monitor.clone();
                    
                    async move {
                        let mut interval = tokio::time::interval(Duration::from_secs(30));
                        
                        loop {
                            interval.tick().await;
                            
                            let mem_usage = memory_monitor.get_memory_usage_mb().await;
                            debug!("Meilisearch memory usage: {}MB", mem_usage);
                            
                            // Check for excessive memory usage
                            let mut process_guard = process_mutex.lock().await;
                            if let Some(meili_process) = &*process_guard {
                                if mem_usage > meili_process.max_memory_mb {
                                    warn!(
                                        "Meilisearch memory usage ({} MB) exceeds limit ({} MB), restarting",
                                        mem_usage, meili_process.max_memory_mb
                                    );
                                    
                                    // Don't restart if we've just started (within 2 minutes)
                                    if meili_process.startup_time.elapsed() > std::time::Duration::from_secs(120) {
                                        if let Err(e) = meili_process.child.kill() {
                                            error!("Failed to kill Meilisearch process: {}", e);
                                        }
                                        *process_guard = None;
                                        break;
                                    }
                                }
                            } else {
                                // Process no longer exists
                                break;
                            }
                        }
                    }
                });
                
                // Wait for server to be ready
                match self.wait_until_ready().await {
                    Ok(_) => {
                        info!("Meilisearch server started successfully");
                        Ok(self.config.clone())
                    },
                    Err(e) => {
                        // Clean up on startup failure
                        let mut process_guard = self.process.lock().await;
                        if let Some(mut meili_process) = process_guard.take() {
                            let _ = meili_process.child.kill();
                        }
                        
                        Err(format!("Meilisearch failed to start: {}", e))
                    }
                }
            },
            Err(e) => Err(format!("Failed to start Meilisearch: {}", e)),
        }
    }
    
    /// Stop the embedded Meilisearch server
    pub async fn stop(&self) -> Result<(), String> {
        let mut process_guard = self.process.lock().await;
        
        if let Some(mut meili_process) = process_guard.take() {
            info!("Stopping embedded Meilisearch server");
            match meili_process.child.kill() {
                Ok(_) => {
                    // Wait for process to exit
                    match meili_process.child.wait() {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("Error waiting for Meilisearch to exit: {}", e)),
                    }
                },
                Err(e) => Err(format!("Failed to stop Meilisearch: {}", e)),
            }
        } else {
            // Not running
            Ok(())
        }
    }
    
    /// Get the configuration for connecting to this server
    pub fn get_config(&self) -> MeiliSearchConfig {
        self.config.clone()
    }
    
    /// Wait until the server is ready with exponential backoff
    async fn wait_until_ready(&self) -> Result<(), String> {
        let host = format!("http://127.0.0.1:{}/health", self.port);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        // Use exponential backoff
        let mut delay_ms = 100;
        let max_delay_ms = 2000;
        let timeout_secs = 30;
        let start = std::time::Instant::now();
        
        while start.elapsed().as_secs() < timeout_secs {
            match client.get(&host).send().await {
                Ok(response) if response.status().is_success() => {
                    info!("Meilisearch server is ready");
                    return Ok(());
                },
                _ => {
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    delay_ms = (delay_ms * 2).min(max_delay_ms);
                }
            }
        }
        
        Err("Timed out waiting for Meilisearch to start".to_string())
    }
    
    /// Process output from Meilisearch
    async fn process_output(reader: impl std::io::BufRead + Send + 'static, source: &'static str) {
        use std::io::BufRead;
        
        let mut lines = reader.lines();
        
        while let Ok(Some(line)) = lines.next().transpose() {
            if line.contains("error") || line.contains("Error") {
                error!("Meilisearch {}: {}", source, line);
            } else if line.contains("warn") || line.contains("Warn") {
                warn!("Meilisearch {}: {}", source, line);
            } else {
                debug!("Meilisearch {}: {}", source, line);
            }
        }
    }
    
    /// Check if Meilisearch is currently running
    pub async fn is_running(&self) -> bool {
        let process_guard = self.process.lock().await;
        process_guard.is_some()
    }
    
    /// Explicitly limit memory usage
    pub async fn set_memory_limit(&self, max_memory_mb: u64) -> Result<(), String> {
        let mut process_guard = self.process.lock().await;
        
        if let Some(meili_process) = &mut *process_guard {
            meili_process.max_memory_mb = max_memory_mb;
            Ok(())
        } else {
            Err("Meilisearch not running".to_string())
        }
    }
}