#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc, TimeZone};
    use uuid::Uuid;
    use crate::models::content::submission::{Submission, SubmissionStatus, SubmissionComment};
    use serde_json::json;

    #[test]
    fn test_submission_creation() {
        let assignment_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        let submission = Submission::new(assignment_id, user_id);
        
        // Verify basic fields
        assert_eq!(submission.assignment_id, assignment_id);
        assert_eq!(submission.user_id, user_id);
        assert_eq!(submission.status, SubmissionStatus::NotSubmitted);
        assert_eq!(submission.attempt, 1);
        assert!(submission.submitted_at.is_none());
        assert!(submission.graded_at.is_none());
        
        // Verify created_at and updated_at are set
        let now = Utc::now();
        let diff_created = (submission.created_at - now).num_seconds();
        assert!(diff_created.abs() < 5); // Within 5 seconds
        
        let diff_updated = (submission.updated_at - now).num_seconds();
        assert!(diff_updated.abs() < 5); // Within 5 seconds
    }

    #[test]
    fn test_submission_validation() {
        let assignment_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Valid submission with content
        let mut submission = Submission::new(assignment_id, user_id);
        submission.content = Some("This is my submission".to_string());
        assert!(submission.validate().is_ok());
        
        // Valid submission with URL
        let mut submission = Submission::new(assignment_id, user_id);
        submission.url = Some("https://example.com/mysubmission".to_string());
        assert!(submission.validate().is_ok());
        
        // Valid submission with attachment
        let mut submission = Submission::new(assignment_id, user_id);
        submission.attachment_ids.push(Uuid::new_v4());
        assert!(submission.validate().is_ok());
        
        // Invalid submission with no content
        let submission = Submission::new(assignment_id, user_id);
        assert!(submission.validate().is_err());
        
        // Invalid submission with negative score
        let mut submission = Submission::new(assignment_id, user_id);
        submission.content = Some("Content".to_string());
        submission.score = Some(-10.0);
        assert!(submission.validate().is_err());
    }

    #[test]
    fn test_submission_from_canvas_api() {
        let assignment_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        let canvas_json = json!({
            "id": "12345",
            "assignment_id": assignment_id.to_string(),
            "user_id": user_id.to_string(),
            "body": "This is my submission content",
            "url": "https://example.com/submission",
            "submission_type": "online_text_entry",
            "grade": "A",
            "score": 95.5,
            "workflow_state": "graded",
            "late": true,
            "attempt": 2,
            "submitted_at": "2025-04-05T14:30:00Z",
            "graded_at": "2025-04-06T09:15:00Z"
        });
        
        let submission = Submission::from_canvas_api(&canvas_json).unwrap();
        
        // Verify basic fields
        assert_eq!(submission.assignment_id, assignment_id);
        assert_eq!(submission.user_id, user_id);
        assert_eq!(submission.content, Some("This is my submission content".to_string()));
        assert_eq!(submission.body, Some("This is my submission content".to_string()));
        assert_eq!(submission.url, Some("https://example.com/submission".to_string()));
        assert_eq!(submission.submission_type, "online_text_entry");
        assert_eq!(submission.grade, Some("A".to_string()));
        assert_eq!(submission.score, Some(95.5));
        assert_eq!(submission.status, SubmissionStatus::Graded);
        assert_eq!(submission.is_late, true);
        assert_eq!(submission.attempt, 2);
        
        // Verify dates were parsed correctly
        assert_eq!(submission.submitted_at.unwrap().year(), 2025);
        assert_eq!(submission.submitted_at.unwrap().month(), 4);
        assert_eq!(submission.submitted_at.unwrap().day(), 5);
        
        assert_eq!(submission.graded_at.unwrap().year(), 2025);
        assert_eq!(submission.graded_at.unwrap().month(), 4);
        assert_eq!(submission.graded_at.unwrap().day(), 6);
    }

    #[test]
    fn test_submission_serialization() {
        let assignment_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Create submission with comments
        let mut submission = Submission::new(assignment_id, user_id);
        submission.content = Some("Test submission content".to_string());
        submission.status = SubmissionStatus::Submitted;
        submission.submitted_at = Some(Utc::now());
        
        // Add a comment
        let comment = SubmissionComment {
            id: Uuid::new_v4(),
            submission_id: submission.id,
            user_id,
            comment: "This is a test comment".to_string(),
            created_at: Utc::now(),
            attachment_ids: vec![],
        };
        submission.comments.push(comment);
        
        // Serialize to JSON
        let json = serde_json::to_string(&submission).unwrap();
        
        // Deserialize from JSON
        let deserialized: Submission = serde_json::from_str(&json).unwrap();
        
        // Verify fields
        assert_eq!(submission.id, deserialized.id);
        assert_eq!(submission.content, deserialized.content);
        assert_eq!(submission.status, deserialized.status);
        assert_eq!(
            submission.submitted_at.unwrap().timestamp(),
            deserialized.submitted_at.unwrap().timestamp()
        );
        assert_eq!(submission.comments.len(), deserialized.comments.len());
    }
}