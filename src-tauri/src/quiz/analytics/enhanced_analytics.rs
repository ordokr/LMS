use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crate::quiz::models::{Quiz, Question, Answer, QuizAttempt};
use crate::quiz::analytics::AnalyticsService;
use std::sync::Arc;

/// Enhanced analytics service
pub struct EnhancedAnalyticsService {
    /// Base analytics service
    analytics: Arc<AnalyticsService>,
}

/// Quiz performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizPerformanceMetrics {
    /// Quiz ID
    pub quiz_id: Uuid,
    
    /// Quiz title
    pub quiz_title: String,
    
    /// Number of attempts
    pub attempt_count: usize,
    
    /// Average score
    pub average_score: f32,
    
    /// Median score
    pub median_score: f32,
    
    /// Highest score
    pub highest_score: f32,
    
    /// Lowest score
    pub lowest_score: f32,
    
    /// Score distribution
    pub score_distribution: HashMap<String, usize>,
    
    /// Average time spent
    pub average_time_spent: i32,
    
    /// Completion rate
    pub completion_rate: f32,
    
    /// Question performance
    pub question_performance: Vec<QuestionPerformanceMetrics>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Question performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionPerformanceMetrics {
    /// Question ID
    pub question_id: Uuid,
    
    /// Question text
    pub question_text: String,
    
    /// Correct answer rate
    pub correct_rate: f32,
    
    /// Average time spent
    pub average_time_spent: Option<i32>,
    
    /// Answer distribution
    pub answer_distribution: HashMap<String, usize>,
    
    /// Difficulty level (calculated)
    pub difficulty_level: DifficultyLevel,
    
    /// Discrimination index
    pub discrimination_index: Option<f32>,
}

/// Difficulty level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    /// Very easy (>90% correct)
    VeryEasy,
    
    /// Easy (70-90% correct)
    Easy,
    
    /// Medium (40-70% correct)
    Medium,
    
    /// Hard (20-40% correct)
    Hard,
    
    /// Very hard (<20% correct)
    VeryHard,
}

/// User performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPerformanceMetrics {
    /// User ID
    pub user_id: Uuid,
    
    /// Number of quizzes attempted
    pub quiz_count: usize,
    
    /// Number of quizzes completed
    pub completed_count: usize,
    
    /// Average score
    pub average_score: f32,
    
    /// Total time spent
    pub total_time_spent: i32,
    
    /// Quiz performance
    pub quiz_performance: Vec<UserQuizPerformance>,
    
    /// Skill mastery
    pub skill_mastery: HashMap<String, f32>,
    
    /// Learning progress over time
    pub learning_progress: Vec<LearningProgressPoint>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// User quiz performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuizPerformance {
    /// Quiz ID
    pub quiz_id: Uuid,
    
    /// Quiz title
    pub quiz_title: String,
    
    /// Score
    pub score: f32,
    
    /// Time spent
    pub time_spent: i32,
    
    /// Completed
    pub completed: bool,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Learning progress point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProgressPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Score
    pub score: f32,
    
    /// Quiz ID
    pub quiz_id: Uuid,
    
    /// Quiz title
    pub quiz_title: String,
}

/// Cohort performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortPerformanceMetrics {
    /// Cohort ID
    pub cohort_id: Option<Uuid>,
    
    /// Cohort name
    pub cohort_name: String,
    
    /// Number of users
    pub user_count: usize,
    
    /// Average score
    pub average_score: f32,
    
    /// Median score
    pub median_score: f32,
    
    /// Score distribution
    pub score_distribution: HashMap<String, usize>,
    
    /// Completion rate
    pub completion_rate: f32,
    
    /// Quiz performance
    pub quiz_performance: Vec<CohortQuizPerformance>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Cohort quiz performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortQuizPerformance {
    /// Quiz ID
    pub quiz_id: Uuid,
    
    /// Quiz title
    pub quiz_title: String,
    
    /// Average score
    pub average_score: f32,
    
    /// Completion rate
    pub completion_rate: f32,
}

