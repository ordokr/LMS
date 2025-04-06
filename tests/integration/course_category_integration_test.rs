#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::sync::Arc;
    use crate::{
        db::{course_repository::CourseRepository, category_repository::CategoryRepository},
        models::course::Course,
        services::course_category_mapper::CourseCategoryMapper,
    };

    #[tokio::test]
    async fn test_map_course_to_category() -> Result<(), Box<dyn std::error::Error>> {
        // Set up database connection (replace with your actual database URL)
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await?;

        // Create repositories
        let course_repo = CourseRepository::new(pool.clone());
        let category_repo = CategoryRepository::new(pool.clone());

        // Create AppState
        let app_state = Arc::new(crate::AppState {
            pool: pool.clone(),
            db: crate::db::user_repository::UserRepository::new(pool.clone()),
            course_repo: course_repo.clone(),
            category_repo: category_repo.clone(),
            topic_repo: crate::db::topic_repository::TopicRepository::new(pool.clone()),
            post_repo: crate::db::post_repository::PostRepository::new(pool.clone()),
            assignment_repo: crate::db::assignment_repository::AssignmentRepository::new(pool.clone()),
        });

        // Create a course
        let course = Course {
            id: uuid::Uuid::new_v4(),
            canvas_id: "12345".to_string(),
            name: "Test Course".to_string(),
            code: "TC101".to_string(),
            description: Some("Test course description".to_string()),
            instructor_id: uuid::Uuid::new_v4(),
            start_date: None,
            end_date: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            category_id: None,
        };
        course_repo.create_course(&course).await?;

        // Create a mapper
        let mapper = CourseCategoryMapper::new(course_repo.clone(), category_repo.clone());

        // Map the course to a category
        let category_name = Some("Test Category".to_string());
        let (updated_course, category) = mapper.map_course_to_category(&course.id, category_name).await?;

        // Verify that the course is mapped to a category
        assert_eq!(updated_course.category_id, Some(category.id));

        Ok(())
    }
}
        // TODO: Implement test logic here
    }
}