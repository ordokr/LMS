mod home;
mod layout;
mod auth;
mod lms;
mod forum;
mod shared;

pub use home::Home;
pub use layout::AppLayout as Layout; // Export as Layout if that's what's used in routes
pub use auth::{
    LoginForm as Login,     // Map to route-expected names
    RegisterForm as Register,
    UserProfile
};
pub use lms::{
    CoursesList, CourseDetail, CourseForm,
    AssignmentsList, AssignmentDetail, AssignmentForm,
    ModulesList, ModuleDetail, ModuleForm, ModuleItemForm
};
pub use forum::{
    ForumCategories as CategoriesList,  // Map to expected route component names
    ForumThreads as TopicsList,
    ThreadDetail as TopicDetail,
    CategoryForm,
    CategoryDetail,
    TopicForm
};
pub use shared::{OfflineIndicator, ErrorDisplay};

// You'll need to create these missing components
// or map them to existing ones