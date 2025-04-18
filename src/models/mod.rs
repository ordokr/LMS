// Auto-generated from index.js
// Source: src/models/index.js

// Re-export the model factory
pub mod model_factory;
pub use model_factory::ModelFactory;
pub use model_factory::SourceSystem;
pub use model_factory::ModelFactoryError;

// Quiz module
pub mod quiz;
pub mod quiz_course;
pub mod quiz_notification;
pub mod quiz_collaboration;
pub mod quiz_template;
pub mod quiz_ai_generation;
pub mod quiz_adaptive_learning;
pub mod quiz_question_types;

// Network module
pub mod network;

// Re-export the canvas models
pub mod canvas {
    pub mod user;
    pub mod course;
    pub mod discussion;
    pub mod assignment;
    pub mod notification;
    pub mod file;
    pub mod calendar;
    pub mod rubric;
    pub mod base_model;
    pub mod user_model;

    // Re-export primary models for convenience
    pub use self::user::User;
    pub use self::course::Course;
    pub use self::discussion::Discussion;
    pub use self::assignment::Assignment;
    pub use self::notification::Notification;
    pub use self::file::File;
    pub use self::calendar::Calendar;
    pub use self::rubric::Rubric;
    pub use self::base_model::BaseModel;
    pub use self::user_model::UserModel;
}

// Entity relationship models
pub mod entity_relationship;
pub use entity_relationship::{EntityType, RelationshipType, EntityRelationship, EntityRelationshipGraph};

// Additional entity models
pub mod submission;
pub mod group;
pub mod page;
pub mod file;
pub mod announcement;

// Re-export for backward compatibility
pub use canvas::{User, Course, Discussion, Assignment, Notification, Calendar, Rubric};
pub use submission::Submission;
pub use group::Group;
pub use page::Page;
pub use file::File;
pub use announcement::Announcement;
