// Central model registry
pub mod auth;
pub mod user;
pub mod course;
pub mod content;
pub mod forum;
pub mod grade;
pub mod blockchain;
pub mod ids; // Add new ids module

// Re-export commonly used models for convenience
pub use user::user::User;
pub use user::profile::Profile;
pub use user::preferences::UserPreferences;

pub use course::course::{Course, CourseStatus};
pub use course::enrollment::Enrollment;
pub use course::module::Module;

pub use content::assignment::Assignment;
pub use content::submission::Submission;
pub use content::resource::Resource;
pub use content::attachment::Attachment;

pub use forum::topic::Topic;
pub use forum::post::Post;
pub use forum::category::Category;
pub use forum::tag::Tag;

pub use grade::grade::Grade;
pub use grade::rubric::Rubric;
pub use grade::criteria::Criteria;

pub use blockchain::block::Block;
pub use blockchain::transaction::Transaction;

// Re-export ID types
pub use ids::{UserId, CourseId, AssignmentId, TopicId};