use super::models::{Quiz, Question};
use crate::auth::models::{User, Role, Permission};
use uuid::Uuid;
use std::error::Error;
use std::fmt;

/// Quiz permission types
#[derive(Debug, Clone, PartialEq)]
pub enum QuizPermission {
    View,
    Edit,
    Delete,
    Attempt,
    ViewResults,
    ViewAnalytics,
    ManageCourseIntegration,
}

/// Quiz permission error
#[derive(Debug)]
pub struct PermissionError {
    pub message: String,
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Permission denied: {}", self.message)
    }
}

impl Error for PermissionError {}

/// Quiz authentication service
pub struct QuizAuthService {
    auth_service: Arc<crate::auth::AuthService>,
}

impl QuizAuthService {
    pub fn new(auth_service: Arc<crate::auth::AuthService>) -> Self {
        Self {
            auth_service,
        }
    }
    
    /// Check if a user has permission to perform an action on a quiz
    pub async fn check_quiz_permission(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
        permission: QuizPermission,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Get the user
        let user = self.auth_service.get_user(user_id).await?;
        
        // Admins have all permissions
        if user.has_role(Role::Admin) {
            return Ok(true);
        }
        
        // Check if the user is the author of the quiz
        let is_author = self.is_quiz_author(user_id, quiz_id).await?;
        
        // Authors have all permissions except for attempting their own quizzes
        if is_author && permission != QuizPermission::Attempt {
            return Ok(true);
        }
        
        // Check specific permissions based on the quiz visibility and user roles
        match permission {
            QuizPermission::View => {
                // Public quizzes can be viewed by anyone
                let is_public = self.is_quiz_public(quiz_id).await?;
                if is_public {
                    return Ok(true);
                }
                
                // Check if the user has been granted access to the quiz
                let has_access = self.user_has_quiz_access(user_id, quiz_id).await?;
                if has_access {
                    return Ok(true);
                }
                
                // Instructors can view all quizzes
                if user.has_role(Role::Instructor) {
                    return Ok(true);
                }
                
                Ok(false)
            },
            QuizPermission::Edit | QuizPermission::Delete => {
                // Only authors, instructors with specific permissions, and admins can edit or delete
                if is_author {
                    return Ok(true);
                }
                
                // Instructors need specific permissions
                if user.has_role(Role::Instructor) {
                    let perm = match permission {
                        QuizPermission::Edit => Permission::EditAnyQuiz,
                        QuizPermission::Delete => Permission::DeleteAnyQuiz,
                        _ => unreachable!(),
                    };
                    
                    return Ok(user.has_permission(perm));
                }
                
                Ok(false)
            },
            QuizPermission::Attempt => {
                // Anyone can attempt public quizzes
                let is_public = self.is_quiz_public(quiz_id).await?;
                if is_public {
                    return Ok(true);
                }
                
                // Check if the user has been granted access to the quiz
                let has_access = self.user_has_quiz_access(user_id, quiz_id).await?;
                if has_access {
                    return Ok(true);
                }
                
                Ok(false)
            },
            QuizPermission::ViewResults => {
                // Users can view their own results
                let has_attempted = self.user_has_attempted_quiz(user_id, quiz_id).await?;
                if has_attempted {
                    return Ok(true);
                }
                
                // Authors can view all results
                if is_author {
                    return Ok(true);
                }
                
                // Instructors with specific permissions can view results
                if user.has_role(Role::Instructor) {
                    return Ok(user.has_permission(Permission::ViewQuizResults));
                }
                
                Ok(false)
            },
            QuizPermission::ViewAnalytics => {
                // Only authors, instructors with specific permissions, and admins can view analytics
                if is_author {
                    return Ok(true);
                }
                
                // Instructors need specific permissions
                if user.has_role(Role::Instructor) {
                    return Ok(user.has_permission(Permission::ViewQuizAnalytics));
                }
                
                Ok(false)
            },
            QuizPermission::ManageCourseIntegration => {
                // Only instructors with specific permissions and admins can manage course integration
                if user.has_role(Role::Instructor) {
                    return Ok(user.has_permission(Permission::ManageCourseContent));
                }
                
                Ok(false)
            },
        }
    }
    
    /// Check if a user has permission to perform an action on a question
    pub async fn check_question_permission(
        &self,
        user_id: Uuid,
        question_id: Uuid,
        permission: QuizPermission,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Get the question's quiz ID
        let quiz_id = self.get_quiz_id_for_question(question_id).await?;
        
        // Check permission on the quiz
        self.check_quiz_permission(user_id, quiz_id, permission).await
    }
    
    /// Enforce a permission check and return an error if the user doesn't have permission
    pub async fn enforce_quiz_permission(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
        permission: QuizPermission,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let has_permission = self.check_quiz_permission(user_id, quiz_id, permission.clone()).await?;
        
        if has_permission {
            Ok(())
        } else {
            let message = format!("User {} does not have {:?} permission for quiz {}", 
                                 user_id, permission, quiz_id);
            Err(Box::new(PermissionError { message }))
        }
    }
    
