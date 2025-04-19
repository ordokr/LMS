use sqlx::{SqlitePool, Row};
use crate::models::quiz::{
    Quiz, CreateQuizRequest, UpdateQuizRequest, QuizSummary,
    Question, CreateQuestionRequest, UpdateQuestionRequest, QuestionWithAnswers,
    AnswerOption, CreateAnswerOptionRequest, UpdateAnswerOptionRequest,
    QuizAttempt, StartAttemptRequest, AttemptStatus, AttemptResult,
    QuizSettings, CreateQuizSettingsRequest, UpdateQuizSettingsRequest,
    Cmi5Session, CreateCmi5SessionRequest, Cmi5SessionStatus,
};
use uuid::Uuid;
use anyhow::{Result, anyhow};

pub struct QuizRepository {
    pool: SqlitePool,
}

impl QuizRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
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

    pub async fn get_quiz_by_id(&self, id: i64) -> Result<Quiz> {
        let quiz = sqlx::query_as!(
            Quiz,
            r#"
            SELECT 
                id, title, description, course_id, author_id, 
                time_limit, passing_score, 
                shuffle_questions as "shuffle_questions: bool", 
                show_results as "show_results: bool",
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

    pub async fn create_quiz(&self, user_id: i64, quiz: CreateQuizRequest) -> Result<i64> {
        let mut tx = self.pool.begin().await?;

        let quiz_id = sqlx::query!(
            r#"
            INSERT INTO quizzes (
                title, description, course_id, author_id, 
                time_limit, passing_score, shuffle_questions, show_results
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            quiz.title,
            quiz.description,
            quiz.course_id,
            user_id,
            quiz.time_limit,
            quiz.passing_score,
            quiz.shuffle_questions.unwrap_or(false),
            quiz.show_results.unwrap_or(true)
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

        // Create default settings
        sqlx::query!(
            r#"
            INSERT INTO quiz_settings (
                quiz_id, allow_retakes, max_attempts, 
                show_correct_answers, show_correct_answers_after_completion
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
            quiz_id,
            true,
            null::<i64>,
            true,
            true
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(quiz_id)
    }

    pub async fn update_quiz(&self, id: i64, quiz: UpdateQuizRequest) -> Result<()> {
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
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
            "#,
            quiz.title,
            quiz.description,
            quiz.course_id,
            quiz.time_limit,
            quiz.passing_score,
            quiz.shuffle_questions,
            quiz.show_results,
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

    pub async fn delete_quiz(&self, id: i64) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE quizzes
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE id = ? AND deleted_at IS NULL
            "#,
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
        let summaries = sqlx::query_as!(
            QuizSummary,
            r#"
            SELECT 
                q.id, q.title, q.description, q.time_limit,
                COUNT(qn.id) as question_count,
                u.username as author_name,
                q.created_at
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

        Ok(summaries)
    }

    // Question methods
    pub async fn get_questions_by_quiz_id(&self, quiz_id: i64) -> Result<Vec<Question>> {
        let questions = sqlx::query_as!(
            Question,
            r#"
            SELECT 
                id, quiz_id, question_text, 
                question_type as "question_type: QuestionType",
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

    pub async fn get_question_by_id(&self, id: i64) -> Result<Question> {
        let question = sqlx::query_as!(
            Question,
            r#"
            SELECT 
                id, quiz_id, question_text, 
                question_type as "question_type: QuestionType",
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

    pub async fn create_question(&self, question: CreateQuestionRequest) -> Result<i64> {
        let mut tx = self.pool.begin().await?;

        let question_id = sqlx::query!(
            r#"
            INSERT INTO questions (
                quiz_id, question_text, question_type, points, position
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
            question.quiz_id,
            question.question_text,
            question.question_type as QuestionType,
            question.points.unwrap_or(1),
            question.position
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

        // Create answer options if provided
        if let Some(answer_options) = question.answer_options {
            for option in answer_options {
                sqlx::query!(
                    r#"
                    INSERT INTO answer_options (
                        question_id, option_text, is_correct, position
                    )
                    VALUES (?, ?, ?, ?)
                    "#,
                    question_id,
                    option.option_text,
                    option.is_correct,
                    option.position
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(question_id)
    }

    pub async fn update_question(&self, id: i64, question: UpdateQuestionRequest) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE questions
            SET 
                question_text = COALESCE(?, question_text),
                question_type = COALESCE(?, question_type),
                points = COALESCE(?, points),
                position = COALESCE(?, position),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            question.question_text,
            question.question_type as Option<QuestionType>,
            question.points,
            question.position,
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

    pub async fn delete_question(&self, id: i64) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Delete answer options first
        sqlx::query!(
            r#"
            DELETE FROM answer_options
            WHERE question_id = ?
            "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        // Delete user answers
        sqlx::query!(
            r#"
            DELETE FROM user_answers
            WHERE question_id = ?
            "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        // Delete the question
        let rows_affected = sqlx::query!(
            r#"
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

    pub async fn get_question_with_answers(&self, id: i64) -> Result<QuestionWithAnswers> {
        let question = self.get_question_by_id(id).await?;
        
        let answer_options = sqlx::query_as!(
            AnswerOption,
            r#"
            SELECT 
                id, question_id, option_text, 
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
    pub async fn get_answer_options_by_question_id(&self, question_id: i64) -> Result<Vec<AnswerOption>> {
        let options = sqlx::query_as!(
            AnswerOption,
            r#"
            SELECT 
                id, question_id, option_text, 
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

    pub async fn create_answer_option(&self, option: CreateAnswerOptionRequest) -> Result<i64> {
        let option_id = sqlx::query!(
            r#"
            INSERT INTO answer_options (
                question_id, option_text, is_correct, position
            )
            VALUES (?, ?, ?, ?)
            "#,
            option.question_id,
            option.option_text,
            option.is_correct,
            option.position
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(option_id)
    }

    pub async fn update_answer_option(&self, id: i64, option: UpdateAnswerOptionRequest) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE answer_options
            SET 
                option_text = COALESCE(?, option_text),
                is_correct = COALESCE(?, is_correct),
                position = COALESCE(?, position),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            option.option_text,
            option.is_correct,
            option.position,
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

    pub async fn delete_answer_option(&self, id: i64) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
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
    pub async fn start_quiz_attempt(&self, user_id: i64, request: StartAttemptRequest) -> Result<i64> {
        // Check if quiz exists
        let quiz = self.get_quiz_by_id(request.quiz_id).await?;
        
        // Check if user has active attempts
        let active_attempts = sqlx::query!(
            r#"
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
        let settings = self.get_quiz_settings(request.quiz_id).await?;
        if let Some(max_attempts) = settings.max_attempts {
            let completed_attempts = sqlx::query!(
                r#"
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
        let attempt_id = sqlx::query!(
            r#"
            INSERT INTO quiz_attempts (
                quiz_id, user_id, start_time, status
            )
            VALUES (?, ?, CURRENT_TIMESTAMP, 'in_progress')
            "#,
            request.quiz_id,
            user_id
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(attempt_id)
    }

    pub async fn complete_quiz_attempt(&self, attempt_id: i64) -> Result<AttemptResult> {
        let mut tx = self.pool.begin().await?;

        // Get the attempt
        let attempt = sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT 
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: AttemptStatus",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            attempt_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow!("Attempt not found"))?;

        if attempt.status != AttemptStatus::InProgress {
            return Err(anyhow!("Attempt is not in progress"));
        }

        // Get the quiz
        let quiz = self.get_quiz_by_id(attempt.quiz_id).await?;

        // Calculate score
        let total_questions = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM questions
            WHERE quiz_id = ?
            "#,
            attempt.quiz_id
        )
        .fetch_one(&mut *tx)
        .await?
        .count;

        let correct_answers = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_answers
            WHERE attempt_id = ? AND is_correct = 1
            "#,
            attempt_id
        )
        .fetch_one(&mut *tx)
        .await?
        .count;

        let score_percentage = if total_questions > 0 {
            (correct_answers as f64 / total_questions as f64) * 100.0
        } else {
            0.0
        };

        let passed = match quiz.passing_score {
            Some(passing_score) => score_percentage >= passing_score as f64,
            None => true, // If no passing score is set, consider it passed
        };

        // Calculate time taken
        let end_time = chrono::Utc::now().to_rfc3339();
        let start_time = chrono::DateTime::parse_from_rfc3339(&attempt.start_time)
            .map_err(|_| anyhow!("Invalid start time format"))?;
        let end_time_parsed = chrono::DateTime::parse_from_rfc3339(&end_time)
            .map_err(|_| anyhow!("Invalid end time format"))?;
        let time_taken = (end_time_parsed - start_time).num_seconds() as i64;

        // Update the attempt
        sqlx::query!(
            r#"
            UPDATE quiz_attempts
            SET 
                end_time = CURRENT_TIMESTAMP,
                score = ?,
                status = 'completed',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            score_percentage,
            attempt_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Return the result
        Ok(AttemptResult {
            attempt: QuizAttempt {
                id: attempt.id,
                quiz_id: attempt.quiz_id,
                user_id: attempt.user_id,
                start_time: attempt.start_time,
                end_time: Some(end_time),
                score: Some(score_percentage),
                status: AttemptStatus::Completed,
                created_at: attempt.created_at,
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            quiz,
            total_questions,
            correct_answers,
            score_percentage,
            passed,
            time_taken: Some(time_taken),
        })
    }

    pub async fn abandon_quiz_attempt(&self, attempt_id: i64) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE quiz_attempts
            SET 
                end_time = CURRENT_TIMESTAMP,
                status = 'abandoned',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND status = 'in_progress'
            "#,
            attempt_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("Attempt not found or not in progress"));
        }

        Ok(())
    }

    pub async fn get_quiz_attempts_by_user(&self, user_id: i64) -> Result<Vec<QuizAttempt>> {
        let attempts = sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT 
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: AttemptStatus",
                created_at, updated_at
            FROM quiz_attempts
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(attempts)
    }

    pub async fn get_quiz_attempts_by_quiz_id(&self, quiz_id: i64) -> Result<Vec<QuizAttempt>> {
        let attempts = sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT 
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: AttemptStatus",
                created_at, updated_at
            FROM quiz_attempts
            WHERE quiz_id = ?
            ORDER BY created_at DESC
            "#,
            quiz_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(attempts)
    }

    pub async fn get_quiz_attempt(&self, attempt_id: i64) -> Result<QuizAttempt> {
        let attempt = sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT 
                id, quiz_id, user_id, start_time, end_time, score,
                status as "status: AttemptStatus",
                created_at, updated_at
            FROM quiz_attempts
            WHERE id = ?
            "#,
            attempt_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Attempt not found"))?;

        Ok(attempt)
    }

    // Quiz settings methods
    pub async fn get_quiz_settings(&self, quiz_id: i64) -> Result<QuizSettings> {
        let settings = sqlx::query_as!(
            QuizSettings,
            r#"
            SELECT 
                id, quiz_id, 
                allow_retakes as "allow_retakes: bool",
                max_attempts,
                show_correct_answers as "show_correct_answers: bool",
                show_correct_answers_after_completion as "show_correct_answers_after_completion: bool",
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

    pub async fn create_quiz_settings(&self, settings: CreateQuizSettingsRequest) -> Result<i64> {
        let settings_id = sqlx::query!(
            r#"
            INSERT INTO quiz_settings (
                quiz_id, allow_retakes, max_attempts, 
                show_correct_answers, show_correct_answers_after_completion
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
            settings.quiz_id,
            settings.allow_retakes.unwrap_or(true),
            settings.max_attempts,
            settings.show_correct_answers.unwrap_or(true),
            settings.show_correct_answers_after_completion.unwrap_or(true)
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(settings_id)
    }

    pub async fn update_quiz_settings(&self, quiz_id: i64, settings: UpdateQuizSettingsRequest) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE quiz_settings
            SET 
                allow_retakes = COALESCE(?, allow_retakes),
                max_attempts = COALESCE(?, max_attempts),
                show_correct_answers = COALESCE(?, show_correct_answers),
                show_correct_answers_after_completion = COALESCE(?, show_correct_answers_after_completion),
                updated_at = CURRENT_TIMESTAMP
            WHERE quiz_id = ?
            "#,
            settings.allow_retakes,
            settings.max_attempts,
            settings.show_correct_answers,
            settings.show_correct_answers_after_completion,
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

    // CMI5 methods
    pub async fn create_cmi5_session(&self, user_id: i64, request: CreateCmi5SessionRequest) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO cmi5_sessions (
                quiz_id, user_id, session_id, registration_id,
                actor_json, activity_id, return_url, status
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, 'initialized')
            "#,
            request.quiz_id,
            user_id,
            session_id,
            request.registration_id,
            request.actor_json,
            request.activity_id,
            request.return_url,
        )
        .execute(&self.pool)
        .await?;

        Ok(session_id)
    }

    pub async fn get_cmi5_session(&self, session_id: &str) -> Result<Cmi5Session> {
        let session = sqlx::query_as!(
            Cmi5Session,
            r#"
            SELECT 
                id, quiz_id, user_id, session_id, registration_id,
                actor_json, activity_id, return_url, score,
                status as "status: Cmi5SessionStatus",
                created_at, updated_at
            FROM cmi5_sessions
            WHERE session_id = ?
            "#,
            session_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("CMI5 session not found"))?;

        Ok(session)
    }

    pub async fn update_cmi5_session_status(&self, session_id: &str, status: Cmi5SessionStatus, score: Option<f64>) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE cmi5_sessions
            SET 
                status = ?,
                score = COALESCE(?, score),
                updated_at = CURRENT_TIMESTAMP
            WHERE session_id = ?
            "#,
            status as Cmi5SessionStatus,
            score,
            session_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow!("CMI5 session not found or not updated"));
        }

        Ok(())
    }

    pub async fn get_cmi5_sessions_by_user(&self, user_id: i64) -> Result<Vec<Cmi5Session>> {
        let sessions = sqlx::query_as!(
            Cmi5Session,
            r#"
            SELECT 
                id, quiz_id, user_id, session_id, registration_id,
                actor_json, activity_id, return_url, score,
                status as "status: Cmi5SessionStatus",
                created_at, updated_at
            FROM cmi5_sessions
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }
}
