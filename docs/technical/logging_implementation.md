# Logging Implementation: tracing + tracing-subscriber

_Last updated: 2025-04-17_

This guide details the best practices and steps for implementing robust, structured, async-friendly logging in the Ordo project, based on current Rust ecosystem recommendations and Ordo's architectural principles.

## Table of Contents

1. [Overview](#overview)
2. [Dependencies](#dependencies)
3. [Initialization](#initialization)
4. [Instrumenting Your Code](#instrumenting-your-code)
5. [HTTP/API Logging with Tower/Axum](#httpapi-logging-with-toweraxum)
6. [Advanced: Spans for Context](#advanced-spans-for-context)
7. [Exporting and Analyzing Logs](#exporting-and-analyzing-logs)
8. [Best Practices](#best-practices)
9. [Implementation Examples](#implementation-examples)
10. [Summary](#summary)

## Overview

### Why Structured Logging and Tracing?

- **Async & Modular**: Ordo uses async Rust (Tokio, Axum, Tower), where operations interleave. Structured logging with tracing keeps logs tied to the right operation, even across tasks and threads.

- **Debuggability**: Structured logs (with fields, not just plain text) make it much easier to filter, search, and analyze issues—especially in offline-first, event-driven, and modular systems.

- **Performance**: The tracing ecosystem is built for high performance and async compatibility, unlike many traditional loggers.

- **Extensibility**: Structured logs can be exported to dashboards, monitoring, or analytics tools as JSON or sent to distributed tracing systems.

## Dependencies

Add the following to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tower-http = { version = "0.4", features = ["trace"] }
```

- **tracing**: Core for structured, async-aware logging and tracing
- **tracing-subscriber**: Collects, formats, and exports logs (to stdout, files, JSON, etc.)
- **tower-http::TraceLayer**: Auto-instruments HTTP requests in Axum/Tower APIs

## Initialization

Initialize tracing in your main binary (e.g., `src-tauri/src/main.rs`):

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Initialize subscriber with environment-based log level and JSON output
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json() // Use .pretty() for human-readable logs in dev
        .init();

    // Example log
    tracing::info!("Ordo app started");
    
    // Continue with application initialization
    // ...
}
```

Use `RUST_LOG=info cargo run` to control log levels at runtime. The environment variable can be set to different levels for different modules:

```
RUST_LOG=ordo=debug,ordo::sync=trace,tower_http=info cargo run
```

## Instrumenting Your Code

Use the tracing macros for structured, contextual logs:

```rust
use tracing::{info, warn, error, debug, trace, span, Level};

fn sync_data() {
    // Create a span with structured fields
    let span = span!(Level::INFO, "sync", user_id = 42, operation = "full_sync");
    let _enter = span.enter();

    // Log with structured fields
    info!(action = "start", "Starting sync operation");
    
    // ... sync logic ...
    
    // Warning with context
    warn!(
        action = "retry", 
        reason = "network_error", 
        attempt = 2, 
        "Sync retrying"
    );
    
    // Error with detailed context
    error!(
        action = "fail", 
        reason = "timeout", 
        duration_ms = 5000, 
        "Sync failed"
    );
}
```

Use fields (e.g., `user_id`, `action`, `reason`) for structured logs that can be easily filtered and analyzed.

## HTTP/API Logging with Tower/Axum

Instrument your API with automatic request/response tracing:

```rust
use axum::{Router, routing::get};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::Level;

let app = Router::new()
    .route("/api/courses", get(list_courses))
    .route("/api/assignments", get(list_assignments))
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::INFO)
                    .include_headers(true)
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(tower_http::LatencyUnit::Millis)
            )
    );
```

This creates a tracing span for every HTTP request, tagging all logs within the request context with request information like method, path, and response status.

## Advanced: Spans for Context

Use spans to track operations across async boundaries:

```rust
use tracing::Instrument;

async fn process_assignment(assignment_id: i64, user_id: i64) -> Result<(), Error> {
    // Create a span for this operation
    let span = tracing::span!(
        Level::INFO, 
        "process_assignment", 
        assignment_id = assignment_id, 
        user_id = user_id
    );
    
    // Instrument the async block with the span
    async {
        tracing::info!("Processing assignment");
        
        // This will be associated with the process_assignment span
        let result = do_processing().await;
        
        if let Err(e) = &result {
            // Error details are structured
            tracing::error!(
                error.type = %std::any::type_name::<Error>(),
                error.message = %e,
                "Assignment processing failed"
            );
        }
        
        result
    }
    .instrument(span)
    .await
}
```

This ensures that all logs within the async block are associated with the span, even if the task is scheduled on different threads.

## Exporting and Analyzing Logs

By default, logs go to stdout/stderr. For production, configure tracing-subscriber to write to files or serialize as JSON for ingestion by log analysis tools:

```rust
use std::fs::File;
use tracing_subscriber::fmt::writer::MakeWriterExt;

// Create a file writer
let file = File::create("logs/ordo.log").expect("Failed to create log file");

// Set up subscriber with both stdout and file output
let (non_blocking, _guard) = tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_writer(std::io::stdout.and(file))
    .json()
    .finish()
    .with_default();

tracing::subscriber::set_global_default(non_blocking)
    .expect("Failed to set global default subscriber");
```

For production environments, consider using a more sophisticated logging setup:

```rust
// Production logging setup
fn init_production_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create log directory if it doesn't exist
    std::fs::create_dir_all("logs")?;
    
    // Set up file rotation
    let file_appender = tracing_appender::rolling::daily("logs", "ordo.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Initialize with JSON formatting for log aggregation
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .json()
        .init();
    
    // Store _guard in a static or keep it alive for the duration of the program
    Box::leak(Box::new(_guard));
    
    Ok(())
}
```

## Best Practices

1. **Use Structured Fields**
   - Always include relevant context as structured fields
   - Use consistent field names across the codebase
   - Include IDs for entities (user_id, course_id, etc.)

2. **Set Appropriate Log Levels**
   - `error`: For errors that need immediate attention
   - `warn`: For potentially problematic situations
   - `info`: For important events in normal operation
   - `debug`: For detailed information useful during development
   - `trace`: For very detailed diagnostic information

3. **Instrument All Entry Points**
   - API handlers
   - Background jobs
   - Sync engine operations
   - User-initiated actions

4. **Document Log Fields**
   - Maintain a list of standard field names and their meanings
   - Document expected log patterns for common operations

5. **Use Spans for Context**
   - Create spans for operations that span multiple functions
   - Use `.instrument()` for async functions
   - Nest spans for hierarchical operations

## Implementation Examples

### Logging in a Sync Operation

```rust
use tracing::{info, error, instrument};
use serde::Serialize;

#[derive(Serialize)]
struct SyncStats {
    items_processed: usize,
    items_succeeded: usize,
    items_failed: usize,
    duration_ms: u64,
}

#[instrument(
    skip(user, sync_engine),
    fields(
        operation = "sync",
        user_id = %user.id,
        device_id = %device_id
    )
)]
async fn perform_sync(
    user: &User,
    device_id: &str,
    sync_engine: &SyncEngine
) -> Result<SyncStats, SyncError> {
    let start = std::time::Instant::now();
    
    info!("Sync started");
    
    let mut stats = SyncStats {
        items_processed: 0,
        items_succeeded: 0,
        items_failed: 0,
        duration_ms: 0,
    };
    
    // ... sync logic ...
    
    if let Err(e) = do_sync().await {
        error!(
            error.type = %std::any::type_name::<SyncError>(),
            error.message = %e,
            "Sync failed"
        );
        return Err(e);
    }
    
    stats.duration_ms = start.elapsed().as_millis() as u64;
    
    info!(
        stats = %serde_json::to_string(&stats).unwrap_or_default(),
        "Sync completed successfully"
    );
    
    Ok(stats)
}
```

### Logging in Database Operations

```rust
use tracing::{debug, error, instrument};

#[instrument(
    skip(db),
    fields(
        operation = "db_query",
        entity = "course",
        query_type = "select"
    )
)]
async fn get_course(db: &Database, course_id: i64) -> Result<Course, DbError> {
    debug!(course_id = course_id, "Fetching course from database");
    
    let result = db.query("SELECT * FROM courses WHERE id = ?", &[&course_id]).await;
    
    match result {
        Ok(course) => {
            debug!("Course fetched successfully");
            Ok(course)
        }
        Err(e) => {
            error!(
                error.message = %e,
                "Failed to fetch course"
            );
            Err(e.into())
        }
    }
}
```

### Logging in API Handlers

```rust
use axum::{extract::Path, Json};
use tracing::{info, error, instrument};

#[instrument(
    skip(state),
    fields(
        operation = "api_request",
        endpoint = "get_assignment"
    )
)]
async fn get_assignment(
    Path(assignment_id): Path<i64>,
    state: axum::extract::State<AppState>,
) -> Result<Json<Assignment>, ApiError> {
    info!(assignment_id = assignment_id, "Assignment request received");
    
    match state.db.get_assignment(assignment_id).await {
        Ok(assignment) => {
            info!("Assignment retrieved successfully");
            Ok(Json(assignment))
        }
        Err(e) => {
            error!(
                error.message = %e,
                "Failed to retrieve assignment"
            );
            Err(ApiError::NotFound(format!("Assignment {assignment_id} not found")))
        }
    }
}
```

## Summary

| Step | What to Do | Where/How |
|------|------------|-----------|
| 1 | Add crates | `tracing`, `tracing-subscriber`, `tower-http` in Cargo.toml |
| 2 | Init tracing | Set up subscriber in main.rs |
| 3 | Log events | Use tracing macros with fields in all modules |
| 4 | HTTP tracing | Add TraceLayer to Axum/Tower routers in API setup |
| 5 | Use spans | Track context across async boundaries in async functions, jobs |
| 6 | Export logs | Configure for stdout, file, or JSON in tracing-subscriber setup |

Implementing logging with tracing and tracing-subscriber ensures Ordo's logs are structured, performant, async-friendly, and ready for future growth and observability needs. This approach aligns with Ordo's modular, offline-first architecture and provides the foundation for robust debugging and monitoring capabilities.
