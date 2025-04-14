use crate::models::blockchain::Certificate;
use std::collections::HashMap;
use chrono::Utc;

pub struct Blockchain {
    certificates: HashMap<String, Certificate>, // Simulated blockchain storage
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            certificates: HashMap::new(),
        }
    }

    pub fn add_certificate(&mut self, user_id: &str, course_id: &str, metadata: &str) -> Certificate {
        let id = uuid::Uuid::new_v4().to_string();
        let certificate = Certificate {
            id: id.clone(),
            user_id: user_id.to_string(),
            course_id: course_id.to_string(),
            issued_at: Utc::now(),
            metadata: metadata.to_string(),
        };
        self.certificates.insert(id.clone(), certificate.clone());
        certificate
    }

    pub fn get_certificate(&self, id: &str) -> Option<&Certificate> {
        self.certificates.get(id)
    }
}