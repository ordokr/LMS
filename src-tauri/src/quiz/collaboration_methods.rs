use super::QuizEngine;
use super::collaboration::{CollaborationRole, QuizCollaborator, CollaborationInvitation, QuizEditHistory, QuizComment};
use uuid::Uuid;
use std::error::Error;

impl QuizEngine {
    // Collaborator methods
    
    /// Add a collaborator to a quiz
    pub async fn add_collaborator(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        role: CollaborationRole,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.add_collaborator(quiz_id, user_id, role).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Update a collaborator's role
    pub async fn update_collaborator_role(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        role: CollaborationRole,
        updated_by: Uuid,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.update_collaborator_role(quiz_id, user_id, role, updated_by).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Remove a collaborator from a quiz
    pub async fn remove_collaborator(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        removed_by: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.remove_collaborator(quiz_id, user_id, removed_by).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get a collaborator
    pub async fn get_collaborator(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_collaborator(quiz_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get all collaborators for a quiz
    pub async fn get_collaborators(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<QuizCollaborator>, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_collaborators(quiz_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    // Invitation methods
    
    /// Create a collaboration invitation for a user
    pub async fn invite_user(
        &self,
        quiz_id: Uuid,
        inviter_id: Uuid,
        invitee_id: Uuid,
        role: CollaborationRole,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.invite_user(quiz_id, inviter_id, invitee_id, role).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Create a collaboration invitation for an email
    pub async fn invite_by_email(
        &self,
        quiz_id: Uuid,
        inviter_id: Uuid,
        email: String,
        role: CollaborationRole,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.invite_by_email(quiz_id, inviter_id, email, role).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Accept a collaboration invitation
    pub async fn accept_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.accept_invitation(invitation_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Decline a collaboration invitation
    pub async fn decline_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.decline_invitation(invitation_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Cancel a collaboration invitation
    pub async fn cancel_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.cancel_invitation(invitation_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get a collaboration invitation
    pub async fn get_invitation(
        &self,
        invitation_id: Uuid,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_invitation(invitation_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get all invitations for a quiz
    pub async fn get_invitations_for_quiz(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<CollaborationInvitation>, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_invitations_for_quiz(quiz_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get all pending invitations for a user
    pub async fn get_pending_invitations_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CollaborationInvitation>, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_pending_invitations_for_user(user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    // Comment methods
    
    /// Add a comment to a quiz
    pub async fn add_comment(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.add_comment(quiz_id, user_id, content).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Add a comment to a specific question
    pub async fn add_question_comment(
        &self,
        quiz_id: Uuid,
        question_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.add_question_comment(quiz_id, question_id, user_id, content).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Add a reply to a comment
    pub async fn add_reply(
        &self,
        parent_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.add_reply(parent_id, user_id, content).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Update a comment
    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.update_comment(comment_id, user_id, content).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Delete a comment
    pub async fn delete_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.delete_comment(comment_id, user_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get a comment
    pub async fn get_comment(
        &self,
        comment_id: Uuid,
    ) -> Result<QuizComment, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_comment(comment_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get all comments for a quiz
    pub async fn get_comments_for_quiz(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<QuizComment>, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_comments_for_quiz(quiz_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get all comments for a question
    pub async fn get_comments_for_question(
        &self,
        question_id: Uuid,
    ) -> Result<Vec<QuizComment>, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_comments_for_question(question_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
    
    /// Get all replies to a comment
    pub async fn get_replies(
        &self,
        comment_id: Uuid,
    ) -> Result<Vec<QuizComment>, Box<dyn Error + Send + Sync>> {
        if let Some(collaboration_service) = &self.collaboration_service {
            collaboration_service.get_replies(comment_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("Collaboration service is not available".into())
        }
    }
}
