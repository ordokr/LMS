use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub assignment_id: String,
    pub user_id: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub status: SubmissionStatus,
    pub score: Option<f64>,
    pub feedback: Option<String>,
    pub submitted_at: String,
    pub graded_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionCreate {
    pub assignment_id: String,
    pub user_id: String,
    pub content: String,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmissionStatus {
    Draft,
    Submitted,
    Graded,
    Resubmitted,
    Late,
}

impl ToString for SubmissionStatus {
    fn to_string(&self) -> String {
        match self {
            SubmissionStatus::Draft => "draft".to_string(),
            SubmissionStatus::Submitted => "submitted".to_string(),
            SubmissionStatus::Graded => "graded".to_string(),
            SubmissionStatus::Resubmitted => "resubmitted".to_string(),
            SubmissionStatus::Late => "late".to_string(),
        }
    }
}