/// Learning insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsights {
    /// User ID
    pub user_id: Uuid,
    
    /// Strengths
    pub strengths: Vec<LearningInsight>,
    
    /// Weaknesses
    pub weaknesses: Vec<LearningInsight>,
    
    /// Improvement suggestions
    pub improvement_suggestions: Vec<String>,
    
    /// Learning patterns
    pub learning_patterns: Vec<LearningPattern>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Learning insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    /// Topic
    pub topic: String,
    
    /// Performance level
    pub performance_level: f32,
    
    /// Related questions
    pub related_questions: Vec<Uuid>,
    
    /// Related quizzes
    pub related_quizzes: Vec<Uuid>,
}

/// Learning pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPattern {
    /// Pattern type
    pub pattern_type: LearningPatternType,
    
    /// Description
    pub description: String,
    
    /// Confidence
    pub confidence: f32,
    
    /// Supporting data
    pub supporting_data: HashMap<String, serde_json::Value>,
}

/// Learning pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LearningPatternType {
    /// Time of day preference
    TimeOfDay,
    
    /// Session duration
    SessionDuration,
    
    /// Question type preference
    QuestionTypePreference,
    
    /// Topic preference
    TopicPreference,
    
    /// Learning style
    LearningStyle,
    
    /// Other
    Other,
}

impl EnhancedAnalyticsService {
    /// Create a new enhanced analytics service
    pub fn new(analytics: Arc<AnalyticsService>) -> Self {
        Self {
            analytics,
        }
    }
    
    /// Get quiz performance metrics
    pub async fn get_quiz_performance_metrics(&self, quiz_id: &Uuid) -> Result<QuizPerformanceMetrics> {
        // Get the quiz
        let quiz = self.analytics.get_quiz(quiz_id).await?;
        
        // Get all attempts for this quiz
        let attempts = self.analytics.get_quiz_attempts(quiz_id).await?;
        
        if attempts.is_empty() {
            return Err(anyhow!("No attempts found for quiz"));
        }
        
        // Calculate metrics
        let attempt_count = attempts.len();
        
        // Calculate scores
        let scores: Vec<f32> = attempts.iter()
            .filter_map(|a| a.score)
            .collect();
        
        let average_score = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f32>() / scores.len() as f32
        };
        
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median_score = if sorted_scores.is_empty() {
            0.0
        } else if sorted_scores.len() % 2 == 0 {
            (sorted_scores[sorted_scores.len() / 2 - 1] + sorted_scores[sorted_scores.len() / 2]) / 2.0
        } else {
            sorted_scores[sorted_scores.len() / 2]
        };
        
        let highest_score = sorted_scores.last().cloned().unwrap_or(0.0);
        let lowest_score = sorted_scores.first().cloned().unwrap_or(0.0);
        
        // Calculate score distribution
        let mut score_distribution = HashMap::new();
        for score in &scores {
            let bucket = match *score {
                s if s < 20.0 => "0-19",
                s if s < 40.0 => "20-39",
                s if s < 60.0 => "40-59",
                s if s < 80.0 => "60-79",
                _ => "80-100",
            };
            
            *score_distribution.entry(bucket.to_string()).or_insert(0) += 1;
        }
        
        // Calculate average time spent
        let average_time_spent = if attempts.is_empty() {
            0
        } else {
            attempts.iter().map(|a| a.time_spent).sum::<i32>() / attempts.len() as i32
        };
        
        // Calculate completion rate
        let completed_count = attempts.iter().filter(|a| a.completed_at.is_some()).count();
        let completion_rate = completed_count as f32 / attempt_count as f32;
        
        // Calculate question performance
        let mut question_performance = Vec::new();
        
