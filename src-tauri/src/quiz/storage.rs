use sqlx::{SqlitePool, sqlite::SqliteRow, Row, Error as SqlxError};
use redb::{Database, TableDefinition, Error as RedbError};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Arc;
use std::path::Path;
use chrono::{DateTime, Utc};

use super::models::{Quiz, Question, Answer, QuizSettings, StudyMode, QuizVisibility, FlashcardData};
use super::session::QuizSession;
use crate::core::config::Config;

// Custom error type for the hybrid store
#[derive(Debug)]
pub enum StoreError {
    Sqlx(SqlxError),
    Redb(RedbError),
    Serde(serde_json::Error),
    Encryption(String),
    NotFound(String),
    Other(String),
}

impl From<SqlxError> for StoreError {
    fn from(err: SqlxError) -> Self {
        StoreError::Sqlx(err)
    }
}

impl From<RedbError> for StoreError {
    fn from(err: RedbError) -> Self {
        StoreError::Redb(err)
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(err: serde_json::Error) -> Self {
        StoreError::Serde(err)
    }
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Sqlx(e) => write!(f, "SQLx error: {}", e),
            StoreError::Redb(e) => write!(f, "Redb error: {}", e),
            StoreError::Serde(e) => write!(f, "Serialization error: {}", e),
            StoreError::Encryption(e) => write!(f, "Encryption error: {}", e),
            StoreError::NotFound(e) => write!(f, "Not found: {}", e),
            StoreError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for StoreError {}

pub type Result<T> = std::result::Result<T, StoreError>;

// Simple encryption wrapper
pub struct ContentEncryption {
    key: Option<String>,
}

impl ContentEncryption {
    pub fn new(key: Option<&str>) -> Result<Self> {
        Ok(Self {
            key: key.map(|k| k.to_string()),
        })
    }

    pub fn encrypt_quiz(&self, quiz: &Quiz) -> Result<Vec<u8>> {
        let serialized = serde_json::to_vec(quiz)
            .map_err(|e| StoreError::Serde(e))?;

        // In a real implementation, this would use proper encryption
        // For now, just return the serialized data
        Ok(serialized)
    }

    pub fn decrypt_quiz(&self, data: &[u8]) -> Result<Quiz> {
        // In a real implementation, this would decrypt the data
        // For now, just deserialize
        serde_json::from_slice(data)
            .map_err(|e| StoreError::Serde(e))
    }
}

pub struct HybridQuizStore {
    sqlite: SqlitePool,
    redb: Database,
    encryption: Arc<ContentEncryption>,
    spaced_repetition_scheduler: Option<Arc<super::spaced_repetition::SpacedRepetitionScheduler>>,
}

impl HybridQuizStore {
    pub fn new(config: &Config) -> Result<Self> {
        // Create database directory if it doesn't exist
        let db_dir = Path::new(&config.database.path).parent()
            .ok_or_else(|| StoreError::Other("Invalid database path".to_string()))?;

        if !db_dir.exists() {
            std::fs::create_dir_all(db_dir)
                .map_err(|e| StoreError::Other(format!("Failed to create database directory: {}", e)))?;
        }

        // Use tokio runtime to create SQLite connection
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| StoreError::Other(format!("Failed to create runtime: {}", e)))?;

        let sqlite = rt.block_on(async {
            SqlitePool::connect(&config.database.path).await
                .map_err(|e| StoreError::Sqlx(e))
        })?;

        // Create Redb database for ephemeral data
        let redb_path = format!("{}_quiz_ephemeral.redb", config.database.path);
        let redb = Database::create(&redb_path)
            .map_err(|e| StoreError::Redb(e))?;

        let encryption = Arc::new(ContentEncryption::new(config.security.encryption_key.as_deref())?);

        // Initialize SQLite schema
        rt.block_on(async {
            sqlx::query(include_str!("../sql/quiz_schema.sql"))
                .execute(&sqlite)
                .await
                .map_err(|e| StoreError::Sqlx(e))
        })?;

        // Create store instance without scheduler first
        let store = Self {
            sqlite,
            redb,
            encryption,
            spaced_repetition_scheduler: None,
        };

        Ok(store)
    }

    pub async fn store_quiz(&self, quiz: &Quiz) -> Result<()> {
        let encrypted_content = self.encryption.encrypt_quiz(quiz)?;

        // Store basic quiz metadata in SQLite
        sqlx::query!(
            r#"
            INSERT INTO quizzes (id, title, description, created_at, updated_at, author_id, visibility, tags, study_mode)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            quiz.id.to_string(),
            quiz.title,
            quiz.description,
            quiz.created_at,
            quiz.updated_at,
            quiz.author_id.map(|id| id.to_string()),
            serde_json::to_string(&quiz.visibility)?,
            serde_json::to_string(&quiz.tags)?,
            serde_json::to_string(&quiz.study_mode)?
        )
        .execute(&self.sqlite)
        .await?;

        // Store quiz settings
        sqlx::query!(
            r#"
            INSERT INTO quiz_settings (quiz_id, shuffle_questions, time_limit, allow_retries, show_correct_answers, passing_score, study_mode)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            quiz.id.to_string(),
            quiz.settings.shuffle_questions,
            quiz.settings.time_limit,
            quiz.settings.allow_retries,
            quiz.settings.show_correct_answers,
            quiz.settings.passing_score,
            serde_json::to_string(&quiz.settings.study_mode)?
        )
        .execute(&self.sqlite)
        .await?;

        // Store questions
        for question in &quiz.questions {
            sqlx::query!(
                r#"
                INSERT INTO questions (id, quiz_id, content, answer_type, correct_answer, explanation)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                question.id.to_string(),
                quiz.id.to_string(),
                serde_json::to_string(&question.content)?,
                serde_json::to_string(&question.answer_type)?,
                serde_json::to_string(&question.correct_answer)?,
                question.explanation
            )
            .execute(&self.sqlite)
            .await?;

            // Store choices for multiple choice questions
            for choice in &question.choices {
                sqlx::query!(
                    r#"
                    INSERT INTO choices (id, question_id, text, rich_text, image_url)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    choice.id.to_string(),
                    question.id.to_string(),
                    choice.text,
                    choice.rich_text,
                    choice.image_url
                )
                .execute(&self.sqlite)
                .await?;
            }
        }

        // Store full quiz data in Redb for faster access
        let quiz_table = TableDefinition::<&str, &[u8]>::new("quizzes");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(quiz_table)?;
        table.insert(quiz.id.to_string().as_str(), &encrypted_content)?;
        write_txn.commit()?;

        Ok(())
    }

    pub async fn get_quiz(&self, quiz_id: Uuid) -> Result<Quiz> {
        // Try to get from Redb first (faster)
        let quiz_table = TableDefinition::<&str, &[u8]>::new("quizzes");
        let read_txn = self.redb.begin_read()?;
        let table = read_txn.open_table(quiz_table)?;

        if let Ok(encrypted_data) = table.get(quiz_id.to_string().as_str()) {
            return self.encryption.decrypt_quiz(encrypted_data.value());
        }

        // Fall back to SQLite if not in Redb
        let quiz_row = sqlx::query!(
            r#"
            SELECT id, title, description, created_at, updated_at, author_id, visibility, tags, study_mode
            FROM quizzes
            WHERE id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_one(&self.sqlite)
        .await?;

        let settings_row = sqlx::query!(
            r#"
            SELECT shuffle_questions, time_limit, allow_retries, show_correct_answers, passing_score, study_mode
            FROM quiz_settings
            WHERE quiz_id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_one(&self.sqlite)
        .await?;

        let questions = sqlx::query!(
            r#"
            SELECT id, content, answer_type, correct_answer, explanation
            FROM questions
            WHERE quiz_id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut quiz_questions = Vec::new();

        for q in questions {
            let question_id = Uuid::parse_str(&q.id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", q.id)))?;

            let choices = sqlx::query!(
                r#"
                SELECT id, text, rich_text, image_url
                FROM choices
                WHERE question_id = ?
                "#,
                question_id.to_string()
            )
            .fetch_all(&self.sqlite)
            .await?;

            let content: super::models::QuestionContent = serde_json::from_str(&q.content)?;
            let answer_type: super::models::AnswerType = serde_json::from_str(&q.answer_type)?;
            let correct_answer: super::models::Answer = serde_json::from_str(&q.correct_answer)?;

            let mut question_choices = Vec::new();

            for c in choices {
                question_choices.push(super::models::Choice {
                    id: Uuid::parse_str(&c.id)
                        .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", c.id)))?,
                    text: c.text,
                    rich_text: c.rich_text,
                    image_url: c.image_url,
                });
            }

            quiz_questions.push(super::models::Question {
                id: question_id,
                quiz_id,
                content,
                answer_type,
                choices: question_choices,
                correct_answer,
                explanation: q.explanation,
            });
        }

        let visibility: super::models::QuizVisibility = serde_json::from_str(&quiz_row.visibility)?;
        let tags: Vec<String> = serde_json::from_str(&quiz_row.tags)?;
        let study_mode: super::models::StudyMode = serde_json::from_str(&quiz_row.study_mode)?;
        let settings_study_mode: super::models::StudyMode = serde_json::from_str(&settings_row.study_mode)?;

        let quiz = super::models::Quiz {
            id: quiz_id,
            title: quiz_row.title,
            description: quiz_row.description,
            created_at: quiz_row.created_at.parse::<DateTime<Utc>>()
                .map_err(|_| StoreError::Other("Invalid datetime".to_string()))?,
            updated_at: quiz_row.updated_at.parse::<DateTime<Utc>>()
                .map_err(|_| StoreError::Other("Invalid datetime".to_string()))?,
            questions: quiz_questions,
            settings: super::models::QuizSettings {
                shuffle_questions: settings_row.shuffle_questions != 0,
                time_limit: if settings_row.time_limit > 0 { Some(settings_row.time_limit) } else { None },
                allow_retries: settings_row.allow_retries != 0,
                show_correct_answers: settings_row.show_correct_answers != 0,
                passing_score: if settings_row.passing_score > 0.0 { Some(settings_row.passing_score) } else { None },
                study_mode: settings_study_mode,
            },
            author_id: quiz_row.author_id.map(|id| Uuid::parse_str(&id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", id)))).transpose()?,
            visibility,
            tags,
            study_mode,
        };

        // Cache in Redb for faster access next time
        let encrypted_content = self.encryption.encrypt_quiz(&quiz)?;
        let quiz_table = TableDefinition::<&str, &[u8]>::new("quizzes");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(quiz_table)?;
        table.insert(quiz.id.to_string().as_str(), &encrypted_content)?;
        write_txn.commit()?;

        Ok(quiz)
    }

    pub async fn store_session(&self, session: &QuizSession) -> Result<()> {
        let serialized = serde_json::to_vec(session)?;

        let session_table = TableDefinition::<&str, &[u8]>::new("quiz_sessions");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(session_table)?;
        table.insert(session.id.to_string().as_str(), &serialized)?;
        write_txn.commit()?;

        Ok(())
    }

    pub async fn get_session(&self, session_id: Uuid) -> Result<QuizSession> {
        let session_table = TableDefinition::<&str, &[u8]>::new("quiz_sessions");
        let read_txn = self.redb.begin_read()?;
        let table = read_txn.open_table(session_table)?;

        let session_data = table.get(session_id.to_string().as_str())
            .map_err(|_| StoreError::NotFound(format!("Session not found: {}", session_id)))?;

        let session: QuizSession = serde_json::from_slice(session_data.value())?;

        Ok(session)
    }

    pub async fn update_session(&self, session: &QuizSession) -> Result<()> {
        self.store_session(session).await
    }

    pub async fn list_quizzes(&self, limit: usize, offset: usize) -> Result<Vec<Quiz>> {
        let quiz_ids = sqlx::query!("SELECT id FROM quizzes ORDER BY created_at DESC LIMIT ? OFFSET ?", limit as i64, offset as i64)
            .fetch_all(&self.sqlite)
            .await?;

        let mut quizzes = Vec::new();

        for row in quiz_ids {
            let id = Uuid::parse_str(&row.id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.id)))?;

            let quiz = self.get_quiz(id).await?;
            quizzes.push(quiz);
        }

        Ok(quizzes)
    }

    pub async fn delete_quiz(&self, quiz_id: Uuid) -> Result<()> {
        // Delete from SQLite
        sqlx::query!("DELETE FROM choices WHERE question_id IN (SELECT id FROM questions WHERE quiz_id = ?)", quiz_id.to_string())
            .execute(&self.sqlite)
            .await?;

        sqlx::query!("DELETE FROM questions WHERE quiz_id = ?", quiz_id.to_string())
            .execute(&self.sqlite)
            .await?;

        sqlx::query!("DELETE FROM quiz_settings WHERE quiz_id = ?", quiz_id.to_string())
            .execute(&self.sqlite)
            .await?;

        sqlx::query!("DELETE FROM quizzes WHERE id = ?", quiz_id.to_string())
            .execute(&self.sqlite)
            .await?;

        // Delete from Redb
        let quiz_table = TableDefinition::<&str, &[u8]>::new("quizzes");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(quiz_table)?;
        let _ = table.remove(quiz_id.to_string().as_str()); // Ignore if not found
        write_txn.commit()?;

        Ok(())
    }

    // Flashcard data methods

    /// Store flashcard data for a specific question and user
    pub async fn store_flashcard_data(&self, data: &FlashcardData) -> Result<()> {
        // Check if the flashcard data already exists
        let existing = sqlx::query!(
            r#"
            SELECT question_id FROM flashcard_data
            WHERE question_id = ? AND user_id = ?
            "#,
            data.question_id.to_string(),
            data.user_id.to_string()
        )
        .fetch_optional(&self.sqlite)
        .await?;

        if existing.is_some() {
            // Update existing record
            sqlx::query!(
                r#"
                UPDATE flashcard_data
                SET ease_factor = ?, interval = ?, repetitions = ?, due_date = ?, last_reviewed = ?
                WHERE question_id = ? AND user_id = ?
                "#,
                data.ease_factor,
                data.interval,
                data.repetitions,
                data.due_date,
                data.last_reviewed,
                data.question_id.to_string(),
                data.user_id.to_string()
            )
            .execute(&self.sqlite)
            .await?;
        } else {
            // Insert new record
            sqlx::query!(
                r#"
                INSERT INTO flashcard_data
                (question_id, user_id, ease_factor, interval, repetitions, due_date, last_reviewed)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
                data.question_id.to_string(),
                data.user_id.to_string(),
                data.ease_factor,
                data.interval,
                data.repetitions,
                data.due_date,
                data.last_reviewed
            )
            .execute(&self.sqlite)
            .await?;
        }

        // Also store in Redb for faster access
        let flashcard_table = TableDefinition::<&str, &[u8]>::new("flashcard_data");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(flashcard_table)?;

        // Use a composite key of question_id + user_id
        let key = format!("{}-{}", data.question_id, data.user_id);
        let serialized = serde_json::to_vec(data)?;

        table.insert(key.as_str(), &serialized)?;
        write_txn.commit()?;

        Ok(())
    }

    /// Get flashcard data for a specific question and user
    pub async fn get_flashcard_data(&self, question_id: Uuid, user_id: Uuid) -> Result<FlashcardData> {
        // Try to get from Redb first (faster)
        let flashcard_table = TableDefinition::<&str, &[u8]>::new("flashcard_data");
        let read_txn = self.redb.begin_read()?;

        if let Ok(table) = read_txn.open_table(flashcard_table) {
            let key = format!("{}-{}", question_id, user_id);

            if let Ok(data) = table.get(key.as_str()) {
                let flashcard_data: FlashcardData = serde_json::from_slice(data.value())?;
                return Ok(flashcard_data);
            }
        }

        // Fall back to SQLite
        let row = sqlx::query!(
            r#"
            SELECT question_id, user_id, ease_factor, interval, repetitions, due_date, last_reviewed
            FROM flashcard_data
            WHERE question_id = ? AND user_id = ?
            "#,
            question_id.to_string(),
            user_id.to_string()
        )
        .fetch_one(&self.sqlite)
        .await?;

        let flashcard_data = FlashcardData {
            question_id,
            user_id,
            ease_factor: row.ease_factor,
            interval: row.interval,
            repetitions: row.repetitions,
            due_date: row.due_date.parse::<DateTime<Utc>>()
                .map_err(|_| StoreError::Other("Invalid due_date datetime".to_string()))?,
            last_reviewed: row.last_reviewed.parse::<DateTime<Utc>>()
                .map_err(|_| StoreError::Other("Invalid last_reviewed datetime".to_string()))?,
        };

        // Cache in Redb for faster access next time
        let flashcard_table = TableDefinition::<&str, &[u8]>::new("flashcard_data");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(flashcard_table)?;

        let key = format!("{}-{}", question_id, user_id);
        let serialized = serde_json::to_vec(&flashcard_data)?;

        table.insert(key.as_str(), &serialized)?;
        write_txn.commit()?;

        Ok(flashcard_data)
    }

    /// Get flashcard data for a specific question (without user ID)
    pub async fn get_flashcard_data_by_question(&self, question_id: Uuid) -> Result<FlashcardData> {
        // This is a simplified version that gets the first flashcard data for a question
        // In a real implementation, you'd need to specify the user ID
        let row = sqlx::query!(
            r#"
            SELECT question_id, user_id, ease_factor, interval, repetitions, due_date, last_reviewed
            FROM flashcard_data
            WHERE question_id = ?
            LIMIT 1
            "#,
            question_id.to_string()
        )
        .fetch_one(&self.sqlite)
        .await?;

        let user_id = Uuid::parse_str(&row.user_id)
            .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.user_id)))?;

        let flashcard_data = FlashcardData {
            question_id,
            user_id,
            ease_factor: row.ease_factor,
            interval: row.interval,
            repetitions: row.repetitions,
            due_date: row.due_date.parse::<DateTime<Utc>>()
                .map_err(|_| StoreError::Other("Invalid due_date datetime".to_string()))?,
            last_reviewed: row.last_reviewed.parse::<DateTime<Utc>>()
                .map_err(|_| StoreError::Other("Invalid last_reviewed datetime".to_string()))?,
        };

        Ok(flashcard_data)
    }

    /// Get due flashcards for a user
    pub async fn get_due_flashcards(&self, user_id: Uuid, now: DateTime<Utc>, limit: usize) -> Result<Vec<FlashcardData>> {
        let rows = sqlx::query!(
            r#"
            SELECT question_id, user_id, ease_factor, interval, repetitions, due_date, last_reviewed
            FROM flashcard_data
            WHERE user_id = ? AND due_date <= ?
            ORDER BY due_date ASC
            LIMIT ?
            "#,
            user_id.to_string(),
            now,
            limit as i64
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut flashcards = Vec::new();

        for row in rows {
            let question_id = Uuid::parse_str(&row.question_id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.question_id)))?;

            flashcards.push(FlashcardData {
                question_id,
                user_id,
                ease_factor: row.ease_factor,
                interval: row.interval,
                repetitions: row.repetitions,
                due_date: row.due_date.parse::<DateTime<Utc>>()
                    .map_err(|_| StoreError::Other("Invalid due_date datetime".to_string()))?,
                last_reviewed: row.last_reviewed.parse::<DateTime<Utc>>()
                    .map_err(|_| StoreError::Other("Invalid last_reviewed datetime".to_string()))?,
            });
        }

        Ok(flashcards)
    }

    /// Get all flashcard data for a user
    pub async fn get_all_flashcard_data(&self, user_id: Uuid) -> Result<Vec<FlashcardData>> {
        let rows = sqlx::query!(
            r#"
            SELECT question_id, user_id, ease_factor, interval, repetitions, due_date, last_reviewed
            FROM flashcard_data
            WHERE user_id = ?
            "#,
            user_id.to_string()
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut flashcards = Vec::new();

        for row in rows {
            let question_id = Uuid::parse_str(&row.question_id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.question_id)))?;

            flashcards.push(FlashcardData {
                question_id,
                user_id,
                ease_factor: row.ease_factor,
                interval: row.interval,
                repetitions: row.repetitions,
                due_date: row.due_date.parse::<DateTime<Utc>>()
                    .map_err(|_| StoreError::Other("Invalid due_date datetime".to_string()))?,
                last_reviewed: row.last_reviewed.parse::<DateTime<Utc>>()
                    .map_err(|_| StoreError::Other("Invalid last_reviewed datetime".to_string()))?,
            });
        }

        Ok(flashcards)
    }

