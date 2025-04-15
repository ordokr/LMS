# Blockchain API Documentation

This document describes the API for the LMS blockchain implementation.

## HybridChain

Core blockchain implementation with CRDT-based consensus

### Methods

#### `create_entity(entity_type, data)`

Creates a new entity in the blockchain

#### `update_entity(entity_id, data)`

Updates an existing entity

#### `get_entity(entity_id)`

Retrieves an entity by ID

#### `verify_entity(entity_id)`

Verifies an entity exists in the blockchain

#### `create_block()`

Creates a new block with the current state

---

## AdaptiveBatcher

Intelligent batching system for transaction processing

### Methods

#### `add_change(change, priority)`

Adds a change to the batch queue

#### `process_batch()`

Processes pending changes in a batch

#### `start_batch_loop()`

Starts the background batch processing loop

---

## AdaptiveSyncManager

Manages synchronization of blockchain events

### Methods

#### `sync_event(event)`

Synchronizes an event to the blockchain

#### `force_sync(event)`

Forces immediate synchronization of an event

#### `determine_sync_priority(event)`

Determines the sync priority for an event

---

