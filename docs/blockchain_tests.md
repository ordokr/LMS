# Blockchain Implementation Tests

## Unit Tests

The blockchain implementation includes comprehensive unit tests located in:
`src-tauri/src/blockchain/tests/`

These tests verify:
- Memory-efficient transaction processing
- Block creation and verification
- Entity storage and retrieval
- Differential anchoring
- Resource governance
- Context-aware batching

## Performance Tests

Performance benchmarks are available in:
`src-tauri/benches/blockchain_bench.rs`

These benchmarks measure:
- Transaction throughput (transactions per second)
- Memory overhead
- Block creation time
- Sync latency
- Storage efficiency

## Running Tests

Run unit tests with:
```bash
cd src-tauri
cargo test --package lms-core --lib blockchain