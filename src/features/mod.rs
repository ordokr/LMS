pub mod dashboard;
pub mod courses;
pub mod assignments;
pub mod forum;

pub use dashboard::Dashboard;
pub use courses::{CoursesList, CourseDetail};
pub use assignments::{AssignmentsList, AssignmentDetail};
pub use forum::{ForumCategories, ForumThreads, ThreadDetail};