# SQLite/Redb Storage Implementation Guide

## Overview

This document outlines the implementation strategy for our optional hybrid SQLite/Redb storage system. The system is designed to be toggleable while maintaining sophisticated integration capabilities with Canvas LMS and Discourse when enabled.

## Feature Flag Configuration

### 1. Cargo Features

```toml
[features]
default = ["sqlite-storage"]

# Storage features
sqlite-storage = ["dep:sqlx", "dep:redb"]
canvas-integration = ["sqlite-storage", "dep:canvas-api"]
discourse-integration = ["sqlite-storage", "dep:discourse-api"]

# Full integration mode
full-integration = [
    "canvas-integration",
    "discourse-integration",
    "sync-engine"
]

# Standalone mode
standalone = ["sqlite-storage"]
```

### 2. Conditional Compilation

```rust
pub struct StorageManager {
    #[cfg(feature = "sqlite-storage")]
    sqlite: SqlitePool,
    #[cfg(feature = "sqlite-storage")]
    redb: Database,
    
    #[cfg(feature = "canvas-integration")]
    canvas_sync: Option<CanvasIntegrationService>,
    
    #[cfg(feature = "discourse-integration")]
    discourse_sync: Option<DiscourseIntegrationService>,
}

impl StorageManager {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let mut manager = Self {
            #[cfg(feature = "sqlite-storage")]
            sqlite: setup_sqlite_pool(config).await?,
            #[cfg(feature = "sqlite-storage")]
            redb: setup_redb_store(config)?,
            
            #[cfg(feature = "canvas-integration")]
            canvas_sync: None,
            
            #[cfg(feature = "discourse-integration")]
            discourse_sync: None,
        };

        #[cfg(feature = "canvas-integration")]
        if config.enable_canvas_integration {
            manager.canvas_sync = Some(CanvasIntegrationService::new(
                manager.sqlite.clone(),
                config.canvas_url.clone(),
                config.canvas_token.clone(),
            ));
        }

        #[cfg(feature = "discourse-integration")]
        if config.enable_discourse_integration {
            manager.discourse_sync = Some(DiscourseIntegrationService::new(
                manager.sqlite.clone(),
                config.discourse_url.clone(),
                config.discourse_api_key.clone(),
            ));
        }

        Ok(manager)
    }
}
```

## Integration Architecture

### 1. Canvas LMS Considerations

Canvas uses a Ruby on Rails architecture with:
- PostgreSQL for persistent storage
- Redis for caching
- Delayed Job for background processing

Our implementation needs to mirror these key Canvas models:
```rust
pub struct CanvasSpecificFields {
    pub unlock_at: Option<DateTime<Utc>>,
    pub lock_at: Option<DateTime<Utc>>,
    pub position: Option<i64>,
    pub published: Option<bool>
}
```

### 2. Discourse Integration Points

Discourse's architecture requires consideration of:
- PostgreSQL database schema
- Redis for caching/pub-sub
- Sidekiq for background jobs

Key Discourse-specific fields:
```rust
pub struct DiscourseSpecificFields {
    pub topic_id: Option<String>,
    pub category_id: Option<String>
}
```

### 3. Conflict Resolution Strategy

Based on the existing conflict resolver:

```rust
pub enum ConflictStrategy {
    PreferCanvas,
    PreferDiscourse,
    PreferMostRecent,
    MergePreferCanvas,
    MergePreferDiscourse
}
```

## Integration Implementation

### 1. Modular Integration Services

```rust
#[cfg(feature = "canvas-integration")]
pub struct CanvasIntegrationService {
    db: SqlitePool,
    client: CanvasClient,
    sync_manager: Arc<SyncManager>,
}

#[cfg(feature = "discourse-integration")]
pub struct DiscourseIntegrationService {
    db: SqlitePool,
    client: DiscourseClient,
    sync_manager: Arc<SyncManager>,
}

// Core storage functionality remains available without integration
pub struct CoreStorageService {
    db: SqlitePool,
    cache: RedbStore,
}
```

### 2. Feature-Gated Schema

```sql
-- Core schema (always present)
CREATE TABLE core_entities (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Canvas integration schema (only when canvas-integration is enabled)
CREATE TABLE IF EXISTS canvas_mappings (
    local_id TEXT PRIMARY KEY,
    canvas_id TEXT UNIQUE,
    entity_type TEXT NOT NULL,
    sync_status TEXT NOT NULL,
    FOREIGN KEY(local_id) REFERENCES core_entities(id)
);

-- Discourse integration schema (only when discourse-integration is enabled)
CREATE TABLE IF EXISTS discourse_mappings (
    local_id TEXT PRIMARY KEY,
    discourse_id TEXT UNIQUE,
    entity_type TEXT NOT NULL,
    sync_status TEXT NOT NULL,
    FOREIGN KEY(local_id) REFERENCES core_entities(id)
);
```

### 3. Integration Manager

