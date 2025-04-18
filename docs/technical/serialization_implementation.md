# Serialization Implementation: serde + bincode

_Last updated: 2025-04-17_

This document details how to implement high-performance, type-safe serialization and deserialization in Ordo using serde and bincode, in line with the project's modular, offline-first, and clean architecture principles.

## Table of Contents

1. [Overview](#overview)
2. [Dependencies](#dependencies)
3. [Data Model Annotations](#data-model-annotations)
4. [Serialization for Storage](#serialization-for-storage)
5. [Deserialization When Loading](#deserialization-when-loading)
6. [Integration with Sync Engine](#integration-with-sync-engine)
7. [Testing and Validation](#testing-and-validation)
8. [Best Practices](#best-practices)
9. [Implementation Examples](#implementation-examples)
10. [Performance Considerations](#performance-considerations)

## Overview

### Why Use serde + bincode?

- **serde** provides a universal, type-safe (de)serialization API for Rust
- **bincode** is a compact, fast binary format that works with serde

### Key Use Cases in Ordo

- Efficiently store sync operations, ephemeral state, and offline queues in Redb
- Serialize/deserialize domain models for local storage and sync
- Minimize storage and bandwidth for offline-first operation
- Support for the version vector system in conflict resolution
- Efficient storage of blockchain anchoring data

## Dependencies

Add the following to `src-tauri/Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
bincode = "2.0"
```

- **serde** provides the serialization/deserialization traits and derive macros
- **bincode** implements the binary encoding format

## Data Model Annotations

Annotate your data models with the appropriate derive macros:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SyncOperation {
    pub op_id: u64,
    pub entity_type: String,
    pub entity_id: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub vector_clock: HashMap<String, i64>,
}
```

### Important Considerations

- All fields must be serializable for use with bincode
- Consider adding `#[serde(skip)]` for fields that shouldn't be serialized
- Use `#[serde(with = "...")]` for custom serialization of specific fields
- For large collections, consider using `#[serde(skip_serializing_if = "Vec::is_empty")]` to optimize size

## Serialization for Storage

When storing operations or state in Redb or SQLite, use bincode's serialization:

```rust
use bincode;
use crate::models::SyncOperation;

// Example: Storing a sync operation in Redb
pub async fn store_operation(&self, operation: SyncOperation) -> Result<(), Error> {
    let op_table = TableDefinition::<u64, &[u8]>::new("sync_operations");
    let write_txn = self.redb.begin_write()?;
    let mut table = write_txn.open_table(op_table)?;
    
    // Serialize the operation to binary format
    let serialized = bincode::serialize(&operation)?;
    
    // Store in Redb
    table.insert(operation.op_id, serialized.as_slice())?;
    write_txn.commit()?;
    
    Ok(())
}
```

## Deserialization When Loading

When retrieving data, use bincode's deserialization:

```rust
pub async fn get_operation(&self, op_id: u64) -> Result<Option<SyncOperation>, Error> {
    let op_table = TableDefinition::<u64, &[u8]>::new("sync_operations");
    let read_txn = self.redb.begin_read()?;
    let table = read_txn.open_table(op_table)?;
    
    if let Some(bytes) = table.get(op_id)? {
        // Deserialize from binary format
        let operation: SyncOperation = bincode::deserialize(bytes)?;
        return Ok(Some(operation));
    }
    
    Ok(None)
}
```

## Integration with Sync Engine

The sync engine should use bincode for efficient storage and transmission of operations:

```rust
// In src-tauri/src/sync/engine.rs
pub async fn queue_operation(&self, operation: SyncOperation) -> Result<(), Error> {
    let op_table = TableDefinition::<u64, &[u8]>::new("sync_operations");
    let write_txn = self.redb.begin_write()?;
    let mut table = write_txn.open_table(op_table)?;
    let serialized = bincode::serialize(&operation)?;
    table.insert(operation.op_id, serialized.as_slice())?;
    write_txn.commit()?;
    Ok(())
}

pub async fn process_operation(&self, op_id: u64) -> Result<SyncOperation, Error> {
    let op_table = TableDefinition::<u64, &[u8]>::new("sync_operations");
    let read_txn = self.redb.begin_read()?;
    let table = read_txn.open_table(op_table)?;
    let bytes = table.get(op_id)?.ok_or(Error::OperationNotFound(op_id))?;
    let operation: SyncOperation = bincode::deserialize(bytes)?;
    Ok(operation)
}
```

## Testing and Validation

Add unit tests to ensure serialization round-trips work correctly:

```rust
// In src-tauri/tests/sync_tests.rs
#[test]
fn test_sync_operation_bincode_roundtrip() {
    // Create a test operation
    let op = SyncOperation {
        op_id: 1,
        entity_type: "course".to_string(),
        entity_id: "course-123".to_string(),
        payload: vec![1, 2, 3, 4],
        timestamp: 1617293982,
        vector_clock: HashMap::from([
            ("device1".to_string(), 1),
            ("device2".to_string(), 3),
        ]),
    };
    
    // Serialize to binary
    let bytes = bincode::serialize(&op).unwrap();
    
    // Deserialize back
    let op2: SyncOperation = bincode::deserialize(&bytes).unwrap();
    
    // Verify equality
    assert_eq!(op, op2);
}
```

## Best Practices

1. **Document All Serializable Models**
   - Keep a comprehensive list in `docs/technical/implementation_details.md`
   - Document any special serialization handling

2. **Version Your Data Structures**
   - Use `#[serde(tag = "version")]` for evolving data structures
   - Implement migration logic for older versions

3. **Format Selection**
   - Use bincode for internal, non-human-readable storage
   - Use serde_json for APIs or user-facing exports
   - Consider serde_yaml for configuration files

4. **Error Handling**
   - Wrap bincode errors in your application's error type
   - Provide context about what was being serialized/deserialized

5. **Performance Optimization**
   - Use `#[serde(skip)]` for derived or cached fields
   - Consider custom serialization for large collections

## Implementation Examples

### Version Vector Serialization

```rust
// In src-tauri/src/sync/version_vector.rs
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VersionVector {
    // Maps device IDs to their logical clocks
    counters: HashMap<String, i64>,
    
    // Skip transient fields during serialization
    #[serde(skip)]
    last_accessed: Option<Instant>,
    
    #[serde(skip)]
    cached_hash: Option<u64>,
}

impl VersionVector {
    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        bincode::serialize(self).map_err(Error::SerializationError)
    }
    
    pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        bincode::deserialize(bytes).map_err(Error::DeserializationError)
    }
}
```

### Differential Anchoring for Blockchain

```rust
// In src-tauri/src/blockchain/anchoring.rs
pub async fn anchor_changes(&mut self) -> Result<(), BlockchainError> {
    if self.pending_diffs.is_empty() {
        return Ok(());
    }
    
    // Serialize all diffs to a compact format
    let mut hasher = blake3::Hasher::new();
    
    for diff in &self.pending_diffs {
        match diff {
            AnyDiff::ForumPost(d) => {
                // Hash the serialized diff
                let diff_bytes = bincode::serialize(d)?;
                hasher.update(&diff_bytes);
            },
            AnyDiff::Achievement(d) => {
                let diff_bytes = bincode::serialize(d)?;
                hasher.update(&diff_bytes);
            }
        }
    }
    
    // Create blockchain anchor with the hash
    let hash = hasher.finalize();
    self.blockchain.add_anchor(hash.as_bytes())?;
    
    // Clear pending diffs
    self.pending_diffs.clear();
    Ok(())
}
```

## Performance Considerations

### Memory Efficiency

- Bincode produces compact binary representations
- Use stack allocation where possible for small objects
- Consider zero-copy deserialization for performance-critical paths

### Serialization Speed

- Bincode is one of the fastest serialization formats for Rust
- For hot paths, consider pre-allocating buffers
- Profile serialization/deserialization in your application

### Storage Optimization

- For very large collections, consider compression
- Use appropriate Redb or SQLite column types
- Implement pruning strategies for historical data

## Where to Place Code

- **Models**: `src-tauri/src/models/`
- **Sync Logic**: `src-tauri/src/sync/`
- **Database Logic**: `src-tauri/src/db/`
- **Tests**: `src-tauri/tests/`
- **Blockchain**: `src-tauri/src/blockchain/`

## Summary

- Use serde for all model (de)serialization
- Use bincode for compact, efficient storage and sync
- Integrate serialization in sync engine and storage layers
- Add tests to ensure correctness
- Document all serializable models and their versions

This approach aligns with Ordo's clean, modular, and offline-first architecture, ensuring efficient and reliable data handling throughout the application.
