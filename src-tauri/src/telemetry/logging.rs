use serde::Serialize;
use std::time::{Duration, Instant};
use tracing::{info, warn, error};

// Structured event for blockchain operations
#[derive(Serialize)]
struct BlockEvent<'a> {
    tx_count: usize,
    hash: &'a str,
    duration_ms: u64,
}

// Structured event for certificate issuance
#[derive(Serialize)]
struct CertificateEvent<'a> {
    user_id: &'a str,
    course_id: &'a str,
    achievement_type: &'a str,
    verification_url: &'a str,
}

// Initialize logging with optimized settings
pub fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_thread_ids(true)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .json()
        .init();
}

// Log blockchain events with structured data
pub fn log_block_commit(tx_count: usize, hash: &str, duration: Duration) {
    let event = BlockEvent {
        tx_count,
        hash,
        duration_ms: duration.as_millis() as u64,
    };
    
    info!(
        event = "block_committed",
        details = %serde_json::to_string(&event).unwrap_or_default()
    );
}

// Log certificate issuance
pub fn log_certificate_issuance(user_id: &str, course_id: &str, achievement_type: &str) {
    let verification_url = format!("/verify/{}/{}", user_id, course_id);
    
    let event = CertificateEvent {
        user_id,
        course_id,
        achievement_type,
        verification_url: &verification_url,
    };
    
    info!(
        event = "certificate_issued",
        details = %serde_json::to_string(&event).unwrap_or_default()
    );
}

// Performance monitoring span
pub struct LoggedOperation<'a> {
    name: &'a str,
    start: Instant,
    success: bool,
}

impl<'a> LoggedOperation<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            start: Instant::now(),
            success: false,
        }
    }
    
    pub fn complete(mut self) {
        self.success = true;
    }
}

impl<'a> Drop for LoggedOperation<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let level = if self.success { "info" } else { "error" };
        
        let event_details = format!(
            "{{ \"operation\": \"{}\", \"duration_ms\": {}, \"success\": {} }}",
            self.name,
            duration.as_millis(),
            self.success
        );
        
        if self.success {
            info!(event = "operation_complete", details = %event_details);
        } else {
            error!(event = "operation_failed", details = %event_details);
        }
    }
}