    /// Set the spaced repetition scheduler
    pub fn set_spaced_repetition_scheduler(&mut self, scheduler: Arc<super::spaced_repetition::SpacedRepetitionScheduler>) {
        self.spaced_repetition_scheduler = Some(scheduler);
    }

    /// Get the spaced repetition scheduler
    pub fn get_spaced_repetition_scheduler(&self) -> Option<Arc<super::spaced_repetition::SpacedRepetitionScheduler>> {
        self.spaced_repetition_scheduler.clone()
    }

    /// Get a specific question by ID
    pub async fn get_question(&self, question_id: Uuid) -> Result<Question> {
        let q = sqlx::query!(
            r#"
            SELECT quiz_id, content, answer_type, correct_answer, explanation
            FROM questions
            WHERE id = ?
            "#,
            question_id.to_string()
        )
        .fetch_one(&self.sqlite)
        .await?;

        let quiz_id = Uuid::parse_str(&q.quiz_id)
            .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", q.quiz_id)))?;

        let choices = sqlx::query!(
            r#"
            SELECT id, text, rich_text, image_url
            FROM choices
            WHERE question_id = ?
            "#,
            question_id.to_string()
        )
        .fetch_all(&self.sqlite)
        .await?;

        let content: super::models::QuestionContent = serde_json::from_str(&q.content)?;
        let answer_type: super::models::AnswerType = serde_json::from_str(&q.answer_type)?;
        let correct_answer: super::models::Answer = serde_json::from_str(&q.correct_answer)?;

        let mut question_choices = Vec::new();

        for c in choices {
            question_choices.push(super::models::Choice {
                id: Uuid::parse_str(&c.id)
                    .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", c.id)))?,
                text: c.text,
                rich_text: c.rich_text,
                image_url: c.image_url,
            });
        }

        Ok(Question {
            id: question_id,
            quiz_id,
            content,
            answer_type,
            choices: question_choices,
            correct_answer,
            explanation: q.explanation,
        })
    }

