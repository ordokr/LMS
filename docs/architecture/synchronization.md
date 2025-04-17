# Synchronization Architecture

_Last updated: 2025-04-17_

This document describes the synchronization architecture of the Ordo LMS system, focusing on the offline-first capabilities and conflict resolution mechanisms.

## Overview

Ordo uses a sophisticated synchronization system to ensure data consistency between local and remote databases. The system is designed to work in offline environments, allowing users to continue working even when disconnected from the network.

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

## Offline-First Capabilities

Ordo's synchronization system provides robust offline-first capabilities:

1. **Local Storage**: All data is stored locally first
2. **Change Tracking**: Changes made offline are tracked with version vectors
3. **Sync Queue**: Changes are queued for synchronization when online
4. **Conflict Resolution**: Conflicts are resolved when synchronizing
5. **Background Sync**: Synchronization happens in the background without blocking the UI

## Related Documentation

- [Database Architecture](database.md)
- [Offline-First Implementation](../technical/offline_readiness.md)
- [Data Synchronization](../technical/data_synchronization.md)
