# Ordo Quiz Module - Technical Reference

This document provides detailed technical information about the Ordo Quiz module implementation for developers.

## Database Schema

### quizzes

The main table for storing quiz data:

```sql
CREATE TABLE IF NOT EXISTS quizzes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    course_id TEXT,
    author_id TEXT NOT NULL,
    time_limit INTEGER, -- in seconds, NULL means no limit
    passing_score INTEGER, -- percentage
    shuffle_questions INTEGER DEFAULT 0,
    show_results INTEGER DEFAULT 1,
    visibility TEXT NOT NULL DEFAULT 'private', -- 'private', 'public', 'course'
    tags TEXT, -- JSON array of tags
    study_mode TEXT NOT NULL DEFAULT 'multiple_choice', -- 'multiple_choice', 'flashcard', 'adaptive'
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT,
    FOREIGN KEY (course_id) REFERENCES courses(id),
    FOREIGN KEY (author_id) REFERENCES users(id)
);
```

### questions

```sql
CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    content TEXT NOT NULL, -- JSON content with text, rich_text, and media
    question_type TEXT NOT NULL, -- 'multiple_choice', 'true_false', 'short_answer', 'matching', 'essay'
    points INTEGER DEFAULT 1,
    position INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
);
```

### answer_options

```sql
CREATE TABLE IF NOT EXISTS answer_options (
    id TEXT PRIMARY KEY,
    question_id TEXT NOT NULL,
    content TEXT NOT NULL, -- JSON content with text, rich_text, and media
    is_correct INTEGER DEFAULT 0,
    position INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE
);
```

### quiz_settings

```sql
CREATE TABLE IF NOT EXISTS quiz_settings (
    quiz_id TEXT PRIMARY KEY,
    allow_retakes INTEGER DEFAULT 1,
    max_attempts INTEGER,
    show_correct_answers INTEGER DEFAULT 1,
    show_correct_answers_after_completion INTEGER DEFAULT 1,
    time_limit INTEGER, -- in seconds
    passing_score INTEGER, -- percentage
    shuffle_questions INTEGER DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
);
```

### quiz_attempts

