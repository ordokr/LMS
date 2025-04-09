#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;
    use chrono::Utc;
    use crate::{
        db::{course_repository::CourseRepository, category_repository::CategoryRepository},
        models::{course::Course, category::Category},
        services::course_category_mapper::CourseCategoryMapper,
    };

    // Helper function to set up test environment
    async fn setup_test_environment() -> Result<(PgPool, CourseRepository, CategoryRepository), Box<dyn std::error::Error>> {
        // Set up database connection
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await?;

        // Create repositories
        let course_repo = CourseRepository::new(pool.clone());
        let category_repo = CategoryRepository::new(pool.clone());

        Ok((pool, course_repo, category_repo))
    }

    // Helper function to create a test course
    async fn create_test_course(repo: &CourseRepository, name: &str, code: &str) -> Result<Course, Box<dyn std::error::Error>> {
        let course = Course {
            id: Uuid::new_v4(),
            canvas_id: format!("canvas-{}", Uuid::new_v4()),
            name: name.to_string(),
            code: code.to_string(),
            description: Some(format!("Description for {}", name)),
            instructor_id: Uuid::new_v4(),
            start_date: None,
            end_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            category_id: None,
            is_published: true,
        };
        
        repo.create_course(&course).await?;
        Ok(course)
    }

    #[tokio::test]
    async fn test_map_course_to_category() -> Result<(), Box<dyn std::error::Error>> {
        // Set up test environment
        let (pool, course_repo, category_repo) = setup_test_environment().await?;

        // Create a course
        let course = create_test_course(&course_repo, "Test Course", "TC101").await?;

        // Create a mapper
        let mapper = CourseCategoryMapper::new(course_repo.clone(), category_repo.clone());

        // Map the course to a category
        let category_name = Some("Test Category".to_string());
        let (updated_course, category) = mapper.map_course_to_category(&course.id, category_name).await?;

        // Verify that the course is mapped to a category
        assert_eq!(updated_course.category_id, Some(category.id));
        assert_eq!(category.name, "Test Category");
        assert!(category.course_id.is_some());
        assert_eq!(category.course_id.unwrap(), course.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_course_with_category() -> Result<(), Box<dyn std::error::Error>> {
        // Set up test environment
        let (pool, course_repo, category_repo) = setup_test_environment().await?;

        // Create a course
        let course = create_test_course(&course_repo, "Course With Category", "CWC101").await?;

        // Create a mapper
        let mapper = CourseCategoryMapper::new(course_repo.clone(), category_repo.clone());

        // Map the course to a category
        let category_name = Some("Category For Course".to_string());
        let (_, _) = mapper.map_course_to_category(&course.id, category_name).await?;

        // Now fetch the course with its category
        let (fetched_course, maybe_category) = mapper.get_course_with_category(&course.id).await?;

        // Verify that we got the course with its category
        assert_eq!(fetched_course.id, course.id);
        assert!(maybe_category.is_some());
        let category = maybe_category.unwrap();
        assert_eq!(category.name, "Category For Course");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_category_with_course() -> Result<(), Box<dyn std::error::Error>> {
        // Set up test environment
        let (pool, course_repo, category_repo) = setup_test_environment().await?;

        // Create a course
        let course = create_test_course(&course_repo, "Course For Category", "CFC101").await?;

        // Create a mapper
        let mapper = CourseCategoryMapper::new(course_repo.clone(), category_repo.clone());

        // Map the course to a category
        let category_name = Some("Test Category With Course".to_string());
        let (_, created_category) = mapper.map_course_to_category(&course.id, category_name).await?;

        // Now fetch the category with its course
        let (fetched_category, maybe_course) = mapper.get_category_with_course(&created_category.id).await?;

        // Verify that we got the category with its course
        assert_eq!(fetched_category.id, created_category.id);
        assert!(maybe_course.is_some());
        let fetched_course = maybe_course.unwrap();
        assert_eq!(fetched_course.id, course.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_remapping_course() -> Result<(), Box<dyn std::error::Error>> {
        // Set up test environment
        let (pool, course_repo, category_repo) = setup_test_environment().await?;

        // Create two courses
        let course1 = create_test_course(&course_repo, "Course One", "C1").await?;
        let course2 = create_test_course(&course_repo, "Course Two", "C2").await?;

        // Create a mapper
        let mapper = CourseCategoryMapper::new(course_repo.clone(), category_repo.clone());

        // Map the first course to a category
        let category_name1 = Some("Category One".to_string());
        let (updated_course1, category1) = mapper.map_course_to_category(&course1.id, category_name1).await?;

        // Try to map the second course to the same category - this should fail
        let result = mapper.map_course_to_category(&course2.id, Some(category1.name.clone())).await;
        assert!(result.is_err());
        
        // The error message should indicate the course already has a category
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("already has a course") || error_msg.contains("already mapped"));

        // Verify the first mapping is still intact
        let (fetched_course, maybe_category) = mapper.get_course_with_category(&course1.id).await?;
        assert_eq!(fetched_course.id, course1.id);
        assert!(maybe_category.is_some());
        assert_eq!(maybe_category.unwrap().id, category1.id);

        Ok(())
    }
}