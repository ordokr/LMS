# Implementation Details

This document provides detailed technical implementation information for various components of the Ordo project.

## Table of Contents

- [Tower Middleware Implementation](#tower-middleware-implementation)
- [Serialization Implementation](#serialization-implementation)
- [Logging Implementation](#logging-implementation)
- [Environment Variables](#environment-variables)

## Tower Middleware Implementation

### 1. Overview: Why Tower?

Tower provides a modular, composable middleware system for async Rust services.

Axum (the recommended HTTP framework for Ordo) natively uses Tower for all middleware.

You get access to a rich ecosystem (logging, CORS, rate-limiting, timeouts, etc.) and can write your own middleware for custom needs.

### 2. Add Tower to Your Project

In your Cargo.toml:

```toml
[dependencies]
axum = { version = "0.6", features = ["headers"] }
tower = { version = "0.4", features = ["full"] }
tower-http = "0.4"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

- `tower` provides the core middleware system.
- `tower-http` gives you ready-to-use HTTP middleware (logging, CORS, compression, etc.).
- `axum` is your web framework, fully Tower-compatible.

### 3. Project Structure Placement

Place all API/middleware logic in src-tauri/src/api/ (per Ordo's modular backend structure).

Custom middleware can live in src-tauri/src/api/middleware/.

### 4. Basic Example: Adding Middleware to Axum Router

```rust
// src-tauri/src/api/main.rs

use axum::{Router, routing::get, Extension};
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;

async fn handler() -> &'static str {
    "Hello, Ordo!"
}

#[derive(Clone)]
struct AppState {
    // Shared state here
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()) // Logging
                .layer(Extension(state))           // Shared state injection
        )
}
```

ServiceBuilder lets you stack multiple middleware efficiently.

Use `.layer()` to add middleware globally or per route.

### 5. Using Multiple Middleware

```rust
use tower_http::{cors::CorsLayer, compression::CompressionLayer};

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .layer(CompressionLayer::new())
    );
```

Stack as many as needed. Each `.layer()` adds a new middleware.

### 6. Custom Middleware Example

Create a file: src-tauri/src/api/middleware/auth.rs

```rust
use axum::{http::Request, middleware::Next, response::Response};
use tower_http::auth::RequireAuthorizationLayer;

pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    // Insert your auth logic here (e.g., check headers, JWT, etc.)
    next.run(req).await
}
```

Add to your router:

```rust
use axum::middleware;

let app = Router::new()
    .route("/protected", get(protected_handler))
    .route_layer(middleware::from_fn(auth_middleware));
```

### 7. Error Handling in Middleware

If your middleware can fail, use HandleErrorLayer:

```rust
use axum::{http::StatusCode, BoxError};
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(axum::error_handling::HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::REQUEST_TIMEOUT
            }))
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
    );
```

Always convert errors to valid HTTP responses.

### 8. Advanced: Writing Custom Tower Middleware

For advanced use, implement the tower::Service trait directly:

```rust
use tower::{Service, Layer};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S, Request> Service<Request> for MyMiddleware<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // Do something with the request here
        self.inner.call(req)
    }
}
```

Wrap your custom middleware with `.layer()` or ServiceBuilder.

### 9. Where to Document and Test

Place all middleware documentation in docs/technical/implementation_details.md and reference in the central hub.

Add integration tests in src-tauri/tests/api_middleware.rs.

### 10. Best Practices for Ordo

- Use feature flags in Cargo.toml to enable/disable middleware for modular builds.
- Document all custom middleware and their configuration.
- Use Tower's ready-made middleware for common needs; only write custom middleware for Ordo-specific logic.

### References

- [Tower Docs](https://docs.rs/tower/latest/tower/)
- [Axum Middleware Docs](https://docs.rs/axum/latest/axum/middleware/index.html)
- [Tower Middleware Guide](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md)
- [Ordo Central Reference Hub](../../central_reference_hub.md) (project structure, docs)

### Summary Table

| Step | What to Do | Where |
|------|------------|-------|
| 1 | Add dependencies | Cargo.toml |
| 2 | Place API/middleware code | src-tauri/src/api/ |
| 3 | Use ServiceBuilder for stacking | main.rs or router module |
| 4 | Add ready-made or custom middleware | .layer() or ServiceBuilder |
| 5 | Document and test | docs/technical, src-tauri/tests/ |

Tower middleware gives Ordo a scalable, modular, and type-safe way to handle all HTTP and API concerns, fully aligned with the project's architecture and Rust best practices.

## Serialization Implementation

For detailed information about implementing serialization in Ordo using serde and bincode, see the [Serialization Implementation](serialization_implementation.md) document.

## Logging Implementation

For detailed information about implementing structured, async-friendly logging in Ordo using tracing and tracing-subscriber, see the [Logging Implementation](logging_implementation.md) document.

## Environment Variables

For detailed information about implementing secure, flexible environment variable management in Ordo using dotenvy, see the [Environment Variables](environment_variables.md) document.
