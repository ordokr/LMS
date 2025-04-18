//! Structured logging implementation for Ordo
//!
//! This module provides structured, async-friendly logging using tracing and tracing-subscriber.
//! For detailed implementation guide, see docs/technical/logging_implementation.md

use serde::Serialize;
use std::time::{Duration, Instant};
use tracing::{info, warn, error, debug, instrument, Level, span};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

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

/// Initialize structured logging with optimized settings
///
/// This function sets up tracing with the following features:
/// - Environment-based log level filtering (RUST_LOG env var)
/// - Thread IDs for better async debugging
/// - JSON output for production (can be switched to pretty for development)
/// - Uptime-based timestamps
///
/// See docs/technical/logging_implementation.md for more details and best practices.
pub fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // For development, use .pretty() instead of .json() for human-readable logs
    #[cfg(debug_assertions)]
    let format = fmt::format().pretty();

    #[cfg(not(debug_assertions))]
    let format = fmt::format().json();

    fmt()
        .with_env_filter(env_filter)
        .with_thread_ids(true)
        .with_target(true) // Include module path for better filtering
        .with_timer(fmt::time::uptime())
        .with_file(true) // Include source file for better debugging
        .with_line_number(true) // Include line numbers
        .event_format(format)
        .init();

    debug!("Logging initialized");
    info!(event = "application_start", version = env!("CARGO_PKG_VERSION"));
}

/// Log blockchain events with structured data
///
/// This function logs blockchain commit events with structured fields for better
/// filtering and analysis.
#[instrument(level = "info", skip_all)]
pub fn log_block_commit(tx_count: usize, hash: &str, duration: Duration) {
    let duration_ms = duration.as_millis() as u64;

    info!(
        event = "block_committed",
        tx_count = tx_count,
        hash = hash,
        duration_ms = duration_ms
    );
}

/// Log certificate issuance with structured data
///
/// This function logs certificate issuance events with structured fields for better
/// filtering and analysis.
#[instrument(level = "info", skip_all)]
pub fn log_certificate_issuance(user_id: &str, course_id: &str, achievement_type: &str) {
    let verification_url = format!("/verify/{}/{}", user_id, course_id);

    // Create a span for this event
    let span = span!(Level::INFO, "certificate",
        user_id = user_id,
        course_id = course_id,
        achievement_type = achievement_type
    );
    let _guard = span.enter();

    info!(
        event = "certificate_issued",
        user_id = user_id,
        course_id = course_id,
        achievement_type = achievement_type,
        verification_url = verification_url
    );
}

/// Performance monitoring span for tracking operation duration and success
///
/// This struct creates a span for an operation and logs its duration and success status
/// when it is dropped. It uses structured logging to make it easy to filter and analyze.
///
/// # Example
/// ```
/// let op = LoggedOperation::new("sync_data");
/// // Perform operation
/// if success {
///     op.complete();
/// }
/// // LoggedOperation will log duration and success on drop
/// ```
pub struct LoggedOperation<'a> {
    name: &'a str,
    start: Instant,
    success: bool,
    attributes: Vec<(&'static str, String)>,
}

impl<'a> LoggedOperation<'a> {
    /// Create a new logged operation with the given name
    pub fn new(name: &'a str) -> Self {
        debug!(operation = name, "Starting operation");
        Self {
            name,
            start: Instant::now(),
            success: false,
            attributes: Vec::new(),
        }
    }

    /// Mark the operation as successful
    pub fn complete(mut self) {
        self.success = true;
    }

    /// Add an attribute to the operation log
    pub fn with_attribute(mut self, key: &'static str, value: impl ToString) -> Self {
        self.attributes.push((key, value.to_string()));
        self
    }
}

impl<'a> Drop for LoggedOperation<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let duration_ms = duration.as_millis() as u64;

        // Create a span for this operation
        let span = span!(Level::INFO, "operation",
            name = self.name,
            duration_ms = duration_ms,
            success = self.success
        );
        let _guard = span.enter();

        // Log with structured fields
        if self.success {
            let mut log = info!(event = "operation_complete", duration_ms = duration_ms);

            // Add any custom attributes
            for (key, value) in &self.attributes {
                log = info!(parent: &span, {key} = %value);
            }
        } else {
            let mut log = error!(event = "operation_failed", duration_ms = duration_ms);

            // Add any custom attributes
            for (key, value) in &self.attributes {
                log = error!(parent: &span, {key} = %value);
            }
        }
    }
}

/// Example of instrumenting an async function
///
/// This demonstrates how to use the #[instrument] attribute to automatically
/// create spans for async functions and track context across await points.
#[instrument(level = "info", fields(entity_type = "course", operation = "sync"))]
pub async fn instrument_async_example(course_id: i64, user_id: Option<i64>) {
    // All logs within this function will be associated with the span
    info!("Starting async operation");

    // Even across await points
    async_sub_operation(course_id).await;

    if let Some(uid) = user_id {
        // You can add fields to the current span
        tracing::Span::current().record("user_id", &uid);
        info!("Processing for specific user");
    }

    info!("Async operation completed");
}

/// Helper function to demonstrate nested spans
#[instrument(level = "debug", skip(course_id))]
async fn async_sub_operation(course_id: i64) {
    debug!("Sub-operation in progress");
    // Simulate some async work
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    debug!("Sub-operation completed");
}