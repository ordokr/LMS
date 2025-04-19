use sqlx::{SqlitePool, Row};
use crate::models::quiz::{
    Quiz, CreateQuizRequest, UpdateQuizRequest, QuizSummary,
    Question, CreateQuestionRequest, UpdateQuestionRequest, QuestionWithAnswers,
    AnswerOption, CreateAnswerOptionRequest, UpdateAnswerOptionRequest,
    QuizAttempt, StartAttemptRequest, AttemptStatus, AttemptResult,
    QuizSettings, CreateQuizSettingsRequest, UpdateQuizSettingsRequest,
    Cmi5Session, CreateCmi5SessionRequest, Cmi5SessionStatus,
    QuizVisibility, StudyMode,
    QuizActivity, CreateQuizActivityRequest, QuizActivitySummary, QuizActivityStats, ActivityType,
};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use chrono::Utc;
use tracing::{info, error};

pub struct QuizRepository {
    pool: SqlitePool,
}

impl QuizRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get the database pool
    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }

    // Quiz methods
    pub async fn get_quizzes(&self) -> Result<Vec<Quiz>> {
        let quizzes = sqlx::query_as!(
            Quiz,
            r#"
            SELECT
                id, title, description, course_id, author_id,
                time_limit, passing_score,
                shuffle_questions as "shuffle_questions: bool",
                show_results as "show_results: bool",
                visibility as "visibility: Option<QuizVisibility>",
                tags,
                study_mode as "study_mode: Option<StudyMode>",
                created_at, updated_at, deleted_at
            FROM quizzes
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(quizzes)
    }

    pub async fn get_quiz_by_id(&self, id: &str) -> Result<Quiz> {
        let quiz = sqlx::query_as!(
            Quiz,
            r#"
            SELECT
                id, title, description, course_id, author_id,
                time_limit, passing_score,
                shuffle_questions as "shuffle_questions: bool",
                show_results as "show_results: bool",
                visibility as "visibility: Option<QuizVisibility>",
                tags,
                study_mode as "study_mode: Option<StudyMode>",
                created_at, updated_at, deleted_at
            FROM quizzes
            WHERE id = ? AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Quiz not found"))?;

        Ok(quiz)
    }

    pub async fn create_quiz(&self, user_id: &str, quiz: CreateQuizRequest) -> Result<String> {
        let mut tx = self.pool.begin().await?;

        let quiz_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // Convert tags to JSON string
        let tags_json = match &quiz.tags {
            Some(tags) => serde_json::to_string(tags)?,
            None => "[]".to_string(),
        };

        sqlx::query!(
            r#"
            INSERT INTO quizzes (
                id, title, description, course_id, author_id,
                time_limit, passing_score, shuffle_questions, show_results,
                visibility, tags, study_mode,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            quiz_id,
            quiz.title,
            quiz.description,
            quiz.course_id,
            user_id,
            quiz.time_limit,
            quiz.passing_score,
            quiz.shuffle_questions.unwrap_or(false),
            quiz.show_results.unwrap_or(true),
            quiz.visibility.map(|v| format!("{:?}", v).to_lowercase()),
            tags_json,
            quiz.study_mode.map(|m| format!("{:?}", m).to_lowercase()),
            now,
            now
        )
        .execute(&mut *tx)
        .await?;

        // Create default settings
        sqlx::query!(
            r#"
            INSERT INTO quiz_settings (
                quiz_id, allow_retakes, max_attempts,
                show_correct_answers, show_correct_answers_after_completion,
                time_limit, passing_score, shuffle_questions
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            quiz_id,
            true,
            null::<i64>,
            true,
            true,
            quiz.time_limit,
            quiz.passing_score,
            quiz.shuffle_questions.unwrap_or(false)
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(quiz_id)
    }

    pub async fn update_quiz(&self, id: &str, quiz: UpdateQuizRequest) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        // Convert tags to JSON string if provided
        let tags_json = match &quiz.tags {
            Some(tags) => Some(serde_json::to_string(tags)?),
            None => None,
        };

        let rows_affected = sqlx::query!(
            r#"
            UPDATE quizzes
            SET
                title = COALESCE(?, title),
                description = COALESCE(?, description),
                course_id = COALESCE(?, course_id),
                time_limit = COALESCE(?, time_limit),
                passing_score = COALESCE(?, passing_score),
                shuffle_questions = COALESCE(?, shuffle_questions),
                show_results = COALESCE(?, show_results),
                visibility = COALESCE(?, visibility),
                tags = COALESCE(?, tags),
                study_mode = COALESCE(?, study_mode),
                updated_at = ?
            WHERE id = ? AND deleted_at IS NULL
            "#,
            quiz.title,
            quiz.description,
            quiz.course_id,
            quiz.time_limit,
            quiz.passing_score,
            quiz.shuffle_questions,
            quiz.show_results,
            quiz.visibility.map(|v| format!("{:?}", v).to_lowercase()),
            tags_json,
            quiz.study_mode.map(|m| format!("{:?}", m).to_lowercase()),
            now,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Quiz not found or not updated"));
        }

        Ok(())
    }

    pub async fn delete_quiz(&self, id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        let rows_affected = sqlx::query!(
            r#"
            UPDATE quizzes
            SET deleted_at = ?
            WHERE id = ? AND deleted_at IS NULL
            "#,
            now,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Quiz not found or not deleted"));
        }

        Ok(())
    }

    pub async fn get_quiz_summaries(&self) -> Result<Vec<QuizSummary>> {
        let summaries = sqlx::query!(
            r#"
            SELECT
                q.id, q.title, q.description, q.time_limit,
                COUNT(qn.id) as question_count,
                u.name as author_name,
                q.created_at,
                q.visibility as "visibility: Option<QuizVisibility>",
                q.tags,
                q.study_mode as "study_mode: Option<StudyMode>"
            FROM quizzes q
            LEFT JOIN questions qn ON q.id = qn.quiz_id
            LEFT JOIN users u ON q.author_id = u.id
            WHERE q.deleted_at IS NULL
            GROUP BY q.id
            ORDER BY q.created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert to QuizSummary objects
        let result = summaries
            .into_iter()
            .map(|row| {
                // Parse tags from JSON string
                let tags = match row.tags {
                    Some(tags_str) => {
                        serde_json::from_str::<Vec<String>>(&tags_str).unwrap_or_default()
                    },
                    None => Vec::new(),
                };

                QuizSummary {
                    id: row.id,
                    title: row.title,
                    description: row.description,
                    question_count: row.question_count,
                    time_limit: row.time_limit,
                    author_name: row.author_name.unwrap_or_else(|| "Unknown".to_string()),
                    created_at: row.created_at,
                    visibility: row.visibility,
                    tags: Some(tags),
                    study_mode: row.study_mode,
                }
            })
            .collect();

        Ok(result)
    }

    // Question methods
    pub async fn get_questions_by_quiz_id(&self, quiz_id: &str) -> Result<Vec<Question>> {
        let questions = sqlx::query_as!(
            Question,
            r#"
            SELECT
                id, quiz_id, question_text, content,
                question_type as "question_type: _",
                points, position, created_at, updated_at
            FROM questions
            WHERE quiz_id = ?
            ORDER BY COALESCE(position, id)
            "#,
            quiz_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(questions)
    }

    pub async fn get_question_by_id(&self, id: &str) -> Result<Question> {
        let question = sqlx::query_as!(Question,
            r#"
            SELECT
                id, quiz_id, question_text, content,
                question_type as "question_type: _",
                points, position, created_at, updated_at
            FROM questions
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Question not found"))?;

        Ok(question)
    }

    pub async fn create_question(&self, question: CreateQuestionRequest) -> Result<String> {
        let mut tx = self.pool.begin().await?;

        let question_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // Convert content to JSON string if provided
        let content_json = match &question.content {
            Some(content) => Some(serde_json::to_string(&content)?),
            None => None,
        };

        sqlx::query!(r#"
            INSERT INTO questions (
                id, quiz_id, question_text, content, question_type, points, position,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            question_id,
            question.quiz_id,
            question.question_text,
            content_json,
            format!("{:?}", question.question_type).to_lowercase(),
            question.points.unwrap_or(1),
            question.position,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;

        // Create answer options if provided
        if let Some(answer_options) = question.answer_options {
            for option in answer_options {
                let option_id = Uuid::new_v4().to_string();

                // Convert content to JSON string if provided
                let content_json = match &option.content {
                    Some(content) => Some(serde_json::to_string(&content)?),
                    None => None,
                };

                sqlx::query!(r#"
                    INSERT INTO answer_options (
                        id, question_id, option_text, content, is_correct, position,
                        created_at, updated_at
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    option_id,
                    question_id,
                    option.option_text,
                    content_json,
                    option.is_correct,
                    option.position,
                    now,
                    now
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(question_id)
    }

    pub async fn update_question(&self, id: &str, question: UpdateQuestionRequest) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        // Convert content to JSON string if provided
        let content_json = match &question.content {
            Some(content) => Some(serde_json::to_string(&content)?),
            None => None,
        };

        let rows_affected = sqlx::query!(r#"
            UPDATE questions
            SET
                question_text = COALESCE(?, question_text),
                content = COALESCE(?, content),
                question_type = COALESCE(?, question_type),
                points = COALESCE(?, points),
                position = COALESCE(?, position),
                updated_at = ?
            WHERE id = ?
            "#,
            question.question_text,
            content_json,
            question.question_type.map(|t| format!("{:?}", t).to_lowercase()),
            question.points,
            question.position,
            now,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Question not found or not updated"));
        }

        Ok(())
    }

    pub async fn delete_question(&self, id: &str) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Delete answer options first
        sqlx::query!(r#"
            DELETE FROM answer_options
            WHERE question_id = ?
            "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        // Delete the question
        let rows_affected = sqlx::query!(r#"
            DELETE FROM questions
            WHERE id = ?
            "#,
            id
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Question not found or not deleted"));
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_question_with_answers(&self, id: &str) -> Result<QuestionWithAnswers> {
        let question = self.get_question_by_id(id).await?;

        let answer_options = sqlx::query_as!(AnswerOption,
            r#"
            SELECT
                id, question_id, option_text, content,
                is_correct as "is_correct: bool",
                position, created_at, updated_at
            FROM answer_options
            WHERE question_id = ?
            ORDER BY COALESCE(position, id)
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(QuestionWithAnswers {
            question,
            answer_options,
        })
    }

    // Answer option methods
    pub async fn get_answer_options_by_question_id(&self, question_id: &str) -> Result<Vec<AnswerOption>> {
        let options = sqlx::query_as!(AnswerOption,
            r#"
            SELECT
                id, question_id, option_text, content,
                is_correct as "is_correct: bool",
                position, created_at, updated_at
            FROM answer_options
            WHERE question_id = ?
            ORDER BY COALESCE(position, id)
            "#,
            question_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(options)
    }

    pub async fn create_answer_option(&self, option: CreateAnswerOptionRequest) -> Result<String> {
        let option_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // Convert content to JSON string if provided
        let content_json = match &option.content {
            Some(content) => Some(serde_json::to_string(&content)?),
            None => None,
        };

        sqlx::query!(r#"
            INSERT INTO answer_options (
                id, question_id, option_text, content, is_correct, position,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            option_id,
            option.question_id,
            option.option_text,
            content_json,
            option.is_correct,
            option.position,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(option_id)
    }

    pub async fn update_answer_option(&self, id: &str, option: UpdateAnswerOptionRequest) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        // Convert content to JSON string if provided
        let content_json = match &option.content {
            Some(content) => Some(serde_json::to_string(&content)?),
            None => None,
        };

        let rows_affected = sqlx::query!(r#"
            UPDATE answer_options
            SET
                option_text = COALESCE(?, option_text),
                content = COALESCE(?, content),
                is_correct = COALESCE(?, is_correct),
                position = COALESCE(?, position),
                updated_at = ?
            WHERE id = ?
            "#,
            option.option_text,
            content_json,
            option.is_correct,
            option.position,
            now,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Answer option not found or not updated"));
        }

        Ok(())
    }

    pub async fn delete_answer_option(&self, id: &str) -> Result<()> {
        let rows_affected = sqlx::query!(r#"
            DELETE FROM answer_options
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Answer option not found or not deleted"));
        }

        Ok(())
    }

    // Quiz attempt methods
    pub async fn start_quiz_attempt(&self, user_id: &str, request: StartAttemptRequest) -> Result<QuizAttempt> {
        // Check if quiz exists
        let _quiz = self.get_quiz_by_id(&request.quiz_id).await?;

        // Check if user has active attempts
        let active_attempts = sqlx::query!(r#"
            SELECT COUNT(*) as count
            FROM quiz_attempts
            WHERE quiz_id = ? AND user_id = ? AND status = 'in_progress'
            "#,
            request.quiz_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        if active_attempts.count > 0 {
            return Err(anyhow!("User already has an active attempt for this quiz"));
        }

        // Check if user has reached max attempts
        let settings = self.get_quiz_settings(&request.quiz_id).await?;
        if let Some(max_attempts) = settings.max_attempts {
            let completed_attempts = sqlx::query!(r#"
                SELECT COUNT(*) as count
                FROM quiz_attempts
                WHERE quiz_id = ? AND user_id = ? AND status = 'completed'
                "#,
                request.quiz_id,
                user_id
            )
            .fetch_one(&self.pool)
            .await?;

            if completed_attempts.count >= max_attempts {
                return Err(anyhow!("User has reached the maximum number of attempts for this quiz"));
            }
        }

        // Create new attempt
        let attempt_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query!(r#"
            INSERT INTO quiz_attempts (
                id, quiz_id, user_id, start_time, status, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            attempt_id,
            request.quiz_id,
            user_id,
            now,
            "in_progress",
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        // Return the created attempt
        let attempt = sqlx::query_as!(QuizAttempt,
            r#"
            SELECT
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: _",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            attempt_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(attempt)
    }

    pub async fn complete_quiz_attempt(&self, request: CompleteAttemptRequest) -> Result<QuizAttempt> {
        let mut tx = self.pool.begin().await?;

        // Get the attempt
        let attempt = sqlx::query_as!(QuizAttempt,
            r#"
            SELECT
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: _",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            request.attempt_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow!("Attempt not found"))?;

        if attempt.status != AttemptStatus::InProgress {
            return Err(anyhow!("Attempt is not in progress"));
        }

        // Get the quiz
        let quiz = self.get_quiz_by_id(&attempt.quiz_id).await?;

        // Calculate score
        let total_questions = sqlx::query!(r#"
            SELECT COUNT(*) as count
            FROM questions
            WHERE quiz_id = ?
            "#,
            attempt.quiz_id
        )
        .fetch_one(&mut *tx)
        .await?
        .count;

        let correct_answers = sqlx::query!(r#"
            SELECT COUNT(*) as count
            FROM user_answers
            WHERE attempt_id = ? AND is_correct = 1
            "#,
            attempt.attempt_id
        )
        .fetch_one(&mut *tx)
        .await?
        .count;

        let score = if total_questions > 0 {
            (correct_answers as f64 / total_questions as f64) * 100.0
        } else {
            0.0
        };

        // Update the attempt
        let now = Utc::now().to_rfc3339();
        sqlx::query!(r#"
            UPDATE quiz_attempts
            SET
                end_time = ?,
                score = ?,
                status = 'completed',
                updated_at = ?
            WHERE id = ?
            "#,
            now,
            score,
            now,
            attempt.attempt_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Return the updated attempt
        let updated_attempt = sqlx::query_as!(QuizAttempt,
            r#"
            SELECT
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: _",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            request.attempt_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_attempt)
    }

    pub async fn abandon_quiz_attempt(&self, request: AbandonAttemptRequest) -> Result<QuizAttempt> {
        // Get the attempt
        let attempt = sqlx::query_as!(QuizAttempt,
            r#"
            SELECT
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: _",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            request.attempt_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Attempt not found"))?;

        if attempt.status != AttemptStatus::InProgress {
            return Err(anyhow!("Attempt is not in progress"));
        }

        // Update the attempt
        let now = Utc::now().to_rfc3339();
        sqlx::query!(r#"
            UPDATE quiz_attempts
            SET
                end_time = ?,
                status = 'abandoned',
                updated_at = ?
            WHERE id = ?
            "#,
            now,
            now,
            request.attempt_id
        )
        .execute(&self.pool)
        .await?;

        // Return the updated attempt
        let updated_attempt = sqlx::query_as!(QuizAttempt,
            r#"
            SELECT
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: _",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            request.attempt_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_attempt)
    }

    // Quiz settings methods
    pub async fn get_quiz_settings(&self, quiz_id: &str) -> Result<QuizSettings> {
        let settings = sqlx::query_as!(QuizSettings,
            r#"
            SELECT
                quiz_id,
                allow_retakes as "allow_retakes: bool",
                max_attempts,
                show_correct_answers as "show_correct_answers: bool",
                show_correct_answers_after_completion as "show_correct_answers_after_completion: bool",
                time_limit, passing_score,
                shuffle_questions as "shuffle_questions: Option<bool>",
                created_at, updated_at
            FROM quiz_settings
            WHERE quiz_id = ?
            "#,
            quiz_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Quiz settings not found"))?;

        Ok(settings)
    }

    pub async fn create_quiz_settings(&self, settings: CreateQuizSettingsRequest) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query!(r#"
            INSERT INTO quiz_settings (
                quiz_id, allow_retakes, max_attempts,
                show_correct_answers, show_correct_answers_after_completion,
                time_limit, passing_score, shuffle_questions,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            settings.quiz_id,
            settings.allow_retakes.unwrap_or(true),
            settings.max_attempts,
            settings.show_correct_answers.unwrap_or(true),
            settings.show_correct_answers_after_completion.unwrap_or(true),
            settings.time_limit,
            settings.passing_score,
            settings.shuffle_questions,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_quiz_settings(&self, quiz_id: &str, settings: UpdateQuizSettingsRequest) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        let rows_affected = sqlx::query!(r#"
            UPDATE quiz_settings
            SET
                allow_retakes = COALESCE(?, allow_retakes),
                max_attempts = COALESCE(?, max_attempts),
                show_correct_answers = COALESCE(?, show_correct_answers),
                show_correct_answers_after_completion = COALESCE(?, show_correct_answers_after_completion),
                time_limit = COALESCE(?, time_limit),
                passing_score = COALESCE(?, passing_score),
                shuffle_questions = COALESCE(?, shuffle_questions),
                updated_at = ?
            WHERE quiz_id = ?
            "#,
            settings.allow_retakes,
            settings.max_attempts,
            settings.show_correct_answers,
            settings.show_correct_answers_after_completion,
            settings.time_limit,
            settings.passing_score,
            settings.shuffle_questions,
            now,
            quiz_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Quiz settings not found or not updated"));
        }

        Ok(())
    }

    // Activity tracking methods
    pub async fn track_activity(&self, activity: CreateQuizActivityRequest) -> Result<String> {
        let activity_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // Convert activity type to string
        let activity_type_str = activity.activity_type.to_string();

        // Convert data to JSON string if provided
        let data_json = match &activity.data {
            Some(data) => Some(serde_json::to_string(&data)?),
            None => None,
        };

        sqlx::query!(r#"
            INSERT INTO quiz_activities (
                id, user_id, quiz_id, question_id, attempt_id, activity_type,
                data, duration_ms, timestamp, created_at, synced
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            activity_id,
            activity.user_id,
            activity.quiz_id,
            activity.question_id,
            activity.attempt_id,
            activity_type_str,
            data_json,
            activity.duration_ms,
            now,
            now,
            false
        )
        .execute(&self.pool)
        .await?;

        Ok(activity_id)
    }

    pub async fn get_activities_by_user(&self, user_id: &str, limit: Option<i64>) -> Result<Vec<QuizActivity>> {
        let limit = limit.unwrap_or(100);

        let activities = sqlx::query_as!(QuizActivity,
            r#"
            SELECT
                id, user_id, quiz_id, question_id, attempt_id, activity_type,
                data, duration_ms, timestamp, created_at, synced as "synced: bool"
            FROM quiz_activities
            WHERE user_id = ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(activities)
    }

    pub async fn get_activities_by_quiz(&self, quiz_id: &str, limit: Option<i64>) -> Result<Vec<QuizActivity>> {
        let limit = limit.unwrap_or(100);

        let activities = sqlx::query_as!(QuizActivity,
            r#"
            SELECT
                id, user_id, quiz_id, question_id, attempt_id, activity_type,
                data, duration_ms, timestamp, created_at, synced as "synced: bool"
            FROM quiz_activities
            WHERE quiz_id = ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
            quiz_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(activities)
    }

    pub async fn get_activity_summary_by_user(&self, user_id: &str) -> Result<QuizActivitySummary> {
        // Get total activities
        let total_activities = sqlx::query!(r#"
            SELECT COUNT(*) as count
            FROM quiz_activities
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        // Get total duration
        let total_duration = sqlx::query!(r#"
            SELECT COALESCE(SUM(duration_ms), 0) as total
            FROM quiz_activities
            WHERE user_id = ? AND duration_ms IS NOT NULL
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .total;

        // Get activity counts by type
        let activity_counts = sqlx::query!(r#"
            SELECT activity_type, COUNT(*) as count
            FROM quiz_activities
            WHERE user_id = ?
            GROUP BY activity_type
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert to JSON
        let mut counts_map = serde_json::Map::new();
        for row in activity_counts {
            counts_map.insert(row.activity_type, json!(row.count));
        }

        // Get first and last activity timestamps
        let timestamps = sqlx::query!(r#"
            SELECT
                MIN(timestamp) as first_activity,
                MAX(timestamp) as last_activity
            FROM quiz_activities
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(QuizActivitySummary {
            user_id: user_id.to_string(),
            quiz_id: None,
            total_activities,
            total_duration_ms: total_duration,
            activity_counts: json!(counts_map),
            first_activity_at: timestamps.first_activity.unwrap_or_default(),
            last_activity_at: timestamps.last_activity.unwrap_or_default(),
        })
    }

    pub async fn get_activity_summary_by_quiz(&self, quiz_id: &str) -> Result<QuizActivitySummary> {
        // Get total activities
        let total_activities = sqlx::query!(r#"
            SELECT COUNT(*) as count
            FROM quiz_activities
            WHERE quiz_id = ?
            "#,
            quiz_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        // Get total duration
        let total_duration = sqlx::query!(r#"
            SELECT COALESCE(SUM(duration_ms), 0) as total
            FROM quiz_activities
            WHERE quiz_id = ? AND duration_ms IS NOT NULL
            "#,
            quiz_id
        )
        .fetch_one(&self.pool)
        .await?
        .total;

        // Get activity counts by type
        let activity_counts = sqlx::query!(r#"
            SELECT activity_type, COUNT(*) as count
            FROM quiz_activities
            WHERE quiz_id = ?
            GROUP BY activity_type
            "#,
            quiz_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert to JSON
        let mut counts_map = serde_json::Map::new();
        for row in activity_counts {
            counts_map.insert(row.activity_type, json!(row.count));
        }

        // Get first and last activity timestamps
        let timestamps = sqlx::query!(r#"
            SELECT
                MIN(timestamp) as first_activity,
                MAX(timestamp) as last_activity
            FROM quiz_activities
            WHERE quiz_id = ?
            "#,
            quiz_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(QuizActivitySummary {
            user_id: String::new(),
            quiz_id: Some(quiz_id.to_string()),
            total_activities,
            total_duration_ms: total_duration,
            activity_counts: json!(counts_map),
            first_activity_at: timestamps.first_activity.unwrap_or_default(),
            last_activity_at: timestamps.last_activity.unwrap_or_default(),
        })
    }

    pub async fn get_activity_stats(&self, user_id: Option<&str>) -> Result<QuizActivityStats> {
        // Base query parts
        let user_filter = match user_id {
            Some(id) => format!("WHERE user_id = '{}'\n", id),
            None => String::new(),
        };

        // Get quiz stats
        let quiz_stats = sqlx::query!(r#"
            SELECT
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_started' THEN quiz_id END) as quizzes_started,
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_completed' THEN quiz_id END) as quizzes_completed,
                COUNT(CASE WHEN activity_type = 'question_answered' THEN 1 END) as questions_answered,
                COALESCE(SUM(duration_ms), 0) as total_study_time
            FROM quiz_activities
            "#.to_string() + &user_filter
        )
        .fetch_one(&self.pool)
        .await?;

        // Get average quiz duration
        let avg_quiz_duration = sqlx::query!(r#"
            SELECT AVG(duration_ms) as avg_duration
            FROM quiz_activities
            WHERE activity_type = 'quiz_completed' AND duration_ms IS NOT NULL
            "#.to_string() + &user_filter
        )
        .fetch_one(&self.pool)
        .await?
        .avg_duration;

        // Get average question time
        let avg_question_time = sqlx::query!(r#"
            SELECT AVG(duration_ms) as avg_duration
            FROM quiz_activities
            WHERE activity_type = 'question_answered' AND duration_ms IS NOT NULL
            "#.to_string() + &user_filter
        )
        .fetch_one(&self.pool)
        .await?
        .avg_duration;

        // Get activity by day
        let activity_by_day = sqlx::query!(r#"
            SELECT
                strftime('%Y-%m-%d', timestamp) as day,
                COUNT(*) as count
            FROM quiz_activities
            "#.to_string() + &user_filter + "
            GROUP BY day
            ORDER BY day"
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert to JSON
        let mut day_map = serde_json::Map::new();
        for row in activity_by_day {
            if let Some(day) = row.day {
                day_map.insert(day, json!(row.count));
            }
        }

        Ok(QuizActivityStats {
            total_quizzes_started: quiz_stats.quizzes_started,
            total_quizzes_completed: quiz_stats.quizzes_completed,
            total_questions_answered: quiz_stats.questions_answered,
            total_study_time_ms: quiz_stats.total_study_time,
            average_quiz_duration_ms: avg_quiz_duration,
            average_question_time_ms: avg_question_time,
            activity_by_day: json!(day_map),
        })
    }

    pub async fn mark_activities_synced(&self, activity_ids: &[String]) -> Result<usize> {
        if activity_ids.is_empty() {
            return Ok(0);
        }

        let now = Utc::now().to_rfc3339();
        let placeholders = activity_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");

        let query = format!(r#"
            UPDATE quiz_activities
            SET synced = 1, updated_at = ?
            WHERE id IN ({})
            "#, placeholders);

        let mut query = sqlx::query(&query);
        query = query.bind(now);

        for id in activity_ids {
            query = query.bind(id);
        }

        let result = query.execute(&self.pool).await?;

        Ok(result.rows_affected() as usize)
    }
}
