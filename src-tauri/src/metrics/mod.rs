use log::{debug, warn};
use std::process::Command;

pub struct ResourceMonitor {
    process_id: u32,
}

impl ResourceMonitor {
    pub fn new(process_id: u32) -> Self {
        Self { process_id }
    }
    
    pub async fn get_memory_usage_mb(&self) -> u64 {
        #[cfg(target_os = "windows")]
        {
            self.get_memory_usage_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            self.get_memory_usage_macos()
        }
        
        #[cfg(target_os = "linux")]
        {
            self.get_memory_usage_linux()
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            // Default fallback for unsupported platforms
            0
        }
    }
    
    #[cfg(target_os = "windows")]
    fn get_memory_usage_windows(&self) -> u64 {
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                &format!("Get-Process -Id {} | Select-Object -ExpandProperty WorkingSet64", self.process_id)
            ])
            .output();
            
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let bytes: u64 = stdout.trim().parse().unwrap_or(0);
                bytes / 1024 / 1024 // Convert to MB
            },
            _ => {
                debug!("Failed to get memory usage for process {}", self.process_id);
                0
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    fn get_memory_usage_macos(&self) -> u64 {
        let output = Command::new("ps")
            .args(&["-o", "rss=", "-p", &self.process_id.to_string()])
            .output();
            
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let kb: u64 = stdout.trim().parse().unwrap_or(0);
                kb / 1024 // Convert to MB
            },
            _ => {
                debug!("Failed to get memory usage for process {}", self.process_id);
                0
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    fn get_memory_usage_linux(&self) -> u64 {
        let output = Command::new("ps")
            .args(&["-o", "rss=", "-p", &self.process_id.to_string()])
            .output();
            
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let kb: u64 = stdout.trim().parse().unwrap_or(0);
                kb / 1024 // Convert to MB
            },
            _ => {
                debug!("Failed to get memory usage for process {}", self.process_id);
                0
            }
        }
    }
}