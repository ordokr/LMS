use super::models::{Quiz, QuizAttempt};
use super::storage::HybridQuizStore;
use crate::course::models::{Course, Module, Section};
use crate::course::storage::CourseStore;
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Quiz course mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizCourseMapping {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub course_id: Uuid,
    pub module_id: Option<Uuid>,
    pub section_id: Option<Uuid>,
    pub position: i32,
    pub is_required: bool,
    pub passing_score: Option<f32>,
    pub due_date: Option<DateTime<Utc>>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub max_attempts: Option<i32>,
    pub time_limit: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz assignment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizAssignmentStatus {
    NotStarted,
    InProgress,
    Completed,
    Overdue,
}

/// Quiz assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAssignment {
    pub id: Uuid,
    pub mapping_id: Uuid,
    pub student_id: Uuid,
    pub status: QuizAssignmentStatus,
    pub attempts: i32,
    pub best_score: Option<f32>,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quiz with course context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizWithContext {
    pub quiz: Quiz,
    pub mapping: QuizCourseMapping,
    pub course: Course,
    pub module: Option<Module>,
    pub section: Option<Section>,
    pub assignment: Option<QuizAssignment>,
}

/// Course integration service
pub struct CourseIntegrationService {
    quiz_store: Arc<HybridQuizStore>,
    course_store: Arc<CourseStore>,
}

impl CourseIntegrationService {
    pub fn new(quiz_store: Arc<HybridQuizStore>, course_store: Arc<CourseStore>) -> Self {
        Self {
            quiz_store,
            course_store,
        }
    }
    
    /// Add a quiz to a course
    pub async fn add_quiz_to_course(
        &self,
        quiz_id: Uuid,
        course_id: Uuid,
        module_id: Option<Uuid>,
        section_id: Option<Uuid>,
        position: Option<i32>,
    ) -> Result<QuizCourseMapping, Box<dyn Error + Send + Sync>> {
        // Verify that the quiz exists
        let _quiz = self.quiz_store.get_quiz(quiz_id).await?;
        
        // Verify that the course exists
        let _course = self.course_store.get_course(course_id).await?;
        
        // Verify module and section if provided
        if let Some(module_id) = module_id {
            let _module = self.course_store.get_module(module_id).await?;
            
            if let Some(section_id) = section_id {
                let _section = self.course_store.get_section(section_id).await?;
            }
        }
        
        // Determine position if not provided
        let position = if let Some(pos) = position {
            pos
        } else {
            // Get the highest position in the course/module/section and add 1
            let existing_mappings = self.get_quizzes_for_course(course_id).await?;
            let filtered_mappings = existing_mappings.iter()
                .filter(|m| m.module_id == module_id && m.section_id == section_id)
                .collect::<Vec<_>>();
            
            if filtered_mappings.is_empty() {
                0
            } else {
                filtered_mappings.iter()
                    .map(|m| m.position)
                    .max()
                    .unwrap_or(0) + 1
            }
        };
        
        // Create the mapping
        let mapping = QuizCourseMapping {
            id: Uuid::new_v4(),
            quiz_id,
            course_id,
            module_id,
            section_id,
            position,
            is_required: true,
            passing_score: Some(70.0),
            due_date: None,
            available_from: None,
            available_until: None,
            max_attempts: None,
            time_limit: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Store the mapping
        self.store_mapping(&mapping).await?;
        
        Ok(mapping)
    }
    
    /// Remove a quiz from a course
    pub async fn remove_quiz_from_course(
        &self,
        mapping_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Delete the mapping
        self.delete_mapping(mapping_id).await?;
        
        Ok(())
    }
    
    /// Update a quiz-course mapping
    pub async fn update_mapping(
        &self,
        mapping: &QuizCourseMapping,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Store the updated mapping
        self.store_mapping(mapping).await?;
        
        Ok(())
    }
    
    /// Get all quizzes for a course
    pub async fn get_quizzes_for_course(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<QuizCourseMapping>, Box<dyn Error + Send + Sync>> {
        // Query the database for mappings
        let mappings = sqlx::query!(
            r#"
            SELECT id, quiz_id, course_id, module_id, section_id, position, is_required,
                   passing_score, due_date, available_from, available_until, max_attempts,
                   time_limit, created_at, updated_at
            FROM quiz_course_mappings
            WHERE course_id = ?
            ORDER BY position ASC
            "#,
            course_id.to_string()
        )
        .fetch_all(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        let mut result = Vec::new();
        
        for row in mappings {
            let mapping = QuizCourseMapping {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                course_id: Uuid::parse_str(&row.course_id)?,
                module_id: if let Some(id) = row.module_id {
                    Some(Uuid::parse_str(&id)?)
                } else {
                    None
                },
                section_id: if let Some(id) = row.section_id {
                    Some(Uuid::parse_str(&id)?)
                } else {
                    None
                },
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
            
            result.push(mapping);
        }
        
        Ok(result)
    }
    
    /// Get all courses for a quiz
    pub async fn get_courses_for_quiz(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<(QuizCourseMapping, Course)>, Box<dyn Error + Send + Sync>> {
        // Query the database for mappings
        let mappings = sqlx::query!(
            r#"
            SELECT id, quiz_id, course_id, module_id, section_id, position, is_required,
                   passing_score, due_date, available_from, available_until, max_attempts,
                   time_limit, created_at, updated_at
            FROM quiz_course_mappings
            WHERE quiz_id = ?
            "#,
            quiz_id.to_string()
        )
        .fetch_all(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        let mut result = Vec::new();
        
        for row in mappings {
            let mapping = QuizCourseMapping {
                id: Uuid::parse_str(&row.id)?,
                quiz_id: Uuid::parse_str(&row.quiz_id)?,
                course_id: Uuid::parse_str(&row.course_id)?,
                module_id: if let Some(id) = row.module_id {
                    Some(Uuid::parse_str(&id)?)
                } else {
                    None
                },
                section_id: if let Some(id) = row.section_id {
                    Some(Uuid::parse_str(&id)?)
                } else {
                    None
                },
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
            
            // Get the course
            let course = self.course_store.get_course(mapping.course_id).await?;
            
            result.push((mapping, course));
        }
        
        Ok(result)
    }
    
    /// Get a quiz with course context
    pub async fn get_quiz_with_context(
        &self,
        mapping_id: Uuid,
        student_id: Option<Uuid>,
    ) -> Result<QuizWithContext, Box<dyn Error + Send + Sync>> {
        // Get the mapping
        let mapping = self.get_mapping(mapping_id).await?;
        
        // Get the quiz
        let quiz = self.quiz_store.get_quiz(mapping.quiz_id).await?;
        
        // Get the course
        let course = self.course_store.get_course(mapping.course_id).await?;
        
        // Get the module if applicable
        let module = if let Some(module_id) = mapping.module_id {
            Some(self.course_store.get_module(module_id).await?)
        } else {
            None
        };
        
        // Get the section if applicable
        let section = if let Some(section_id) = mapping.section_id {
            Some(self.course_store.get_section(section_id).await?)
        } else {
            None
        };
        
        // Get the assignment if student_id is provided
        let assignment = if let Some(student_id) = student_id {
            match self.get_assignment(mapping_id, student_id).await {
                Ok(assignment) => Some(assignment),
                Err(_) => None,
            }
        } else {
            None
        };
        
        Ok(QuizWithContext {
            quiz,
            mapping,
            course,
            module,
            section,
            assignment,
        })
    }
    
    /// Get all quizzes for a student in a course
    pub async fn get_student_quizzes(
        &self,
        course_id: Uuid,
        student_id: Uuid,
    ) -> Result<Vec<QuizWithContext>, Box<dyn Error + Send + Sync>> {
        // Get all mappings for the course
        let mappings = self.get_quizzes_for_course(course_id).await?;
        
        let mut result = Vec::new();
        
        for mapping in mappings {
            // Get the quiz with context
            let quiz_context = self.get_quiz_with_context(mapping.id, Some(student_id)).await?;
            
            result.push(quiz_context);
        }
        
        Ok(result)
    }
    
    /// Create or update a quiz assignment for a student
    pub async fn assign_quiz_to_student(
        &self,
        mapping_id: Uuid,
        student_id: Uuid,
    ) -> Result<QuizAssignment, Box<dyn Error + Send + Sync>> {
        // Check if an assignment already exists
        let assignment = match self.get_assignment(mapping_id, student_id).await {
            Ok(assignment) => assignment,
            Err(_) => {
                // Create a new assignment
                let assignment = QuizAssignment {
                    id: Uuid::new_v4(),
                    mapping_id,
                    student_id,
                    status: QuizAssignmentStatus::NotStarted,
                    attempts: 0,
                    best_score: None,
                    last_attempt_at: None,
                    completed_at: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                
                // Store the assignment
                self.store_assignment(&assignment).await?;
                
                assignment
            }
        };
        
        Ok(assignment)
    }
    
    /// Update a quiz assignment status based on an attempt
    pub async fn update_assignment_from_attempt(
        &self,
        mapping_id: Uuid,
        student_id: Uuid,
        attempt: &QuizAttempt,
    ) -> Result<QuizAssignment, Box<dyn Error + Send + Sync>> {
        // Get the mapping to check passing score
        let mapping = self.get_mapping(mapping_id).await?;
        
        // Get the current assignment or create a new one
        let mut assignment = match self.get_assignment(mapping_id, student_id).await {
            Ok(assignment) => assignment,
            Err(_) => {
                // Create a new assignment
                QuizAssignment {
                    id: Uuid::new_v4(),
                    mapping_id,
                    student_id,
                    status: QuizAssignmentStatus::NotStarted,
                    attempts: 0,
                    best_score: None,
                    last_attempt_at: None,
                    completed_at: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }
            }
        };
        
        // Update the assignment
        assignment.attempts += 1;
        assignment.last_attempt_at = Some(Utc::now());
        
        if let Some(score) = attempt.score {
            // Update best score
            if assignment.best_score.is_none() || score > assignment.best_score.unwrap() {
                assignment.best_score = Some(score);
            }
            
            // Check if the attempt is completed and meets the passing score
            if attempt.completed_at.is_some() {
                let passing_score = mapping.passing_score.unwrap_or(70.0);
                
                if score >= passing_score {
                    assignment.status = QuizAssignmentStatus::Completed;
                    assignment.completed_at = attempt.completed_at;
                } else {
                    assignment.status = QuizAssignmentStatus::InProgress;
                }
            } else {
                assignment.status = QuizAssignmentStatus::InProgress;
            }
        } else {
            assignment.status = QuizAssignmentStatus::InProgress;
        }
        
        // Check if the assignment is overdue
        if let Some(due_date) = mapping.due_date {
            if due_date < Utc::now() && assignment.status != QuizAssignmentStatus::Completed {
                assignment.status = QuizAssignmentStatus::Overdue;
            }
        }
        
        // Store the updated assignment
        self.store_assignment(&assignment).await?;
        
        Ok(assignment)
    }
    
    /// Get a quiz assignment
    async fn get_assignment(
        &self,
        mapping_id: Uuid,
        student_id: Uuid,
    ) -> Result<QuizAssignment, Box<dyn Error + Send + Sync>> {
        // Query the database for the assignment
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
        .fetch_one(&self.quiz_store.get_sqlite_pool())
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
    
    /// Store a quiz assignment
    async fn store_assignment(
        &self,
        assignment: &QuizAssignment,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Convert status to string
        let status_str = match assignment.status {
            QuizAssignmentStatus::NotStarted => "NotStarted",
            QuizAssignmentStatus::InProgress => "InProgress",
            QuizAssignmentStatus::Completed => "Completed",
            QuizAssignmentStatus::Overdue => "Overdue",
        };
        
        // Check if the assignment already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_assignments
            WHERE id = ?
            "#,
            assignment.id.to_string()
        )
        .fetch_optional(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        if existing.is_some() {
            // Update existing assignment
            sqlx::query!(
                r#"
                UPDATE quiz_assignments
                SET status = ?, attempts = ?, best_score = ?,
                    last_attempt_at = ?, completed_at = ?, updated_at = ?
                WHERE id = ?
                "#,
                status_str,
                assignment.attempts,
                assignment.best_score,
                assignment.last_attempt_at,
                assignment.completed_at,
                assignment.updated_at,
                assignment.id.to_string()
            )
            .execute(&self.quiz_store.get_sqlite_pool())
            .await?;
        } else {
            // Insert new assignment
            sqlx::query!(
                r#"
                INSERT INTO quiz_assignments
                (id, mapping_id, student_id, status, attempts, best_score,
                 last_attempt_at, completed_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                assignment.id.to_string(),
                assignment.mapping_id.to_string(),
                assignment.student_id.to_string(),
                status_str,
                assignment.attempts,
                assignment.best_score,
                assignment.last_attempt_at,
                assignment.completed_at,
                assignment.created_at,
                assignment.updated_at
            )
            .execute(&self.quiz_store.get_sqlite_pool())
            .await?;
        }
        
        Ok(())
    }
    
    /// Get a quiz-course mapping
    async fn get_mapping(
        &self,
        mapping_id: Uuid,
    ) -> Result<QuizCourseMapping, Box<dyn Error + Send + Sync>> {
        // Query the database for the mapping
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
        .fetch_one(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        let mapping = QuizCourseMapping {
            id: Uuid::parse_str(&row.id)?,
            quiz_id: Uuid::parse_str(&row.quiz_id)?,
            course_id: Uuid::parse_str(&row.course_id)?,
            module_id: if let Some(id) = row.module_id {
                Some(Uuid::parse_str(&id)?)
            } else {
                None
            },
            section_id: if let Some(id) = row.section_id {
                Some(Uuid::parse_str(&id)?)
            } else {
                None
            },
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
    
    /// Store a quiz-course mapping
    async fn store_mapping(
        &self,
        mapping: &QuizCourseMapping,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check if the mapping already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_course_mappings
            WHERE id = ?
            "#,
            mapping.id.to_string()
        )
        .fetch_optional(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        if existing.is_some() {
            // Update existing mapping
            sqlx::query!(
                r#"
                UPDATE quiz_course_mappings
                SET quiz_id = ?, course_id = ?, module_id = ?, section_id = ?,
                    position = ?, is_required = ?, passing_score = ?, due_date = ?,
                    available_from = ?, available_until = ?, max_attempts = ?,
                    time_limit = ?, updated_at = ?
                WHERE id = ?
                "#,
                mapping.quiz_id.to_string(),
                mapping.course_id.to_string(),
                mapping.module_id.map(|id| id.to_string()),
                mapping.section_id.map(|id| id.to_string()),
                mapping.position,
                mapping.is_required as i32,
                mapping.passing_score,
                mapping.due_date,
                mapping.available_from,
                mapping.available_until,
                mapping.max_attempts,
                mapping.time_limit,
                mapping.updated_at,
                mapping.id.to_string()
            )
            .execute(&self.quiz_store.get_sqlite_pool())
            .await?;
        } else {
            // Insert new mapping
            sqlx::query!(
                r#"
                INSERT INTO quiz_course_mappings
                (id, quiz_id, course_id, module_id, section_id, position, is_required,
                 passing_score, due_date, available_from, available_until, max_attempts,
                 time_limit, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                mapping.id.to_string(),
                mapping.quiz_id.to_string(),
                mapping.course_id.to_string(),
                mapping.module_id.map(|id| id.to_string()),
                mapping.section_id.map(|id| id.to_string()),
                mapping.position,
                mapping.is_required as i32,
                mapping.passing_score,
                mapping.due_date,
                mapping.available_from,
                mapping.available_until,
                mapping.max_attempts,
                mapping.time_limit,
                mapping.created_at,
                mapping.updated_at
            )
            .execute(&self.quiz_store.get_sqlite_pool())
            .await?;
        }
        
        Ok(())
    }
    
    /// Delete a quiz-course mapping
    async fn delete_mapping(
        &self,
        mapping_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Delete the mapping
        sqlx::query!(
            r#"
            DELETE FROM quiz_course_mappings
            WHERE id = ?
            "#,
            mapping_id.to_string()
        )
        .execute(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        // Delete associated assignments
        sqlx::query!(
            r#"
            DELETE FROM quiz_assignments
            WHERE mapping_id = ?
            "#,
            mapping_id.to_string()
        )
        .execute(&self.quiz_store.get_sqlite_pool())
        .await?;
        
        Ok(())
    }
}
