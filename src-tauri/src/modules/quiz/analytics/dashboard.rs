use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;
use sqlx::SqlitePool;
use tracing::{info, error};

use crate::models::quiz::{
    QuizActivityStats, QuizActivitySummary, ActivityType,
    Quiz, Question, QuizAttempt, UserAnswer
};

/// Dashboard data for the analytics dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    pub activity_stats: QuizActivityStats,
    pub quiz_summaries: Vec<QuizSummaryData>,
    pub user_summaries: Vec<UserSummaryData>,
    pub recent_activities: Vec<ActivityData>,
    pub time_analysis: TimeAnalysisData,
}

/// Quiz summary data for the dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizSummaryData {
    pub quiz_id: String,
    pub title: String,
    pub total_attempts: i64,
    pub completion_rate: f64,
    pub average_score: f64,
    pub average_duration_ms: i64,
}

/// User summary data for the dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSummaryData {
    pub user_id: String,
    pub name: String,
    pub total_activities: i64,
    pub total_quizzes_completed: i64,
    pub total_study_time_ms: i64,
    pub average_score: f64,
}

/// Activity data for the dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityData {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub quiz_id: Option<String>,
    pub quiz_title: Option<String>,
    pub activity_type: String,
    pub timestamp: String,
    pub duration_ms: Option<i64>,
    pub data: Option<serde_json::Value>,
}

/// Time analysis data for the dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeAnalysisData {
    pub activity_by_day: HashMap<String, i64>,
    pub activity_by_hour: HashMap<i64, i64>,
    pub activity_by_day_of_week: HashMap<String, i64>,
}

/// Analytics dashboard service
pub struct AnalyticsDashboard {
    pool: SqlitePool,
}