```sql
CREATE TABLE IF NOT EXISTS quiz_attempts (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    start_time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TEXT,
    score REAL,
    status TEXT DEFAULT 'in_progress', -- 'in_progress', 'completed', 'abandoned'
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### user_answers

```sql
CREATE TABLE IF NOT EXISTS user_answers (
    id TEXT PRIMARY KEY,
    attempt_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    answer_option_id TEXT,
    text_answer TEXT,
    is_correct INTEGER,
    points_awarded REAL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (attempt_id) REFERENCES quiz_attempts(id) ON DELETE CASCADE,
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (answer_option_id) REFERENCES answer_options(id)
);
```

### cmi5_sessions

```sql
CREATE TABLE IF NOT EXISTS cmi5_sessions (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    session_id TEXT NOT NULL UNIQUE,
    registration_id TEXT NOT NULL,
    actor_json TEXT NOT NULL,
    activity_id TEXT NOT NULL,
    return_url TEXT,
    status TEXT DEFAULT 'initialized', -- 'initialized', 'launched', 'in_progress', 'completed', 'passed', 'failed', 'abandoned', 'waived'
    score REAL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### quiz_activities

Table for tracking user activities in the quiz module:

```sql
CREATE TABLE IF NOT EXISTS quiz_activities (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    quiz_id TEXT,
    question_id TEXT,
    attempt_id TEXT,
    activity_type TEXT NOT NULL,
    data TEXT, -- JSON data
    duration_ms INTEGER,
    timestamp TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    synced INTEGER DEFAULT 0,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (attempt_id) REFERENCES quiz_attempts(id)
);

CREATE INDEX IF NOT EXISTS idx_quiz_activities_user_id ON quiz_activities(user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_activities_quiz_id ON quiz_activities(quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_activities_timestamp ON quiz_activities(timestamp);
CREATE INDEX IF NOT EXISTS idx_quiz_activities_synced ON quiz_activities(synced);
```

### quiz_sync_items

Table for tracking sync items between standalone and main app:

```sql
CREATE TABLE IF NOT EXISTS quiz_sync_items (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'create', 'update', 'delete'
    data TEXT NOT NULL, -- JSON data
    priority TEXT NOT NULL, -- 'critical', 'high', 'medium', 'low'
    status TEXT NOT NULL, -- 'pending', 'in_progress', 'completed', 'failed'
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_at TEXT,
    error TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_status ON quiz_sync_items(status);
CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_entity ON quiz_sync_items(entity_type, entity_id);
```

## Model Definitions

### Quiz

```rust
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub course_id: Option<String>,
    pub author_id: String,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: bool,
    pub show_results: bool,
    pub visibility: Option<QuizVisibility>,
    pub tags: Option<String>, // JSON array of tags
    pub study_mode: Option<StudyMode>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub enum QuizVisibility {
    Private,
    Public,
    Course,
}

pub enum StudyMode {
    MultipleChoice,
    Flashcard,
    Adaptive,
}
```

### Question

```rust
pub struct Question {
    pub id: String,
    pub quiz_id: String,
    pub question_text: String,
    pub content: Option<String>, // JSON content with text, rich_text, and media
    pub question_type: QuestionType,
    pub points: i64,
    pub position: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Matching,
    Essay,
}
```

### AnswerOption

```rust
pub struct AnswerOption {
    pub id: String,
    pub question_id: String,
    pub option_text: String,
    pub content: Option<String>, // JSON content with text, rich_text, and media
    pub is_correct: bool,
    pub position: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}
```

### QuizAttempt

```rust
pub struct QuizAttempt {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub score: Option<f64>,
    pub status: AttemptStatus,
    pub created_at: String,
    pub updated_at: String,
}

pub enum AttemptStatus {
    InProgress,
    Completed,
    Abandoned,
}
```

### QuizSettings

```rust
pub struct QuizSettings {
    pub quiz_id: String, // Primary key
    pub allow_retakes: bool,
    pub max_attempts: Option<i64>,
    pub show_correct_answers: bool,
    pub show_correct_answers_after_completion: bool,
    pub time_limit: Option<i64>,
    pub passing_score: Option<i64>,
    pub shuffle_questions: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
}
```

### Cmi5Session

```rust
pub struct Cmi5Session {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub session_id: String,
    pub registration_id: String,
    pub actor_json: String,
    pub activity_id: String,
    pub return_url: Option<String>,
    pub status: Cmi5SessionStatus,
    pub score: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

pub enum Cmi5SessionStatus {
    Initialized,
    Launched,
    InProgress,
    Completed,
    Passed,
    Failed,
    Abandoned,
    Waived,
}
```

## Repository Methods

The `QuizRepository` provides the following key methods:

### Quiz Methods

- `get_quizzes()`: Retrieves all quizzes
- `get_quiz_by_id(id: &str)`: Retrieves a quiz by ID
- `create_quiz(user_id: &str, quiz: CreateQuizRequest)`: Creates a new quiz
- `update_quiz(id: &str, quiz: UpdateQuizRequest)`: Updates an existing quiz
- `delete_quiz(id: &str)`: Soft-deletes a quiz
- `get_quiz_summaries()`: Retrieves summaries of all quizzes

### Question Methods

- `get_questions_by_quiz_id(quiz_id: &str)`: Retrieves all questions for a quiz
- `get_question_by_id(id: &str)`: Retrieves a question by ID
- `create_question(question: CreateQuestionRequest)`: Creates a new question
- `update_question(id: &str, question: UpdateQuestionRequest)`: Updates an existing question
- `delete_question(id: &str)`: Deletes a question
- `get_question_with_answers(id: &str)`: Retrieves a question with its answer options

### Answer Methods

- `get_answer_options_by_question_id(question_id: &str)`: Retrieves all answer options for a question
- `create_answer_option(option: CreateAnswerOptionRequest)`: Creates a new answer option
- `update_answer_option(id: &str, option: UpdateAnswerOptionRequest)`: Updates an existing answer option
- `delete_answer_option(id: &str)`: Deletes an answer option

### Attempt Methods

- `start_quiz_attempt(user_id: &str, request: StartAttemptRequest)`: Starts a new quiz attempt
- `complete_quiz_attempt(attempt_id: &str)`: Completes a quiz attempt
- `abandon_quiz_attempt(attempt_id: &str)`: Abandons a quiz attempt
- `get_quiz_attempts_by_user(user_id: &str)`: Retrieves all attempts for a user
- `get_quiz_attempts_by_quiz_id(quiz_id: &str)`: Retrieves all attempts for a quiz
- `get_quiz_attempt(attempt_id: &str)`: Retrieves an attempt by ID

### Settings Methods

- `get_quiz_settings(quiz_id: &str)`: Retrieves settings for a quiz
- `create_quiz_settings(settings: CreateQuizSettingsRequest)`: Creates settings for a quiz
- `update_quiz_settings(quiz_id: &str, settings: UpdateQuizSettingsRequest)`: Updates settings for a quiz

### CMI5 Methods

- `create_cmi5_session(user_id: &str, request: CreateCmi5SessionRequest)`: Creates a new CMI5 session
- `get_cmi5_session(session_id: &str)`: Retrieves a CMI5 session by ID
- `update_cmi5_session_status(session_id: &str, status: Cmi5SessionStatus, score: Option<f64>)`: Updates a CMI5 session
- `get_cmi5_sessions_by_user(user_id: &str)`: Retrieves all CMI5 sessions for a user

## Analytics Dashboard

The analytics dashboard provides a comprehensive view of quiz activity data for instructors.

### Dashboard Components

```rust
pub struct DashboardData {
    pub activity_stats: QuizActivityStats,
    pub quiz_summaries: Vec<QuizSummaryData>,
    pub user_summaries: Vec<UserSummaryData>,
    pub recent_activities: Vec<ActivityData>,
    pub time_analysis: TimeAnalysisData,
}
```

### Chart Types

```rust
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Doughnut,
    Radar,
    PolarArea,
    Bubble,
    Scatter,
}
```

### Chart Data

```rust
pub struct ChartData {
    pub chart_type: ChartType,
    pub title: String,
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
    pub x_axis_label: Option<String>,
    pub y_axis_label: Option<String>,
}
```

### Report Types

```rust
pub enum ReportType {
    UserActivity,
    QuizPerformance,
    QuestionAnalysis,
    TimeAnalysis,
    Custom,
}
```

### Report Data

```rust
pub struct ReportData {
    pub report_type: ReportType,
    pub title: String,
    pub description: String,
    pub generated_at: String,
    pub data: serde_json::Value,
}
```

## Activity Models

### ActivityType

```rust
pub enum ActivityType {
    QuizStarted,
    QuizCompleted,
    QuizAbandoned,
    QuestionAnswered,
    FlashcardViewed,
    FlashcardFlipped,
    FlashcardRated,
    StudySessionStarted,
    StudySessionEnded,
    ContentViewed,
    Custom(String),
}
```

### QuizActivity

```rust
pub struct QuizActivity {
    pub id: String,
    pub user_id: String,
    pub quiz_id: Option<String>,
    pub question_id: Option<String>,
    pub attempt_id: Option<String>,
    pub activity_type: String, // Stored as string in DB
    pub data: Option<String>,  // JSON data
    pub duration_ms: Option<i64>,
    pub timestamp: String,
    pub created_at: String,
    pub synced: bool,
}
```

### QuizActivitySummary

```rust
pub struct QuizActivitySummary {
    pub user_id: String,
    pub quiz_id: Option<String>,
    pub total_activities: i64,
    pub total_duration_ms: i64,
    pub activity_counts: serde_json::Value, // Map of activity types to counts
    pub first_activity_at: String,
    pub last_activity_at: String,
}
```

### QuizActivityStats

```rust
pub struct QuizActivityStats {
    pub total_quizzes_started: i64,
    pub total_quizzes_completed: i64,
    pub total_questions_answered: i64,
    pub total_study_time_ms: i64,
    pub average_quiz_duration_ms: Option<i64>,
    pub average_question_time_ms: Option<i64>,
    pub activity_by_day: serde_json::Value, // Map of days to activity counts
}
```

## Sync Models

### SyncItem

```rust
pub struct SyncItem {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: SyncOperation,
    pub data: serde_json::Value,
    pub priority: SyncPriority,
    pub status: SyncStatus,
    pub created_at: String,
    pub updated_at: String,
    pub synced_at: Option<String>,
    pub error: Option<String>,
    pub retry_count: i32,
}

pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

pub enum SyncPriority {
    Low,
    Medium,
    High,
    Critical,
}

pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
```

## AppState Integration

The Ordo Quiz module integrates with the main app through the `AppState` struct:

```rust
pub struct AppState {
    pub db_pool: SqlitePool,
    pub jwt_secret: Vec<u8>,
    pub quiz_repository: Option<Arc<QuizRepository>>,
    pub quiz_service: Option<Arc<QuizService>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, jwt_secret: Vec<u8>, data_dir: PathBuf) -> Self {
        Self {
            db_pool,
            jwt_secret,
            quiz_repository: None,
            quiz_service: None,
            data_dir,
        }
    }

    pub fn with_quiz_repository(mut self) -> Self {
        let repository = QuizRepository::new(self.db_pool.clone());
        self.quiz_repository = Some(Arc::new(repository));
        self
    }

    pub fn get_quiz_repository(&self) -> Arc<QuizRepository> {
        self.quiz_repository.clone().expect("Quiz repository not initialized")
    }

    pub async fn with_quiz_service(mut self) -> Result<Self> {
        let service = QuizService::new(self.db_pool.clone(), self.data_dir.clone());
        let mut service = service;
        service.initialize().await?;
        self.quiz_service = Some(Arc::new(service));
        Ok(self)
    }

    pub fn get_quiz_service(&self) -> Result<Arc<QuizService>> {
        self.quiz_service.clone().ok_or_else(|| anyhow!("Quiz service not initialized"))
    }
}
```

## Standalone Binary

The standalone binary is defined in `src-tauri/src/bin/quiz_standalone.rs`:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting Ordo Quiz...");

    // Initialize database connection
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ordo_quiz.db?mode=rwc".to_string());

    info!("Connecting to database: {}", db_url);
    let db_pool = SqlitePool::connect(&db_url).await?;

    // Initialize quiz database
    info!("Initializing quiz database...");
    match lms_lib::database::init_quiz_db::init_quiz_db(&db_pool).await {
        Ok(_) => info!("Quiz database initialized successfully"),
        Err(e) => error!("Failed to initialize quiz database: {}", e),
    }

    // Create test data if needed
    match lms_lib::database::init_quiz_db::check_quiz_tables(&db_pool).await {
        Ok(true) => {
            info!("Quiz tables exist, creating test data if needed...");
            if let Err(e) = lms_lib::database::init_quiz_db::create_test_data(&db_pool).await {
                error!("Failed to create test data: {}", e);
            }
        },
        Ok(false) => error!("Quiz tables do not exist"),
        Err(e) => error!("Failed to check quiz tables: {}", e),
    }

    // Create data directory
    let data_dir = PathBuf::from("ordo_quiz_data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    // Create sync directory
    let sync_dir = data_dir.join("sync");
    if !sync_dir.exists() {
        fs::create_dir_all(&sync_dir)?;
    }

    // Create app state
    let app_state = AppState::new(db_pool, "ordo_quiz_secret_key".as_bytes().to_vec(), data_dir.clone())
        .with_quiz_repository();

    // Initialize quiz service
    let app_state = match app_state.with_quiz_service().await {
        Ok(state) => Arc::new(state),
        Err(e) => {
            error!("Failed to initialize quiz service: {}", e);
            return Err(e);
        }
    };

    // Check for sync file from main app
    let main_app_sync_path = PathBuf::from("main_app_sync.json");
    if main_app_sync_path.exists() {
        info!("Found sync file from main app, syncing...");
        if let Ok(quiz_service) = app_state.get_quiz_service() {
            match quiz_service.sync_with_main_app(&main_app_sync_path).await {
                Ok(_) => info!("Synced with main app successfully"),
                Err(e) => error!("Failed to sync with main app: {}", e),
            }
        }
    }

    // Start the application
    info!("Ordo Quiz is running!");
    info!("Press Ctrl+C to exit");

    // Keep the application running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Ordo Quiz...");

    // Export sync data before shutdown
    if let Ok(quiz_service) = app_state.get_quiz_service() {
        let export_path = sync_dir.join("standalone_sync.json");
        match quiz_service.shutdown().await {
            Ok(_) => info!("Quiz service shut down successfully"),
            Err(e) => error!("Failed to shut down quiz service: {}", e),
        }
        info!("Exported sync data to {}", export_path.display());
    }

    Ok(())
}
```

## Database Initialization

The database initialization is handled by the `init_quiz_db.rs` module:

```rust
pub async fn init_quiz_db(pool: &SqlitePool) -> Result<()> {
    info!("Initializing quiz database...");

    // Apply migrations
    apply_quiz_migrations(pool).await?;

    info!("Quiz database initialization complete");
    Ok(())
}

async fn apply_quiz_migrations(pool: &SqlitePool) -> Result<()> {
    // Check if migrations directory exists
    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        error!("Migrations directory not found");
        return Err(anyhow!("Migrations directory not found"));
    }

    // Apply quiz schema migration
    let migration_path = migrations_dir.join("20240421_ordo_quiz_schema.sql");
    if migration_path.exists() {
        info!("Applying quiz schema migration: {:?}", migration_path);
        let sql = std::fs::read_to_string(&migration_path)?;
        sqlx::query(&sql).execute(pool).await?;
    } else {
        error!("Quiz schema migration not found: {:?}", migration_path);
        return Err(anyhow!("Quiz schema migration not found"));
    }

    Ok(())
}
```

## Sync Service

The sync service is responsible for synchronizing data between the standalone app and the main app. It provides methods for queuing, processing, exporting, and importing sync items.

### Key Methods

```rust
pub struct QuizSyncService {
    repository: Arc<QuizRepository>,
    sync_dir: PathBuf,
}

