# Quiz Module Developer Guide

## Architecture Overview

The Quiz Module is built with a layered architecture that separates concerns and promotes maintainability:

1. **Data Layer**: Models and storage
2. **Business Logic Layer**: Core functionality and services
3. **API Layer**: Commands and interfaces for the frontend
4. **Presentation Layer**: UI components and pages

### Technology Stack

- **Backend**: Rust with Tauri
- **Storage**: SQLite (via SQLx) and Redb
- **Frontend**: Leptos (Rust WASM framework)
- **Styling**: CSS with Tailwind

## Core Components

### Data Models

The core data models are defined in `src-tauri/src/quiz/models.rs`:

```rust
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub questions: Vec<Question>,
    pub settings: QuizSettings,
    pub author_id: Option<Uuid>,
    pub visibility: QuizVisibility,
    pub tags: Vec<String>,
    pub study_mode: StudyMode,
}

pub struct Question {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub content: QuestionContent,
    pub answer_type: AnswerType,
    pub choices: Vec<Choice>,
    pub correct_answer: Answer,
    pub explanation: Option<String>,
}
```

### Storage Layer

The storage layer is implemented in `src-tauri/src/quiz/storage.rs` and provides:

- CRUD operations for quizzes and questions
- Session storage
- Caching and optimization
- Encryption for sensitive data

The `HybridQuizStore` combines SQLite (for structured data) and Redb (for fast key-value storage):

```rust
pub struct HybridQuizStore {
    sqlite_pool: SqlitePool,
    redb: Arc<RwLock<RedbStore>>,
    encryption_key: Option<[u8; 32]>,
}
```

### Quiz Engine

The `QuizEngine` in `src-tauri/src/quiz/mod.rs` is the main entry point for the module:

```rust
pub struct QuizEngine {
    store: Arc<storage::HybridQuizStore>,
    scheduler: Arc<SpacedRepetitionScheduler>,
    analytics: Arc<AnalyticsEngine>,
    export_engine: Arc<QuizExportEngine>,
    session_queue: mpsc::UnboundedSender<QuizSession>,
    // ... other components
    query_optimizer: Arc<QuizQueryOptimizer>,
    asset_cache: Arc<AssetCache>,
}
```

## Performance Optimizations

### Query Optimizer

The query optimizer in `src-tauri/src/quiz/query_optimizer.rs` provides:

- Cached database queries
- Parameterized queries for efficient filtering
- Batch loading of related entities

Example usage:

```rust
// Create filters
let filters = QuizFilters::new()
    .with_visibility(QuizVisibility::Public)
    .with_limit(10);

// Use the optimizer
let quizzes = engine.get_quizzes_optimized(filters).await?;
```

### Asset Cache

The asset cache in `src-tauri/src/quiz/asset_cache.rs` provides:

- Tiered caching (memory + disk)
- Automatic cache cleanup
- ETag generation for client-side caching

Example usage:

```rust
// Store an asset
let asset_metadata = engine.store_asset(
    data,
    "image.png",
    Some(quiz_id),
    Some(question_id)
).await?;

// Retrieve the asset
let (data, asset_type, etag) = engine.get_asset(&asset_metadata.id).await?;
```

## Frontend Components

### Lazy Media

The `LazyMedia` component in `src/components/quiz/lazy_media.rs` provides:

- Lazy loading of images, audio, and video
- Intersection observer for viewport detection
- Placeholder support during loading

Example usage:

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

### Virtualized List

The `VirtualizedList` component in `src/components/quiz/virtualized_list.rs` provides:

- Efficient rendering of large lists
- Only renders visible items
- Configurable buffer zones for smooth scrolling

Example usage:

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

## Advanced Features

### Spaced Repetition

The spaced repetition system is implemented in `src-tauri/src/quiz/spaced_repetition.rs`:

- SM-2 algorithm with modifications
- Configurable parameters
- Performance tracking

### Collaborative Editing

Collaborative editing is implemented in `src-tauri/src/quiz/collaboration.rs`:

- Real-time updates
- Conflict resolution
- Permission management

### AI-Assisted Generation

AI-assisted quiz generation is implemented in `src-tauri/src/quiz/ai_generation.rs`:

- Multiple AI providers (OpenAI, Anthropic)
- Text-to-quiz conversion
- Topic-based generation

### Adaptive Learning

Adaptive learning paths are implemented in `src-tauri/src/quiz/adaptive_learning.rs`:

- Conditional progression
- Branching paths
- Performance-based adaptation

## Extending the Module

### Adding a New Question Type

1. Add the new type to the `AnswerType` enum in `models.rs`:

```rust
pub enum AnswerType {
    MultipleChoice,
    MultipleSelect,
    TrueFalse,
    Matching,
    Ordering,
    FillInTheBlank,
    ShortAnswer,
    Essay,
    Hotspot,
    DragAndDrop,
    YourNewType,
}
```

2. Add a new variant to the `Answer` enum:

```rust
pub enum Answer {
    SingleChoice(Uuid),
    MultipleChoice(Vec<Uuid>),
    Text(String),
    Matching(HashMap<Uuid, Uuid>),
    Ordering(Vec<Uuid>),
    // ... other types
    YourNewAnswer(YourDataType),
}
```

