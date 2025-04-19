# Ordo Quiz Module

This module provides a comprehensive quiz system with enhanced performance optimizations and testing.

## Features

### Core Functionality
- Quiz creation, editing, and management
- Multiple question types (multiple choice, flashcards, written)
- Quiz sessions with scoring and analytics
- Spaced repetition learning
- Collaboration features
- Template system

### Performance Optimizations
- Query optimization with caching
- Asset caching (memory + disk)
- Lazy loading of media assets
- Virtualized lists for efficient rendering

### Testing
- Unit tests for all components
- Integration tests for end-to-end functionality
- Performance benchmarks
- Accessibility audits

## Architecture

The quiz module is structured as follows:

- `models.rs` - Data models for quizzes, questions, answers, etc.
- `storage.rs` - Storage layer for persisting quiz data
- `session.rs` - Quiz session management
- `query_optimizer.rs` - Optimized database queries with caching
- `asset_cache.rs` - Caching system for quiz media assets
- `commands.rs` - Tauri commands for frontend integration
- Various specialized modules for specific features

## Performance Optimizations

### Query Optimization
The `query_optimizer.rs` module provides:
- Cached database queries with configurable TTL
- Parameterized queries for efficient filtering
- Batch loading of related entities
- LRU cache eviction strategy
- Cache statistics tracking

### Asset Caching
The `asset_cache.rs` module provides:
- Tiered caching (memory + disk)
- Automatic cache cleanup
- ETag generation for client-side caching
- Memory usage tracking
- Support for various media types

### Lazy Loading
The frontend components include:
- `lazy_media.rs` - Lazy loading of media assets
- `virtualized_list.rs` - Efficient rendering of large lists

## Testing

The module includes comprehensive tests:
- Unit tests for individual components
- Integration tests for end-to-end functionality
- Performance benchmarks for optimization features
- Accessibility audits for UI components

## Usage

### Creating a Quiz
```rust
let quiz = Quiz {
    id: Uuid::new_v4(),
    title: "My Quiz".to_string(),
    description: Some("A sample quiz".to_string()),
    visibility: QuizVisibility::Public,
    // ... other fields
};

engine.create_quiz(quiz).await?;
```

### Starting a Quiz Session
```rust
let session = engine.start_session(quiz_id, user_id).await?;
```

### Using Performance Optimizations
```rust
// Optimized query with caching
let filters = QuizFilters::new()
    .with_visibility(QuizVisibility::Public)
    .with_limit(10);

let quizzes = engine.get_quizzes_optimized(filters).await?;

// Asset caching
let asset_metadata = engine.store_asset(
    data,
    "image.png",
    Some(quiz_id),
    Some(question_id)
).await?;

// Later, retrieve the cached asset
let (data, asset_type, etag) = engine.get_asset(&asset_metadata.id).await?;
```

## Frontend Components

### Lazy Media Loading
```rust
view! {
    <LazyImage
        src="/api/assets/12345"
        alt="Quiz image"
        width="500px"
        height="300px"
        placeholder="Loading..."
    />
}
```

### Virtualized Lists
```rust
view! {
    <VirtualizedList
        items=quizzes_signal
        render_item=|quiz| view! { <QuizCard quiz=quiz /> }
        item_height=150
        height="500px"
    />
}
```

## Testing

Run the tests with:
```bash
cargo test --package app --lib -- quiz::tests
```

Run performance benchmarks with:
```bash
cargo test --package app --lib -- quiz::tests::performance_test
```