impl QuizSyncService {
    /// Create a new QuizSyncService
    pub fn new(repository: Arc<QuizRepository>, data_dir: &Path) -> Result<Self>;

    /// Queue an item for sync
    pub async fn queue_sync_item(
        &self,
        entity_type: &str,
        entity_id: &str,
        operation: SyncOperation,
        data: serde_json::Value,
        priority: SyncPriority,
    ) -> Result<String>;

    /// Get pending sync items
    pub async fn get_pending_sync_items(&self) -> Result<Vec<SyncItem>>;

    /// Process sync items
    pub async fn process_sync_items(&self) -> Result<usize>;

    /// Export sync data to a file
    pub async fn export_sync_data(&self, path: &Path) -> Result<()>;

    /// Import sync data from a file
    pub async fn import_sync_data(&self, path: &Path) -> Result<usize>;

    /// Initialize the sync database
    pub async fn init_sync_db(&self) -> Result<()>;
}
```

### Sync Process Flow

1. Changes are tracked and queued using `queue_sync_item`
2. Pending items can be processed using `process_sync_items`
3. Sync data can be exported to a file using `export_sync_data`
4. Sync data can be imported from a file using `import_sync_data`

## CMI5 Integration

The Ordo Quiz module integrates with CMI5 through the `Cmi5Session` model and related repository methods. The integration follows the CMI5 specification for tracking learning activities.

### CMI5 Session States

The CMI5 session can be in one of the following states:

1. **Initialized**: The session has been created but not yet launched
2. **Launched**: The session has been launched but the learner has not yet started
3. **InProgress**: The learner is actively taking the quiz
4. **Completed**: The learner has completed the quiz
5. **Passed**: The learner has passed the quiz
6. **Failed**: The learner has failed the quiz
7. **Abandoned**: The learner has abandoned the quiz
8. **Waived**: The quiz has been waived for the learner

### CMI5 Data Model

The CMI5 data model includes:

- **Actor**: The learner taking the quiz
- **Activity**: The quiz being taken
- **Registration**: The registration ID for the session
- **Session**: The session ID for the attempt
- **Score**: The score achieved by the learner
- **Status**: The current status of the session

## Future Development

### Planned Enhancements

1. **Rich Content Editor**: Add a rich content editor for questions and answers
2. **Advanced Question Types**: Add support for more complex question types
3. **Analytics Dashboard**: Add a dashboard for viewing quiz analytics
4. **Mobile Optimization**: Improve the mobile experience
5. **Offline Sync**: Enhance offline functionality with sync

### Technical Debt

1. **Test Coverage**: Increase test coverage for the repository
2. **Error Handling**: Improve error handling and reporting
3. **Documentation**: Add more detailed documentation for the API
4. **Performance Optimization**: Optimize database queries for large datasets