impl AnalyticsDashboard {
    /// Create a new analytics dashboard
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Get dashboard data
    pub async fn get_dashboard_data(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<DashboardData> {
        info!("Getting dashboard data");
        
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
        
        // Get activity stats
        let activity_stats = self.get_activity_stats(user_id, quiz_id, &date_filter).await?;
        
        // Get quiz summaries
        let quiz_summaries = self.get_quiz_summaries(user_id, &date_filter).await?;
        
        // Get user summaries
        let user_summaries = self.get_user_summaries(quiz_id, &date_filter).await?;
        
        // Get recent activities
        let recent_activities = self.get_recent_activities(user_id, quiz_id, &date_filter).await?;
        
        // Get time analysis
        let time_analysis = self.get_time_analysis(user_id, quiz_id, &date_filter).await?;
        
        Ok(DashboardData {
            activity_stats,
            quiz_summaries,
            user_summaries,
            recent_activities,
            time_analysis,
        })
    }
    
    /// Get activity stats
    async fn get_activity_stats(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        date_filter: &str,
    ) -> Result<QuizActivityStats> {
        // Build filters
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get quiz stats
        let quiz_stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_started' THEN quiz_id END) as quizzes_started,
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_completed' THEN quiz_id END) as quizzes_completed,
                COUNT(CASE WHEN activity_type = 'question_answered' THEN 1 END) as questions_answered,
                COALESCE(SUM(duration_ms), 0) as total_study_time
            FROM quiz_activities
            WHERE 1=1 
            "#.to_string() + &user_filter + &quiz_filter + date_filter
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Get average quiz duration
        let avg_quiz_duration = sqlx::query!(
            r#"
            SELECT AVG(duration_ms) as avg_duration
            FROM quiz_activities
            WHERE activity_type = 'quiz_completed' AND duration_ms IS NOT NULL
            "#.to_string() + &user_filter + &quiz_filter + date_filter
        )
        .fetch_one(&self.pool)
        .await?
        .avg_duration;
        
        // Get average question time
        let avg_question_time = sqlx::query!(
            r#"
            SELECT AVG(duration_ms) as avg_duration
            FROM quiz_activities
            WHERE activity_type = 'question_answered' AND duration_ms IS NOT NULL
            "#.to_string() + &user_filter + &quiz_filter + date_filter
        )
        .fetch_one(&self.pool)
        .await?
        .avg_duration;
        
        // Get activity by day
        let activity_by_day = sqlx::query!(
            r#"
            SELECT 
                strftime('%Y-%m-%d', timestamp) as day,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + date_filter + "
            GROUP BY day
            ORDER BY day"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Convert to HashMap
        let mut day_map = HashMap::new();
        for row in activity_by_day {
            if let Some(day) = row.day {
                day_map.insert(day, row.count);
            }
        }
        
        Ok(QuizActivityStats {
            total_quizzes_started: quiz_stats.quizzes_started,
            total_quizzes_completed: quiz_stats.quizzes_completed,
            total_questions_answered: quiz_stats.questions_answered,
            total_study_time_ms: quiz_stats.total_study_time,
            average_quiz_duration_ms: avg_quiz_duration,
            average_question_time_ms: avg_question_time,
            activity_by_day: serde_json::to_value(day_map)?,
        })
    }
    
    /// Get quiz summaries
    async fn get_quiz_summaries(
        &self,
        user_id: Option<&str>,
        date_filter: &str,
    ) -> Result<Vec<QuizSummaryData>> {
        // Build user filter
        let user_filter = match user_id {
            Some(id) => format!("AND a.user_id = '{}'", id),
            None => String::new(),
        };
        
        // Get quiz summaries
        let rows = sqlx::query!(
            r#"
            SELECT 
                q.id as quiz_id,
                q.title,
                COUNT(DISTINCT a.id) as total_attempts,
                SUM(CASE WHEN a.status = 'completed' THEN 1 ELSE 0 END) as completed_attempts,
                AVG(a.score) as average_score,
                AVG(
                    CASE 
                        WHEN a.end_time IS NOT NULL AND a.start_time IS NOT NULL 
                        THEN strftime('%s', a.end_time) - strftime('%s', a.start_time) 
                        ELSE NULL 
                    END
                ) * 1000 as average_duration_ms
            FROM quizzes q
            LEFT JOIN quiz_attempts a ON q.id = a.quiz_id
            WHERE q.deleted_at IS NULL
            "#.to_string() + &user_filter + date_filter + "
            GROUP BY q.id
            ORDER BY total_attempts DESC
            LIMIT 10"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Convert to QuizSummaryData
        let summaries = rows.into_iter().map(|row| {
            let completion_rate = if row.total_attempts > 0 {
                (row.completed_attempts as f64) / (row.total_attempts as f64)
            } else {
                0.0
            };
            
            QuizSummaryData {
                quiz_id: row.quiz_id,
                title: row.title,
                total_attempts: row.total_attempts,
                completion_rate,
                average_score: row.average_score.unwrap_or(0.0),
                average_duration_ms: row.average_duration_ms.unwrap_or(0),
            }
        }).collect();
        
        Ok(summaries)
    }
    
    /// Get user summaries
    async fn get_user_summaries(
        &self,
        quiz_id: Option<&str>,
        date_filter: &str,
    ) -> Result<Vec<UserSummaryData>> {
        // Build quiz filter
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND a.quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get user summaries
        let rows = sqlx::query!(
            r#"
            SELECT 
                u.id as user_id,
                u.name,
                COUNT(DISTINCT act.id) as total_activities,
                COUNT(DISTINCT CASE WHEN a.status = 'completed' THEN a.id END) as completed_quizzes,
                COALESCE(SUM(act.duration_ms), 0) as total_study_time,
                AVG(a.score) as average_score
            FROM users u
            LEFT JOIN quiz_activities act ON u.id = act.user_id
            LEFT JOIN quiz_attempts a ON u.id = a.user_id
            WHERE 1=1
            "#.to_string() + &quiz_filter + date_filter + "
            GROUP BY u.id
            ORDER BY total_activities DESC
            LIMIT 10"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Convert to UserSummaryData
        let summaries = rows.into_iter().map(|row| {
            UserSummaryData {
                user_id: row.user_id,
                name: row.name,
                total_activities: row.total_activities,
                total_quizzes_completed: row.completed_quizzes,
                total_study_time_ms: row.total_study_time,
                average_score: row.average_score.unwrap_or(0.0),
            }
        }).collect();
        
        Ok(summaries)
    }
    
    /// Get recent activities
    async fn get_recent_activities(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        date_filter: &str,
    ) -> Result<Vec<ActivityData>> {
        // Build filters
        let user_filter = match user_id {
            Some(id) => format!("AND a.user_id = '{}'", id),
            None => String::new(),
        };
        
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND a.quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get recent activities
        let rows = sqlx::query!(
            r#"
            SELECT 
                a.id,
                a.user_id,
                u.name as user_name,
                a.quiz_id,
                q.title as quiz_title,
                a.activity_type,
                a.timestamp,
                a.duration_ms,
                a.data
            FROM quiz_activities a
            LEFT JOIN users u ON a.user_id = u.id
            LEFT JOIN quizzes q ON a.quiz_id = q.id
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + date_filter + "
            ORDER BY a.timestamp DESC
            LIMIT 50"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Convert to ActivityData
        let activities = rows.into_iter().map(|row| {
            let data = row.data.map(|d| serde_json::from_str(&d).unwrap_or(serde_json::Value::Null));
            
            ActivityData {
                id: row.id,
                user_id: row.user_id,
                user_name: row.user_name.unwrap_or_else(|| "Unknown".to_string()),
                quiz_id: row.quiz_id,
                quiz_title: row.quiz_title,
                activity_type: row.activity_type,
                timestamp: row.timestamp,
                duration_ms: row.duration_ms,
                data,
            }
        }).collect();
        
        Ok(activities)
    }
    
    /// Get time analysis
    async fn get_time_analysis(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        date_filter: &str,
    ) -> Result<TimeAnalysisData> {
        // Build filters
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
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
            "#.to_string() + &user_filter + &quiz_filter + date_filter + "
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
            "#.to_string() + &user_filter + &quiz_filter + date_filter + "
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
            "#.to_string() + &user_filter + &quiz_filter + date_filter + "
            GROUP BY day_of_week
            ORDER BY strftime('%w', timestamp)"
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
        
        Ok(TimeAnalysisData {
            activity_by_day: day_map,
            activity_by_hour: hour_map,
            activity_by_day_of_week: day_of_week_map,
        })
    }
}
