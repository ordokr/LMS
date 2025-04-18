# Architecture Overview

This document provides a high-level overview of the Canvas and Discourse architectures and proposes an architecture for the Rust/Tauri/Leptos implementation.

## Canvas LMS Architecture

Canvas LMS is built using:

- **Backend**: Ruby on Rails
- **Frontend**: React.js
- **Database**: PostgreSQL
- **Caching**: Redis
- **Background Jobs**: Delayed Job

### Key Components

1. **Models**: ActiveRecord models representing the domain entities
2. **Controllers**: Rails controllers handling HTTP requests
3. **API**: RESTful API endpoints for frontend communication
4. **React Components**: UI components for the frontend
5. **Background Jobs**: Asynchronous processing for tasks like notifications and grading

## Discourse Architecture

Discourse is built using:

- **Backend**: Ruby on Rails
- **Frontend**: Ember.js
- **Database**: PostgreSQL
- **Caching**: Redis
- **Background Jobs**: Sidekiq

### Key Components

1. **Models**: ActiveRecord models for forum entities (posts, topics, users, etc.)
2. **Controllers**: Rails controllers for handling requests
3. **API**: JSON API for frontend communication
4. **Ember Components**: UI components and templates
5. **Plugins**: Modular extension system

## Proposed Rust/Tauri/Leptos Architecture

The proposed architecture for the offline-first implementation uses the latest stable versions of all dependencies (see [Dependency Management](../development/dependency_management.md) for details):

- **Backend**: Rust with Axum
- **Frontend**: Leptos (Rust-based reactive framework)
- **Desktop Shell**: Tauri (Rust-based desktop framework)
- **Database**: SQLite for structured data, Redb for ephemeral state
- **Sync Engine**: Custom Rust-based sync engine with version vectors
- **Background Jobs**: Hybrid system using background_jobs and tokio-beat

### Key Components

1. **Domain Models**: Rust structs representing the domain entities
2. **API Handlers**: Rust functions handling API requests
3. **Database Layer**: SQLite ORM for structured data, Redb for ephemeral state
4. **Leptos Components**: Reactive UI components written in Rust
5. **Sync Engine**: Handles data synchronization with domain-specific conflict resolution
6. **Background Job System**: Manages asynchronous tasks with different reliability requirements
7. **Offline Queue**: Manages operations performed while offline with priority-based processing

### Architecture Diagram

```
+----------------------------------+
|            Tauri Shell           |
+----------------------------------+
|                                  |
|  +----------------------------+  |
|  |      Leptos Frontend       |  |
|  +----------------------------+  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   UI Components     |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  +----------------------------+  |
|                                  |
|  +----------------------------+  |
|  |      Rust Backend         |  |
|  +----------------------------+  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   Domain Logic      |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   Sync Engine       |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  | Background Job System|  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   SQLite (Structured)|  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   Redb (Ephemeral)  |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  +----------------------------+  |
|                                  |
+----------------------------------+
              |   |
              |   |  (Sync when online)
              v   v
+----------------------------------+
|         Remote Server            |
|                                  |
|  +----------------------------+  |
|  |      API Endpoints         |  |
|  +----------------------------+  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   Database Layer        |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  |  +----------------------+  |  |
|  |  |   Sync Engine        |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  +----------------------------+  |
|                                  |
+----------------------------------+
```

## Modular Architecture

The Ordo project implements a modular architecture that allows for extending the application with additional app-like modules that can be turned on and off. This approach enables:

1. **Extensibility**: New modules can be added without modifying the core application
2. **Flexibility**: Users can enable only the modules they need
3. **Maintainability**: Modules can be developed and tested independently
4. **Performance**: Disabled modules have no runtime overhead

For detailed information about the modular architecture, see the [Modular Architecture](modular_architecture.md) document.
