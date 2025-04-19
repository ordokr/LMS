#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::model_mapper::ModelMapperService;
    use crate::services::model_conversion::{
        ModelConversionService, CanvasUser, DiscourseUser, CanvasCourse, DiscourseCategory,
        CanvasDiscussion, DiscourseTopic, CanvasComment, DiscoursePost
    };
    use std::sync::Arc;
    use chrono::Utc;

    #[test]
    fn test_convert_canvas_user_to_local() {
        // Create a model mapper service
        let model_mapper = Arc::new(ModelMapperService::new());
        
        // Create a model conversion service
        let model_conversion = ModelConversionService::new(model_mapper.clone());
        
        // Create a Canvas user
        let canvas_user = CanvasUser {
            id: "12345".to_string(),
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            login_id: Some("jdoe".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };
        
        // Convert the Canvas user to a local user
        let (user, profile) = model_conversion.convert_canvas_user_to_local(&canvas_user);
        
        // Check the user
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john.doe@example.com");
        
        // Check the profile
        assert_eq!(profile.name, "John Doe");
        assert_eq!(profile.canvas_id, Some("12345".to_string()));
        assert_eq!(profile.login_id, Some("jdoe".to_string()));
        assert_eq!(profile.primary_email, Some("john.doe@example.com".to_string()));
        assert_eq!(profile.avatar_url, Some("https://example.com/avatar.jpg".to_string()));
        
        // Check the mapping
        let mapping = model_mapper.find_by_canvas_id("user", "12345").unwrap();
        assert_eq!(mapping.entity_type, "user");
        assert_eq!(mapping.canvas_id, Some("12345".to_string()));
        assert_eq!(mapping.discourse_id, None);
        assert_eq!(mapping.local_id, user.id.unwrap());
    }
    
    #[test]
    fn test_convert_discourse_user_to_local() {
        // Create a model mapper service
        let model_mapper = Arc::new(ModelMapperService::new());
        
        // Create a model conversion service
        let model_conversion = ModelConversionService::new(model_mapper.clone());
        
        // Create a Discourse user
        let discourse_user = DiscourseUser {
            id: 67890,
            name: "Jane Smith".to_string(),
            username: "jsmith".to_string(),
            email: "jane.smith@example.com".to_string(),
            avatar_template: Some("/user_avatar/discourse.example.com/jsmith/{size}/12345_2.png".to_string()),
            bio_raw: Some("I am a software developer.".to_string()),
            website: Some("https://janesmith.com".to_string()),
            location: Some("New York".to_string()),
        };
        
        // Convert the Discourse user to a local user
        let (user, profile) = model_conversion.convert_discourse_user_to_local(&discourse_user);
        
        // Check the user
        assert_eq!(user.name, "Jane Smith");
        assert_eq!(user.email, "jane.smith@example.com");
        
        // Check the profile
        assert_eq!(profile.name, "Jane Smith");
        assert_eq!(profile.discourse_id, Some(67890));
        assert_eq!(profile.username, Some("jsmith".to_string()));
        assert_eq!(profile.bio, Some("I am a software developer.".to_string()));
        assert_eq!(profile.website, Some("https://janesmith.com".to_string()));
        assert_eq!(profile.location, Some("New York".to_string()));
        
        // Check the mapping
        let mapping = model_mapper.find_by_discourse_id("user", "67890").unwrap();
        assert_eq!(mapping.entity_type, "user");
        assert_eq!(mapping.canvas_id, None);
        assert_eq!(mapping.discourse_id, Some("67890".to_string()));
        assert_eq!(mapping.local_id, user.id.unwrap());
    }
    
    #[test]
    fn test_convert_canvas_course_to_local() {
        // Create a model mapper service
        let model_mapper = Arc::new(ModelMapperService::new());
        
        // Create a model conversion service
        let model_conversion = ModelConversionService::new(model_mapper.clone());
        
        // Create a Canvas course
        let canvas_course = CanvasCourse {
            id: "12345".to_string(),
            name: "Introduction to Computer Science".to_string(),
            course_code: "CS101".to_string(),
            description: Some("An introduction to computer science.".to_string()),
            start_at: Some(Utc::now()),
            end_at: Some(Utc::now()),
        };
        
        // Convert the Canvas course to a local course
        let course = model_conversion.convert_canvas_course_to_local(&canvas_course);
        
        // Check the course
        assert_eq!(course.name, "Introduction to Computer Science");
        assert_eq!(course.code, "CS101");
        assert_eq!(course.description, Some("An introduction to computer science.".to_string()));
        
        // Check the mapping
        let mapping = model_mapper.find_by_canvas_id("course", "12345").unwrap();
        assert_eq!(mapping.entity_type, "course");
        assert_eq!(mapping.canvas_id, Some("12345".to_string()));
        assert_eq!(mapping.discourse_id, None);
        assert_eq!(mapping.local_id, course.id.unwrap());
    }
    
    #[test]
    fn test_convert_discourse_category_to_local() {
        // Create a model mapper service
        let model_mapper = Arc::new(ModelMapperService::new());
        
        // Create a model conversion service
        let model_conversion = ModelConversionService::new(model_mapper.clone());
        
        // Create a Discourse category
        let discourse_category = DiscourseCategory {
            id: 67890,
            name: "General Discussion".to_string(),
            slug: "general-discussion".to_string(),
            description: Some("A place for general discussion.".to_string()),
        };
        
        // Convert the Discourse category to a local course
        let course = model_conversion.convert_discourse_category_to_local(&discourse_category);
        
        // Check the course
        assert_eq!(course.name, "General Discussion");
        assert_eq!(course.code, "general-discussion");
        assert_eq!(course.description, Some("A place for general discussion.".to_string()));
        
        // Check the mapping
        let mapping = model_mapper.find_by_discourse_id("course", "67890").unwrap();
        assert_eq!(mapping.entity_type, "course");
        assert_eq!(mapping.canvas_id, None);
        assert_eq!(mapping.discourse_id, Some("67890".to_string()));
        assert_eq!(mapping.local_id, course.id.unwrap());
    }
}
