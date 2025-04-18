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

The proposed architecture for the offline-first implementation uses:

- **Backend**: Rust with Axum or Actix Web
- **Frontend**: Leptos (Rust-based reactive framework)
- **Desktop Shell**: Tauri (Rust-based desktop framework)
- **Database**: SQLite for local storage, PostgreSQL for server sync
- **Sync Engine**: Custom Rust-based sync engine

### Key Components

1. **Domain Models**: Rust structs representing the domain entities
2. **API Handlers**: Rust functions handling API requests
3. **Database Layer**: Diesel ORM for database interactions
4. **Leptos Components**: Reactive UI components written in Rust
5. **Sync Engine**: Handles data synchronization between local and remote databases
6. **Offline Queue**: Manages operations performed while offline

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
|  |  |   SQLite (Local)    |  |  |
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
|  |  |   PostgreSQL        |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  +----------------------------+  |
|                                  |
+----------------------------------+
```
