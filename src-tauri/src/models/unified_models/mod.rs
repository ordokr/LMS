// Unified models module
// This module contains consolidated model implementations that replace redundant models

mod user;
mod course;
mod group;
mod assignment;
mod topic;
mod submission;

// Re-export models for convenience
pub use user::User;
pub use course::{Course, CourseStatus, CourseVisibility, HomepageType};
pub use group::{Group, GroupJoinLevel, GroupMembership, GroupMembershipStatus};
pub use assignment::{Assignment, SubmissionType, GradingType, AssignmentStatus};
pub use topic::{Topic, TopicStatus, TopicVisibility, TopicType};
pub use submission::{Submission, SubmissionStatus, SubmissionType as SubmissionContentType, SubmissionComment};