    // Analytics-related methods

    /// Get all quiz attempts for a user within a time range
    pub async fn get_user_quiz_attempts(&self, user_id: Uuid, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<super::models::QuizAttempt>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, user_id, started_at, completed_at, score, time_spent
            FROM quiz_attempts
            WHERE user_id = ? AND started_at >= ? AND started_at <= ?
            ORDER BY started_at DESC
            "#,
            user_id.to_string(),
            start_date,
            end_date
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut attempts = Vec::new();

        for row in rows {
            let id = Uuid::parse_str(&row.id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.id)))?;

            let quiz_id = Uuid::parse_str(&row.quiz_id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.quiz_id)))?;

            attempts.push(super::models::QuizAttempt {
                id,
                quiz_id,
                user_id,
                started_at: row.started_at.parse::<DateTime<Utc>>()
                    .map_err(|_| StoreError::Other("Invalid started_at datetime".to_string()))?,
                completed_at: if let Some(completed_at) = row.completed_at {
                    Some(completed_at.parse::<DateTime<Utc>>()
                        .map_err(|_| StoreError::Other("Invalid completed_at datetime".to_string()))?)
                } else {
                    None
                },
                score: row.score,
                time_spent: row.time_spent,
                answers: Vec::new(), // We'll load these separately if needed
            });
        }

        Ok(attempts)
    }

    /// Get all quiz attempts for a quiz within a time range
    pub async fn get_quiz_attempts(&self, quiz_id: Uuid, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<super::models::QuizAttempt>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, user_id, started_at, completed_at, score, time_spent
            FROM quiz_attempts
            WHERE quiz_id = ? AND started_at >= ? AND started_at <= ?
            ORDER BY started_at DESC
            "#,
            quiz_id.to_string(),
            start_date,
            end_date
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut attempts = Vec::new();

        for row in rows {
            let id = Uuid::parse_str(&row.id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.id)))?;

            let user_id = Uuid::parse_str(&row.user_id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.user_id)))?;

            attempts.push(super::models::QuizAttempt {
                id,
                quiz_id,
                user_id,
                started_at: row.started_at.parse::<DateTime<Utc>>()
                    .map_err(|_| StoreError::Other("Invalid started_at datetime".to_string()))?,
                completed_at: if let Some(completed_at) = row.completed_at {
                    Some(completed_at.parse::<DateTime<Utc>>()
                        .map_err(|_| StoreError::Other("Invalid completed_at datetime".to_string()))?)
                } else {
                    None
                },
                score: row.score,
                time_spent: row.time_spent,
                answers: Vec::new(), // We'll load these separately if needed
            });
        }

        Ok(attempts)
    }

    /// Get answers for a specific quiz attempt
    pub async fn get_attempt_answers(&self, attempt_id: Uuid) -> Result<Vec<super::models::QuestionAnswer>> {
        let rows = sqlx::query!(
            r#"
            SELECT question_id, answer, is_correct, time_spent
            FROM question_answers
            WHERE attempt_id = ?
            "#,
            attempt_id.to_string()
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut answers = Vec::new();

        for row in rows {
            let question_id = Uuid::parse_str(&row.question_id)
                .map_err(|_| StoreError::Other(format!("Invalid UUID: {}", row.question_id)))?;

            let answer: super::models::Answer = serde_json::from_str(&row.answer)?;

            answers.push(super::models::QuestionAnswer {
                question_id,
                answer,
                is_correct: if let Some(is_correct) = row.is_correct {
                    Some(is_correct != 0)
                } else {
                    None
                },
                time_spent: row.time_spent,
            });
        }

        Ok(answers)
    }

    /// Get all study dates for a user (for calculating streaks)
    pub async fn get_user_study_dates(&self, user_id: Uuid) -> Result<Vec<DateTime<Utc>>> {
        let rows = sqlx::query!(
            r#"
            SELECT DISTINCT date(started_at) as study_date
            FROM quiz_attempts
            WHERE user_id = ?
            ORDER BY study_date DESC
            "#,
            user_id.to_string()
        )
        .fetch_all(&self.sqlite)
        .await?;

        let mut dates = Vec::new();

        for row in rows {
            // Parse the date string into a DateTime<Utc>
            // The date is in the format YYYY-MM-DD
            let date_str = format!("{} 00:00:00Z", row.study_date);
            let date = DateTime::parse_from_rfc3339(&date_str)
                .map_err(|_| StoreError::Other(format!("Invalid date: {}", row.study_date)))?;

            dates.push(date.with_timezone(&Utc));
        }

        Ok(dates)
    }
}