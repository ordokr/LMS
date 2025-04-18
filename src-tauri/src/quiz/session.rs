use super::models::{Quiz, Question, Answer, QuizSettings, StudyMode};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSession {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub current_question_index: usize,
    pub answers: Vec<SessionAnswer>,
    pub score: Option<f32>,
    pub time_remaining: Option<i32>, // in seconds
    pub quiz_settings: QuizSettings,
    pub question_order: Vec<usize>, // Shuffled indices if shuffle_questions is true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnswer {
    pub question_id: Uuid,
    pub answer: Answer,
    pub timestamp: DateTime<Utc>,
    pub is_correct: Option<bool>,
    pub time_spent: i32, // in seconds
}

impl QuizSession {
    pub fn new(quiz_id: Uuid, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            user_id,
            started_at: Utc::now(),
            completed_at: None,
            current_question_index: 0,
            answers: Vec::new(),
            score: None,
            time_remaining: None,
            quiz_settings: QuizSettings {
                shuffle_questions: false,
                time_limit: None,
                allow_retries: true,
                show_correct_answers: true,
                passing_score: None,
                study_mode: StudyMode::MultipleChoice,
            },
            question_order: Vec::new(),
        }
    }

    pub fn with_quiz(quiz: &Quiz, user_id: Uuid) -> Self {
        let mut session = Self {
            id: Uuid::new_v4(),
            quiz_id: quiz.id,
            user_id,
            started_at: Utc::now(),
            completed_at: None,
            current_question_index: 0,
            answers: Vec::new(),
            score: None,
            time_remaining: quiz.settings.time_limit.map(|mins| mins * 60),
            quiz_settings: quiz.settings.clone(),
            question_order: (0..quiz.questions.len()).collect(),
        };

        // Shuffle questions if needed
        if quiz.settings.shuffle_questions && !quiz.questions.is_empty() {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            session.question_order.shuffle(&mut rng);
        }

        session
    }

    pub fn submit_answer(&mut self, question_id: Uuid, answer: Answer, quiz: &Quiz) -> Result<bool, String> {
        // Find the question in the quiz
        let question = quiz.questions.iter()
            .find(|q| q.id == question_id)
            .ok_or_else(|| format!("Question not found: {}", question_id))?;

        let timestamp = Utc::now();
        let time_spent = (timestamp - self.started_at).num_seconds() as i32;

        // Check if the answer is correct
        let is_correct = question.check_answer(&answer);

        self.answers.push(SessionAnswer {
            question_id,
            answer,
            timestamp,
            is_correct: Some(is_correct),
            time_spent,
        });

        // Move to the next question
        if self.current_question_index < self.question_order.len() - 1 {
            self.current_question_index += 1;
        }

        Ok(is_correct)
    }

    pub fn complete(&mut self) -> Result<f32, String> {
        self.completed_at = Some(Utc::now());

        // Calculate final score
        let total_questions = self.answers.len() as f32;
        if total_questions == 0.0 {
            self.score = Some(0.0);
            return Ok(0.0);
        }

        let correct_answers = self.answers.iter()
            .filter(|a| a.is_correct.unwrap_or(false))
            .count() as f32;

        let score = (correct_answers / total_questions) * 100.0;
        self.score = Some(score);

        Ok(score)
    }

    pub fn get_current_question_index(&self) -> usize {
        if self.question_order.is_empty() {
            return 0;
        }

        self.question_order[self.current_question_index]
    }

    pub fn get_progress(&self) -> f32 {
        if self.question_order.is_empty() {
            return 0.0;
        }

        (self.current_question_index as f32 + 1.0) / (self.question_order.len() as f32)
    }

    pub fn update_time_remaining(&mut self) -> Option<i32> {
        if let Some(time_limit) = self.quiz_settings.time_limit {
            let elapsed = (Utc::now() - self.started_at).num_seconds() as i32;
            let time_limit_seconds = time_limit * 60;
            let remaining = time_limit_seconds - elapsed;

            self.time_remaining = Some(remaining.max(0));

            // Auto-complete if time is up
            if remaining <= 0 && self.completed_at.is_none() {
                let _ = self.complete();
            }

            self.time_remaining
        } else {
            None
        }
    }
}