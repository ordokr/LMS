#[cfg(test)]
mod tests {
    use super::super::course_category_mapper::CourseCategoryMapper;
    use crate::{
        db::{course_repository::CourseRepository, category_repository::CategoryRepository},
        models::{course::Course, category::Category},
    };
    use chrono::Utc;
    use uuid::Uuid;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        CourseRepository {}
        impl CourseRepository {
            pub fn new(_pool: sqlx::PgPool) -> Self;
            pub async fn find_course_by_id(&self, id: &Uuid) -> Result<Option<Course>, sqlx::Error>;
            pub async fn update_course_category(&self, course_id: &Uuid, category_id: &Uuid) -> Result<(), sqlx::Error>;
        }
    }

    mock! {
        CategoryRepository {}
        impl CategoryRepository {
            pub fn new(_pool: sqlx::PgPool) -> Self;
            pub async fn create_category(&self, category: &Category) -> Result<Category, sqlx::Error>;
            pub async fn find_category_by_id(&self, id: &Uuid) -> Result<Option<Category>, sqlx::Error>;
        }
    }

    #[tokio::test]
    async fn test_map_course_to_category() {
        let course_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let instructor_id = Uuid::new_v4();
        
        let now = Utc::now();
        
        let course = Course {
            id: course_id,
            canvas_id: "canvas-123".to_string(),
            name: "Test Course".to_string(),
            code: "TEST101".to_string(),
            description: Some("Test description".to_string()),
            instructor_id,
            start_date: None,
            end_date: None,
            created_at: now,
            updated_at: now,
            category_id: None,
            is_published: false,
        };
        
        let updated_course = Course {
            category_id: Some(category_id),
            ..course.clone()
        };
        
        let category = Category {
            id: category_id,
            name: "Test Course".to_string(),
            slug: "test-course".to_string(),
            description: Some("Test description".to_string()),
            parent_id: None,
            created_at: now,
            updated_at: now,
            course_id: Some(course_id),
            position: