// Central model registry
pub mod auth;
pub mod blockchain;
pub mod ids; // ID types module
pub mod unified_models; // Unified models

// Re-export ID types
pub use ids::{UserId, CourseId, AssignmentId, TopicId};

// Export unified models as the primary models
pub use unified_models::User;
pub use unified_models::{Course, CourseStatus, CourseVisibility, HomepageType};
pub use unified_models::{Group, GroupJoinLevel, GroupMembership, GroupMembershipStatus};
pub use unified_models::{Assignment, SubmissionType, GradingType, AssignmentStatus};
pub use unified_models::{Topic, TopicStatus, TopicVisibility, TopicType};
pub use unified_models::{Submission, SubmissionStatus, SubmissionContentType, SubmissionComment};

// For backward compatibility, also export them with the Unified prefix
pub use unified_models::User as UnifiedUser;
pub use unified_models::{Course as UnifiedCourse, CourseStatus as UnifiedCourseStatus};
pub use unified_models::{Group as UnifiedGroup, GroupMembership as UnifiedGroupMembership};
pub use unified_models::{Assignment as UnifiedAssignment, AssignmentStatus as UnifiedAssignmentStatus};
pub use unified_models::{Topic as UnifiedTopic, TopicStatus as UnifiedTopicStatus};
pub use unified_models::{Submission as UnifiedSubmission, SubmissionStatus as UnifiedSubmissionStatus, SubmissionContentType as UnifiedSubmissionType};