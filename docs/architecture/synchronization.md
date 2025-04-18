# Synchronization Architecture

_Last updated: 2025-04-18_

This document describes the synchronization architecture of the Ordo LMS system, focusing on the offline-first capabilities and conflict resolution mechanisms.

## Overview

Ordo uses a sophisticated synchronization system to ensure data consistency between local and remote databases. The system is designed to work in offline environments, allowing users to continue working even when disconnected from the network.

## Core Architecture

The synchronization system is built around a hybrid storage approach that combines SQLite for structured data and Redb for ephemeral state:

```rust
// Sync between SQLite (structured data) and Redb (ephemeral state)
enum SyncTarget {
    StructuredData(SqlitePool),
    EphemeralState(Database),
    FileSystem(PathBuf),
}
```

### Key Components

1. **Sync Engine**: Manages the synchronization process
2. **Vector Clocks**: Track causality between operations
3. **Conflict Resolution**: Resolve conflicts when the same data is modified in different locations
4. **Sync Queue**: Queue operations for later synchronization
5. **Sync Status Tracking**: Track the synchronization status of each record

## Version Vector Conflict Resolution

Ordo implements a version vector (also known as vector clock) approach for conflict detection and resolution. This is a proven technique used in distributed systems to track causality between events.

### Version Vectors

A version vector is a data structure that associates a counter with each device or node in the system. When an operation occurs on a device, its counter is incremented. By comparing version vectors, we can determine the causal relationship between operations:

- **Happens-Before**: One operation happened before another
- **Happens-After**: One operation happened after another
- **Concurrent**: Operations happened concurrently (potential conflict)
- **Identical**: Operations have the same version vector

### Implementation

The version vector implementation in Ordo consists of:

1. **VersionVector Struct**: Encapsulates the vector clock functionality
   ```rust
   pub struct VersionVector {
       counters: HashMap<String, i64>,
   }
   ```

2. **Causal Relations**: Defines the possible relationships between version vectors
   ```rust
   pub enum CausalRelation {
       HappensBefore,
       HappensAfter,
       Concurrent,
       Identical,
   }
   ```

3. **Operations**: Methods for creating, incrementing, merging, and comparing version vectors

### Conflict Detection

Conflicts are detected by comparing the version vectors of operations:

1. If operations affect different entities, there's no conflict
2. If one operation happens before another, there's no conflict
3. If operations are concurrent and affect the same entity, there's a potential conflict

```rust
// Check causal relationship using version vectors
let vv1 = VersionVector::from_hashmap(op1.vector_clock.clone());
let vv2 = VersionVector::from_hashmap(op2.vector_clock.clone());
let relation = vv1.causal_relation(&vv2);

// Only concurrent operations can conflict
if relation == CausalRelation::HappensBefore || relation == CausalRelation::HappensAfter {
    return None; // No conflict
}
```

### Conflict Resolution

When conflicts are detected, they are resolved based on the operation type and the causal relationship:

1. **Create-Create Conflicts**: Keep the operation with more fields or the later timestamp
2. **Create-Update Conflicts**: Apply the update to the create operation
3. **Create-Delete Conflicts**: Depends on the timestamp (later operation wins)
4. **Update-Update Conflicts**: Merge the updates if concurrent, otherwise keep the later one
5. **Update-Delete Conflicts**: Depends on the timestamp (later operation wins)
6. **Delete-Delete Conflicts**: Only need one delete operation

For update-update conflicts with concurrent operations, fields from both updates are merged:

```rust
// Merge vector clocks using VersionVector
let vv1 = VersionVector::from_hashmap(op1.vector_clock.clone());
let vv2 = VersionVector::from_hashmap(op2.vector_clock.clone());
let merged_vv = vv1.merged_with(&vv2);
merged_op.vector_clock = merged_vv.to_hashmap();
```

## Sync Process

The synchronization process follows these steps:

1. **Queue Operations**: When a user makes changes offline, operations are queued
2. **Create Sync Batch**: When online, a batch of operations is created for synchronization
3. **Send to Server**: The batch is sent to the server
4. **Apply Remote Operations**: Remote operations are applied with conflict resolution
5. **Mark as Synced**: Successfully synced operations are marked as synced

## Conflict Resolution Strategies

The sync engine implements domain-specific conflict resolution strategies:

```rust
// src-tauri/src/sync/conflict.rs
pub enum ResolutionStrategy {
    AcademicRecord(VersionVector),  // For grades/credentials
    ForumPost(LastWriteWins),       // For discussions
    UserData(MergePreserveHistory), // For profile changes
    ManualIntervention,             // High-stakes conflicts
}
```

This approach ensures that:
- Academic records maintain integrity through version vectors
- Forum posts use simple last-write-wins for better user experience
- User data attempts to merge changes when possible
- Critical conflicts can be flagged for manual resolution

## Sync Workflow

### For Academic Data (Grades/Credentials)

```
1. Local write → SQLite + Redb journal
2. Version vector increment (local)
3. Background sync attempt every 15m
4. On conflict: Version merge → Manual approval
5. Post-sync: Prune journal entries
```

### For Forum Content

```
1. Local write → Redb with LWW timestamp
2. Immediate sync attempt if online
3. On conflict: Last write wins
4. Async event propagation via Leptos signals
```

## Offline-First Capabilities

Ordo's synchronization system provides robust offline-first capabilities:

1. **Local Storage**: All data is stored locally first
2. **Change Tracking**: Changes made offline are tracked with version vectors
3. **Sync Queue**: Changes are queued for synchronization when online
4. **Conflict Resolution**: Conflicts are resolved when synchronizing
5. **Background Sync**: Synchronization happens in the background without blocking the UI

## Performance Considerations

The sync engine is optimized for performance and resource efficiency:

- **Batch Processing**: Operations are processed in batches for efficiency
- **Memory Constraints**: Configurable memory limits for resource-constrained environments
- **Windows Optimization**: Specific optimizations for Windows environments

## Monitoring and Health

The sync engine includes comprehensive monitoring capabilities:

```rust
// src-tauri/src/sync/metrics.rs
pub struct SyncHealth {
    pending_ops: Gauge,
    sync_latency: Histogram,
    conflict_rate: Counter,
    memory_usage: Gauge,
}
```

These metrics are integrated with the UI to provide users with visibility into sync status and health.

## Integration with Background Job System

The synchronization system integrates with the background job system to handle asynchronous processing of sync operations:

```rust
// src-tauri/src/background/sync_jobs.rs
#[job]
pub struct SyncJob {
    channel_id: Uuid,
    retry_count: usize,
}

impl background_jobs::Job for SyncJob {
    fn run(&self, _: Arc<Context>) -> Result<(), Box<dyn Error>> {
        let sync_engine = SyncEngine::current();
        block_on(sync_engine.process_channel(self.channel_id))?;
        Ok(())
    }

    fn retry_policy(&self) -> RetryPolicy {
        RetryPolicy::exponential_backoff(3)
    }
}
```

This integration ensures that sync operations are reliable, persistent, and can be retried in case of failure.

## Related Documentation

- [Database Architecture](database.md)
- [Offline-First Implementation](../technical/offline_readiness.md)
- [Sync Engine Implementation](../technical/sync_engine_implementation.md)
- [Background Job System](../technical/background_job_system.md)
