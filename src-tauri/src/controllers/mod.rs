pub mod forum;

// Re-export forum controllers
pub use forum::topics_controller::*;
pub use forum::posts_controller::*;
pub use forum::categories_controller::*;