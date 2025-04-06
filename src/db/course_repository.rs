use sqlx::PgPool;
use uuid::Uuid;
use crate::models::course::Course;

pub struct CourseRepository {
    pool: PgPool,
}

impl CourseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_course(&self, course: &Course) -> Result<Course, sqlx::Error> {
        let created_course = sqlx::query_as!(
            Course,
            r#"
            INSERT INTO courses (
                id, canvas_id, name, code, description, instructor_id, 
                start_date, end_date, created_at, updated_at, category_id, is_published
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
            course.id,
            course.canvas_id,
            course.name,
            course.code,
            course.description,
            course.instructor_id,
            course.start_date,
            course.end_date,
            course.created_at,
            course.updated_at,
            course.category_id,
            course.is_published
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_course)
    }

    pub async fn find_course_by_id(&self, id: &Uuid) -> Result<Option<Course>, sqlx::Error> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT * FROM courses
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course)
    }

    pub async fn find_course_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Course>, sqlx::Error> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT * FROM courses
            WHERE canvas_id = $1
            "#,
            canvas_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(course)
    }

    pub async fn update_course_category(&self, course_id: &Uuid, category_id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE courses
            SET category_id = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            category_id,
            course_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_courses(&self) -> Result<Vec<Course>, sqlx::Error> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT * FROM courses
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(courses)
    }

    pub async fn list_courses_by_instructor(&self, instructor_id: &Uuid) -> Result<Vec<Course>, sqlx::Error> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT * FROM courses
            WHERE instructor_id = $1
            ORDER BY created_at DESC
            "#,
            instructor_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(courses)
    }
}