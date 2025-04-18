use super::models::{Quiz, Question, Answer, QuizAttempt, FlashcardData};
use super::storage::HybridQuizStore;
use super::spaced_repetition::{FlashcardStatistics};
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;

/// Analytics and reporting engine for quiz data
pub struct AnalyticsEngine {
    store: Arc<HybridQuizStore>,
}

/// Time period for analytics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimePeriod {
    Day,
    Week,
    Month,
    Year,
    AllTime,
}

/// Quiz performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizPerformance {
    pub quiz_id: Uuid,
    pub quiz_title: String,
    pub attempts: usize,
    pub average_score: f32,
    pub best_score: f32,
    pub average_time: i32, // in seconds
    pub completion_rate: f32, // percentage of attempts completed
    pub last_attempt_date: Option<DateTime<Utc>>,
}

/// User study statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStudyStats {
    pub user_id: Uuid,
    pub total_quizzes_taken: usize,
    pub total_questions_answered: usize,
    pub correct_answers: usize,
    pub accuracy_rate: f32,
    pub total_study_time: i32, // in seconds
    pub average_session_time: i32, // in seconds
    pub flashcard_stats: Option<FlashcardStatistics>,
    pub quiz_performance: Vec<QuizPerformance>,
    pub study_streak: i32, // consecutive days of study
    pub last_study_date: Option<DateTime<Utc>>,
}

/// Question difficulty statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionDifficulty {
    pub question_id: Uuid,
    pub quiz_id: Uuid,
    pub correct_rate: f32, // percentage of correct answers
    pub average_time: i32, // in seconds
    pub attempts: usize,
}

/// Quiz analytics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAnalytics {
    pub quiz_id: Uuid,
    pub quiz_title: String,
    pub total_attempts: usize,
    pub unique_users: usize,
    pub average_score: f32,
    pub completion_rate: f32,
    pub average_time: i32, // in seconds
    pub question_difficulties: Vec<QuestionDifficulty>,
    pub most_common_mistakes: Vec<(Uuid, String, f32)>, // question_id, question_text, error_rate
}

impl AnalyticsEngine {
    pub fn new(store: Arc<HybridQuizStore>) -> Self {
        Self { store }
    }
    
