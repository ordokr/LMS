use super::models::{Quiz, Question};
use super::storage::HybridQuizStore;
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

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

/// Quiz collaboration service
pub struct CollaborationService {
    db_pool: SqlitePool,
    quiz_store: Arc<HybridQuizStore>,
}

impl CollaborationService {
    pub fn new(db_pool: SqlitePool, quiz_store: Arc<HybridQuizStore>) -> Self {
        Self {
            db_pool,
            quiz_store,
        }
    }

    /// Add a collaborator to a quiz
    pub async fn add_collaborator(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        role: CollaborationRole,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        // Check if the user is already a collaborator
        let existing = self.get_collaborator(quiz_id, user_id).await;

        if let Ok(_) = existing {
            return Err("User is already a collaborator".into());
        }

        // Create a new collaborator
        let collaborator = QuizCollaborator {
            id: Uuid::new_v4(),
            quiz_id,
            user_id,
            role,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store the collaborator
        self.store_collaborator(&collaborator).await?;

        // Add an edit history entry
        let edit_history = QuizEditHistory {
            id: Uuid::new_v4(),
            quiz_id,
            user_id,
            edit_type: "add_collaborator".to_string(),
            description: format!("Added as {:?}", role),
            details: None,
            created_at: Utc::now(),
        };

        self.store_edit_history(&edit_history).await?;

        Ok(collaborator)
    }

    /// Update a collaborator's role
    pub async fn update_collaborator_role(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        role: CollaborationRole,
        updated_by: Uuid,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        // Get the existing collaborator
        let mut collaborator = self.get_collaborator(quiz_id, user_id).await?;

        // Update the role
        collaborator.role = role;
        collaborator.updated_at = Utc::now();

        // Store the updated collaborator
        self.store_collaborator(&collaborator).await?;

        // Add an edit history entry
        let edit_history = QuizEditHistory {
            id: Uuid::new_v4(),
            quiz_id,
            user_id: updated_by,
            edit_type: "update_collaborator".to_string(),
            description: format!("Updated role to {:?}", role),
            details: Some(format!("User ID: {}", user_id)),
            created_at: Utc::now(),
        };

        self.store_edit_history(&edit_history).await?;

        Ok(collaborator)
    }

    /// Remove a collaborator from a quiz
    pub async fn remove_collaborator(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        removed_by: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check if the user is a collaborator
        let collaborator = self.get_collaborator(quiz_id, user_id).await?;

        // Cannot remove the owner
        if collaborator.role == CollaborationRole::Owner {
            return Err("Cannot remove the owner".into());
        }

        // Delete the collaborator
        sqlx::query!(
            r#"
            DELETE FROM quiz_collaborators
            WHERE quiz_id = ? AND user_id = ?
            "#,
            quiz_id.to_string(),
            user_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        // Add an edit history entry
        let edit_history = QuizEditHistory {
            id: Uuid::new_v4(),
            quiz_id,
            user_id: removed_by,
            edit_type: "remove_collaborator".to_string(),
            description: "Removed collaborator".to_string(),
            details: Some(format!("User ID: {}", user_id)),
            created_at: Utc::now(),
        };

        self.store_edit_history(&edit_history).await?;

        Ok(())
    }

    /// Get a collaborator
    pub async fn get_collaborator(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, quiz_id, user_id, role, created_at, updated_at
            FROM quiz_collaborators
            WHERE quiz_id = ? AND user_id = ?
            "#,
            quiz_id.to_string(),
            user_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = row {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let collaborator = QuizCollaborator {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                role,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            Ok(collaborator)
        } else {
            Err("Collaborator not found".into())
        }
    }

    /// Get all collaborators for a quiz
    pub async fn get_collaborators(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<QuizCollaborator>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, user_id, role, created_at, updated_at
            FROM quiz_collaborators
            WHERE quiz_id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;

        let mut collaborators = Vec::new();

        for row in rows {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let collaborator = QuizCollaborator {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                user_id: Uuid::parse_str(&row.user_id)?,
                role,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            collaborators.push(collaborator);
        }

        Ok(collaborators)
    }

    /// Store a collaborator
    async fn store_collaborator(
        &self,
        collaborator: &QuizCollaborator,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let role_str = match collaborator.role {
            CollaborationRole::Owner => "Owner",
            CollaborationRole::Editor => "Editor",
            CollaborationRole::Viewer => "Viewer",
        };

        // Check if the collaborator already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_collaborators
            WHERE id = ?
            "#,
            collaborator.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;

        if existing.is_some() {
            // Update existing collaborator
            sqlx::query!(
                r#"
                UPDATE quiz_collaborators
                SET role = ?, updated_at = ?
                WHERE id = ?
                "#,
                role_str,
                collaborator.updated_at,
                collaborator.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new collaborator
            sqlx::query!(
                r#"
                INSERT INTO quiz_collaborators
                (id, quiz_id, user_id, role, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                collaborator.id.to_string(),
                collaborator.quiz_id.to_string(),
                collaborator.user_id.to_string(),
                role_str,
                collaborator.created_at,
                collaborator.updated_at
            )
            .execute(&self.db_pool)
            .await?;
        }

        Ok(())
    }

    /// Store an edit history entry
    async fn store_edit_history(
        &self,
        edit_history: &QuizEditHistory,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO quiz_edit_history
            (id, quiz_id, user_id, edit_type, description, details, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            edit_history.id.to_string(),
            edit_history.quiz_id.to_string(),
            edit_history.user_id.to_string(),
            edit_history.edit_type,
            edit_history.description,
            edit_history.details,
            edit_history.created_at
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Create a collaboration invitation
    pub async fn create_invitation(
        &self,
        quiz_id: Uuid,
        inviter_id: Uuid,
        role: CollaborationRole,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        // Create a new invitation
        let invitation = CollaborationInvitation {
            id: Uuid::new_v4(),
            quiz_id,
            inviter_id,
            invitee_id: None,
            invitee_email: None,
            role,
            status: InvitationStatus::Pending,
            token: Uuid::new_v4().to_string(),
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store the invitation
        self.store_invitation(&invitation).await?;

        Ok(invitation)
    }

    /// Create a collaboration invitation for a user
    pub async fn invite_user(
        &self,
        quiz_id: Uuid,
        inviter_id: Uuid,
        invitee_id: Uuid,
        role: CollaborationRole,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        // Check if the user is already a collaborator
        let existing_collaborator = self.get_collaborator(quiz_id, invitee_id).await;

        if let Ok(_) = existing_collaborator {
            return Err("User is already a collaborator".into());
        }

        // Check if there's already a pending invitation
        let existing_invitation = self.get_pending_invitation_for_user(quiz_id, invitee_id).await;

        if let Ok(_) = existing_invitation {
            return Err("User already has a pending invitation".into());
        }

        // Create a new invitation
        let mut invitation = self.create_invitation(quiz_id, inviter_id, role).await?;
        invitation.invitee_id = Some(invitee_id);

        // Update the invitation
        self.store_invitation(&invitation).await?;

        Ok(invitation)
    }

    /// Create a collaboration invitation for an email
    pub async fn invite_by_email(
        &self,
        quiz_id: Uuid,
        inviter_id: Uuid,
        email: String,
        role: CollaborationRole,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        // Check if there's already a pending invitation for this email
        let existing_invitation = self.get_pending_invitation_for_email(quiz_id, &email).await;

        if let Ok(_) = existing_invitation {
            return Err("Email already has a pending invitation".into());
        }

        // Create a new invitation
        let mut invitation = self.create_invitation(quiz_id, inviter_id, role).await?;
        invitation.invitee_email = Some(email);

        // Update the invitation
        self.store_invitation(&invitation).await?;

        Ok(invitation)
    }

    /// Accept a collaboration invitation
    pub async fn accept_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<QuizCollaborator, Box<dyn Error + Send + Sync>> {
        // Get the invitation
        let mut invitation = self.get_invitation(invitation_id).await?;

        // Check if the invitation is pending
        if invitation.status != InvitationStatus::Pending {
            return Err("Invitation is not pending".into());
        }

        // Check if the invitation is expired
        if invitation.is_expired() {
            invitation.status = InvitationStatus::Expired;
            self.store_invitation(&invitation).await?;
            return Err("Invitation has expired".into());
        }

        // Check if the invitation is for this user
        if let Some(invitee_id) = invitation.invitee_id {
            if invitee_id != user_id {
                return Err("Invitation is not for this user".into());
            }
        }

        // Update the invitation status
        invitation.status = InvitationStatus::Accepted;
        invitation.updated_at = Utc::now();
        self.store_invitation(&invitation).await?;

        // Add the user as a collaborator
        let collaborator = self.add_collaborator(invitation.quiz_id, user_id, invitation.role).await?;

        Ok(collaborator)
    }

    /// Decline a collaboration invitation
    pub async fn decline_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the invitation
        let mut invitation = self.get_invitation(invitation_id).await?;

        // Check if the invitation is pending
        if invitation.status != InvitationStatus::Pending {
            return Err("Invitation is not pending".into());
        }

        // Check if the invitation is for this user
        if let Some(invitee_id) = invitation.invitee_id {
            if invitee_id != user_id {
                return Err("Invitation is not for this user".into());
            }
        }

        // Update the invitation status
        invitation.status = InvitationStatus::Declined;
        invitation.updated_at = Utc::now();
        self.store_invitation(&invitation).await?;

        Ok(())
    }

    /// Cancel a collaboration invitation
    pub async fn cancel_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the invitation
        let invitation = self.get_invitation(invitation_id).await?;

        // Check if the invitation is pending
        if invitation.status != InvitationStatus::Pending {
            return Err("Invitation is not pending".into());
        }

        // Check if the user is the inviter
        if invitation.inviter_id != user_id {
            // Check if the user is the quiz owner
            let collaborator = self.get_collaborator(invitation.quiz_id, user_id).await?;
            if collaborator.role != CollaborationRole::Owner {
                return Err("Only the inviter or quiz owner can cancel an invitation".into());
            }
        }

        // Delete the invitation
        sqlx::query!(
            r#"
            DELETE FROM quiz_collaboration_invitations
            WHERE id = ?
            "#,
            invitation_id.to_string()
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// Get a collaboration invitation
    pub async fn get_invitation(
        &self,
        invitation_id: Uuid,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, quiz_id, inviter_id, invitee_id, invitee_email, role, status, token, expires_at, created_at, updated_at
            FROM quiz_collaboration_invitations
            WHERE id = ?
            "#,
            invitation_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = row {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let status = match row.status.as_str() {
                "Pending" => InvitationStatus::Pending,
                "Accepted" => InvitationStatus::Accepted,
                "Declined" => InvitationStatus::Declined,
                "Expired" => InvitationStatus::Expired,
                _ => InvitationStatus::Pending, // Default
            };

            let invitation = CollaborationInvitation {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                invitee_id: row.invitee_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                invitee_email: row.invitee_email,
                role,
                status,
                token: row.token,
                expires_at: row.expires_at.parse::<DateTime<Utc>>()?,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            Ok(invitation)
        } else {
            Err("Invitation not found".into())
        }
    }

    /// Get a pending invitation for a user
    pub async fn get_pending_invitation_for_user(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, quiz_id, inviter_id, invitee_id, invitee_email, role, status, token, expires_at, created_at, updated_at
            FROM quiz_collaboration_invitations
            WHERE quiz_id = ? AND invitee_id = ? AND status = 'Pending'
            "#,
            quiz_id.to_string(),
            user_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = row {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let invitation = CollaborationInvitation {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                invitee_id: row.invitee_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                invitee_email: row.invitee_email,
                role,
                status: InvitationStatus::Pending,
                token: row.token,
                expires_at: row.expires_at.parse::<DateTime<Utc>>()?,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            Ok(invitation)
        } else {
            Err("Invitation not found".into())
        }
    }

    /// Get a pending invitation for an email
    pub async fn get_pending_invitation_for_email(
        &self,
        quiz_id: Uuid,
        email: &str,
    ) -> Result<CollaborationInvitation, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, quiz_id, inviter_id, invitee_id, invitee_email, role, status, token, expires_at, created_at, updated_at
            FROM quiz_collaboration_invitations
            WHERE quiz_id = ? AND invitee_email = ? AND status = 'Pending'
            "#,
            quiz_id.to_string(),
            email
        )
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = row {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let invitation = CollaborationInvitation {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                invitee_id: row.invitee_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                invitee_email: row.invitee_email,
                role,
                status: InvitationStatus::Pending,
                token: row.token,
                expires_at: row.expires_at.parse::<DateTime<Utc>>()?,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            Ok(invitation)
        } else {
            Err("Invitation not found".into())
        }
    }

    /// Get all invitations for a quiz
    pub async fn get_invitations_for_quiz(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<CollaborationInvitation>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, inviter_id, invitee_id, invitee_email, role, status, token, expires_at, created_at, updated_at
            FROM quiz_collaboration_invitations
            WHERE quiz_id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;

        let mut invitations = Vec::new();

        for row in rows {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let status = match row.status.as_str() {
                "Pending" => InvitationStatus::Pending,
                "Accepted" => InvitationStatus::Accepted,
                "Declined" => InvitationStatus::Declined,
                "Expired" => InvitationStatus::Expired,
                _ => InvitationStatus::Pending, // Default
            };

            let invitation = CollaborationInvitation {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                invitee_id: row.invitee_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                invitee_email: row.invitee_email,
                role,
                status,
                token: row.token,
                expires_at: row.expires_at.parse::<DateTime<Utc>>()?,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            invitations.push(invitation);
        }

        Ok(invitations)
    }

    /// Get all pending invitations for a user
    pub async fn get_pending_invitations_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CollaborationInvitation>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, quiz_id, inviter_id, invitee_id, invitee_email, role, status, token, expires_at, created_at, updated_at
            FROM quiz_collaboration_invitations
            WHERE invitee_id = ? AND status = 'Pending'
            "#,
            user_id.to_string()
        )
        .fetch_all(&self.db_pool)
        .await?;

        let mut invitations = Vec::new();

        for row in rows {
            let role = match row.role.as_str() {
                "Owner" => CollaborationRole::Owner,
                "Editor" => CollaborationRole::Editor,
                "Viewer" => CollaborationRole::Viewer,
                _ => CollaborationRole::Viewer, // Default
            };

            let invitation = CollaborationInvitation {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                invitee_id: row.invitee_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                invitee_email: row.invitee_email,
                role,
                status: InvitationStatus::Pending,
                token: row.token,
                expires_at: row.expires_at.parse::<DateTime<Utc>>()?,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };

            invitations.push(invitation);
        }

        Ok(invitations)
    }

    /// Store a collaboration invitation
    async fn store_invitation(
        &self,
        invitation: &CollaborationInvitation,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let role_str = match invitation.role {
            CollaborationRole::Owner => "Owner",
            CollaborationRole::Editor => "Editor",
            CollaborationRole::Viewer => "Viewer",
        };

        let status_str = match invitation.status {
            InvitationStatus::Pending => "Pending",
            InvitationStatus::Accepted => "Accepted",
            InvitationStatus::Declined => "Declined",
            InvitationStatus::Expired => "Expired",
        };

        // Check if the invitation already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_collaboration_invitations
            WHERE id = ?
            "#,
            invitation.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;

        if existing.is_some() {
            // Update existing invitation
            sqlx::query!(
                r#"
                UPDATE quiz_collaboration_invitations
                SET invitee_id = ?, invitee_email = ?, role = ?, status = ?, updated_at = ?
                WHERE id = ?
                "#,
                invitation.invitee_id.map(|id| id.to_string()),
                invitation.invitee_email,
                role_str,
                status_str,
                invitation.updated_at,
                invitation.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new invitation
            sqlx::query!(
                r#"
                INSERT INTO quiz_collaboration_invitations
                (id, quiz_id, inviter_id, invitee_id, invitee_email, role, status, token, expires_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                invitation.id.to_string(),
                invitation.quiz_id.to_string(),
                invitation.inviter_id.to_string(),
                invitation.invitee_id.map(|id| id.to_string()),
                invitation.invitee_email,
                role_str,
                status_str,
                invitation.token,
                invitation.expires_at,
                invitation.created_at,
                invitation.updated_at
            )
            .execute(&self.db_pool)
            .await?;
        }

        Ok(())
    }
}
