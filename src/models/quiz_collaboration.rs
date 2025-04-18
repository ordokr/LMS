use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Collaboration role for a quiz
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollaborationRole {
    Owner,
    Editor,
    Viewer,
}

/// Collaboration invitation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

/// Quiz collaborator model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizCollaborator {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub role: CollaborationRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz collaboration invitation model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationInvitation {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub inviter_id: Uuid,
    pub invitee_id: Option<Uuid>,
    pub invitee_email: Option<String>,
    pub role: CollaborationRole,
    pub status: InvitationStatus,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz edit history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizEditHistory {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub edit_type: String,
    pub description: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Quiz comment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizComment {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question_id: Option<Uuid>,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl QuizCollaborator {
    pub fn new(quiz_id: Uuid, user_id: Uuid, role: CollaborationRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            user_id,
            role,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl CollaborationInvitation {
    pub fn new(
        quiz_id: Uuid,
        inviter_id: Uuid,
        role: CollaborationRole,
    ) -> Self {
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::days(7);
        
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            inviter_id,
            invitee_id: None,
            invitee_email: None,
            role,
            status: InvitationStatus::Pending,
            token,
            expires_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.invitee_id = Some(user_id);
        self
    }
    
    pub fn with_email(mut self, email: String) -> Self {
        self.invitee_email = Some(email);
        self
    }
    
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

impl QuizEditHistory {
    pub fn new(
        quiz_id: Uuid,
        user_id: Uuid,
        edit_type: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            user_id,
            edit_type,
            description,
            details: None,
            created_at: Utc::now(),
        }
    }
    
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
}

impl QuizComment {
    pub fn new(
        quiz_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            question_id: None,
            user_id,
            parent_id: None,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn for_question(mut self, question_id: Uuid) -> Self {
        self.question_id = Some(question_id);
        self
    }
    
    pub fn as_reply(mut self, parent_id: Uuid) -> Self {
        self.parent_id = Some(parent_id);
        self
    }
}
