use sqlx::PgPool;
use uuid::Uuid;
use crate::models::assignment::Assignment;

pub struct AssignmentRepository {
    pool: PgPool,
}

impl AssignmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_assignment(&self, assignment: &Assignment) -> Result<Assignment, sqlx::Error> {
        let created_assignment = sqlx::query_as!(
            Assignment,
            r#"
            INSERT INTO assignments (
                id, course_id, title, description, points_possible, 
                due_date, available_from, available_until, submission_types, 
                canvas_id, topic_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
            assignment.id,
            assignment.course_id,
            assignment.title,
            assignment.description,
            assignment.points_possible,
            assignment.due_date,
            assignment.available_from,
            assignment.available_until,
            assignment.submission_types,
            assignment.canvas_id,
            assignment.topic_id,
            assignment.created_at,
            assignment.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_assignment)
    }

    pub async fn find_assignment_by_id(&self, id: &Uuid) -> Result<Option<Assignment>, sqlx::Error> {
        let assignment = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(assignment)
    }

    pub async fn find_assignment_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Assignment>, sqlx::Error> {
        let assignment = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE canvas_id = $1
            "#,
            canvas_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(assignment)
    }

    pub async fn find_assignment_by_topic(&self, topic_id: &Uuid) -> Result<Option<Assignment>, sqlx::Error> {
        let assignment = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE topic_id = $1
            "#,
            topic_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(assignment)
    }

    pub async fn update_assignment_topic(&self, assignment_id: &Uuid, topic_id: &Option<Uuid>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE assignments
            SET topic_id = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            topic_id,
            assignment_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_assignments_by_course(&self, course_id: &Uuid) -> Result<Vec<Assignment>, sqlx::Error> {
        let assignments = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE course_id = $1
            ORDER BY due_date ASC NULLS LAST
            "#,
            course_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(assignments)
    }

    pub async fn list_discussion_assignments_by_course(&self, course_id: &Uuid) -> Result<Vec<Assignment>, sqlx::Error> {
        let assignments = sqlx::query_as!(
            Assignment,
            r#"
            SELECT * FROM assignments
            WHERE course_id = $1 
            AND submission_types @> ARRAY['discussion_topic']::text[]
            ORDER BY due_date ASC NULLS LAST
            "#,
            course_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(assignments)
    }
}