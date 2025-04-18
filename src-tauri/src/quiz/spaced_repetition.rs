use super::models::{FlashcardData, Quiz, Question, Answer};
use super::session::QuizSession;
use super::storage::HybridQuizStore;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use std::error::Error;

/// Original SM-2 parameters structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SM2Parameters {
    pub easiness_factor: f32,
    pub interval: i32,
    pub repetitions: i32,
}

/// Implementation of the SuperMemo SM-2 algorithm with enhancements
/// for spaced repetition flashcard learning
pub struct SpacedRepetitionScheduler {
    store: Arc<HybridQuizStore>,
}

/// Rating provided by the user for a flashcard
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FlashcardRating {
    /// Rating 1: Complete blackout, wrong response
    Blackout = 1,
    /// Rating 2: Incorrect response, but upon seeing the answer it felt familiar
    Familiar = 2,
    /// Rating 3: Incorrect response, but correct answer was easy to recall once shown
    Difficult = 3,
    /// Rating 4: Correct response after hesitation
    Hesitation = 4,
    /// Rating 5: Correct response with perfect recall
    Perfect = 5,
}

impl From<i32> for FlashcardRating {
    fn from(value: i32) -> Self {
        match value {
            1 => FlashcardRating::Blackout,
            2 => FlashcardRating::Familiar,
            3 => FlashcardRating::Difficult,
            4 => FlashcardRating::Hesitation,
            5 => FlashcardRating::Perfect,
            _ => {
                // Default to Difficult for invalid values
                FlashcardRating::Difficult
            }
        }
    }
}

impl SpacedRepetitionScheduler {
    pub fn new(store: Arc<HybridQuizStore>) -> Self {
        Self { store }
    }

    /// Legacy method for backward compatibility
    pub async fn schedule_review(&self, card_id: Uuid, performance: u8) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
        let rating = match performance {
            0..=1 => FlashcardRating::Blackout,
            2 => FlashcardRating::Familiar,
            3 => FlashcardRating::Difficult,
            4 => FlashcardRating::Hesitation,
            _ => FlashcardRating::Perfect,
        };

        // Get user ID from the card
        let card_data = self.store.get_flashcard_data_by_question(card_id).await?;

        // Process the rating using the new system
        let updated_data = self.process_rating(card_id, card_data.user_id, rating).await?;