    /// Get study statistics for a user
    pub async fn get_user_stats(&self, user_id: Uuid, period: TimePeriod) -> Result<UserStudyStats, Box<dyn Error + Send + Sync>> {
        // Get time range based on period
        let (start_date, end_date) = self.get_time_range(period);
        
        // Get quiz attempts for the user in the time range
        let attempts = self.store.get_user_quiz_attempts(user_id, start_date, end_date).await?;
        
        // Calculate basic statistics
        let total_quizzes_taken = attempts.len();
        let mut total_questions_answered = 0;
        let mut correct_answers = 0;
        let mut total_study_time = 0;
        let mut quiz_performance_map: HashMap<Uuid, QuizPerformance> = HashMap::new();
        let mut last_study_date: Option<DateTime<Utc>> = None;
        
        for attempt in &attempts {
            // Update last study date
            if let Some(completed_at) = attempt.completed_at {
                if last_study_date.is_none() || completed_at > last_study_date.unwrap() {
                    last_study_date = Some(completed_at);
                }
            }
            
            // Get quiz details
            let quiz = self.store.get_quiz(attempt.quiz_id).await?;
            
            // Update total study time
            total_study_time += attempt.time_spent;
            
            // Get question answers for this attempt
            let answers = self.store.get_attempt_answers(attempt.id).await?;
            
            total_questions_answered += answers.len();
            
            // Count correct answers
            for answer in &answers {
                if let Some(is_correct) = answer.is_correct {
                    if is_correct {
                        correct_answers += 1;
                    }
                }
            }
            
            // Update quiz performance
            let entry = quiz_performance_map.entry(attempt.quiz_id).or_insert_with(|| {
                QuizPerformance {
                    quiz_id: attempt.quiz_id,
                    quiz_title: quiz.title.clone(),
                    attempts: 0,
                    average_score: 0.0,
                    best_score: 0.0,
                    average_time: 0,
                    completion_rate: 0.0,
                    last_attempt_date: None,
                }
            });
            
            entry.attempts += 1;
            
            if let Some(score) = attempt.score {
                entry.average_score = (entry.average_score * (entry.attempts - 1) as f32 + score) / entry.attempts as f32;
                entry.best_score = entry.best_score.max(score);
            }
            
            entry.average_time = (entry.average_time * (entry.attempts - 1) + attempt.time_spent) / entry.attempts;
            
            if attempt.completed_at.is_some() {
                entry.completion_rate = (entry.completion_rate * (entry.attempts - 1) as f32 + 1.0) / entry.attempts as f32;
            }
            
            if let Some(completed_at) = attempt.completed_at {
                if entry.last_attempt_date.is_none() || completed_at > entry.last_attempt_date.unwrap() {
                    entry.last_attempt_date = Some(completed_at);
                }
            }
        }
        
        // Calculate study streak
        let study_streak = self.calculate_study_streak(user_id).await?;
        
        // Get flashcard statistics if available
        let flashcard_stats = match self.store.get_spaced_repetition_scheduler() {
            Some(scheduler) => {
                match scheduler.get_user_statistics(user_id).await {
                    Ok(stats) => Some(stats),
                    Err(_) => None,
                }
            },
            None => None,
        };
        
        // Calculate accuracy rate
        let accuracy_rate = if total_questions_answered > 0 {
            correct_answers as f32 / total_questions_answered as f32
        } else {
            0.0
        };
        
        // Calculate average session time
        let average_session_time = if total_quizzes_taken > 0 {
            total_study_time / total_quizzes_taken as i32
        } else {
            0
        };
        
        Ok(UserStudyStats {
            user_id,
            total_quizzes_taken,
            total_questions_answered,
            correct_answers,
            accuracy_rate,
            total_study_time,
            average_session_time,
            flashcard_stats,
            quiz_performance: quiz_performance_map.into_values().collect(),
            study_streak,
            last_study_date,
        })
    }
    
    /// Get analytics for a specific quiz
    pub async fn get_quiz_analytics(&self, quiz_id: Uuid, period: TimePeriod) -> Result<QuizAnalytics, Box<dyn Error + Send + Sync>> {
        // Get time range based on period
        let (start_date, end_date) = self.get_time_range(period);
        
        // Get quiz details
        let quiz = self.store.get_quiz(quiz_id).await?;
        
        // Get all attempts for this quiz in the time range
        let attempts = self.store.get_quiz_attempts(quiz_id, start_date, end_date).await?;
        
        // Calculate basic statistics
        let total_attempts = attempts.len();
        let unique_users: std::collections::HashSet<Uuid> = attempts.iter().map(|a| a.user_id).collect();
        let unique_users_count = unique_users.len();
        
        let mut total_score = 0.0;
        let mut completed_attempts = 0;
        let mut total_time = 0;
        
        for attempt in &attempts {
            if let Some(score) = attempt.score {
                total_score += score;
            }
            
            if attempt.completed_at.is_some() {
                completed_attempts += 1;
            }
            
            total_time += attempt.time_spent;
        }
        
        let average_score = if total_attempts > 0 {
            total_score / total_attempts as f32
        } else {
            0.0
        };
        
        let completion_rate = if total_attempts > 0 {
            completed_attempts as f32 / total_attempts as f32
        } else {
            0.0
        };
        
        let average_time = if total_attempts > 0 {
            total_time / total_attempts as i32
        } else {
            0
        };
        
        // Calculate question difficulties
        let mut question_stats: HashMap<Uuid, (usize, usize, i32)> = HashMap::new(); // question_id -> (correct, attempts, total_time)
        
        for attempt in &attempts {
            let answers = self.store.get_attempt_answers(attempt.id).await?;
            
            for answer in &answers {
                let entry = question_stats.entry(answer.question_id).or_insert((0, 0, 0));
                
                entry.1 += 1; // increment attempts
                entry.2 += answer.time_spent; // add time spent
                
                if let Some(is_correct) = answer.is_correct {
                    if is_correct {
                        entry.0 += 1; // increment correct answers
                    }
                }
            }
        }
        
        let mut question_difficulties = Vec::new();
        let mut most_common_mistakes = Vec::new();
        
        for (question_id, (correct, attempts, total_time)) in question_stats {
            let correct_rate = if attempts > 0 {
                correct as f32 / attempts as f32
            } else {
                0.0
            };
            
            let average_time = if attempts > 0 {
                total_time / attempts as i32
            } else {
                0
            };
            
            question_difficulties.push(QuestionDifficulty {
                question_id,
                quiz_id,
                correct_rate,
                average_time,
                attempts,
            });
            
            // If error rate is high, add to most common mistakes
            if attempts >= 5 && correct_rate < 0.6 {
                // Get question text
                let question = self.store.get_question(question_id).await?;
                let question_text = question.content.text.clone();
                
                most_common_mistakes.push((question_id, question_text, 1.0 - correct_rate));
            }
        }
        
        // Sort most common mistakes by error rate (descending)
        most_common_mistakes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit to top 5 mistakes
        let most_common_mistakes = most_common_mistakes.into_iter().take(5).collect();
        
        Ok(QuizAnalytics {
            quiz_id,
            quiz_title: quiz.title,
            total_attempts,
            unique_users: unique_users_count,
            average_score,
            completion_rate,
            average_time,
            question_difficulties,
            most_common_mistakes,
        })
    }
    