    /// Enforce a permission check for a question
    pub async fn enforce_question_permission(
        &self,
        user_id: Uuid,
        question_id: Uuid,
        permission: QuizPermission,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let has_permission = self.check_question_permission(user_id, question_id, permission.clone()).await?;
        
        if has_permission {
            Ok(())
        } else {
            let message = format!("User {} does not have {:?} permission for question {}", 
                                 user_id, permission, question_id);
            Err(Box::new(PermissionError { message }))
        }
    }
    
    /// Check if a user is the author of a quiz
    async fn is_quiz_author(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Query the database
        let row = sqlx::query!(
            r#"
            SELECT author_id FROM quizzes
            WHERE id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_optional(&self.auth_service.get_db_pool())
        .await?;
        
        if let Some(row) = row {
            if let Some(author_id) = row.author_id {
                return Ok(author_id == user_id.to_string());
            }
        }
        
        Ok(false)
    }
    
    /// Check if a quiz is public
    async fn is_quiz_public(
        &self,
        quiz_id: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Query the database
        let row = sqlx::query!(
            r#"
            SELECT visibility FROM quizzes
            WHERE id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_optional(&self.auth_service.get_db_pool())
        .await?;
        
        if let Some(row) = row {
            return Ok(row.visibility == "Public");
        }
        
        Ok(false)
    }
    
    /// Check if a user has been granted access to a quiz
    async fn user_has_quiz_access(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Check if the quiz is part of a course the user is enrolled in
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM quiz_course_mappings m
            JOIN course_enrollments e ON m.course_id = e.course_id
            WHERE m.quiz_id = ? AND e.user_id = ?
            "#,
            quiz_id.to_string(),
            user_id.to_string()
        )
        .fetch_one(&self.auth_service.get_db_pool())
        .await?;
        
        if row.count > 0 {
            return Ok(true);
        }
        
        // Check if the user has been explicitly granted access
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM quiz_access_grants
            WHERE quiz_id = ? AND user_id = ?
            "#,
            quiz_id.to_string(),
            user_id.to_string()
        )
        .fetch_one(&self.auth_service.get_db_pool())
        .await?;
        
        Ok(row.count > 0)
    }
    
    /// Check if a user has attempted a quiz
    async fn user_has_attempted_quiz(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Query the database
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM quiz_attempts
            WHERE quiz_id = ? AND user_id = ?
            "#,
            quiz_id.to_string(),
            user_id.to_string()
        )
        .fetch_one(&self.auth_service.get_db_pool())
        .await?;
        
        Ok(row.count > 0)
    }
    
    /// Get the quiz ID for a question
    async fn get_quiz_id_for_question(
        &self,
        question_id: Uuid,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        // Query the database
        let row = sqlx::query!(
            r#"
            SELECT quiz_id FROM questions
            WHERE id = ?
            "#,
            question_id.to_string()
        )
        .fetch_one(&self.auth_service.get_db_pool())
        .await?;
        
        Uuid::parse_str(&row.quiz_id).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }
}

use std::sync::Arc;

/// Quiz authentication middleware for Tauri commands
pub struct QuizAuthMiddleware {
    auth_service: Arc<QuizAuthService>,
}

impl QuizAuthMiddleware {
    pub fn new(auth_service: Arc<QuizAuthService>) -> Self {
        Self {
            auth_service,
        }
    }
    
    /// Check if a user has permission to view a quiz
    pub async fn can_view_quiz(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::View)
            .await
            .map_err(|e| e.to_string())
    }
    
    /// Check if a user has permission to edit a quiz
    pub async fn can_edit_quiz(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::Edit)
            .await
            .map_err(|e| e.to_string())
    }
    
    /// Check if a user has permission to delete a quiz
    pub async fn can_delete_quiz(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::Delete)
            .await
            .map_err(|e| e.to_string())
    }
    
    /// Check if a user has permission to attempt a quiz
    pub async fn can_attempt_quiz(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::Attempt)
            .await
            .map_err(|e| e.to_string())
    }
    
    /// Check if a user has permission to view quiz results
    pub async fn can_view_quiz_results(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::ViewResults)
            .await
            .map_err(|e| e.to_string())
    }
    
    /// Check if a user has permission to view quiz analytics
    pub async fn can_view_quiz_analytics(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::ViewAnalytics)
            .await
            .map_err(|e| e.to_string())
    }
    
    /// Check if a user has permission to manage course integration
    pub async fn can_manage_course_integration(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        self.auth_service.enforce_quiz_permission(user_id, quiz_id, QuizPermission::ManageCourseIntegration)
            .await
            .map_err(|e| e.to_string())
    }
}