        Ok(updated_data.due_date)
    }

    /// Legacy calculation method for backward compatibility
    fn calculate_sm2(&self, mut params: SM2Parameters, performance: u8) -> SM2Parameters {
        let performance = performance.clamp(0, 5) as f32;

        params.easiness_factor = params.easiness_factor.max(1.3)
            * (0.1 - (5.0 - performance) * (0.08 + (5.0 - performance) * 0.02));

        if performance < 3.0 {
            params.repetitions = 0;
            params.interval = 1;
        } else {
            params.repetitions += 1;
            params.interval = match params.repetitions {
                1 => 1,
                2 => 6,
                _ => (params.interval as f32 * params.easiness_factor) as i32
            };
        }

        params
    }

    /// Process a flashcard rating and update the spaced repetition data
    pub async fn process_rating(
        &self,
        question_id: Uuid,
        user_id: Uuid,
        rating: FlashcardRating,
    ) -> Result<FlashcardData, Box<dyn Error + Send + Sync>> {
        // Get existing flashcard data or create new
        let mut flashcard_data = match self.store.get_flashcard_data(question_id, user_id).await {
            Ok(data) => data,
            Err(_) => {
                // Create new flashcard data for first-time cards
                FlashcardData {
                    question_id,
                    user_id,
                    ease_factor: 2.5, // Initial ease factor
                    interval: 0,      // Initial interval (days)
                    repetitions: 0,   // Number of successful repetitions
                    due_date: Utc::now(), // Due immediately
                    last_reviewed: Utc::now(),
                }
            }
        };

        // Update flashcard data based on rating
        self.update_flashcard_data(&mut flashcard_data, rating);

        // Store updated flashcard data
        self.store.store_flashcard_data(&flashcard_data).await?;

        Ok(flashcard_data)
    }

    /// Update flashcard data using the SM-2 algorithm with enhancements
    fn update_flashcard_data(&self, data: &mut FlashcardData, rating: FlashcardRating) {
        // Convert rating to numeric value
        let rating_value = rating as i32;

        // Record review time
        data.last_reviewed = Utc::now();

        // SM-2 algorithm implementation with enhancements
        if rating_value >= 3 {
            // Correct response
            if data.repetitions == 0 {
                // First successful repetition
                data.interval = 1;
            } else if data.repetitions == 1 {
                // Second successful repetition
                data.interval = 6;
            } else {
                // Calculate new interval based on previous interval and ease factor
                data.interval = (data.interval as f32 * data.ease_factor).round() as i32;
            }

            // Increment repetition counter
            data.repetitions += 1;
        } else {
            // Incorrect response - reset repetitions but keep some progress
            // This is an enhancement over standard SM-2 which resets to 0
            data.repetitions = (data.repetitions as f32 * 0.6).floor() as i32;
            data.interval = match rating {
                FlashcardRating::Blackout => 0, // Complete reset for total blackout
                FlashcardRating::Familiar => {
                    // For familiar cards, keep some interval progress
                    (data.interval as f32 * 0.25).max(1.0).round() as i32
                }
                _ => 1, // Default case (shouldn't happen)
            };
        }

        // Update ease factor based on rating
        // This is the standard SM-2 formula with a slight modification
        // to make it more responsive to user performance
        let ease_factor_delta = 0.1 - (5 - rating_value) as f32 * 0.08;
        data.ease_factor = (data.ease_factor + ease_factor_delta).max(1.3);

        // Calculate next due date
        // Add jitter to prevent cards from clumping together
        let jitter = if data.interval > 0 {
            let jitter_factor = 0.05; // 5% jitter
            let max_jitter = (data.interval as f32 * jitter_factor).max(0.5).round() as i32;
            fastrand::i32(-max_jitter..=max_jitter)
        } else {
            0
        };

        let interval_with_jitter = (data.interval + jitter).max(0);
        data.due_date = Utc::now() + Duration::days(interval_with_jitter as i64);
    }

    /// Get due flashcards for a user
    pub async fn get_due_flashcards(
        &self,
        user_id: Uuid,
        limit: usize,
    ) -> Result<Vec<(Question, FlashcardData)>, Box<dyn Error + Send + Sync>> {
        // Get flashcard data for cards that are due
        let flashcard_data = self.store.get_due_flashcards(user_id, Utc::now(), limit).await?;

        let mut result = Vec::new();

        // Fetch the corresponding questions
        for data in flashcard_data {
            match self.store.get_question(data.question_id).await {
                Ok(question) => {
                    result.push((question, data));
                }
                Err(e) => {
                    eprintln!("Failed to get question {}: {}", data.question_id, e);
                    // Continue with other flashcards
                    continue;
                }
            }
        }

        Ok(result)
    }

    /// Create a study session with due flashcards
    pub async fn create_flashcard_session(
        &self,
        user_id: Uuid,
        limit: usize,
    ) -> Result<(QuizSession, Vec<Question>), Box<dyn Error + Send + Sync>> {
        // Get due flashcards
        let due_flashcards = self.get_due_flashcards(user_id, limit).await?;

        if due_flashcards.is_empty() {
            return Err("No flashcards due for review".into());
        }

        // Extract questions
        let questions: Vec<Question> = due_flashcards.into_iter().map(|(q, _)| q).collect();

        // Create a temporary quiz for the session
        let quiz_id = Uuid::new_v4();
        let mut temp_quiz = Quiz::new(format!("Flashcard Session {}", Utc::now()), Some(user_id));
        temp_quiz.id = quiz_id;
        temp_quiz.study_mode = super::models::StudyMode::Flashcards;
        temp_quiz.questions = questions.clone();

        // Store the temporary quiz
        self.store.store_quiz(&temp_quiz).await?;

        // Create a session
        let session = QuizSession::with_quiz(&temp_quiz, user_id);
        self.store.store_session(&session).await?;

        Ok((session, questions))
    }

    /// Get statistics for a user's flashcard learning
    pub async fn get_user_statistics(
        &self,
        user_id: Uuid,
    ) -> Result<FlashcardStatistics, Box<dyn Error + Send + Sync>> {
        let all_flashcards = self.store.get_all_flashcard_data(user_id).await?;

        let now = Utc::now();
        let mut stats = FlashcardStatistics {
            total_cards: all_flashcards.len(),
            cards_due_today: 0,
            cards_due_next_week: 0,
            new_cards: 0,
            mature_cards: 0,
            average_ease_factor: 0.0,
            retention_rate: 0.0,
        };

        let today_end = now + Duration::days(1);
        let week_end = now + Duration::days(7);

        let mut total_ease = 0.0;
        let mature_threshold = 21; // Cards with interval > 21 days are considered mature

        for card in &all_flashcards {
            // Count cards due today
            if card.due_date <= today_end {
                stats.cards_due_today += 1;
            }

            // Count cards due in the next week
            if card.due_date > today_end && card.due_date <= week_end {
                stats.cards_due_next_week += 1;
            }

            // Count new cards (never successfully reviewed)
            if card.repetitions == 0 {
                stats.new_cards += 1;
            }

            // Count mature cards
            if card.interval > mature_threshold {
                stats.mature_cards += 1;
            }

            total_ease += card.ease_factor;
        }

        // Calculate average ease factor
        if !all_flashcards.is_empty() {
            stats.average_ease_factor = total_ease / all_flashcards.len() as f32;
        }

        // Calculate retention rate (percentage of cards that were answered correctly)
        // This requires looking at review history, which we'll implement in a future version
        // For now, we'll estimate based on ease factors
        if !all_flashcards.is_empty() {
            let retention_sum: f32 = all_flashcards.iter()
                .map(|card| {
                    // Convert ease factor to estimated retention rate
                    // Ease factor 2.5 (default) corresponds to about 85% retention
                    let base_retention = 0.85;
                    let ease_adjustment = (card.ease_factor - 2.5) * 0.1;
                    (base_retention + ease_adjustment).clamp(0.0, 1.0)
                })
                .sum();

            stats.retention_rate = retention_sum / all_flashcards.len() as f32;
        }

        Ok(stats)
    }
}

/// Statistics for flashcard learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashcardStatistics {
    pub total_cards: usize,
    pub cards_due_today: usize,
    pub cards_due_next_week: usize,
    pub new_cards: usize,
    pub mature_cards: usize,
    pub average_ease_factor: f32,
    pub retention_rate: f32,
}