    /// Calculate study streak (consecutive days of study)
    async fn calculate_study_streak(&self, user_id: Uuid) -> Result<i32, Box<dyn Error + Send + Sync>> {
        // Get all study dates for the user
        let study_dates = self.store.get_user_study_dates(user_id).await?;
        
        if study_dates.is_empty() {
            return Ok(0);
        }
        
        // Sort dates in descending order
        let mut dates = study_dates;
        dates.sort_by(|a, b| b.cmp(a));
        
        // Check if user studied today
        let today = Utc::now().date().and_hms(0, 0, 0);
        let mut streak = 0;
        let mut current_date = if dates[0].date() == today.date() {
            streak = 1;
            today - Duration::days(1)
        } else {
            return Ok(0); // No study today, streak is 0
        };
        
        // Count consecutive days
        for date in dates.iter().skip(1) {
            if date.date() == current_date.date() {
                current_date = current_date - Duration::days(1);
            } else if date.date() < current_date.date() {
                break;
            }
            
            streak += 1;
        }
        
        Ok(streak)
    }
    
    /// Get time range based on period
    fn get_time_range(&self, period: TimePeriod) -> (DateTime<Utc>, DateTime<Utc>) {
        let end_date = Utc::now();
        
        let start_date = match period {
            TimePeriod::Day => end_date - Duration::days(1),
            TimePeriod::Week => end_date - Duration::days(7),
            TimePeriod::Month => end_date - Duration::days(30),
            TimePeriod::Year => end_date - Duration::days(365),
            TimePeriod::AllTime => DateTime::<Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp(0, 0),
                Utc,
            ),
        };
        
        (start_date, end_date)
    }
    
    /// Generate a PDF report for a user's study statistics
    pub async fn generate_user_report(&self, user_id: Uuid, period: TimePeriod) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // Get user statistics
        let stats = self.get_user_stats(user_id, period).await?;
        
        // In a real implementation, this would generate a PDF
        // For now, just serialize to JSON
        let json = serde_json::to_string_pretty(&stats)?;
        
        Ok(json.into_bytes())
    }
    
    /// Generate a PDF report for a quiz's analytics
    pub async fn generate_quiz_report(&self, quiz_id: Uuid, period: TimePeriod) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        // Get quiz analytics
        let analytics = self.get_quiz_analytics(quiz_id, period).await?;
        
        // In a real implementation, this would generate a PDF
        // For now, just serialize to JSON
        let json = serde_json::to_string_pretty(&analytics)?;
        
        Ok(json.into_bytes())
    }
}
