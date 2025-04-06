use uuid::Uuid;
use chrono::Utc;
use crate::{
    db::{
        assignment_repository::AssignmentRepository,
        topic_repository::TopicRepository,
        post_repository::PostRepository,
    },
    models::{
        assignment::Assignment,
        topic::Topic,
        post::Post,
    },
    services::topic_service::TopicService,
};

pub struct AssignmentTopicMapper {
    assignment_repo: AssignmentRepository,
    topic_repo: TopicRepository,
    post_repo: PostRepository,
    topic_service: TopicService,
}

impl AssignmentTopicMapper {
    pub fn new(
        assignment_repo: AssignmentRepository,
        topic_repo: TopicRepository,
        post_repo: PostRepository,
        topic_service: TopicService,
    ) -> Self {
        Self {
            assignment_repo,
            topic_repo,
            post_repo,
            topic_service,
        }
    }

    /// Creates a discussion topic from an assignment
    pub async fn create_topic_from_assignment(
        &self,
        assignment_id: &Uuid,
        category_id: Uuid,
        author_id: Uuid,
    ) -> Result<(Topic, Post), String> {
        // Find the assignment
        let assignment = self.assignment_repo.find_assignment_by_id(assignment_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Assignment with ID {} not found", assignment_id))?;
        
        // Check if assignment already has a topic
        if assignment.topic_id.is_some() {
            return Err(format!("Assignment {} already has a topic assigned", assignment_id));
        }

        // Create topic with initial post
        let content = assignment.description
            .clone()
            .unwrap_or_else(|| "No description provided.".to_string());

        let (topic, post) = self.topic_service
            .create_topic_with_post(
                &assignment.title,
                &content,
                category_id,
                author_id,
                false, // Not pinned
                false, // Not closed
            )
            .await
            .map_err(|e| format!("Failed to create topic: {}", e))?;

        // Update assignment with topic ID
        self.assignment_repo.update_assignment_topic(assignment_id, &Some(topic.id))
            .await
            .map_err(|e| format!("Failed to update assignment with topic ID: {}", e))?;

        Ok((topic, post))
    }

    /// Gets the topic associated with an assignment
    pub async fn get_topic_for_assignment(&self, assignment_id: &Uuid) -> Result<Option<(Topic, Vec<Post>)>, String> {
        // Find the assignment
        let assignment = self.assignment_repo.find_assignment_by_id(assignment_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Assignment with ID {} not found", assignment_id))?;
        
        // Check if assignment has a topic
        match assignment.topic_id {
            Some(topic_id) => {
                // Get topic
                let topic = self.topic_repo.find_topic_by_id(&topic_id)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?
                    .ok_or_else(|| format!("Topic with ID {} not found", topic_id))?;
                
                // Get posts
                let posts = self.post_repo.find_posts_by_topic_id(&topic_id)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?;
                
                Ok(Some((topic, posts)))
            },
            None => Ok(None)
        }
    }

    /// Gets the assignment associated with a topic
    pub async fn get_assignment_for_topic(&self, topic_id: &Uuid) -> Result<Option<Assignment>, String> {
        let assignment = self.assignment_repo.find_assignment_by_topic(topic_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        Ok(assignment)
    }

    /// Unlinks a topic from an assignment
    pub async fn unlink_topic_from_assignment(&self, assignment_id: &Uuid) -> Result<(), String> {
        // Find the assignment
        let assignment = self.assignment_repo.find_assignment_by_id(assignment_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Assignment with ID {} not found", assignment_id))?;
        
        // If assignment doesn't have a topic, nothing to do
        if assignment.topic_id.is_none() {
            return Ok(());
        }

        // Update assignment topic_id to null
        self.assignment_repo.update_assignment_topic(assignment_id, &None)
            .await
            .map_err(|e| format!("Failed to update assignment: {}", e))?;
        
        Ok(())
    }
    
    /// Update or create a topic for an assignment
    pub async fn update_or_create_topic_for_assignment(
        &self,
        assignment_id: &Uuid,
        category_id: Uuid,
        author_id: Uuid,
    ) -> Result<(Topic, Post), String> {
        // Find the assignment
        let assignment = self.assignment_repo.find_assignment_by_id(assignment_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Assignment with ID {} not found", assignment_id))?;
        
        // If assignment already has a topic, update it instead of creating a new one
        if let Some(topic_id) = assignment.topic_id {
            // Get existing topic
            let topic = self.topic_repo.find_topic_by_id(&topic_id)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or_else(|| format!("Topic with ID {} not found", topic_id))?;
            
            // Update topic title to match the assignment title
            let updated_topic = Topic {
                title: assignment.title.clone(),
                updated_at: Utc::now(),
                ..topic
            };
            
            self.topic_repo.update_topic(&updated_topic)
                .await
                .map_err(|e| format!("Failed to update topic: {}", e))?;
            
            // Get the first post
            let posts = self.post_repo.find_posts_by_topic_id(&topic_id)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
            
            let first_post = posts.first().ok_or_else(|| "Topic has no posts".to_string())?;
            
            // Update first post content if assignment description changed
            if let Some(description) = &assignment.description {
                if &first_post.content != description {
                    let updated_post = Post {
                        content: description.clone(),
                        updated_at: Utc::now(),
                        ..first_post.clone()
                    };
                    
                    self.post_repo.update_post(&updated_post)
                        .await
                        .map_err(|e| format!("Failed to update post: {}", e))?;
                    
                    return Ok((updated_topic, updated_post));
                }
            }
            
            return Ok((updated_topic, first_post.clone()));
        }
        
        // Otherwise, create a new topic
        self.create_topic_from_assignment(assignment_id, category_id, author_id).await
    }
    
    /// Get all assignments with their associated topics
    pub async fn get_all_assignments_with_topics(&self, course_id: &Uuid) -> Result<Vec<(Assignment, Option<Topic>)>, String> {
        let assignments = self.assignment_repo.find_assignments_by_course(course_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        let mut result = Vec::new();
        
        for assignment in assignments {
            let topic = match &assignment.topic_id {
                Some(topic_id) => {
                    match self.topic_service.get_topic(topic_id).await {
                        Ok(topic) => Some(topic),
                        Err(_) => None,
                    }
                },
                None => None,
            };
            
            result.push((assignment, topic));
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
            assignment::Assignment,
            topic::Topic,
            post::Post,
        },
        db::{
            assignment_repository::AssignmentRepository,
            topic_repository::TopicRepository,
            post_repository::PostRepository,
        },
        services::topic_service::TopicService,
    };
    use mockall::predicate::*;
    use mockall::mock;
    use chrono::Utc;

    mock! {
        AssignmentRepository {}
        impl AssignmentRepository {
            pub async fn find_assignment_by_id(&self, id: &Uuid) -> Result<Option<Assignment>, sqlx::Error>;
            pub async fn update_assignment_topic(&self, id: &Uuid, topic_id: &Option<Uuid>) -> Result<(), sqlx::Error>;
            pub async fn find_assignment_by_topic(&self, topic_id: &Uuid) -> Result<Option<Assignment>, sqlx::Error>;
            pub async fn find_assignments_by_course(&self, course_id: &Uuid) -> Result<Vec<Assignment>, sqlx::Error>;
        }
    }

    mock! {
        TopicService {}
        impl TopicService {
            pub async fn create_topic_with_post(
                &self,
                title: String,
                category_id: Uuid,
                author_id: Uuid,
                content: String,
                assignment_id: Option<Uuid>,
            ) -> Result<(Topic, Post), String>;
            pub async fn get_topic_with_posts(&self, id: &Uuid) -> Result<(Topic, Vec<Post>), String>;
            pub async fn get_topic(&self, id: &Uuid) -> Result<Topic, String>;
            pub async fn update_topic(&self, topic: &Topic) -> Result<(), String>;
            pub async fn update_post(&self, post: &Post) -> Result<(), String>;
            pub async fn create_post(
                &self,
                topic_id: Uuid,
                author_id: Uuid,
                content: String,
                parent_id: Option<Uuid>,
            ) -> Result<Post, String>;
        }
    }

    #[tokio::test]
    async fn test_create_topic_from_assignment() {
        let assignment_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let topic_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        
        let now = Utc::now();
        
        let assignment = Assignment {
            id: assignment_id,
            title: "Test Assignment".to_string(),
            description: Some("Test description".to_string()),
            due_date: None,
            points_possible: Some(100),
            course_id: Uuid::new_v4(),
            topic_id: None,
            created_at: now,
            updated_at: now,
        };
        
        let topic = Topic {
            id: topic_id,
            title: "Test Assignment".to_string(),
            slug: "test-assignment".to_string(),
            category_id,
            author_id,
            post_count: 1,
            view_count: 0,
            created_at: now,
            updated_at: now,
            pinned: false,
            closed: false,
            assignment_id: Some(assignment_id),
        };
        
        let post = Post {
            id: post_id,
            topic_id,
            author_id,
            content: "Test description".to_string(),
            parent_id: None,
            created_at: now,
            updated_at: now,
        };
        
        let mut mock_assignment_repo = MockAssignmentRepository::new();
        mock_assignment_repo
            .expect_find_assignment_by_id()
            .with(eq(assignment_id))
            .times(1)
            .returning(move |_| Ok(Some(assignment.clone())));
        
        mock_assignment_repo
            .expect_update_assignment_topic()
            .with(eq(assignment_id), eq(Some(topic_id)))
            .times(1)
            .returning(|_, _| Ok(()));
        
        let mut mock_topic_service = MockTopicService::new();
        mock_topic_service
            .expect_create_topic_with_post()
            .with(
                eq("Test Assignment".to_string()),
                eq(category_id),
                eq(author_id),
                eq("Test description".to_string()),
                eq(Some(assignment_id))
            )
            .times(1)
            .returning(move |_, _, _, _, _| Ok((topic.clone(), post.clone())));
        
        let mapper = AssignmentTopicMapper {
            assignment_repo: mock_assignment_repo,
            topic_service: mock_topic_service,
        };
        
        let result = mapper.create_topic_from_assignment(&assignment_id, category_id, author_id).await;
        assert!(result.is_ok());
        
        let (result_topic, result_post) = result.unwrap();
        assert_eq!(result_topic.id, topic_id);
        assert_eq!(result_post.id, post_id);
    }
}