```rust
pub struct IntegrationManager {
    #[cfg(feature = "canvas-integration")]
    canvas: Option<CanvasIntegrationService>,
    
    #[cfg(feature = "discourse-integration")]
    discourse: Option<DiscourseIntegrationService>,
    
    core: CoreStorageService,
}

impl IntegrationManager {
    pub async fn sync_entity(&self, entity_id: &str) -> Result<(), Error> {
        // Core functionality always works
        self.core.update_entity(entity_id).await?;
        
        // Integration-specific sync only runs when enabled
        #[cfg(feature = "canvas-integration")]
        if let Some(canvas) = &self.canvas {
            canvas.sync_entity(entity_id).await?;
        }
        
        #[cfg(feature = "discourse-integration")]
        if let Some(discourse) = &self.discourse {
            discourse.sync_entity(entity_id).await?;
        }
        
        Ok(())
    }
}
```

## State Management

### 1. Version Vectors

Implementation based on existing sync engine:

```rust
pub struct VersionVectorSync {
    entity_id: String,
    entity_type: EntityType,
    canvas_vector: VersionVector,
    discourse_vector: VersionVector,
    local_vector: VersionVector,
    last_sync: DateTime<Utc>,
    status: SyncStatus
}
```

### 2. Sync Status Tracking

```rust
pub enum SyncStatus {
    PendingToCanvas,
    PendingToDiscourse,
    InSync,
    Conflict
}
```

## Performance Optimizations

### 1. SQLite Configuration

SQLite configuration focuses on offline-first performance:

```rust
pub async fn optimize_db_connection(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Enable WAL mode for better concurrent access
    sqlx::query("PRAGMA journal_mode = WAL;").execute(pool).await?;
    // Optimize for offline-first operations
    sqlx::query("PRAGMA synchronous = NORMAL;").execute(pool).await?;
    sqlx::query("PRAGMA cache_size = -64000;").execute(pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON;").execute(pool).await?;
    sqlx::query("PRAGMA temp_store = MEMORY;").execute(pool).await?;
    Ok(())
}
```

Key optimizations:
- WAL mode for better concurrency
- Adjusted synchronous mode for offline operations
- Increased cache size for better performance
- In-memory temp store for faster operations

### 2. Redb Configuration

Redb handles fast-access data needs:

```rust
pub fn configure_redb(db: &Database) -> Result<(), Error> {
    let write_txn = db.begin_write()?;
    {
        // Fast-access tables for offline operations
        let drafts = TableDefinition::<&str, &[u8]>::new("drafts");
        write_txn.open_table(drafts)?;
        
        // Sync state management
        let sync = TableDefinition::<&str, &[u8]>::new("sync_metadata");
        write_txn.open_table(sync)?;
    }
    write_txn.commit()?;
    Ok(())
}
```

Redb is used for:
- Draft content management
- Sync state tracking
- Temporary data storage
- High-performance reads

## Build and Deployment

### 1. Feature Selection

```bash
# Build with no integration
cargo build --no-default-features --features "standalone"

# Build with Canvas integration only
cargo build --features "canvas-integration"

# Build with full integration
cargo build --features "full-integration"
```

### 2. Configuration Management

```rust
pub struct Config {
    // Core config (always present)
    pub database_url: String,
    pub cache_size: usize,
    
    // Integration config (optional)
    #[cfg(feature = "canvas-integration")]
    pub canvas_config: Option<CanvasConfig>,
    
    #[cfg(feature = "discourse-integration")]
    pub discourse_config: Option<DiscourseConfig>,
}
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_core_functionality() {
        // Core tests always run
    }
    
    #[test]
    #[cfg(feature = "canvas-integration")]
    fn test_canvas_integration() {
        // Canvas integration tests
    }
    
    #[test]
    #[cfg(feature = "discourse-integration")]
    fn test_discourse_integration() {
        // Discourse integration tests
    }
}
```

## Implementation Checklist

1. [ ] Implement core SQLite schema
   - Design tables for Canvas compatibility
   - Design tables for Discourse compatibility
   - Implement version vector storage

2. [ ] Set up Redb for ephemeral data
   - Configure sync state tables
   - Implement conflict detection cache
   - Set up performance monitoring

3. [ ] Implement sync engine
   - Canvas API integration
   - Discourse API integration
   - Version vector management
   - Conflict resolution strategies

4. [ ] Implement offline-first CRUD operations
   - Design offline-capable APIs
   - Implement optimistic updates
   - Handle conflict resolution

5. [ ] Add sync state management
   - Implement version vectors
   - Design sync protocol
   - Handle merge conflicts

6. [ ] Configure automated testing
   - Unit test coverage
   - Integration tests
   - Performance benchmarks

7. [ ] Set up performance monitoring
   - Query performance tracking
   - Sync timing metrics
   - Storage usage monitoring

8. [ ] Document API interfaces
   - API documentation
   - Usage examples
   - Integration guides

## Notes

Key architectural decisions:
- Core functionality works independently of integrations
- Integrations are completely optional and feature-gated
- Clean separation between core and integration code
- No performance impact when integrations are disabled
- Easy to enable/disable features at compile time
- Maintains sophisticated integration capabilities when enabled
- Prepared for future default integration
- All integration points use trait-based abstractions

