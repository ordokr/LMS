use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use chrono::{NaiveDate, Utc, DateTime};
use std::collections::HashMap;
use tracing::{info, error};

/// Report types
#[derive(Debug, Serialize, Deserialize)]
pub enum ReportType {
    UserActivity,
    QuizPerformance,
    QuestionAnalysis,
    TimeAnalysis,
    Custom,
}

/// Report data for the analytics dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    pub report_type: ReportType,
    pub title: String,
    pub description: String,
    pub generated_at: String,
    pub data: serde_json::Value,
}

/// User activity report data
#[derive(Debug, Serialize, Deserialize)]
pub struct UserActivityReport {
    pub user_id: String,
    pub user_name: String,
    pub total_activities: i64,
    pub quizzes_started: i64,
    pub quizzes_completed: i64,
    pub questions_answered: i64,
    pub total_study_time_ms: i64,
    pub average_score: f64,
    pub activity_by_day: HashMap<String, i64>,
    pub activity_by_type: HashMap<String, i64>,
    pub quiz_performance: Vec<QuizPerformanceItem>,
}

/// Quiz performance report data
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizPerformanceReport {
    pub quiz_id: String,
    pub quiz_title: String,
    pub total_attempts: i64,
    pub completed_attempts: i64,
    pub abandoned_attempts: i64,
    pub average_score: f64,
    pub average_duration_ms: i64,
    pub question_performance: Vec<QuestionPerformanceItem>,
    pub user_performance: Vec<UserPerformanceItem>,
}

/// Question analysis report data
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnalysisReport {
    pub quiz_id: String,
    pub quiz_title: String,
    pub questions: Vec<QuestionAnalysisItem>,
}

/// Time analysis report data
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeAnalysisReport {
    pub activity_by_day: HashMap<String, i64>,
    pub activity_by_hour: HashMap<i64, i64>,
    pub activity_by_day_of_week: HashMap<String, i64>,
    pub average_session_duration_ms: i64,
    pub peak_activity_hour: i64,
    pub peak_activity_day: String,
}

/// Quiz performance item
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizPerformanceItem {
    pub quiz_id: String,
    pub quiz_title: String,
    pub attempts: i64,
    pub score: f64,
    pub duration_ms: i64,
    pub completed: bool,
}

/// Question performance item
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionPerformanceItem {
    pub question_id: String,
    pub question_text: String,
    pub attempts: i64,
    pub correct_answers: i64,
    pub average_time_ms: i64,
}

/// User performance item
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPerformanceItem {
    pub user_id: String,
    pub user_name: String,
    pub attempts: i64,
    pub score: f64,
    pub duration_ms: i64,
}

/// Question analysis item
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnalysisItem {
    pub question_id: String,
    pub question_text: String,
    pub question_type: String,
    pub attempts: i64,
    pub correct_answers: i64,
    pub incorrect_answers: i64,
    pub average_time_ms: i64,
    pub difficulty_index: f64,
    pub discrimination_index: f64,
    pub answer_distribution: HashMap<String, i64>,
}

/// Report service for generating reports
pub struct ReportService {
    pool: SqlitePool,
}

