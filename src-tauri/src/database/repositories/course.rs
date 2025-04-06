use sqlx::{Pool, Sqlite, SqlitePool};

use crate::core::errors::AppError;
use crate::lms::models::{Course, Enrollment, EnrollmentRole, EnrollmentStatus, CourseStatus};

pub struct CourseRepository<'a> {
    db: &'a SqlitePool
}

impl<'a> CourseRepository<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }
    
    // Create a new course
    pub async fn create_course(&self, course: &Course) -> Result<i64, AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO courses
            (code, name, description, instructor_id, start_date, end_date, status)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            course.code,
            course.name,
            course.description,
            course.instructor_id,
            course.start_date,
            course.end_date,
            course.status as i32, // This requires a custom FROM/TO implementation
        )
        .fetch_one(self.db)
        .await?;
        
        // Automatically enroll the instructor
        self.enroll_user(
            result.id,
            course.instructor_id,
            EnrollmentRole::Teacher,
            EnrollmentStatus::Active,
        ).await?;
        
        Ok(result.id)
    }
    
    // Get all courses (with filtering options)
    pub async fn get_courses(&self, user_id: Option<i64>, status: Option<CourseStatus>) -> Result<Vec<Course>, AppError> {
        let courses = match (user_id, status) {
            (Some(uid), Some(stat)) => {
                sqlx::query_as!(
                    Course,
                    r#"
                    SELECT c.id, c.code, c.name, c.description, c.instructor_id, 
                           c.start_date, c.end_date, c.status, c.created_at, c.updated_at
                    FROM courses c
                    JOIN enrollments e ON c.id = e.course_id
                    WHERE e.user_id = ? AND c.status = ?
                    ORDER BY c.code
                    "#,
                    uid,
                    stat as i32
                )
                .fetch_all(self.db)
                .await?
            },
            (Some(uid), None) => {
                sqlx::query_as!(
                    Course,
                    r#"
                    SELECT c.id, c.code, c.name, c.description, c.instructor_id, 
                           c.start_date, c.end_date, c.status, c.created_at, c.updated_at
                    FROM courses c
                    JOIN enrollments e ON c.id = e.course_id
                    WHERE e.user_id = ?
                    ORDER BY c.code
                    "#,
                    uid
                )
                .fetch_all(self.db)
                .await?
            },
            (None, Some(stat)) => {
                sqlx::query_as!(
                    Course,
                    r#"
                    SELECT id, code, name, description, instructor_id, 
                           start_date, end_date, status, created_at, updated_at
                    FROM courses
                    WHERE status = ?
                    ORDER BY code
                    "#,
                    stat as i32
                )
                .fetch_all(self.db)
                .await?
            },
            (None, None) => {
                sqlx::query_as!(
                    Course,
                    r#"
                    SELECT id, code, name, description, instructor_id, 
                           start_date, end_date, status, created_at, updated_at
                    FROM courses
                    ORDER BY code
                    "#,
                )
                .fetch_all(self.db)
                .await?
            }
        };
        
        Ok(courses)
    }
    
    // Get a specific course by ID
    pub async fn get_course_by_id(&self, course_id: i64) -> Result<Course, AppError> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT id, code, name, description, instructor_id, 
                   start_date, end_date, status, created_at, updated_at
            FROM courses
            WHERE id = ?
            "#,
            course_id
        )
        .fetch_optional(self.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Course with id {} not found", course_id)))?;
        
        Ok(course)
    }
    
    // Update a course
    pub async fn update_course(&self, course: &Course) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE courses
            SET code = ?, name = ?, description = ?, instructor_id = ?,
                start_date = ?, end_date = ?, status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            course.code,
            course.name,
            course.description,
            course.instructor_id,
            course.start_date,
            course.end_date,
            course.status as i32,
            course.id
        )
        .execute(self.db)
        .await?;
        
        Ok(())
    }
    
    // Delete a course
    pub async fn delete_course(&self, course_id: i64) -> Result<(), AppError> {
        // Start a transaction to delete course and related data
        let mut tx = self.db.begin().await?;
        
        // Delete enrollments first (foreign key constraint)
        sqlx::query!(
            r#"
            DELETE FROM enrollments
            WHERE course_id = ?
            "#,
            course_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Delete modules and module items (would need to cascade through all related entities)
        // This is just a basic example - you would need to handle all related data
        
        // Delete the course
        sqlx::query!(
            r#"
            DELETE FROM courses
            WHERE id = ?
            "#,
            course_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    // Enroll a user in a course
    pub async fn enroll_user(
        &self,
        course_id: i64,
        user_id: i64,
        role: EnrollmentRole,
        status: EnrollmentStatus,
    ) -> Result<i64, AppError> {
        // Check if user is already enrolled
        let existing = sqlx::query!(
            r#"
            SELECT id FROM enrollments
            WHERE user_id = ? AND course_id = ?
            "#,
            user_id,
            course_id
        )
        .fetch_optional(self.db)
        .await?;
        
        if let Some(enrollment) = existing {
            // User is already enrolled, update their role/status
            sqlx::query!(
                r#"
                UPDATE enrollments
                SET role = ?, status = ?, updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                "#,
                role as i32,
                status as i32,
                enrollment.id
            )
            .execute(self.db)
            .await?;
            
            return Ok(enrollment.id);
        }
        
        // Create new enrollment
        let result = sqlx::query!(
            r#"
            INSERT INTO enrollments
            (user_id, course_id, role, status)
            VALUES (?, ?, ?, ?)
            RETURNING id
            "#,
            user_id,
            course_id,
            role as i32,
            status as i32
        )
        .fetch_one(self.db)
        .await?;
        
        Ok(result.id)
    }
    
    // Get all enrollments for a course
    pub async fn get_enrollments(&self, course_id: i64) -> Result<Vec<Enrollment>, AppError> {
        let enrollments = sqlx::query_as!(
            Enrollment,
            r#"
            SELECT id, user_id, course_id, role, status, created_at, updated_at
            FROM enrollments
            WHERE course_id = ?
            ORDER BY role, user_id
            "#,
            course_id
        )
        .fetch_all(self.db)
        .await?;
        
        Ok(enrollments)
    }
    
    // Update a user's enrollment
    pub async fn update_enrollment(
        &self,
        course_id: i64,
        user_id: i64,
        role: EnrollmentRole,
        status: EnrollmentStatus,
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            UPDATE enrollments
            SET role = ?, status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE course_id = ? AND user_id = ?
            "#,
            role as i32,
            status as i32,
            course_id,
            user_id
        )
        .execute(self.db)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Enrollment for user {} in course {} not found", user_id, course_id)));
        }
        
        Ok(())
    }
    
    // Remove a user from a course
    pub async fn remove_enrollment(&self, course_id: i64, user_id: i64) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM enrollments
            WHERE course_id = ? AND user_id = ?
            "#,
            course_id,
            user_id
        )
        .execute(self.db)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Enrollment for user {} in course {} not found", user_id, course_id)));
        }
        
        Ok(())
    }
    
    // Check if a user is enrolled in a course with a specific role
    pub async fn check_enrollment(
        &self,
        course_id: i64,
        user_id: i64,
        role: Option<EnrollmentRole>,
    ) -> Result<bool, AppError> {
        let query = match role {
            Some(r) => {
                sqlx::query!(
                    r#"
                    SELECT 1 FROM enrollments
                    WHERE course_id = ? AND user_id = ? AND role = ? AND status = ?
                    "#,
                    course_id,
                    user_id,
                    r as i32,
                    EnrollmentStatus::Active as i32
                )
            },
            None => {
                sqlx::query!(
                    r#"
                    SELECT 1 FROM enrollments
                    WHERE course_id = ? AND user_id = ? AND status = ?
                    "#,
                    course_id,
                    user_id,
                    EnrollmentStatus::Active as i32
                )
            }
        };
        
        let result = query.fetch_optional(self.db).await?;
        
        Ok(result.is_some())
    }

    pub async fn get_all(&self) -> Result<Vec<Course>, AppError> {
        let courses = sqlx::query_as!(
            Course,
            "SELECT * FROM courses WHERE deleted_at IS NULL ORDER BY name"
        )
        .fetch_all(self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;
        
        Ok(courses)
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Course, AppError> {
        let course = sqlx::query_as!(
            Course,
            "SELECT * FROM courses WHERE id = ? AND deleted_at IS NULL",
            id
        )
        .fetch_optional(self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Course with id {} not found", id)))?;
        
        Ok(course)
    }
    
    pub async fn create(
        &self, 
        name: String, 
        code: String, 
        description: Option<String>
    ) -> Result<Course, AppError> {
        let course_id = sqlx::query!(
            "INSERT INTO courses (name, code, description, created_at, updated_at)
             VALUES (?, ?, ?, datetime('now'), datetime('now'))
             RETURNING id",
            name,
            code,
            description
        )
        .fetch_one(self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .id;
        
        self.get_by_id(course_id).await
    }
    
    pub async fn update(
        &self, 
        id: i64, 
        name: String, 
        code: String, 
        description: Option<String>
    ) -> Result<Course, AppError> {
        let rows_affected = sqlx::query!(
            "UPDATE courses 
             SET name = ?, code = ?, description = ?, updated_at = datetime('now')
             WHERE id = ? AND deleted_at IS NULL",
            name,
            code,
            description,
            id
        )
        .execute(self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Course with id {} not found", id)));
        }
        
        self.get_by_id(id).await
    }
    
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            "UPDATE courses SET deleted_at = datetime('now')
             WHERE id = ? AND deleted_at IS NULL",
            id
        )
        .execute(self.db)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Course with id {} not found", id)));
        }
        
        Ok(())
    }
}