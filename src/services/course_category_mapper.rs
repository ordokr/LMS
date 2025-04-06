use uuid::Uuid;
use crate::{
    db::{course_repository::CourseRepository, category_repository::CategoryRepository},
    models::{course::Course, category::Category},
};

pub struct CourseCategoryMapper {
    course_repo: CourseRepository,
    category_repo: CategoryRepository,
}

impl CourseCategoryMapper {
    pub fn new(course_repo: CourseRepository, category_repo: CategoryRepository) -> Self {
        Self {
            course_repo,
            category_repo,
        }
    }

    /// Creates a new category for a course and links them together
    pub async fn map_course_to_category(
        &self,
        course_id: &Uuid,
        category_name: Option<String>,
    ) -> Result<(Course, Category), String> {
        // Find the course
        let course = self.course_repo.find_course_by_id(course_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Course with ID {} not found", course_id))?;
        
        // Check if course already has a category
        if course.category_id.is_some() {
            return Err(format!("Course {} already has a category assigned", course_id));
        }

        // Create a category for the course
        let category_name = category_name.unwrap_or_else(|| course.name.clone());
        let category = Category::new(
            category_name,
            course.description.clone(),
            None, // No parent category
            Some(*course_id),
            0, // Default position
        );

        // Save category
        let saved_category = self.category_repo.create_category(&category)
            .await
            .map_err(|e| format!("Failed to create category: {}", e))?;

        // Update course with category ID
        self.course_repo.update_course_category(course_id, &saved_category.id)
            .await
            .map_err(|e| format!("Failed to update course with category ID: {}", e))?;

        // Get updated course
        let updated_course = self.course_repo.find_course_by_id(course_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Course with ID {} not found after update", course_id))?;

        Ok((updated_course, saved_category))
    }

    /// Gets course and corresponding category
    pub async fn get_course_with_category(&self, course_id: &Uuid) -> Result<(Course, Option<Category>), String> {
        // Find the course
        let course = self.course_repo.find_course_by_id(course_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Course with ID {} not found", course_id))?;
        
        // If course has a category, get it
        let category = match course.category_id {
            Some(category_id) => {
                self.category_repo.find_category_by_id(&category_id)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?
            },
            None => None
        };

        Ok((course, category))
    }

    /// Gets category and corresponding course
    pub async fn get_category_with_course(&self, category_id: &Uuid) -> Result<(Category, Option<Course>), String> {
        // Find the category
        let category = self.category_repo.find_category_by_id(category_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Category with ID {} not found", category_id))?;
        
        // If category has a course, get it
        let course = match category.course_id {
            Some(course_id) => {
                self.course_repo.find_course_by_id(&course_id)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?
            },
            None => None
        };

        Ok((category, course))
    }
}