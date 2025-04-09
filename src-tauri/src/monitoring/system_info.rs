use log::warn;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

// Cache system info to avoid repeated expensive calls
static SYSTEM_MEMORY_MB: Lazy<AtomicUsize> = Lazy::new(|| {
    let mem = detect_system_memory().unwrap_or(0);
    AtomicUsize::new(mem)
});

static AVAILABLE_CPUS: Lazy<AtomicUsize> = Lazy::new(|| {
    let cpus = num_cpus::get();
    AtomicUsize::new(cpus)
});

// Get system memory in MB, from cache
pub fn get_system_memory_mb() -> Option<usize> {
    let mem = SYSTEM_MEMORY_MB.load(Ordering::Relaxed);
    if mem == 0 {
        None
    } else {
        Some(mem)
    }
}

// Get available CPU cores, from cache
pub fn get_available_cpus() -> usize {
    AVAILABLE_CPUS.load(Ordering::Relaxed)
}

// Detect system memory in a cross-platform way
fn detect_system_memory() -> Option<usize> {
    // Try using sysinfo if available
    #[cfg(feature = "sysinfo")]
    {
        use sysinfo::{System, SystemExt};
        let mut system = System::new_all();
        system.refresh_all();
        return Some((system.total_memory() / 1024) as usize); // Convert KB to MB
    }
    
    // Platform-specific fallbacks
    #[cfg(target_os = "windows")]
    {
        use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        use std::mem::{size_of, zeroed};
        
        unsafe {
            let mut status: MEMORYSTATUSEX = zeroed();
            status.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
            
            if GlobalMemoryStatusEx(&mut status) != 0 {
                return Some((status.ullTotalPhys / (1024 * 1024)) as usize);
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try reading from /proc/meminfo
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(mem_kb) = line.split_whitespace().nth(1) {
                        if let Ok(mem_kb) = mem_kb.parse::<usize>() {
                            return Some(mem_kb / 1024);
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        // Use sysctl to get memory
        let output = Command::new("sysctl")
            .arg("-n")
            .arg("hw.memsize")
            .output()
            .ok();
            
        if let Some(output) = output {
            if let Ok(mem_bytes) = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<usize>() 
            {
                return Some(mem_bytes / (1024 * 1024));
            }
        }
    }
    
    // If all methods fail, fall back to a conservative default
    warn!("Couldn't detect system memory, using default");
    Some(1024) // Assume 1GB
}