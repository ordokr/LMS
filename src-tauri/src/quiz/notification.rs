use super::models::{Quiz, QuizAttempt};
use super::course_integration::{QuizCourseMapping, QuizAssignment, QuizAssignmentStatus};
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use crate::notification::NotificationService;

/// Quiz notification types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuizNotificationType {
    QuizAssigned,
    QuizDueSoon,
    QuizOverdue,
    QuizCompleted,
    QuizGraded,
    QuizFeedbackAvailable,
    QuizUpdated,
    QuizRemoved,
}

/// Quiz notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: QuizNotificationType,
    pub quiz_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub mapping_id: Option<Uuid>,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz notification service
pub struct QuizNotificationService {
    db_pool: SqlitePool,
    notification_service: Option<Arc<NotificationService>>,
}

impl QuizNotificationService {
    pub fn new(db_pool: SqlitePool, notification_service: Option<Arc<NotificationService>>) -> Self {
        Self {
            db_pool,
            notification_service,
        }
    }
    
    /// Send a notification when a quiz is assigned to a student
    pub async fn notify_quiz_assigned(
        &self,
        student_id: Uuid,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let title = format!("New Quiz Assigned: {}", quiz.title);
        let message = if let Some(due_date) = mapping.due_date {
            format!(
                "You have been assigned a new quiz: {}. Due date: {}",
                quiz.title,
                due_date.format("%B %d, %Y at %H:%M")
            )
        } else {
            format!("You have been assigned a new quiz: {}", quiz.title)
        };
        
        let link = format!("/courses/{}/quizzes/{}", mapping.course_id, quiz.id);
        
        self.create_notification(
            student_id,
            QuizNotificationType::QuizAssigned,
            title,
            message,
            Some(quiz.id),
            Some(mapping.course_id),
            Some(mapping.id),
            Some(link),
        ).await?;
        
        Ok(())
    }
    
    /// Send a notification when a quiz is due soon
    pub async fn notify_quiz_due_soon(
        &self,
        student_id: Uuid,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
        assignment: &QuizAssignment,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(due_date) = mapping.due_date {
            let now = Utc::now();
            let time_until_due = due_date.signed_duration_since(now);
            
            // Only send notification if the quiz is due within 24 hours and not completed
            if time_until_due > Duration::zero() && 
               time_until_due <= Duration::hours(24) && 
               assignment.status != QuizAssignmentStatus::Completed {
                
                let hours_until_due = time_until_due.num_hours();
                
                let title = format!("Quiz Due Soon: {}", quiz.title);
                let message = format!(
                    "Your quiz '{}' is due in {} hours. Please complete it before {}.",
                    quiz.title,
                    hours_until_due,
                    due_date.format("%B %d, %Y at %H:%M")
                );
                
                let link = format!("/courses/{}/quizzes/{}", mapping.course_id, quiz.id);
                
                self.create_notification(
                    student_id,
                    QuizNotificationType::QuizDueSoon,
                    title,
                    message,
                    Some(quiz.id),
                    Some(mapping.course_id),
                    Some(mapping.id),
                    Some(link),
                ).await?;
            }
        }
        
        Ok(())
    }
    
    /// Send a notification when a quiz is overdue
    pub async fn notify_quiz_overdue(
        &self,
        student_id: Uuid,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
        assignment: &QuizAssignment,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(due_date) = mapping.due_date {
            let now = Utc::now();
            
            // Only send notification if the quiz is overdue and not completed
            if now > due_date && assignment.status != QuizAssignmentStatus::Completed {
                let title = format!("Quiz Overdue: {}", quiz.title);
                let message = format!(
                    "Your quiz '{}' was due on {}. Please complete it as soon as possible.",
                    quiz.title,
                    due_date.format("%B %d, %Y at %H:%M")
                );
                
                let link = format!("/courses/{}/quizzes/{}", mapping.course_id, quiz.id);
                
                self.create_notification(
                    student_id,
                    QuizNotificationType::QuizOverdue,
                    title,
                    message,
                    Some(quiz.id),
                    Some(mapping.course_id),
                    Some(mapping.id),
                    Some(link),
                ).await?;
            }
        }
        
        Ok(())
    }
    