        for question in &quiz.questions {
            // Get all answers for this question
            let answers: Vec<_> = attempts.iter()
                .filter_map(|a| {
                    a.answers.iter()
                        .find(|qa| qa.question_id == question.id)
                        .map(|qa| (a, qa))
                })
                .collect();
            
            if answers.is_empty() {
                continue;
            }
            
            // Calculate correct rate
            let correct_count = answers.iter()
                .filter(|(_, qa)| qa.is_correct)
                .count();
            
            let correct_rate = correct_count as f32 / answers.len() as f32;
            
            // Calculate difficulty level
            let difficulty_level = match correct_rate {
                r if r > 0.9 => DifficultyLevel::VeryEasy,
                r if r > 0.7 => DifficultyLevel::Easy,
                r if r > 0.4 => DifficultyLevel::Medium,
                r if r > 0.2 => DifficultyLevel::Hard,
                _ => DifficultyLevel::VeryHard,
            };
            
            // Calculate answer distribution
            let mut answer_distribution = HashMap::new();
            for (_, qa) in &answers {
                let answer_key = match &qa.answer {
                    Answer::Choice(id) => id.to_string(),
                    Answer::Text(text) => text.clone(),
                    Answer::Matching(_) => "matching".to_string(),
                    Answer::Ordering(_) => "ordering".to_string(),
                    _ => "other".to_string(),
                };
                
                *answer_distribution.entry(answer_key).or_insert(0) += 1;
            }
            
            // Calculate discrimination index
            // This is a more complex calculation that requires sorting attempts by score
            // and comparing performance on this question between high and low scorers
            let discrimination_index = None; // Simplified for now
            
            question_performance.push(QuestionPerformanceMetrics {
                question_id: question.id,
                question_text: question.content.text.clone(),
                correct_rate,
                average_time_spent: None, // We don't track time per question
                answer_distribution,
                difficulty_level,
                discrimination_index,
            });
        }
        