3. Implement validation for the new answer type in the `validate` method.

4. Create a new component for rendering the question type in the frontend.

5. Update the question editor to support creating and editing the new type.

### Adding a New AI Provider

1. Create a new implementation of the `AIModelProvider` trait in `src-tauri/src/quiz/ai_generation_providers`:

```rust
pub struct YourAIProvider {
    // Provider-specific fields
}

impl AIModelProvider for YourAIProvider {
    async fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<AIGenerationResult, Box<dyn std::error::Error + Send + Sync>> {
        // Implementation
    }
    
    // Other required methods
}
```

2. Register the provider in the `AIGenerationService`:

```rust
let provider = match request.model_provider {
    AIModelProvider::OpenAI => Arc::new(OpenAIModelProvider::new(api_key)) as Arc<dyn AIModelProvider>,
    AIModelProvider::Anthropic => Arc::new(AnthropicModelProvider::new(api_key)) as Arc<dyn AIModelProvider>,
    AIModelProvider::YourProvider => Arc::new(YourAIProvider::new(api_key)) as Arc<dyn AIModelProvider>,
    // ... other providers
};
```

### Adding a New Export Format

1. Add the new format to the `ExportFormat` enum in `src-tauri/src/quiz/export.rs`:

```rust
pub enum ExportFormat {
    JSON,
    CSV,
    PDF,
    Print,
    YourFormat,
}
```

2. Implement the export logic in the `QuizExportEngine`:

```rust
match format {
    ExportFormat::JSON => self.export_json(quiz),
    ExportFormat::CSV => self.export_csv(quiz),
    // ... other formats
    ExportFormat::YourFormat => self.export_your_format(quiz),
}
```

3. Add a new method for your format:

```rust
fn export_your_format(&self, quiz: &Quiz) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Implementation
}
```

## Testing

### Unit Tests

Unit tests are in the `tests` directory of each module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quiz_creation() {
        // Test implementation
    }
}
```

### Integration Tests

Integration tests test multiple components together:

```rust
#[tokio::test]
async fn test_quiz_session_flow() {
    let (engine, _temp_dir) = setup_test_environment().await;
    
    // Create a quiz
    // Start a session
    // Submit answers
    // Complete the session
    // Verify results
}
```

### Performance Benchmarks

Performance benchmarks measure the efficiency of optimizations:

```rust
#[tokio::test]
async fn benchmark_quiz_retrieval() {
    // Setup
    // Measure standard retrieval
    // Measure optimized retrieval
    // Compare results
}
```

## Best Practices

### Error Handling

Use the `anyhow` crate for error handling:

```rust
pub async fn create_quiz(&self, quiz: Quiz) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Implementation
    Ok(())
}
```

### Async/Await

Use `async/await` for asynchronous operations:

```rust
pub async fn get_quiz(&self, quiz_id: Uuid) -> Result<Quiz, Box<dyn std::error::Error + Send + Sync>> {
    self.store.get_quiz(quiz_id).await
}
```

### Immutable Data

Prefer immutable data structures:

```rust
// Good
let new_quiz = Quiz {
    id: quiz.id,
    title: new_title,
    ..quiz
};

// Avoid
quiz.title = new_title;
```

### Separation of Concerns

Keep components focused on a single responsibility:

- Models should only define data structures
- Storage should only handle persistence
- Business logic should be in service components
- UI components should only handle presentation

## Troubleshooting

### Common Issues

1. **Database Errors**: Check connection string and migrations
2. **Concurrency Issues**: Use proper locking and transactions
3. **Memory Leaks**: Check for circular references in Arc/Rc
4. **Performance Issues**: Use the query optimizer and asset cache

### Debugging

1. Use the `tracing` crate for logging:

```rust
use tracing::{debug, info, warn, error};

info!("Creating quiz: {}", quiz.id);
```

2. Enable debug mode in Tauri:

```json
"tauri": {
  "allowlist": {
    "all": true
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "identifier": "com.example.app",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  },
  "security": {
    "csp": null
  },
  "windows": [
    {
      "fullscreen": false,
      "resizable": true,
      "title": "app",
      "width": 800,
      "height": 600
    }
  ],
  "cli": {
    "description": "",
    "longDescription": "",
    "beforeHelp": "",
    "afterHelp": "",
    "args": []
  },
  "updater": {
    "active": false
  },
  "macOSPrivateApi": false,
  "windows": [
    {
      "label": "main",
      "url": "index.html",
      "width": 800,
      "height": 600,
      "resizable": true,
      "fullscreen": false,
      "decorations": true,
      "transparent": false,
      "alwaysOnTop": false,
      "focus": true,
      "skipTaskbar": false,
      "title": "App",
      "visible": true,
      "center": true
    }
  ]
}
```

## Conclusion

This developer guide provides an overview of the Quiz Module architecture, components, and extension points. For more detailed information, refer to the code documentation and comments in the source files.

For additional resources, check out the [Module README](../src-tauri/src/quiz/README.md), [Implementation Tracker](quiz-module-tracker.md), and [User Guide](quiz-module-user-guide.md).