    /// Send a notification when a quiz is completed
    pub async fn notify_quiz_completed(
        &self,
        student_id: Uuid,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
        attempt: &QuizAttempt,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let title = format!("Quiz Completed: {}", quiz.title);
        
        let message = if let Some(score) = attempt.score {
            let passing_score = mapping.passing_score.unwrap_or(70.0);
            let passed = score >= passing_score;
            
            if passed {
                format!(
                    "Congratulations! You have completed the quiz '{}' with a score of {}%. You have passed!",
                    quiz.title,
                    score
                )
            } else {
                format!(
                    "You have completed the quiz '{}' with a score of {}%. The passing score is {}%.",
                    quiz.title,
                    score,
                    passing_score
                )
            }
        } else {
            format!("You have completed the quiz '{}'.", quiz.title)
        };
        
        let link = format!("/courses/{}/quizzes/{}/results", mapping.course_id, quiz.id);
        
        self.create_notification(
            student_id,
            QuizNotificationType::QuizCompleted,
            title,
            message,
            Some(quiz.id),
            Some(mapping.course_id),
            Some(mapping.id),
            Some(link),
        ).await?;
        
        Ok(())
    }
    
    /// Send a notification when a quiz is graded
    pub async fn notify_quiz_graded(
        &self,
        student_id: Uuid,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
        attempt: &QuizAttempt,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(score) = attempt.score {
            let title = format!("Quiz Graded: {}", quiz.title);
            
            let passing_score = mapping.passing_score.unwrap_or(70.0);
            let passed = score >= passing_score;
            
            let message = if passed {
                format!(
                    "Your quiz '{}' has been graded. Your score is {}%. Congratulations, you have passed!",
                    quiz.title,
                    score
                )
            } else {
                format!(
                    "Your quiz '{}' has been graded. Your score is {}%. The passing score is {}%.",
                    quiz.title,
                    score,
                    passing_score
                )
            };
            
            let link = format!("/courses/{}/quizzes/{}/results", mapping.course_id, quiz.id);
            
            self.create_notification(
                student_id,
                QuizNotificationType::QuizGraded,
                title,
                message,
                Some(quiz.id),
                Some(mapping.course_id),
                Some(mapping.id),
                Some(link),
            ).await?;
        }
        
        Ok(())
    }
    
    /// Send a notification when feedback is available for a quiz
    pub async fn notify_feedback_available(
        &self,
        student_id: Uuid,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let title = format!("Feedback Available: {}", quiz.title);
        let message = format!(
            "Feedback is now available for your quiz '{}'.",
            quiz.title
        );
        
        let link = format!("/courses/{}/quizzes/{}/feedback", mapping.course_id, quiz.id);
        
        self.create_notification(
            student_id,
            QuizNotificationType::QuizFeedbackAvailable,
            title,
            message,
            Some(quiz.id),
            Some(mapping.course_id),
            Some(mapping.id),
            Some(link),
        ).await?;
        
        Ok(())
    }
    
    /// Send a notification when a quiz is updated
    pub async fn notify_quiz_updated(
        &self,
        student_ids: Vec<Uuid>,
        quiz: &Quiz,
        mapping: &QuizCourseMapping,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let title = format!("Quiz Updated: {}", quiz.title);
        let message = format!(
            "The quiz '{}' has been updated. Please check for any changes.",
            quiz.title
        );
        
        let link = format!("/courses/{}/quizzes/{}", mapping.course_id, quiz.id);
        
        for student_id in student_ids {
            self.create_notification(
                student_id,
                QuizNotificationType::QuizUpdated,
                title.clone(),
                message.clone(),
                Some(quiz.id),
                Some(mapping.course_id),
                Some(mapping.id),
                Some(link.clone()),
            ).await?;
        }
        
        Ok(())
    }
    