        Ok(QuizPerformanceMetrics {
            quiz_id: *quiz_id,
            quiz_title: quiz.title,
            attempt_count,
            average_score,
            median_score,
            highest_score,
            lowest_score,
            score_distribution,
            average_time_spent,
            completion_rate,
            question_performance,
            timestamp: Utc::now(),
        })
    }
    
    /// Get user performance metrics
    pub async fn get_user_performance_metrics(&self, user_id: &Uuid) -> Result<UserPerformanceMetrics> {
        // Get all attempts for this user
        let attempts = self.analytics.get_user_attempts(user_id).await?;
        
        if attempts.is_empty() {
            return Err(anyhow!("No attempts found for user"));
        }
        
        // Get unique quizzes
        let quiz_ids: Vec<Uuid> = attempts.iter()
            .map(|a| a.quiz_id)
            .collect();
        
        let unique_quiz_ids: Vec<Uuid> = quiz_ids.clone().into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        // Get quiz details
        let mut quizzes = HashMap::new();
        for quiz_id in &unique_quiz_ids {
            if let Ok(quiz) = self.analytics.get_quiz(quiz_id).await {
                quizzes.insert(*quiz_id, quiz);
            }
        }
        
        // Calculate metrics
        let quiz_count = unique_quiz_ids.len();
        let completed_count = attempts.iter()
            .filter(|a| a.completed_at.is_some())
            .map(|a| a.quiz_id)
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        // Calculate average score
        let scores: Vec<f32> = attempts.iter()
            .filter_map(|a| a.score)
            .collect();
        
        let average_score = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f32>() / scores.len() as f32
        };
        
        // Calculate total time spent
        let total_time_spent = attempts.iter().map(|a| a.time_spent).sum();
        
        // Calculate quiz performance
        let mut quiz_performance = Vec::new();
        
        for attempt in &attempts {
            if let Some(quiz) = quizzes.get(&attempt.quiz_id) {
                quiz_performance.push(UserQuizPerformance {
                    quiz_id: attempt.quiz_id,
                    quiz_title: quiz.title.clone(),
                    score: attempt.score.unwrap_or(0.0),
                    time_spent: attempt.time_spent,
                    completed: attempt.completed_at.is_some(),
                    timestamp: attempt.started_at,
                });
            }
        }
        
        // Sort by timestamp
        quiz_performance.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        // Calculate skill mastery
        // This would require a more complex analysis of questions and their tags/categories
        let skill_mastery = HashMap::new();
        
        // Calculate learning progress
        let mut learning_progress = Vec::new();
        
        for performance in &quiz_performance {
            if performance.completed {
                learning_progress.push(LearningProgressPoint {
                    timestamp: performance.timestamp,
                    score: performance.score,
                    quiz_id: performance.quiz_id,
                    quiz_title: performance.quiz_title.clone(),
                });
            }
        }
        
        Ok(UserPerformanceMetrics {
            user_id: *user_id,
            quiz_count,
            completed_count,
            average_score,
            total_time_spent,
            quiz_performance,
            skill_mastery,
            learning_progress,
            timestamp: Utc::now(),
        })
    }
    
    /// Get cohort performance metrics
    pub async fn get_cohort_performance_metrics(&self, cohort_id: Option<&Uuid>, cohort_name: &str, user_ids: &[Uuid]) -> Result<CohortPerformanceMetrics> {
        if user_ids.is_empty() {
            return Err(anyhow!("No users in cohort"));
        }
        
        // Get all attempts for all users
        let mut all_attempts = Vec::new();
        for user_id in user_ids {
            if let Ok(attempts) = self.analytics.get_user_attempts(user_id).await {
                all_attempts.extend(attempts);
            }
        }
        
        if all_attempts.is_empty() {
            return Err(anyhow!("No attempts found for cohort"));
        }
        
        // Get unique quizzes
        let quiz_ids: Vec<Uuid> = all_attempts.iter()
            .map(|a| a.quiz_id)
            .collect();
        
        let unique_quiz_ids: Vec<Uuid> = quiz_ids.clone().into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        // Get quiz details
        let mut quizzes = HashMap::new();
        for quiz_id in &unique_quiz_ids {
            if let Ok(quiz) = self.analytics.get_quiz(quiz_id).await {
                quizzes.insert(*quiz_id, quiz);
            }
        }
        
        // Calculate metrics
        let user_count = user_ids.len();
        
        // Calculate scores
        let scores: Vec<f32> = all_attempts.iter()
            .filter_map(|a| a.score)
            .collect();
        
        let average_score = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f32>() / scores.len() as f32
        };
        
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median_score = if sorted_scores.is_empty() {
            0.0
        } else if sorted_scores.len() % 2 == 0 {
            (sorted_scores[sorted_scores.len() / 2 - 1] + sorted_scores[sorted_scores.len() / 2]) / 2.0
        } else {
            sorted_scores[sorted_scores.len() / 2]
        };
        
        // Calculate score distribution
        let mut score_distribution = HashMap::new();
        for score in &scores {
            let bucket = match *score {
                s if s < 20.0 => "0-19",
                s if s < 40.0 => "20-39",
                s if s < 60.0 => "40-59",
                s if s < 80.0 => "60-79",
                _ => "80-100",
            };
            
            *score_distribution.entry(bucket.to_string()).or_insert(0) += 1;
        }
        
        // Calculate completion rate
        let completed_count = all_attempts.iter().filter(|a| a.completed_at.is_some()).count();
        let completion_rate = completed_count as f32 / all_attempts.len() as f32;
        
        // Calculate quiz performance
        let mut quiz_performance = Vec::new();
        
        for quiz_id in &unique_quiz_ids {
            if let Some(quiz) = quizzes.get(quiz_id) {
                // Get all attempts for this quiz
                let quiz_attempts: Vec<_> = all_attempts.iter()
                    .filter(|a| a.quiz_id == *quiz_id)
                    .collect();
                
                if quiz_attempts.is_empty() {
                    continue;
                }
                
                // Calculate average score
                let quiz_scores: Vec<f32> = quiz_attempts.iter()
                    .filter_map(|a| a.score)
                    .collect();
                
                let quiz_average_score = if quiz_scores.is_empty() {
                    0.0
                } else {
                    quiz_scores.iter().sum::<f32>() / quiz_scores.len() as f32
                };
                
                // Calculate completion rate
                let quiz_completed_count = quiz_attempts.iter().filter(|a| a.completed_at.is_some()).count();
                let quiz_completion_rate = quiz_completed_count as f32 / quiz_attempts.len() as f32;
                
                quiz_performance.push(CohortQuizPerformance {
                    quiz_id: *quiz_id,
                    quiz_title: quiz.title.clone(),
                    average_score: quiz_average_score,
                    completion_rate: quiz_completion_rate,
                });
            }
        }
        
        Ok(CohortPerformanceMetrics {
            cohort_id: cohort_id.cloned(),
            cohort_name: cohort_name.to_string(),
            user_count,
            average_score,
            median_score,
            score_distribution,
            completion_rate,
            quiz_performance,
            timestamp: Utc::now(),
        })
    }
    
    /// Get learning insights for a user
    pub async fn get_learning_insights(&self, user_id: &Uuid) -> Result<LearningInsights> {
        // Get user performance metrics
        let user_metrics = self.get_user_performance_metrics(user_id).await?;
        
        // Get all attempts for this user
        let attempts = self.analytics.get_user_attempts(user_id).await?;
        
        if attempts.is_empty() {
            return Err(anyhow!("No attempts found for user"));
        }
        
        // Get unique quizzes
        let quiz_ids: Vec<Uuid> = attempts.iter()
            .map(|a| a.quiz_id)
            .collect();
        
        let unique_quiz_ids: Vec<Uuid> = quiz_ids.clone().into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        // Get quiz details
        let mut quizzes = HashMap::new();
        for quiz_id in &unique_quiz_ids {
            if let Ok(quiz) = self.analytics.get_quiz(quiz_id).await {
                quizzes.insert(*quiz_id, quiz);
            }
        }
        
        // Analyze question performance to identify strengths and weaknesses
        let mut topic_performance = HashMap::new();
        let mut topic_questions = HashMap::new();
        let mut topic_quizzes = HashMap::new();
        
        for attempt in &attempts {
            if let Some(quiz) = quizzes.get(&attempt.quiz_id) {
                for answer in &attempt.answers {
                    if let Some(question) = quiz.questions.iter().find(|q| q.id == answer.question_id) {
                        // Extract topics from question tags
                        let topics = question.tags.clone().unwrap_or_else(Vec::new);
                        
                        for topic in topics {
                            // Update performance for this topic
                            let performance = topic_performance.entry(topic.clone()).or_insert(0.0);
                            *performance += if answer.is_correct { 1.0 } else { 0.0 };
                            
                            // Track questions for this topic
                            let questions = topic_questions.entry(topic.clone()).or_insert_with(Vec::new);
                            if !questions.contains(&question.id) {
                                questions.push(question.id);
                            }
                            
                            // Track quizzes for this topic
                            let quizzes = topic_quizzes.entry(topic.clone()).or_insert_with(Vec::new);
                            if !quizzes.contains(&quiz.id) {
                                quizzes.push(quiz.id);
                            }
                        }
                    }
                }
            }
        }
        
        // Normalize performance scores
        for (topic, count) in topic_questions.iter() {
            if let Some(performance) = topic_performance.get_mut(topic) {
                *performance /= count.len() as f32;
            }
        }
        
        // Identify strengths (top 3 topics)
        let mut strengths = Vec::new();
        let mut topics: Vec<_> = topic_performance.iter().collect();
        topics.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        
        for (topic, performance) in topics.iter().take(3) {
            if **performance >= 0.7 {
                strengths.push(LearningInsight {
                    topic: (*topic).clone(),
                    performance_level: **performance,
                    related_questions: topic_questions.get(*topic).cloned().unwrap_or_default(),
                    related_quizzes: topic_quizzes.get(*topic).cloned().unwrap_or_default(),
                });
            }
        }
        
        // Identify weaknesses (bottom 3 topics)
        let mut weaknesses = Vec::new();
        topics.reverse();
        
        for (topic, performance) in topics.iter().take(3) {
            if **performance <= 0.5 {
                weaknesses.push(LearningInsight {
                    topic: (*topic).clone(),
                    performance_level: **performance,
                    related_questions: topic_questions.get(*topic).cloned().unwrap_or_default(),
                    related_quizzes: topic_quizzes.get(*topic).cloned().unwrap_or_default(),
                });
            }
        }
        
        // Generate improvement suggestions
        let mut improvement_suggestions = Vec::new();
        
        for weakness in &weaknesses {
            improvement_suggestions.push(format!(
                "Focus on improving your knowledge of {}. Try reviewing the related quizzes.",
                weakness.topic
            ));
        }
        
        // If the user has completed few quizzes
        if user_metrics.completed_count < user_metrics.quiz_count / 2 {
            improvement_suggestions.push(
                "Try to complete more quizzes to get a better understanding of your strengths and weaknesses."
                    .to_string()
            );
        }
        
        // Analyze learning patterns
        let mut learning_patterns = Vec::new();
        
        // Time of day pattern
        let mut time_distribution = HashMap::new();
        for attempt in &attempts {
            let hour = attempt.started_at.hour();
            let bucket = match hour {
                0..=5 => "Night (12AM-6AM)",
                6..=11 => "Morning (6AM-12PM)",
                12..=17 => "Afternoon (12PM-6PM)",
                _ => "Evening (6PM-12AM)",
            };
            
            *time_distribution.entry(bucket).or_insert(0) += 1;
        }
        
        let total_attempts = attempts.len();
        let mut max_time_bucket = ("", 0);
        
        for (bucket, count) in &time_distribution {
            if *count > max_time_bucket.1 {
                max_time_bucket = (*bucket, *count);
            }
        }
        
        if max_time_bucket.1 > total_attempts / 3 {
            let confidence = max_time_bucket.1 as f32 / total_attempts as f32;
            
            let mut supporting_data = HashMap::new();
            supporting_data.insert("time_distribution".to_string(), serde_json::to_value(&time_distribution).unwrap());
            
            learning_patterns.push(LearningPattern {
                pattern_type: LearningPatternType::TimeOfDay,
                description: format!("You tend to study during the {} time period.", max_time_bucket.0),
                confidence,
                supporting_data,
            });
        }
        
        // Session duration pattern
        let mut duration_distribution = HashMap::new();
        for attempt in &attempts {
            let duration = attempt.time_spent;
            let bucket = match duration {
                d if d < 300 => "Very short (<5 min)",
                d if d < 900 => "Short (5-15 min)",
                d if d < 1800 => "Medium (15-30 min)",
                d if d < 3600 => "Long (30-60 min)",
                _ => "Very long (>60 min)",
            };
            
            *duration_distribution.entry(bucket).or_insert(0) += 1;
        }
        
        let mut max_duration_bucket = ("", 0);
        
        for (bucket, count) in &duration_distribution {
            if *count > max_duration_bucket.1 {
                max_duration_bucket = (*bucket, *count);
            }
        }
        
        if max_duration_bucket.1 > total_attempts / 3 {
            let confidence = max_duration_bucket.1 as f32 / total_attempts as f32;
            
            let mut supporting_data = HashMap::new();
            supporting_data.insert("duration_distribution".to_string(), serde_json::to_value(&duration_distribution).unwrap());
            
            learning_patterns.push(LearningPattern {
                pattern_type: LearningPatternType::SessionDuration,
                description: format!("Your quiz sessions are typically {} in duration.", max_duration_bucket.0),
                confidence,
                supporting_data,
            });
        }
        
        Ok(LearningInsights {
            user_id: *user_id,
            strengths,
            weaknesses,
            improvement_suggestions,
            learning_patterns,
            timestamp: Utc::now(),
        })
    }
}
