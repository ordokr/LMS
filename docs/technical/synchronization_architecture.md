# Synchronization Architecture

This document describes the synchronization system used in the LMS platform to keep data consistent between Canvas LMS, Discourse forums, and our local database.

## Table of Contents

1. [Overview](#overview)
2. [Components](#components)
3. [Synchronization Process](#synchronization-process)
4. [Version Vector Conflict Detection](#version-vector-conflict-detection)
5. [Conflict Resolution Strategies](#conflict-resolution-strategies)
6. [Transaction Handling](#transaction-handling)
7. [Sync Queue Management](#sync-queue-management)
8. [Error Handling and Recovery](#error-handling-and-recovery)
9. [Maintenance and Cleanup](#maintenance-and-cleanup)
10. [Dashboard and Monitoring](#dashboard-and-monitoring)

## Overview

The LMS platform integrates with both Canvas LMS and Discourse forums, requiring robust bidirectional synchronization to keep discussions, topics, and other content in sync across all platforms. The sync system is designed with the following principles:

- **Bidirectional Sync**: Changes can originate in any system and propagate to others.
- **Conflict Detection**: Using version vectors to accurately detect concurrent modifications.
- **Conflict Resolution**: Smart strategies to resolve conflicts when the same entity is modified in multiple systems.
- **Transactional Integrity**: Ensuring data consistency by wrapping sync operations in transactions.
- **Queued Processing**: Async sync queue for reliable background processing.
- **Monitoring**: Comprehensive dashboards and metrics for sync operations.

## Components

The synchronization architecture consists of the following key components:

### 1. Integration Services

- **Canvas Integration Service**: Handles communication with Canvas LMS API
- **Discourse Integration Service**: Manages interaction with Discourse forums API

### 2. Sync Core

- **Sync Service**: Coordinates sync operations between systems
- **Version Vector Manager**: Tracks entity versions for conflict detection
- **Conflict Resolver**: Implements strategies to resolve conflicts

### 3. Infrastructure

- **Sync Queue**: Manages pending sync operations
- **Transaction Handler**: Ensures atomic operations with rollback capability
- **Maintenance Service**: Handles cleanup and maintenance tasks

### 4. UI Components

- **Sync Status Widgets**: Display sync status information
- **Sync History Widgets**: Show history of sync operations
- **Error Reporting**: Interface for viewing and managing sync errors

## Synchronization Process

The synchronization process follows these general steps:

1. **Entity Change Detection**:
   - When an entity is created or modified in any system, a sync event is generated

2. **Sync Operation Queueing**:
   - The sync event is added to the sync queue with metadata (entity type, ID, operation, etc.)

3. **Queue Processing**:
   - The sync worker processes the queue, performing sync operations in order
   - Each operation is wrapped in a transaction for atomic execution

4. **Version Vector Update**:
   - The version vector for the entity is updated to reflect the change
   - Version vectors track modifications across all systems

5. **Conflict Detection and Resolution**:
   - If conflicts are detected, the configured resolution strategy is applied
   - Resolution strategy can be system-wide or per-entity-type

6. **Completion and Logging**:
   - The sync operation is marked as completed or failed
   - Detailed information is logged for monitoring and troubleshooting

## Version Vector Conflict Detection

Version vectors are used to accurately detect conflicts that arise from concurrent modifications across different systems.

### How Version Vectors Work

1. **Vector Structure**:
   - Each system maintains a vector of counters for each entity
   - Each counter tracks modifications by a specific device/system

2. **Vector Update**:
   - When a system modifies an entity, it increments its own counter in the vector

3. **Comparison**:
   - Version vectors can be compared to determine causal relationships
   - Four possible relationships: identical, happens-before, happens-after, concurrent

4. **Conflict Detection**:
   - Concurrent relationship indicates a conflict (modifications made without knowledge of each other)
   - Other relationships indicate a clear ordering of changes

### Example

```
Entity: Topic#123
Canvas Vector: {device1: 2, device2: 3}
Discourse Vector: {device1: 2, device2: 4, device3: 1}

Relationship: Discourse happens-after Canvas (no conflict)
```

## Conflict Resolution Strategies

When conflicts are detected (concurrent modifications), resolution strategies are applied:

### System Strategies

1. **PreferCanvas**:
   - Always prefer the Canvas version during conflicts
   - Useful when Canvas is considered the system of record

2. **PreferDiscourse**:
   - Always prefer the Discourse version during conflicts
   - Useful when Discourse features better content/metadata

3. **PreferMostRecent**:
   - Choose the version with the most recent timestamp
   - Assumes clock synchronization across systems

4. **MergePreferCanvas**:
   - Merge both versions, with Canvas values taking precedence for conflicts
   - Preserves unique fields from both systems

5. **MergePreferDiscourse**:
   - Merge both versions, with Discourse values taking precedence for conflicts
   - Preserves unique fields from both systems

### Version Vector Based Resolution

When using version vectors, conflict resolution follows these rules:

1. If version vectors show a causal relationship (happens-before/happens-after), always choose the "after" version.
2. If version vectors show a concurrent relationship (true conflict), apply the configured strategy.

## Transaction Handling

All sync operations are wrapped in transactions to ensure data consistency:

### Transaction Features

1. **Atomic Operations**:
   - All changes for a sync operation succeed or fail together
   - Prevents partial updates that could lead to inconsistent state

2. **Rollback Capability**:
   - If any part of a sync operation fails, changes are rolled back
   - Returns the system to its previous consistent state

3. **Isolation Levels**:
   - Configurable isolation to control how concurrent sync operations interact
   - Prevents dirty reads and other transaction anomalies

4. **Nested Transactions**:
   - Support for nested transactions for complex sync operations
   - Allows for fine-grained control over operation groups

### Transaction Logging

- All transactions are logged with detailed information
- Transaction logs can be used for auditing and troubleshooting

## Sync Queue Management

The sync queue ensures reliable processing of sync operations:

### Queue Features

1. **Prioritization**:
   - Critical sync operations can be prioritized
   - Entity relationships can influence processing order

2. **Batching**:
   - Related sync operations can be batched for efficiency
   - Reduces API calls and database operations

3. **Retries**:
   - Failed operations are retried with exponential backoff
   - Configurable retry limits and backoff strategy

4. **Status Tracking**:
   - Each operation's status is tracked (pending, in progress, completed, failed)
   - Detailed error information is stored for failed operations

### Queue Processing

- Background worker processes queue items asynchronously
- Processing can be paused/resumed via admin controls
- Manual intervention possible for stuck or problematic operations

## Error Handling and Recovery

The sync system includes robust error handling and recovery mechanisms:

### Error Handling

1. **Error Classification**:
   - Transient errors (retryable): network issues, rate limits
   - Permanent errors (non-retryable): permission issues, invalid data

2. **Contextualized Errors**:
   - Errors include context about the operation and entity
   - Stack traces and system state information for debugging

3. **Notification System**:
   - Critical sync errors can trigger notifications
   - Configurable notification thresholds and channels

### Recovery Strategies

1. **Automatic Recovery**:
   - Automatic retries for transient errors
   - System health checks to detect and fix inconsistencies

2. **Manual Recovery**:
   - Admin interface for manually resolving sync issues
   - Options to force sync, skip entities, or reset state

3. **Bulk Recovery**:
   - Tools for bulk operations to recover from major outages
   - Can target specific entity types or time ranges

## Maintenance and Cleanup

Regular maintenance is essential for the sync system's health:

### Automated Cleanup

1. **Completed Operations**:
   - Completed sync operations older than 30 days are archived or purged
   - Retention policy is configurable per entity type

2. **Failed Operations**:
   - Failed operations are retained longer (90 days by default)
   - Can be manually purged after resolution

3. **Sync History Compaction**:
   - Detailed history is compacted over time
   - Summary statistics are preserved for long-term analysis

### Maintenance Jobs

1. **Consistency Checks**:
   - Periodic validation of entity sync status
   - Detection and resolution of orphaned operations

2. **Performance Optimization**:
   - Index maintenance for sync-related tables
   - Clean up of temporary transaction data

3. **Database Maintenance**:
   - Regular vacuuming/compaction of sync tables
   - Archiving of old sync history to maintain performance

## Dashboard and Monitoring

Comprehensive dashboards provide visibility into the sync system:

### Sync Status Dashboard

- Overall health indicators for Canvas and Discourse integrations
- Counts of pending, completed, and failed operations
- Sync success rate and performance metrics

### Sync History Dashboard

- Detailed history of sync operations with filtering
- Timeline visualization of sync activity
- Search and filter capabilities by entity type, status, etc.

### Error Reporting

- Detailed view of sync errors
- Error trends and patterns
- Resolution tracking and history

### Performance Metrics

- Sync operation duration
- Queue processing rate
- API call statistics for external systems

## Conclusion

This synchronization architecture provides a robust foundation for maintaining data consistency across Canvas LMS, Discourse forums, and our local database. By leveraging version vectors for conflict detection, implementing smart resolution strategies, and ensuring transactional integrity, the system can handle complex bidirectional sync scenarios while minimizing data inconsistencies.

The comprehensive monitoring and maintenance tools ensure administrators have visibility into the sync process and can quickly address any issues that arise.