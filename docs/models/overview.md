# Ordo Data Models

_Last updated: 2025-04-16_

This document provides an overview of the data models used in Ordo.

## Data Model Overview

Ordo uses a domain-driven design approach to modeling data. The core domain models are implemented as Rust structs with strong typing and validation.

### Core Domain Models

- **User**: Represents a user in the system
- **Course**: Represents a course
- **Assignment**: Represents an assignment within a course
- **Submission**: Represents a student's submission for an assignment
- **Discussion**: Represents a discussion topic
- **Post**: Represents a post within a discussion
- **File**: Represents a file uploaded to the system
- **Notification**: Represents a notification sent to a user

### Model Relationships

The following diagram shows the relationships between the core domain models:

```
+-------+     +--------+     +------------+
| User  |<--->| Course |<--->| Assignment |
+-------+     +--------+     +------------+
    |             |                |
    v             v                v
+-------+     +------------+  +------------+
| Post  |<--->| Discussion |  | Submission |
+-------+     +------------+  +------------+
    |             |                |
    v             v                v
+-------+     +--------+     +------------+
| File  |     | Tag    |     | Grade      |
+-------+     +--------+     +------------+
```

## Data Storage

Ordo employs a hybrid storage architecture:

### SQLite for Structured Data

Ordo uses SQLite for structured data storage, including primary domain entities like courses, assignments, and forum posts. The data models are mapped to database tables using SQLx, a type-safe SQL toolkit for Rust.

### Redb for Real-Time State

Redb, a Rust-native embedded key-value store, complements SQLite by handling:

- Real-time session and state management (drafts, preferences)
- Offline operation queue and conflict resolution metadata
- Caching and indexed metadata for fast access
- Subscription-based notifications

#### Transaction Handling

Ordo implements a robust transaction management system for Redb with the following features:

- **ACID Compliance**: All operations are atomic, consistent, isolated, and durable
- **Transaction Options**: Configurable timeout, retry policies, and isolation levels
- **Error Handling**: Comprehensive error handling with custom error types
- **Retry Mechanism**: Automatic retry for transient errors with configurable retry limits
- **Transaction Hooks**: Support for before/after transaction hooks for auditing and monitoring
- **Timeout Management**: Configurable timeouts to prevent long-running transactions
- **Concurrency Control**: Safe concurrent access to the database from multiple threads
- **Nested Transactions**: Support for nested transactions with savepoints
- **Transaction Logging**: Comprehensive logging of transaction events and metrics
- **Transaction Metrics**: Collection and exposure of transaction performance metrics
- **Transaction Batching**: Support for batching multiple operations in a single transaction
- **Transaction Recovery**: Recovery mechanisms for failed transactions

##### Nested Transactions

Nested transactions allow for more granular control over transaction boundaries, enabling operations to be grouped logically while maintaining the ability to commit or rollback individual groups without affecting the entire transaction. This is particularly useful for complex operations that may need to be partially committed or rolled back.

##### Transaction Logging and Metrics

The transaction logging system provides detailed insights into transaction performance and behavior, including:

- Transaction duration and throughput
- Success and failure rates
- Retry statistics
- Resource utilization
- Error patterns and frequencies

These metrics are invaluable for performance tuning, debugging, and monitoring the health of the database system.

##### Transaction Batching

Transaction batching allows multiple operations to be grouped together and executed as a single atomic unit, improving performance by reducing the overhead of individual transactions. This is particularly useful for bulk operations such as importing or exporting data.

##### Transaction Recovery

The transaction recovery system provides mechanisms for recovering from failed transactions, including:

- Savepoint-based recovery for nested transactions
- Automatic retry of failed transactions
- Recovery of partial transaction results
- Logging of recovery events for auditing and debugging

## Synchronization

Data synchronization between the local and remote databases is handled by the sync engine, which leverages both SQLite and Redb:

- **SQLite**: Handles synchronization of structured domain data
- **Redb**: Manages the sync queue, tracks version vectors, and stores conflict resolution metadata

The sync engine uses a sophisticated conflict resolution strategy to handle conflicts when the same data is modified both locally and remotely. Redb's MVCC (Multi-Version Concurrency Control) capabilities allow background sync operations to proceed without blocking the UI thread, enhancing the user experience during synchronization.

## Model Mapping

The following table shows how Ordo models map to Canvas and Discourse models:

| Canvas | Discourse | Ordo | Notes |
|--------|-----------|------------|-------|
| Course | Category | Course | One-to-one mapping |
| Course Sections | Sub-categories | CourseSection | Optional |
| Discussion | Topic | Discussion | One-to-one mapping |
| Discussion Entry | Post | DiscussionPost | One-to-one mapping |
| Assignment | - | Assignment | Canvas-only |
| User | User | User | Unified user model |
| - | Tags | Tags | Discourse-only |

## Related Documentation

- [Database Schema](database_schema.md)
- [Unified Models](unified_models.md)