impl ReportService {
    /// Create a new report service
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Generate a user activity report
    pub async fn generate_user_activity_report(
        &self,
        user_id: &str,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ReportData> {
        info!("Generating user activity report for user {}", user_id);
        
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND timestamp BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND timestamp >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND timestamp <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Get user info
        let user = sqlx::query!(
            "SELECT id, name FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("User not found"))?;
        
        // Get activity summary
        let activity_summary = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_activities,
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_started' THEN quiz_id END) as quizzes_started,
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_completed' THEN quiz_id END) as quizzes_completed,
                COUNT(CASE WHEN activity_type = 'question_answered' THEN 1 END) as questions_answered,
                COALESCE(SUM(duration_ms), 0) as total_study_time
            FROM quiz_activities
            WHERE user_id = ?
            "#.to_string() + &date_filter,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Get average score
        let average_score = sqlx::query!(
            r#"
            SELECT AVG(score) as average_score
            FROM quiz_attempts
            WHERE user_id = ? AND status = 'completed'
            "#.to_string() + &date_filter,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .average_score
        .unwrap_or(0.0);
        
        // Get activity by day
        let activity_by_day = sqlx::query!(
            r#"
            SELECT 
                strftime('%Y-%m-%d', timestamp) as day,
                COUNT(*) as count
            FROM quiz_activities
            WHERE user_id = ?
            "#.to_string() + &date_filter + "
            GROUP BY day
            ORDER BY day",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Get activity by type
        let activity_by_type = sqlx::query!(
            r#"
            SELECT 
                activity_type,
                COUNT(*) as count
            FROM quiz_activities
            WHERE user_id = ?
            "#.to_string() + &date_filter + "
            GROUP BY activity_type
            ORDER BY count DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Get quiz performance
        let quiz_performance = sqlx::query!(
            r#"
            SELECT 
                q.id as quiz_id,
                q.title as quiz_title,
                COUNT(a.id) as attempts,
                AVG(a.score) as score,
                AVG(
                    CASE 
                        WHEN a.end_time IS NOT NULL AND a.start_time IS NOT NULL 
                        THEN strftime('%s', a.end_time) - strftime('%s', a.start_time) 
                        ELSE NULL 
                    END
                ) * 1000 as duration_ms,
                SUM(CASE WHEN a.status = 'completed' THEN 1 ELSE 0 END) as completed_count
            FROM quizzes q
            JOIN quiz_attempts a ON q.id = a.quiz_id
            WHERE a.user_id = ? AND q.deleted_at IS NULL
            "#.to_string() + &date_filter + "
            GROUP BY q.id
            ORDER BY attempts DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Convert to HashMaps
        let mut day_map = HashMap::new();
        for row in activity_by_day {
            if let Some(day) = row.day {
                day_map.insert(day, row.count);
            }
        }
        
        let mut type_map = HashMap::new();
        for row in activity_by_type {
            type_map.insert(row.activity_type, row.count);
        }
        
        // Convert quiz performance
        let quiz_performance_items = quiz_performance.into_iter().map(|row| {
            QuizPerformanceItem {
                quiz_id: row.quiz_id,
                quiz_title: row.quiz_title,
                attempts: row.attempts,
                score: row.score.unwrap_or(0.0),
                duration_ms: row.duration_ms.unwrap_or(0),
                completed: row.completed_count > 0,
            }
        }).collect();
        
        // Create report data
        let report = UserActivityReport {
            user_id: user.id,
            user_name: user.name,
            total_activities: activity_summary.total_activities,
            quizzes_started: activity_summary.quizzes_started,
            quizzes_completed: activity_summary.quizzes_completed,
            questions_answered: activity_summary.questions_answered,
            total_study_time_ms: activity_summary.total_study_time,
            average_score,
            activity_by_day: day_map,
            activity_by_type: type_map,
            quiz_performance: quiz_performance_items,
        };
        
        // Create report data
        let report_data = ReportData {
            report_type: ReportType::UserActivity,
            title: format!("User Activity Report - {}", user.name),
            description: format!("Activity report for user {} from {} to {}", 
                user.name, 
                start_date.map_or("all time".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                end_date.map_or("present".to_string(), |d| d.format("%Y-%m-%d").to_string())),
            generated_at: Utc::now().to_rfc3339(),
            data: serde_json::to_value(report)?,
        };
        
        Ok(report_data)
    }
    
    /// Generate a quiz performance report
    pub async fn generate_quiz_performance_report(
        &self,
        quiz_id: &str,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ReportData> {
        info!("Generating quiz performance report for quiz {}", quiz_id);
        
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND a.created_at BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND a.created_at >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND a.created_at <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Get quiz info
        let quiz = sqlx::query!(
            "SELECT id, title FROM quizzes WHERE id = ? AND deleted_at IS NULL",
            quiz_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Quiz not found"))?;
        
        // Get attempt summary
        let attempt_summary = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_attempts,
                SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) as completed_attempts,
                SUM(CASE WHEN status = 'abandoned' THEN 1 ELSE 0 END) as abandoned_attempts,
                AVG(score) as average_score,
                AVG(
                    CASE 
                        WHEN end_time IS NOT NULL AND start_time IS NOT NULL 
                        THEN strftime('%s', end_time) - strftime('%s', start_time) 
                        ELSE NULL 
                    END
                ) * 1000 as average_duration_ms
            FROM quiz_attempts
            WHERE quiz_id = ?
            "#.to_string() + &date_filter,
            quiz_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Get question performance
        let question_performance = sqlx::query!(
            r#"
            SELECT 
                q.id as question_id,
                q.question_text,
                COUNT(ua.id) as attempts,
                SUM(CASE WHEN ua.is_correct = 1 THEN 1 ELSE 0 END) as correct_answers,
                AVG(ua.duration_ms) as average_time_ms
            FROM questions q
            LEFT JOIN user_answers ua ON q.id = ua.question_id
            LEFT JOIN quiz_attempts a ON ua.attempt_id = a.id
            WHERE q.quiz_id = ?
            "#.to_string() + &date_filter + "
            GROUP BY q.id
            ORDER BY q.position",
            quiz_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Get user performance
        let user_performance = sqlx::query!(
            r#"
            SELECT 
                u.id as user_id,
                u.name as user_name,
                COUNT(a.id) as attempts,
                AVG(a.score) as score,
                AVG(
                    CASE 
                        WHEN a.end_time IS NOT NULL AND a.start_time IS NOT NULL 
                        THEN strftime('%s', a.end_time) - strftime('%s', a.start_time) 
                        ELSE NULL 
                    END
                ) * 1000 as duration_ms
            FROM users u
            JOIN quiz_attempts a ON u.id = a.user_id
            WHERE a.quiz_id = ?
            "#.to_string() + &date_filter + "
            GROUP BY u.id
            ORDER BY score DESC",
            quiz_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Convert question performance
        let question_performance_items = question_performance.into_iter().map(|row| {
            QuestionPerformanceItem {
                question_id: row.question_id,
                question_text: row.question_text,
                attempts: row.attempts,
                correct_answers: row.correct_answers,
                average_time_ms: row.average_time_ms.unwrap_or(0),
            }
        }).collect();
        
        // Convert user performance
        let user_performance_items = user_performance.into_iter().map(|row| {
            UserPerformanceItem {
                user_id: row.user_id,
                user_name: row.user_name,
                attempts: row.attempts,
                score: row.score.unwrap_or(0.0),
                duration_ms: row.duration_ms.unwrap_or(0),
            }
        }).collect();
        
        // Create report data
        let report = QuizPerformanceReport {
            quiz_id: quiz.id,
            quiz_title: quiz.title,
            total_attempts: attempt_summary.total_attempts,
            completed_attempts: attempt_summary.completed_attempts,
            abandoned_attempts: attempt_summary.abandoned_attempts,
            average_score: attempt_summary.average_score.unwrap_or(0.0),
            average_duration_ms: attempt_summary.average_duration_ms.unwrap_or(0),
            question_performance: question_performance_items,
            user_performance: user_performance_items,
        };
        
        // Create report data
        let report_data = ReportData {
            report_type: ReportType::QuizPerformance,
            title: format!("Quiz Performance Report - {}", quiz.title),
            description: format!("Performance report for quiz {} from {} to {}", 
                quiz.title, 
                start_date.map_or("all time".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                end_date.map_or("present".to_string(), |d| d.format("%Y-%m-%d").to_string())),
            generated_at: Utc::now().to_rfc3339(),
            data: serde_json::to_value(report)?,
        };
        
        Ok(report_data)
    }
    
    /// Generate a question analysis report
    pub async fn generate_question_analysis_report(
        &self,
        quiz_id: &str,
    ) -> Result<ReportData> {
        info!("Generating question analysis report for quiz {}", quiz_id);
        
        // Get quiz info
        let quiz = sqlx::query!(
            "SELECT id, title FROM quizzes WHERE id = ? AND deleted_at IS NULL",
            quiz_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("Quiz not found"))?;
        
        // Get questions
        let questions = sqlx::query!(
            r#"
            SELECT 
                q.id as question_id,
                q.question_text,
                q.question_type,
                COUNT(ua.id) as attempts,
                SUM(CASE WHEN ua.is_correct = 1 THEN 1 ELSE 0 END) as correct_answers,
                COUNT(ua.id) - SUM(CASE WHEN ua.is_correct = 1 THEN 1 ELSE 0 END) as incorrect_answers,
                AVG(ua.duration_ms) as average_time_ms
            FROM questions q
            LEFT JOIN user_answers ua ON q.id = ua.question_id
            WHERE q.quiz_id = ?
            GROUP BY q.id
            ORDER BY q.position
            "#,
            quiz_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Process each question
        let mut question_items = Vec::new();
        
        for question in questions {
            // Calculate difficulty index (p-value)
            let difficulty_index = if question.attempts > 0 {
                question.correct_answers as f64 / question.attempts as f64
            } else {
                0.0
            };
            
            // Get answer distribution
            let answer_distribution = sqlx::query!(
                r#"
                SELECT 
                    ao.option_text as answer,
                    COUNT(ua.id) as count
                FROM answer_options ao
                LEFT JOIN user_answers ua ON ao.id = ua.answer_option_id
                WHERE ao.question_id = ?
                GROUP BY ao.id
                ORDER BY ao.position
                "#,
                question.question_id
            )
            .fetch_all(&self.pool)
            .await?;
            
            // Convert to HashMap
            let mut distribution_map = HashMap::new();
            for row in answer_distribution {
                distribution_map.insert(row.answer, row.count);
            }
            
            // Calculate discrimination index
            // This is a simplified version - in a real implementation, you would need to
            // divide students into upper and lower groups based on total scores
            let discrimination_index = 0.0; // Placeholder
            
            question_items.push(QuestionAnalysisItem {
                question_id: question.question_id,
                question_text: question.question_text,
                question_type: question.question_type,
                attempts: question.attempts,
                correct_answers: question.correct_answers,
                incorrect_answers: question.incorrect_answers,
                average_time_ms: question.average_time_ms.unwrap_or(0),
                difficulty_index,
                discrimination_index,
                answer_distribution: distribution_map,
            });
        }
        
        // Create report data
        let report = QuestionAnalysisReport {
            quiz_id: quiz.id,
            quiz_title: quiz.title,
            questions: question_items,
        };
        
        // Create report data
        let report_data = ReportData {
            report_type: ReportType::QuestionAnalysis,
            title: format!("Question Analysis Report - {}", quiz.title),
            description: format!("Detailed analysis of questions in quiz {}", quiz.title),
            generated_at: Utc::now().to_rfc3339(),
            data: serde_json::to_value(report)?,
        };
        
        Ok(report_data)
    }
    
    /// Generate a time analysis report
    pub async fn generate_time_analysis_report(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ReportData> {
        info!("Generating time analysis report");
        
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND timestamp BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND timestamp >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND timestamp <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Build user filter
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
        // Build quiz filter
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get activity by day
        let activity_by_day = sqlx::query!(
            r#"
            SELECT 
                strftime('%Y-%m-%d', timestamp) as day,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + &date_filter + "
            GROUP BY day
            ORDER BY day"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Get activity by hour
        let activity_by_hour = sqlx::query!(
            r#"
            SELECT 
                CAST(strftime('%H', timestamp) AS INTEGER) as hour,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + &date_filter + "
            GROUP BY hour
            ORDER BY hour"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Get activity by day of week
        let activity_by_day_of_week = sqlx::query!(
            r#"
            SELECT 
                CASE strftime('%w', timestamp)
                    WHEN '0' THEN 'Sunday'
                    WHEN '1' THEN 'Monday'
                    WHEN '2' THEN 'Tuesday'
                    WHEN '3' THEN 'Wednesday'
                    WHEN '4' THEN 'Thursday'
                    WHEN '5' THEN 'Friday'
                    WHEN '6' THEN 'Saturday'
                END as day_of_week,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + &date_filter + "
            GROUP BY day_of_week
            ORDER BY strftime('%w', timestamp)"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Get average session duration
        let average_session_duration = sqlx::query!(
            r#"
            SELECT AVG(
                CASE 
                    WHEN end_time IS NOT NULL AND start_time IS NOT NULL 
                    THEN strftime('%s', end_time) - strftime('%s', start_time) 
                    ELSE NULL 
                END
            ) * 1000 as avg_duration
            FROM quiz_attempts
            WHERE 1=1
            "#.to_string() + &user_filter.replace("user_id", "a.user_id") + 
            &quiz_filter.replace("quiz_id", "a.quiz_id") + 
            &date_filter.replace("timestamp", "a.created_at")
        )
        .fetch_one(&self.pool)
        .await?
        .avg_duration
        .unwrap_or(0);
        
        // Convert to HashMaps
        let mut day_map = HashMap::new();
        for row in activity_by_day {
            if let Some(day) = row.day {
                day_map.insert(day, row.count);
            }
        }
        
        let mut hour_map = HashMap::new();
        for row in activity_by_hour {
            if let Some(hour) = row.hour {
                hour_map.insert(hour, row.count);
            }
        }
        
        let mut day_of_week_map = HashMap::new();
        for row in activity_by_day_of_week {
            if let Some(day_of_week) = row.day_of_week {
                day_of_week_map.insert(day_of_week, row.count);
            }
        }
        
        // Find peak activity hour
        let peak_hour = hour_map.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&hour, _)| hour)
            .unwrap_or(0);
        
        // Find peak activity day
        let peak_day = day_of_week_map.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(day, _)| day.clone())
            .unwrap_or("Unknown".to_string());
        
        // Create report data
        let report = TimeAnalysisReport {
            activity_by_day: day_map,
            activity_by_hour: hour_map,
            activity_by_day_of_week: day_of_week_map,
            average_session_duration_ms: average_session_duration,
            peak_activity_hour: peak_hour,
            peak_activity_day: peak_day,
        };
        
        // Create title and description
        let title = match (user_id, quiz_id) {
            (Some(uid), Some(qid)) => format!("Time Analysis Report - User {} - Quiz {}", uid, qid),
            (Some(uid), None) => format!("Time Analysis Report - User {}", uid),
            (None, Some(qid)) => format!("Time Analysis Report - Quiz {}", qid),
            (None, None) => "Time Analysis Report - All Users and Quizzes".to_string(),
        };
        
        let description = match (user_id, quiz_id) {
            (Some(uid), Some(qid)) => format!("Time analysis for user {} on quiz {} from {} to {}", 
                uid, qid,
                start_date.map_or("all time".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                end_date.map_or("present".to_string(), |d| d.format("%Y-%m-%d").to_string())),
            (Some(uid), None) => format!("Time analysis for user {} from {} to {}", 
                uid,
                start_date.map_or("all time".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                end_date.map_or("present".to_string(), |d| d.format("%Y-%m-%d").to_string())),
            (None, Some(qid)) => format!("Time analysis for quiz {} from {} to {}", 
                qid,
                start_date.map_or("all time".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                end_date.map_or("present".to_string(), |d| d.format("%Y-%m-%d").to_string())),
            (None, None) => format!("Time analysis for all users and quizzes from {} to {}", 
                start_date.map_or("all time".to_string(), |d| d.format("%Y-%m-%d").to_string()),
                end_date.map_or("present".to_string(), |d| d.format("%Y-%m-%d").to_string())),
        };
        
        // Create report data
        let report_data = ReportData {
            report_type: ReportType::TimeAnalysis,
            title,
            description,
            generated_at: Utc::now().to_rfc3339(),
            data: serde_json::to_value(report)?,
        };
        
        Ok(report_data)
    }
}
