# Quiz Module Integration

_Last updated: 2025-04-18_

## Overview

This document outlines the plan for integrating quiz functionality into the Ordo project as a modular component. The implementation is based on migrating functionality from [Quenti](https://github.com/quenti-io/quenti) while maintaining Ordo's Rust/Haskell/Tauri/Leptos stack and modular architecture principles.

## Modular Quiz Integration Blueprint

```rust
// src-tauri/src/quiz/mod.rs
#[cfg(feature = "quiz-module")]
pub struct QuizEngine {
    core: Arc<QuizCore>,
    sync_adapter: QuizSyncAdapter,
    feature_flag: bool,
}

#[cfg(feature = "quiz-module")]
impl QuizEngine {
    pub fn new(config: &Config) -> Self {
        let core = QuizCore::with_storage(config.hybrid_store.clone());
        let sync_adapter = QuizSyncAdapter::new(config.sync_engine.clone());
        Self { core, sync_adapter, feature_flag: config.enable_quiz }
    }
}
```

## Core Architecture Integration

```rust
// src-tauri/src/quiz/mod.rs
#[derive(Clone, Serialize, Deserialize)]
pub struct QuizEngine {
    store: HybridStore,
    scheduler: Arc<Mutex<SpacedRepetitionScheduler>>,
    session_queue: mpsc::UnboundedSender<QuizSession>,
}

impl QuizEngine {
    pub fn new(store: HybridStore) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let scheduler = SpacedRepetitionScheduler::new(store.clone());
        
        tokio::spawn(async move {
            while let Some(session) = rx.recv().await {
                session.process().await;
            }
        });
        
        Self { store, scheduler: Arc::new(Mutex::new(scheduler)), session_queue: tx }
    }
}
```

## 1. Architecture Design Principles

### Modularity Matrix

| Layer | Implementation Strategy | Toggle Mechanism | Standalone Support |
|-------|-------------------------|------------------|-------------------|
| Database | Separate SQLite tables + Redb namespaces | Migration flags | Shared schema subset |
| Business Logic | Feature-gated Rust module | Cargo features | Conditional compilation |
| UI Components | Leptos feature flags | Build-time exclusion | Separate entrypoint |
| Sync | Custom operation types | Runtime config | Independent queue |
| API | Versioned endpoints | Middleware gates | Dedicated API gateway |

## 2. Database Migration Strategy

| Quenti MySQL Table | Ordo Equivalent | Storage Type | Notes |
|-------------------|-----------------|--------------|-------|
| StudySets | QuizBank | SQLite | Add offline_version column |
| Flashcards | QuizCard | SQLite | JSON schema for rich content |
| Folders | QuizCollection | SQLite | Nested collections support |
| UserPreferences | UserQuizPrefs | Redb | Ephemeral session state |
| QuizAttempts | QuizSession | Hybrid | SQLite for metadata, Redb for temp data |

```haskell
-- src/Quiz/Schema.hs
data QuizCard = QuizCard
  { question :: JSONB
  , answer :: JSONB
  , interval :: Int
  , dueDate :: UTCTime
  , efactor :: Double
  } deriving (Generic, SQL.ToRow, SQL.FromRow)
```

## 3. Core Implementation Strategy

### Feature Toggle Implementation

```toml
# Cargo.toml
[features]
quiz-module = [
  "tauri-plugin-quiz", 
  "leptos-quiz-components",
  "quiz-algorithms"
]

# Standalone mode
standalone-quiz = [
  "quiz-module", 
  "minimal-core",
  "quiz-webview"
]
```

### Conditional UI Rendering

```rust
// src/components/main_layout.rs
#[component]
pub fn AppShell() -> impl IntoView {
    view! {
        <Navigation/>
        <main>
            <Show when=move || cfg!(feature = "quiz-module")>
                <QuizDashboard/>
            </Show>
            <CourseView/>
        </main>
    }
}
```

## 4. Frontend Component Porting

### Leptos Components

```rust
// src/components/quiz/FlashcardViewer.leptos
#[component]
pub fn FlashcardViewer(card: ReadSignal<QuizCard>) -> impl IntoView {
    let (is_flipped, set_flipped) = create_signal(false);
    
    view! {
        <div class="flashcard" on:click=move |_| set_flipped.update(|f| *f = !*f)>
            <Show when=move || !is_flipped.get()>
                <RichContentView content=card.get().question/>
            </Show>
            <Show when=move || is_flipped.get()>
                <RichContentView content=card.get().answer/>
            </Show>
        </div>
    }
}
```

### Key Conversions

- React useState → Leptos create_signal
- React Router → Leptos Router
- Styled Components → Tailwind + Leptos class: binding

### Virtualized List Implementation

```rust
// src/components/quiz/QuizList.leptos
#[component]
pub fn QuizList(items: Vec<QuizSet>) -> impl IntoView {
    let (scroll_pos, set_scroll_pos) = create_signal(0.0);
    let container_ref = create_node_ref::<html::Div>();
    
    let visible_items = create_memo(move |_| {
        let container_height = container_ref.get().client_height() as f64;
        let item_height = 72.0;
        let start_idx = (scroll_pos.get() / item_height).floor() as usize;
        let end_idx = start_idx + (container_height / item_height).ceil() as usize;
        
        items.iter()
            .enumerate()
            .skip(start_idx)
            .take(end_idx - start_idx)
            .collect()
    });
    
    view! {
        <div class="quiz-list" node_ref=container_ref on:scroll=move |e| {
            set_scroll_pos.set(e.target().unwrap().scroll_top() as f64)
        }>
            <div style:height=move || format!("{}px", items.len() as f64 * 72.0)>
                <For each=visible_items key=|(idx, _)| *idx let:item>
                    <QuizListItem item=item.1.clone()/>
                </For>
            </div>
        </div>
    }
}
```

## 5. Database Integration

### Hybrid Storage Schema

```rust
// src-tauri/src/quiz/storage.rs
pub struct QuizStorage {
    // SQLite for structured data
    sqlite: SqlitePool,
    // Redb for session state
    redb: Database,
}

impl QuizStorage {
    pub fn store_card(&self, card: QuizCard) -> Result<()> {
        // SQLite insert
        sqlx::query!("INSERT INTO quiz_cards ...")?;
        
        // Redb session tracking
        let session_table = TableDefinition::<&str, &str>::new("quiz_sessions");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(session_table)?;
        table.insert(card.id, serde_json::to_string(&card.state)?)?;
        write_txn.commit()?;
        
        Ok(())
    }
}
```

## 6. Backend Service Migration

### Rust Implementation

```rust
// src-tauri/src/quiz/spaced_repetition.rs
pub struct SpacedRepetitionScheduler {
    store: HybridStore,
    algorithm: SM2,
}

impl SpacedRepetitionScheduler {
    pub fn next_review(&self, card: &QuizCard) -> Result<QuizCard> {
        let mut updated = card.clone();
        let (new_interval, new_ef) = self.algorithm.calculate(
            card.interval,
            card.efactor,
            performance_score
        );
        
        updated.interval = new_interval;
        updated.efactor = new_ef;
        updated.due_date = Utc::now() + Duration::days(new_interval);
        
        self.store.update_card(updated)
    }
}
```

### Haskell Type Safety

```haskell
-- src/Quiz/Algorithms/SM2.hs
data SM2Params = SM2Params
  { efactor :: Double
  , interval :: Int
  , performance :: Int
  } deriving (Generic, Show)

calculateSM2 :: SM2Params -> (Int, Double)
calculateSM2 params
  | performance < 3 = (1, max 1.3 (efactor - 0.2))
  | otherwise = (floor (fromIntegral interval * efactor), efactor + 0.1)
  where performance = params.performance
        efactor = params.efactor
        interval = params.interval
```

## 7. Sync Engine Extension

### Operation Type Registration

```rust
// src-tauri/src/sync/engine.rs
pub enum SyncOperation {
    #[cfg(feature = "quiz-module")]
    QuizUpdate(QuizSyncOp),
    CourseUpdate(CourseSyncOp),
    // ...
}

impl SyncEngine {
    pub fn register_quiz_handlers(&mut self) {
        #[cfg(feature = "quiz-module")]
        self.add_handler(QuizSyncHandler::new());
    }
}
```

### Offline-First Sync Implementation

```rust
// src-tauri/src/quiz/sync.rs
impl QuizSyncEngine {
    pub async fn queue_quiz_attempt(&self, attempt: QuizAttempt) -> Result<()> {
        self.store.enqueue_sync(
            SyncOperation::QuizUpdate(
                attempt.to_operation()?
            )
        ).await?;
        
        if self.is_online().await {
            self.process_queue().await?;
        }
        
        Ok(())
    }
    
    async fn process_queue(&self) -> Result<()> {
        while let Some(op) = self.store.next_sync_op().await? {
            match op {
                SyncOperation::QuizUpdate(update) => {
                    self.remote.apply_quiz_update(update).await?;
                    self.store.ack_sync_op(op.id).await?;
                }
                _ => continue
            }
        }
        Ok(())
    }
}
```

## 8. Standalone Mode Implementation

### Entrypoint Configuration

```rust
// src-tauri/src/main_standalone.rs
#[cfg(feature = "standalone-quiz")]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![quiz_commands])
        .run(tauri::generate_context!())
        .expect("error running standalone quiz");
}

// Shared core library
#[cfg(feature = "standalone-quiz")]
mod quiz_core {
    pub use ordo_quiz::*;
}
```

## 9. Security Implementation

### Encrypted Quiz Content

```rust
// src-tauri/src/quiz/security.rs
pub struct QuizEncryptor {
    key: Aes256GcmKey,
    nonce_generator: NonceSequence,
}

impl QuizEncryptor {
    pub fn encrypt_card(&self, card: &QuizCard) -> Result<Vec<u8>> {
        let plaintext = serde_json::to_vec(card)?;
        let ciphertext = self.key.encrypt(
            &mut self.nonce_generator,
            plaintext.as_ref()
        )?;
        
        Ok(ciphertext)
    }
    
    pub fn decrypt_card(&self, ciphertext: &[u8]) -> Result<QuizCard> {
        let plaintext = self.key.decrypt(
            &mut self.nonce_generator,
            ciphertext
        )?;
        
        Ok(serde_json::from_slice(&plaintext)?)
    }
}
```

## 10. Performance Optimization

### WASM Acceleration

```toml
# quiz-engine/Cargo.toml
[target.'cfg(target_family = "wasm")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["Window"] }
```

## 11. Testing Strategy

### Feature-Specific Test Suite

```rust
// tests/quiz_tests.rs
#[cfg(feature = "quiz-module")]
mod quiz_tests {
    #[test]
    fn test_spaced_repetition() {
        let engine = QuizEngine::new();
        assert_eq!(engine.next_interval(0), 1);
    }
}
```

## 12. Deployment Configuration

### CI/CD Pipeline Rules

```yaml
# .github/workflows/build.yml
jobs:
  build:
    strategy:
      matrix:
        features: ["default", "quiz-module", "standalone-quiz"]
    steps:
      - name: Build with features
        run: cargo build --features ${{ matrix.features }}
```

## Implementation Roadmap

### Phase 1: Core Functionality (6 Weeks)
- Port database schema to hybrid storage
- Implement basic quiz rendering components
- Setup spaced repetition scheduler
- Create offline sync infrastructure
- Implement feature-flagged quiz storage layer
- Create sync engine extension points
- Develop base Leptos components with conditional rendering
- Set up standalone entrypoint scaffolding

### Phase 2: Advanced Features (4 Weeks)
- Implement collaborative quizzing via Hydra channels
- Add rich content editor (LaTeX/Diagram support)
- Port analytics dashboard
- Set up automated test suite
- Port spaced repetition algorithm from Quenti
- Implement encrypted content storage
- Build quiz analytics dashboard
- Add mobile-optimized UI components

### Phase 3: Optimization (2 Weeks)
- WASM-accelerated crypto operations
- Lazy-load heavy dependencies
- Finalize Windows performance tuning
- Performance profiling and tuning
- Cross-platform testing suite
- Documentation finalization

## Critical Architectural Decisions

### Modular Boundaries
- SQLite schema versioning for backward compatibility
- API gateway pattern for standalone/service modes
- Config-driven feature enablement

### Performance Considerations
- Lazy-loading of quiz assets
- WASM-optimized crypto
- Hybrid sync queue prioritization
- Virtualized lists for large datasets

### Security Implementation
- Hardware-backed key storage
- Content encryption at rest
- Fine-grained access controls

### Extensibility Points
- Plugin system for question types
- Custom scoring rule hooks
- Alternative sync backend adapters

### State Management
- Use Leptos signals + Tauri store
- Content Rendering: WASM-based Markdown/LaTeX parser
- Real-time Updates: WebSocket integration via tokio-tungstenite
- AI Integration: Local LLM inference for smart suggestions
- Accessibility: Full ARIA support in Leptos components

## Conclusion

This implementation plan maintains Ordo's core architectural principles while adding quiz functionality through:

- **Compile-Time Modularity**: Feature flags reduce final binary size
- **Runtime Flexibility**: Config-driven enablement without recompilation
- **Shared Core Logic**: Unified codebase for app/standalone modes
- **Security Compliance**: Encrypted storage meets academic requirements
- **Performance Alignment**: WASM optimizations match existing UI standards
- **Offline Resilience**: Hybrid storage + sync queue
- **Type Safety**: Haskell validation layer
- **Extensibility**: Modular component design

The implementation leverages existing infrastructure (hybrid storage, sync engine) while introducing quiz-specific optimizations through conditional compilation, targeted async processing, virtualized lists, and WASM-accelerated content rendering.
