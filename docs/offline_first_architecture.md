# Offline-First Architecture for Ordo Quiz

This document outlines the offline-first architecture implemented in the Ordo Quiz module, which allows it to function seamlessly whether online or offline while maintaining data consistency with the main Ordo LMS.

## Core Principles

1. **Local-First Data Storage**: All data is stored locally first, then synced to the server when connectivity is available.
2. **Queue-Based Sync**: Changes made while offline are queued for synchronization when connectivity is restored.
3. **Conflict Resolution**: Smart conflict resolution strategies are employed when changes are made to the same data both locally and remotely.
4. **Transparent User Experience**: The offline/online state is clearly communicated to users, but doesn't impede their ability to use the app.

## Architecture Components

### 1. Storage Layer

The storage layer uses a hybrid approach combining SQLite and ReDB:

- **SQLite**: Used for structured data with complex relationships (quizzes, questions, attempts, etc.)
- **ReDB**: Used for fast key-value storage and caching

This hybrid approach provides both the flexibility of a relational database and the performance of a key-value store.

### 2. Sync Manager

The Sync Manager is responsible for:

- Tracking changes made while offline
- Queuing operations for later synchronization
- Resolving conflicts between local and remote changes
- Providing status updates to the UI

Operations are prioritized based on their importance:

- **Critical**: Must be synced as soon as possible (e.g., completed quiz attempts)
- **High**: Important but not critical (e.g., quiz updates)
- **Medium**: Standard priority (e.g., new questions)
- **Low**: Can be synced when convenient (e.g., metadata updates)

### 3. Network Monitor

The Network Monitor:

- Continuously checks for network connectivity
- Updates the app's online/offline status
- Triggers sync operations when connectivity is restored
- Provides feedback to the user about the current connection state

### 4. Conflict Resolution

When conflicts occur (changes made to the same data both locally and remotely), they are resolved using:

1. **Timestamp-Based Resolution**: The newer change wins by default
2. **Field-Level Merging**: For complex objects, changes to different fields can be merged
3. **User Intervention**: For critical conflicts, the user can be prompted to choose which version to keep

### 5. User Interface Components

The UI includes:

- **Offline Indicator**: Shows the current online/offline status
- **Sync Status**: Displays the number of pending sync operations
- **Manual Sync Button**: Allows users to trigger sync manually when online

## Data Flow

1. **Create/Update/Delete Operation**:
   - Data is saved to local storage
   - Operation is added to the sync queue
   - UI is updated immediately

2. **Sync Process**:
   - When online, the sync manager processes the queue
   - Operations are sent to the server in priority order
   - Conflicts are resolved as needed
   - Sync status is updated in the UI

3. **Offline to Online Transition**:
   - Network monitor detects connectivity
   - Sync manager is notified
   - Pending operations are processed
   - UI is updated to reflect online status

## Implementation Details

### Sync Queue Schema

```sql
CREATE TABLE quiz_sync_items (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'create', 'update', 'delete'
    data TEXT NOT NULL, -- JSON data
    priority TEXT NOT NULL, -- 'low', 'medium', 'high', 'critical'
    status TEXT NOT NULL, -- 'pending', 'in_progress', 'completed', 'failed'
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_at TEXT,
    error TEXT,
    retry_count INTEGER DEFAULT 0
);
```

### Sync Operation Types

```rust
pub enum SyncOperation {
    Create,
    Update,
    Delete,
}
```

### Priority Levels

```rust
pub enum SyncPriority {
    Critical = 0,
    High = 10,
    Normal = 20,
    Low = 30,
    Background = 40,
}
```

## Best Practices

1. **Minimize Data Transfer**: Only sync essential data to reduce bandwidth usage
2. **Batch Operations**: Group related operations to reduce network overhead
3. **Progressive Enhancement**: Provide additional features when online
4. **Clear Feedback**: Always communicate sync status to users
5. **Graceful Degradation**: Ensure core functionality works offline

## Future Enhancements

1. **Differential Sync**: Only sync changed fields rather than entire objects
2. **Compression**: Compress data before syncing to reduce bandwidth usage
3. **Selective Sync**: Allow users to choose what data to sync
4. **Background Sync**: Use service workers for background synchronization
5. **Conflict Prevention**: Implement optimistic locking to reduce conflicts
