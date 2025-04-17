# Ordo Architecture Overview

_Last updated: 2025-04-16_

This document provides an overview of the Ordo architecture.

## Architecture Overview

Ordo is built with a modern, modular architecture that prioritizes offline-first capabilities, performance, and extensibility.

### Core Components

- **Frontend**: Leptos (Rust-based reactive UI framework)
- **Backend**: Rust and Haskell for business logic
- **Desktop Shell**: Tauri (Rust-based desktop framework)
- **Primary Database**: SQLite for structured data storage and sync
- **Key-Value Store**: Redb for real-time state, caching, and sync metadata
- **Sync Engine**: Custom Rust-based sync engine

### Key Components

1. **Domain Models**: Rust structs representing the domain entities
2. **API Handlers**: Rust functions handling API requests
3. **Database Layer**: SQLx for type-safe database interactions
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
|  |  |   SQLite (Server)   |  |  |
|  |  +----------------------+  |  |
|  |                            |  |
|  +----------------------------+  |
|                                  |
+----------------------------------+
```

## Architecture Principles

EduConnect follows these key architectural principles:

1. **Clean Architecture**: Clear separation of concerns with domain-centric design
2. **SOLID Principles**: Single responsibility, Open-closed, Liskov substitution, Interface segregation, Dependency inversion
3. **Offline-First**: All core functionality works without an internet connection
4. **Domain-Driven Design**: Focus on core domain logic and bounded contexts
5. **Modular Design**: Components can be developed, tested, and maintained independently

## Hybrid Storage Architecture

Ordo employs a hybrid storage architecture that combines SQLite and Redb to optimize for both structured data and real-time operations in an offline-first environment:

### SQLite: Structured Data Storage

- **Primary domain data**: Courses, assignments, forum posts, users
- **Relational integrity**: Maintains relationships between entities
- **Query flexibility**: Complex joins and filters for reporting

### Redb: Real-Time Operations & State

As a Rust-native embedded key-value store, Redb complements SQLite in these key areas:

1. **Real-Time Session & State Management**
   - Tracks ephemeral user states (draft posts, UI preferences, active quizzes)
   - ACID transactions ensure crash-safe persistence of partial work
   - MVCC allows concurrent background sync without blocking UI

2. **Offline Queue & Conflict Resolution**
   - Manages pending sync operations during network outages
   - Thread-safe writes enable background sync while users work offline
   - Simpler queue management without complex JOIN operations

3. **Real-Time Subscriptions**
   - Supports push updates for notifications via Leptos' reactive system
   - Efficiently tracks database changes without polling

4. **Caching & Indexed Metadata**
   - Accelerates access to frequently queried data
   - Memory-mapped tables reduce read latency for hot data
   - Secondary indexes enable fast lookups

5. **Sync Engine Metadata**
   - Tracks version vectors, timestamps, and conflict resolution logs
   - Atomic batch writes ensure sync metadata consistency
   - Type-safe storage prevents mismatches between sync state and models

### Storage Responsibility Matrix

| Component | SQLite Use Case | Redb Use Case |
|-----------|----------------|---------------|
| Forum Posts | Relational storage with threads/replies | Real-time subscriptions & draft caching |
| Course Content | Structured LMS data (quizzes, grades) | Enrollment permissions & quick metadata |
| Offline Operations | Primary data persistence | Sync queue & conflict resolution logs |
| UI State | Rarely used | Active sessions, drafts, preferences |

## Related Documentation

- [Database Architecture](database.md)
- [Synchronization Architecture](synchronization.md)
