// Module declarations
pub mod admin;
pub mod forum;
pub mod lms;
pub mod notification;
pub mod user;
pub mod sync;

// Submodule declarations
pub mod forum {
    pub mod tag;
}

// Re-export common models for easier importing
pub use self::forum::{Group, Site, PostActionType, Category, Topic, Post};
pub use self::lms::{
    Course, Module, ModuleItem, Assignment, AssignmentGroup, 
    Enrollment, Submission, Page, CompletionRequirement
};
pub use self::user::{User, Badge};
pub use self::notification::{Notification, NotificationPreference};
pub use self::admin::{GroupMember, Setting};
pub use self::forum::tag::Tag;