mod user_repository;
mod category_repository;
mod topic_repository;
mod post_repository;

pub use user_repository::UserRepository;
pub use category_repository::CategoryRepository;
pub use topic_repository::TopicRepository;
pub use post_repository::PostRepository;

pub mod user;
pub mod forum;
pub mod course;
pub mod module;
pub mod assignment;

pub use user::UserRepository;
pub use forum::{ForumCategoryRepository, ForumTopicRepository};
pub use course::CourseRepository;
pub use module::ModuleRepository;
pub use assignment::AssignmentRepository;