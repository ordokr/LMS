use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Types of quiz notifications
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

/// Quiz notification model
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

impl QuizNotification {
    pub fn new(
        user_id: Uuid,
        notification_type: QuizNotificationType,
        title: String,
        message: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            quiz_id: None,
            course_id: None,
            mapping_id: None,
            title,
            message,
            link: None,
            read: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_quiz(mut self, quiz_id: Uuid) -> Self {
        self.quiz_id = Some(quiz_id);
        self
    }
    
    pub fn with_course(mut self, course_id: Uuid) -> Self {
        self.course_id = Some(course_id);
        self
    }
    
    pub fn with_mapping(mut self, mapping_id: Uuid) -> Self {
        self.mapping_id = Some(mapping_id);
        self
    }
    
    pub fn with_link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }
    
    pub fn mark_as_read(&mut self) {
        self.read = true;
        self.updated_at = Utc::now();
    }
}