    /// Send a notification when a quiz is removed
    pub async fn notify_quiz_removed(
        &self,
        student_ids: Vec<Uuid>,
        quiz_title: &str,
        course_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let title = format!("Quiz Removed: {}", quiz_title);
        let message = format!(
            "The quiz '{}' has been removed from your course.",
            quiz_title
        );
        
        let link = format!("/courses/{}", course_id);
        
        for student_id in student_ids {
            self.create_notification(
                student_id,
                QuizNotificationType::QuizRemoved,
                title.clone(),
                message.clone(),
                None,
                Some(course_id),
                None,
                Some(link.clone()),
            ).await?;
        }
        
        Ok(())
    }
    
    /// Create a notification
    async fn create_notification(
        &self,
        user_id: Uuid,
        notification_type: QuizNotificationType,
        title: String,
        message: String,
        quiz_id: Option<Uuid>,
        course_id: Option<Uuid>,
        mapping_id: Option<Uuid>,
        link: Option<String>,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        // Create notification in the database
        let notification_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Convert notification type to string
        let notification_type_str = match notification_type {
            QuizNotificationType::QuizAssigned => "QuizAssigned",
            QuizNotificationType::QuizDueSoon => "QuizDueSoon",
            QuizNotificationType::QuizOverdue => "QuizOverdue",
            QuizNotificationType::QuizCompleted => "QuizCompleted",
            QuizNotificationType::QuizGraded => "QuizGraded",
            QuizNotificationType::QuizFeedbackAvailable => "QuizFeedbackAvailable",
            QuizNotificationType::QuizUpdated => "QuizUpdated",
            QuizNotificationType::QuizRemoved => "QuizRemoved",
        };
        
        // Insert into database
        sqlx::query!(
            r#"
            INSERT INTO quiz_notifications
            (id, user_id, notification_type, quiz_id, course_id, mapping_id, title, message, link, read, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            notification_id.to_string(),
            user_id.to_string(),
            notification_type_str,
            quiz_id.map(|id| id.to_string()),
            course_id.map(|id| id.to_string()),
            mapping_id.map(|id| id.to_string()),
            title,
            message,
            link,
            false,
            now,
            now
        )
        .execute(&self.db_pool)
        .await?;
        
        // If the notification service is available, send the notification through it
        if let Some(notification_service) = &self.notification_service {
            notification_service.send_notification(
                user_id,
                &title,
                &message,
                link.as_deref(),
            ).await?;
        }
        
        Ok(notification_id)
    }
    
    /// Get notifications for a user
    pub async fn get_notifications_for_user(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<QuizNotification>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, notification_type, quiz_id, course_id, mapping_id, title, message, link, read, created_at, updated_at
            FROM quiz_notifications
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id.to_string(),
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut notifications = Vec::new();
        
        for row in rows {
            let notification_type = match row.notification_type.as_str() {
                "QuizAssigned" => QuizNotificationType::QuizAssigned,
                "QuizDueSoon" => QuizNotificationType::QuizDueSoon,
                "QuizOverdue" => QuizNotificationType::QuizOverdue,
                "QuizCompleted" => QuizNotificationType::QuizCompleted,
                "QuizGraded" => QuizNotificationType::QuizGraded,
                "QuizFeedbackAvailable" => QuizNotificationType::QuizFeedbackAvailable,
                "QuizUpdated" => QuizNotificationType::QuizUpdated,
                "QuizRemoved" => QuizNotificationType::QuizRemoved,
                _ => QuizNotificationType::QuizAssigned, // Default
            };
            
            let notification = QuizNotification {
                id: Uuid::parse_str(&row.id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                notification_type,
                quiz_id: row.quiz_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                course_id: row.course_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                mapping_id: row.mapping_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                title: row.title,
                message: row.message,
                link: row.link,
                read: row.read != 0,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            notifications.push(notification);
        }
        
        Ok(notifications)
    }
    
    /// Get unread notification count for a user
    pub async fn get_unread_count_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM quiz_notifications
            WHERE user_id = ? AND read = 0
            "#,
            user_id.to_string()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        Ok(row.count)
    }
    
    /// Mark a notification as read
    pub async fn mark_notification_as_read(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE quiz_notifications
            SET read = 1, updated_at = ?
            WHERE id = ?
            "#,
            now,
            notification_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Mark all notifications as read for a user
    pub async fn mark_all_notifications_as_read(
        &self,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE quiz_notifications
            SET read = 1, updated_at = ?
            WHERE user_id = ?
            "#,
            now,
            user_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Delete a notification
    pub async fn delete_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            DELETE FROM quiz_notifications
            WHERE id = ?
            "#,
            notification_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Delete all notifications for a user
    pub async fn delete_all_notifications_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            DELETE FROM quiz_notifications
            WHERE user_id = ?
            "#,
            user_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Check for due soon quizzes and send notifications
    pub async fn check_due_soon_quizzes(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get all quiz mappings with due dates in the next 24 hours
        let now = Utc::now();
        let tomorrow = now + Duration::hours(24);
        
        let rows = sqlx::query!(
            r#"
            SELECT m.id as mapping_id, m.quiz_id, m.course_id, m.due_date,
                   a.id as assignment_id, a.student_id, a.status
            FROM quiz_course_mappings m
            JOIN quiz_assignments a ON m.id = a.mapping_id
            WHERE m.due_date BETWEEN ? AND ?
            AND a.status != 'Completed'
            "#,
            now,
            tomorrow
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        for row in rows {
            let mapping_id = Uuid::parse_str(&row.mapping_id)?;
            let quiz_id = Uuid::parse_str(&row.quiz_id)?;
            let course_id = Uuid::parse_str(&row.course_id)?;
            let student_id = Uuid::parse_str(&row.student_id)?;
            
            // Get the quiz
            let quiz = self.get_quiz(quiz_id).await?;
            
            // Get the mapping
            let mapping = self.get_mapping(mapping_id).await?;
            
            // Get the assignment
            let assignment = self.get_assignment(mapping_id, student_id).await?;
            
            // Send notification
            self.notify_quiz_due_soon(student_id, &quiz, &mapping, &assignment).await?;
        }
        
        Ok(())
    }
    
    /// Check for overdue quizzes and send notifications
    pub async fn check_overdue_quizzes(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get all quiz mappings with due dates in the past
        let now = Utc::now();
        
        let rows = sqlx::query!(
            r#"
            SELECT m.id as mapping_id, m.quiz_id, m.course_id, m.due_date,
                   a.id as assignment_id, a.student_id, a.status
            FROM quiz_course_mappings m
            JOIN quiz_assignments a ON m.id = a.mapping_id
            WHERE m.due_date < ?
            AND a.status != 'Completed'
            AND a.status != 'Overdue'
            "#,
            now
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        for row in rows {
            let mapping_id = Uuid::parse_str(&row.mapping_id)?;
            let quiz_id = Uuid::parse_str(&row.quiz_id)?;
            let course_id = Uuid::parse_str(&row.course_id)?;
            let student_id = Uuid::parse_str(&row.student_id)?;
            
            // Get the quiz
            let quiz = self.get_quiz(quiz_id).await?;
            
            // Get the mapping
            let mapping = self.get_mapping(mapping_id).await?;
            
            // Get the assignment
            let assignment = self.get_assignment(mapping_id, student_id).await?;
            
            // Send notification
            self.notify_quiz_overdue(student_id, &quiz, &mapping, &assignment).await?;
            
            // Update assignment status to overdue
            self.update_assignment_status(mapping_id, student_id, QuizAssignmentStatus::Overdue).await?;
        }
        
        Ok(())
    }
    
    // Helper methods to get quiz, mapping, and assignment
    
    async fn get_quiz(&self, quiz_id: Uuid) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, created_at, updated_at, study_mode, visibility
            FROM quizzes
            WHERE id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        let quiz = Quiz {
            id: Uuid::parse_str(&row.id)?,
            title: row.title,
            description: row.description,
            author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
            created_at: row.created_at.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            updated_at: row.updated_at.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            questions: Vec::new(), // We don't need questions for notifications
            study_mode: row.study_mode.parse()?,
            visibility: row.visibility.parse()?,
            settings: Default::default(), // Default settings
        };
        
        Ok(quiz)
    }
    
    async fn get_mapping(&self, mapping_id: Uuid) -> Result<QuizCourseMapping, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, quiz_id, course_id, module_id, section_id, position, is_required,
                   passing_score, due_date, available_from, available_until, max_attempts,
                   time_limit, created_at, updated_at
            FROM quiz_course_mappings
            WHERE id = ?
            "#,
            mapping_id.to_string()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        let mapping = QuizCourseMapping {
            id: Uuid::parse_str(&row.id)?,
            quiz_id: Uuid::parse_str(&row.quiz_id)?,
            course_id: Uuid::parse_str(&row.course_id)?,
            module_id: row.module_id.map(|id| Uuid::parse_str(&id)).transpose()?,
            section_id: row.section_id.map(|id| Uuid::parse_str(&id)).transpose()?,
            position: row.position,
            is_required: row.is_required != 0,
            passing_score: row.passing_score,
            due_date: row.due_date.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            available_from: row.available_from.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            available_until: row.available_until.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            max_attempts: row.max_attempts,
            time_limit: row.time_limit,
            created_at: row.created_at.parse::<DateTime<Utc>>().unwrap(),
            updated_at: row.updated_at.parse::<DateTime<Utc>>().unwrap(),
        };
        
        Ok(mapping)
    }
    
    async fn get_assignment(&self, mapping_id: Uuid, student_id: Uuid) -> Result<QuizAssignment, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, mapping_id, student_id, status, attempts, best_score,
                   last_attempt_at, completed_at, created_at, updated_at
            FROM quiz_assignments
            WHERE mapping_id = ? AND student_id = ?
            "#,
            mapping_id.to_string(),
            student_id.to_string()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        let status = match row.status.as_str() {
            "NotStarted" => QuizAssignmentStatus::NotStarted,
            "InProgress" => QuizAssignmentStatus::InProgress,
            "Completed" => QuizAssignmentStatus::Completed,
            "Overdue" => QuizAssignmentStatus::Overdue,
            _ => QuizAssignmentStatus::NotStarted,
        };
        
        let assignment = QuizAssignment {
            id: Uuid::parse_str(&row.id)?,
            mapping_id: Uuid::parse_str(&row.mapping_id)?,
            student_id: Uuid::parse_str(&row.student_id)?,
            status,
            attempts: row.attempts,
            best_score: row.best_score,
            last_attempt_at: row.last_attempt_at.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            completed_at: row.completed_at.map(|d| d.parse::<DateTime<Utc>>().unwrap()),
            created_at: row.created_at.parse::<DateTime<Utc>>().unwrap(),
            updated_at: row.updated_at.parse::<DateTime<Utc>>().unwrap(),
        };
        
        Ok(assignment)
    }
    
    async fn update_assignment_status(
        &self,
        mapping_id: Uuid,
        student_id: Uuid,
        status: QuizAssignmentStatus,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let now = Utc::now();
        
        let status_str = match status {
            QuizAssignmentStatus::NotStarted => "NotStarted",
            QuizAssignmentStatus::InProgress => "InProgress",
            QuizAssignmentStatus::Completed => "Completed",
            QuizAssignmentStatus::Overdue => "Overdue",
        };
        
        sqlx::query!(
            r#"
            UPDATE quiz_assignments
            SET status = ?, updated_at = ?
            WHERE mapping_id = ? AND student_id = ?
            "#,
            status_str,
            now,
            mapping_id.to_string(),
            student_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
}
