use crate::models::lms::{User, Topic, Post};
use crate::models::participation::{UserParticipation, ParticipationMetrics, TopicParticipation};
use crate::services::notification_service::NotificationService;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

/// User participation tracking service for discussions
pub struct UserParticipationService {
    db_pool: Arc<Pool<Postgres>>,
    notification_service: Arc<NotificationService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationReport {
    pub user_id: String,
    pub user_name: String,
    pub topics_created: usize,
    pub posts_created: usize,
    pub replies_to_others: usize,
    pub received_replies: usize,
    pub solutions_provided: usize,
    pub last_active_at: Option<DateTime<Utc>>,
    pub total_words_contributed: usize,
    pub avg_response_time_minutes: Option<f64>,
    pub participation_streak_days: usize,
    pub participation_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseParticipationSummary {
    pub course_id: String,
    pub total_participants: usize,
    pub active_participants: usize,
    pub total_topics: usize,
    pub total_posts: usize,
    pub avg_posts_per_user: f64,
    pub most_active_topics: Vec<TopicParticipationSummary>,
    pub most_active_users: Vec<UserParticipationSummary>,
    pub participation_by_day: Vec<DailyParticipation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicParticipationSummary {
    pub topic_id: String,
    pub title: String,
    pub participant_count: usize,
    pub post_count: usize,
    pub view_count: usize,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserParticipationSummary {
    pub user_id: String,
    pub user_name: String,
    pub post_count: usize,
    pub topic_count: usize,
    pub last_active_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyParticipation {
    pub date: String, // ISO date format
    pub post_count: usize,
    pub active_users: usize,
}

impl UserParticipationService {
    pub fn new(db_pool: Arc<Pool<Postgres>>, notification_service: Arc<NotificationService>) -> Self {
        Self {
            db_pool,
            notification_service,
        }
    }
    
    /// Record user participation event for a topic
    pub async fn record_topic_view(&self, user_id: &str, topic_id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_participation_events 
            (user_id, topic_id, event_type, created_at)
            VALUES ($1, $2, 'view', $3)
            ON CONFLICT (user_id, topic_id, event_type, DATE(created_at))
            DO UPDATE SET count = user_participation_events.count + 1,
                         created_at = $3
            "#,
            user_id,
            topic_id,
            Utc::now()
        )
        .execute(&*self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Record user creating a topic
    pub async fn record_topic_creation(&self, user_id: &str, topic_id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_participation_events 
            (user_id, topic_id, event_type, created_at)
            VALUES ($1, $2, 'create_topic', $3)
            "#,
            user_id,
            topic_id,
            Utc::now()
        )
        .execute(&*self.db_pool)
        .await?;
        
        // Update user participation metrics
        self.update_user_participation_metrics(user_id).await?;
        
        Ok(())
    }
    
    /// Record user creating a post (reply)
    pub async fn record_post_creation(&self, user_id: &str, topic_id: &str, post_id: &str, parent_post_id: Option<&str>) -> Result<()> {
        let event_type = if parent_post_id.is_some() { "reply" } else { "post" };
        
        sqlx::query!(
            r#"
            INSERT INTO user_participation_events 
            (user_id, topic_id, post_id, parent_post_id, event_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_id,
            topic_id,
            post_id,
            parent_post_id,
            event_type,
            Utc::now()
        )
        .execute(&*self.db_pool)
        .await?;
        
        // Update user participation metrics
        self.update_user_participation_metrics(user_id).await?;
        
        Ok(())
    }
    
    /// Record when a post is marked as a solution
    pub async fn record_solution(&self, user_id: &str, topic_id: &str, post_id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_participation_events 
            (user_id, topic_id, post_id, event_type, created_at)
            VALUES ($1, $2, $3, 'solution', $4)
            "#,
            user_id,
            topic_id,
            post_id,
            Utc::now()
        )
        .execute(&*self.db_pool)
        .await?;
        
        // Update user participation metrics
        self.update_user_participation_metrics(user_id).await?;
        
        // Notify user that their post was marked as a solution
        let notification = serde_json::json!({
            "type": "solution_marked",
            "userId": user_id,
            "topicId": topic_id,
            "postId": post_id,
            "timestamp": Utc::now()
        });
        
        self.notification_service.send_notification(user_id, "Your post was marked as a solution", &notification.to_string()).await?;
        
        Ok(())
    }
    
    /// Update user participation metrics
    async fn update_user_participation_metrics(&self, user_id: &str) -> Result<()> {
        // Get user's participation counts
        let topics_created = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events
            WHERE user_id = $1 AND event_type = 'create_topic'
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as i32;
        
        let posts_created = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events
            WHERE user_id = $1 AND (event_type = 'post' OR event_type = 'reply')
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as i32;
        
        let solutions_provided = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events
            WHERE user_id = $1 AND event_type = 'solution'
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as i32;
        
        let last_active = sqlx::query!(
            r#"
            SELECT MAX(created_at) as last_active
            FROM user_participation_events
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .last_active;
        
        // Update or insert metrics
        sqlx::query!(
            r#"
            INSERT INTO user_participation_metrics
            (user_id, topics_created, posts_created, solutions_provided, last_active_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id)
            DO UPDATE SET 
                topics_created = $2,
                posts_created = $3,
                solutions_provided = $4,
                last_active_at = $5,
                updated_at = $6
            "#,
            user_id,
            topics_created,
            posts_created,
            solutions_provided,
            last_active,
            Utc::now()
        )
        .execute(&*self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Get participation report for a user
    pub async fn get_user_participation_report(&self, user_id: &str) -> Result<ParticipationReport> {
        // Get user metrics
        let metrics = sqlx::query!(
            r#"
            SELECT 
                user_id,
                topics_created,
                posts_created,
                solutions_provided,
                last_active_at
            FROM user_participation_metrics
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&*self.db_pool)
        .await?;
        
        // Get user name
        let user = sqlx::query!(
            r#"
            SELECT name FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&*self.db_pool)
        .await?;
        
        let user_name = user.map(|u| u.name).unwrap_or_else(|| "Unknown User".to_string());
        
        // Get replies to others count
        let replies_to_others = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events
            WHERE user_id = $1 AND event_type = 'reply'
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        
        // Get received replies count
        let received_replies = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events e1
            JOIN user_participation_events e2 ON e1.post_id = e2.parent_post_id
            WHERE e1.user_id = $1 AND e2.user_id != $1 AND e2.event_type = 'reply'
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        
        // Calculate participation streak
        let streak_days = self.calculate_participation_streak(user_id).await?;
        
        if let Some(metrics) = metrics {
            // Calculate participation score (weighted metric)
            let participation_score = 
                (metrics.topics_created as f64 * 5.0) +  // Creating topics is highly valued
                (metrics.posts_created as f64 * 1.0) +   // Basic posting
                (replies_to_others as f64 * 2.0) +       // Engaging with others
                (received_replies as f64 * 0.5) +        // Content that drives engagement
                (metrics.solutions_provided as f64 * 10.0) + // Solutions are highly valued
                (streak_days as f64 * 0.5);              // Consistency bonus
            
            Ok(ParticipationReport {
                user_id: user_id.to_string(),
                user_name,
                topics_created: metrics.topics_created as usize,
                posts_created: metrics.posts_created as usize,
                replies_to_others,
                received_replies,
                solutions_provided: metrics.solutions_provided as usize,
                last_active_at: metrics.last_active_at,
                total_words_contributed: self.calculate_total_words(user_id).await?,
                avg_response_time_minutes: self.calculate_avg_response_time(user_id).await?,
                participation_streak_days: streak_days,
                participation_score,
            })
        } else {
            // User has no metrics yet
            Ok(ParticipationReport {
                user_id: user_id.to_string(),
                user_name,
                topics_created: 0,
                posts_created: 0,
                replies_to_others: 0,
                received_replies: 0,
                solutions_provided: 0,
                last_active_at: None,
                total_words_contributed: 0,
                avg_response_time_minutes: None,
                participation_streak_days: 0,
                participation_score: 0.0,
            })
        }
    }
    
    /// Calculate the total words contributed by a user
    async fn calculate_total_words(&self, user_id: &str) -> Result<usize> {
        // This would normally join with the posts table to get content and count words
        // For now, we'll use a placeholder implementation
        let posts = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events
            WHERE user_id = $1 AND (event_type = 'post' OR event_type = 'reply')
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        
        // Assuming average 50 words per post as placeholder
        Ok(posts * 50)
    }
    
    /// Calculate the average response time for a user
    async fn calculate_avg_response_time(&self, user_id: &str) -> Result<Option<f64>> {
        // This would normally calculate time between a post and the user's reply
        // For now, we'll use a placeholder implementation
        let reply_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_participation_events
            WHERE user_id = $1 AND event_type = 'reply'
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        
        if reply_count > 0 {
            // Placeholder: random average between 10 and 120 minutes
            let avg_minutes = 10.0 + (user_id.len() as f64 % 110.0);
            Ok(Some(avg_minutes))
        } else {
            Ok(None)
        }
    }
    
    /// Calculate the participation streak in days
    async fn calculate_participation_streak(&self, user_id: &str) -> Result<usize> {
        let activities = sqlx::query!(
            r#"
            SELECT DISTINCT DATE(created_at) as activity_date
            FROM user_participation_events
            WHERE user_id = $1
            ORDER BY activity_date DESC
            "#,
            user_id
        )
        .fetch_all(&*self.db_pool)
        .await?;
        
        if activities.is_empty() {
            return Ok(0);
        }
        
        let mut streak = 1;
        let mut current_date = activities[0].activity_date.unwrap();
        
        for i in 1..activities.len() {
            let next_date = activities[i].activity_date.unwrap();
            let expected_date = current_date - Duration::days(1);
            
            if next_date == expected_date {
                streak += 1;
                current_date = next_date;
            } else {
                break;
            }
        }
        
        Ok(streak)
    }
    
    /// Get participation summary for a course
    pub async fn get_course_participation_summary(&self, course_id: &str) -> Result<CourseParticipationSummary> {
        // Get course topics
        let topics = sqlx::query!(
            r#"
            SELECT 
                id, 
                title, 
                created_at,
                posts_count,
                views
            FROM topics
            WHERE course_id = $1
            ORDER BY created_at DESC
            "#,
            course_id
        )
        .fetch_all(&*self.db_pool)
        .await?;
        
        let total_topics = topics.len();
        
        // Get participants
        let participants = sqlx::query!(
            r#"
            SELECT 
                DISTINCT u.id, 
                u.name,
                COUNT(p.id) as post_count,
                MAX(p.created_at) as last_active_at
            FROM users u
            JOIN posts p ON u.id = p.user_id
            JOIN topics t ON p.topic_id = t.id
            WHERE t.course_id = $1
            GROUP BY u.id, u.name
            ORDER BY post_count DESC
            "#,
            course_id
        )
        .fetch_all(&*self.db_pool)
        .await?;
        
        let total_participants = participants.len();
        
        // Get active participants (posted in last 14 days)
        let two_weeks_ago = Utc::now() - Duration::days(14);
        let active_participants = participants.iter()
            .filter(|p| p.last_active_at.map_or(false, |date| date > two_weeks_ago))
            .count();
        
        // Calculate total posts
        let total_posts: i64 = participants.iter()
            .map(|p| p.post_count.unwrap_or(0))
            .sum();
        
        // Calculate average posts per user
        let avg_posts_per_user = if total_participants > 0 {
            total_posts as f64 / total_participants as f64
        } else {
            0.0
        };
        
        // Get most active topics
        let most_active_topics: Vec<TopicParticipationSummary> = topics.iter()
            .take(5)
            .map(|t| {
                let participant_count = self.get_topic_participant_count(&t.id).await.unwrap_or(0);
                
                TopicParticipationSummary {
                    topic_id: t.id.clone(),
                    title: t.title.clone(),
                    participant_count,
                    post_count: t.posts_count.unwrap_or(0) as usize,
                    view_count: t.views.unwrap_or(0) as usize,
                    created_at: t.created_at,
                }
            })
            .collect();
        
        // Get most active users
        let most_active_users: Vec<UserParticipationSummary> = participants.iter()
            .take(5)
            .map(|p| {
                let topic_count = self.get_user_topic_count(&p.id).await.unwrap_or(0);
                
                UserParticipationSummary {
                    user_id: p.id.clone(),
                    user_name: p.name.clone(),
                    post_count: p.post_count.unwrap_or(0) as usize,
                    topic_count,
                    last_active_at: p.last_active_at,
                }
            })
            .collect();
        
        // Get participation by day (last 7 days)
        let participation_by_day = self.get_daily_participation(course_id).await?;
        
        Ok(CourseParticipationSummary {
            course_id: course_id.to_string(),
            total_participants,
            active_participants,
            total_topics,
            total_posts: total_posts as usize,
            avg_posts_per_user,
            most_active_topics,
            most_active_users,
            participation_by_day,
        })
    }
    
    /// Get the number of participants for a topic
    async fn get_topic_participant_count(&self, topic_id: &str) -> Result<usize> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(DISTINCT user_id) as count
            FROM posts
            WHERE topic_id = $1
            "#,
            topic_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        
        Ok(count)
    }
    
    /// Get the number of topics a user has created
    async fn get_user_topic_count(&self, user_id: &str) -> Result<usize> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM topics
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        
        Ok(count)
    }
    
    /// Get daily participation for a course
    async fn get_daily_participation(&self, course_id: &str) -> Result<Vec<DailyParticipation>> {
        let now = Utc::now();
        let mut result = Vec::new();
        
        // Get data for the last 7 days
        for i in 0..7 {
            let date = now - Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();
            
            let day_stats = sqlx::query!(
                r#"
                SELECT 
                    COUNT(*) as post_count,
                    COUNT(DISTINCT user_id) as user_count
                FROM posts p
                JOIN topics t ON p.topic_id = t.id
                WHERE t.course_id = $1
                AND DATE(p.created_at) = $2::date
                "#,
                course_id,
                date_str
            )
            .fetch_one(&*self.db_pool)
            .await?;
            
            result.push(DailyParticipation {
                date: date_str,
                post_count: day_stats.post_count.unwrap_or(0) as usize,
                active_users: day_stats.user_count.unwrap_or(0) as usize,
            });
        }
        
        Ok(result)
    }
    
    /// Initialize database schema for participation tracking
    pub async fn initialize_schema(&self) -> Result<()> {
        // Create events table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_participation_events (
                id SERIAL PRIMARY KEY,
                user_id VARCHAR(255) NOT NULL,
                topic_id VARCHAR(255) NOT NULL,
                post_id VARCHAR(255),
                parent_post_id VARCHAR(255),
                event_type VARCHAR(50) NOT NULL,
                count INTEGER DEFAULT 1,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                UNIQUE(user_id, topic_id, event_type, DATE(created_at))
            );
            "#
        )
        .execute(&*self.db_pool)
        .await?;
        
        // Create participation metrics table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_participation_metrics (
                user_id VARCHAR(255) PRIMARY KEY,
                topics_created INTEGER NOT NULL DEFAULT 0,
                posts_created INTEGER NOT NULL DEFAULT 0,
                solutions_provided INTEGER NOT NULL DEFAULT 0,
                last_active_at TIMESTAMP WITH TIME ZONE,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL
            );
            "#
        )
        .execute(&*self.db_pool)
        .await?;
        
        // Create indexes
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_user_participation_events_user_id ON user_participation_events(user_id);
            CREATE INDEX IF NOT EXISTS idx_user_participation_events_topic_id ON user_participation_events(topic_id);
            CREATE INDEX IF NOT EXISTS idx_user_participation_events_created_at ON user_participation_events(created_at);
            CREATE INDEX IF NOT EXISTS idx_user_participation_events_event_type ON user_participation_events(event_type);
            "#
        )
        .execute(&*self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Ensure database schema exists
    pub async fn ensure_schema_exists(&self) -> Result<()> {
        match self.initialize_schema().await {
            Ok(_) => {
                info!("User participation schema initialized");
                Ok(())
            }
            Err(e) => {
                error!("Failed to initialize user participation schema: {}", e);
                Err(anyhow!("Failed to initialize user participation schema: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    
    // Mock for DB pool
    mock! {
        PgPool {}
        impl Clone for PgPool {
            fn clone(&self) -> Self;
        }
    }
    
    // Mock for notification service
    mock! {
        NotificationService {}
        impl NotificationService {
            fn send_notification(&self, user_id: &str, title: &str, content: &str) -> Result<()>;
        }
    }
    
    #[tokio::test]
    async fn test_record_topic_view() {
        // Implement test
    }
    
    #[tokio::test]
    async fn test_get_user_participation_report() {
        // Implement test
    }
}
