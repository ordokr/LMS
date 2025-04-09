use automerge::{Automerge, transaction::Transactable};
use std::collections::BTreeSet;
use uuid::Uuid;

pub struct LmsCrdtOps {
    doc: Automerge,
}

impl LmsCrdtOps {
    pub fn new() -> Self {
        Self {
            doc: Automerge::new(),
        }
    }
    
    pub fn award_badge(&mut self, user_id: Uuid, badge_id: Uuid) -> Result<(), automerge::AutomergeError> {
        let txn = self.doc.transaction();
        
        // Create path to user badges if it doesn't exist
        let user_path = format!("users.{}.badges", user_id);
        if !txn.get_at_path(vec![&user_path]).is_some() {
            txn.put_object_at_path(vec![&user_path], automerge::ObjType::List)?;
        }
        
        // Add badge to user's collection
        txn.insert_at_path(vec![&user_path], 0, &badge_id.to_string())?;
        txn.commit();
        
        Ok(())
    }
    
    pub fn create_certificate(&mut self, student_id: Uuid, course_id: Uuid, metadata: &str) -> Result<Uuid, automerge::AutomergeError> {
        let certificate_id = Uuid::new_v4();
        let txn = self.doc.transaction();
        
        let cert_path = format!("certificates.{}", certificate_id);
        txn.put_object_at_path(vec![&cert_path], automerge::ObjType::Map)?;
        txn.put_at_path(vec![&cert_path, "student_id"], &student_id.to_string())?;
        txn.put_at_path(vec![&cert_path, "course_id"], &course_id.to_string())?;
        txn.put_at_path(vec![&cert_path, "metadata"], metadata)?;
        txn.put_at_path(vec![&cert_path, "timestamp"], &chrono::Utc::now().to_rfc3339())?;
        
        txn.commit();
        
        Ok(certificate_id)
    }
    
    pub fn save(&self) -> Result<Vec<u8>, automerge::AutomergeError> {
        let mut bytes = Vec::new();
        self.doc.save(&mut bytes)?;
        Ok(bytes)
    }
    
    pub fn load(&mut self, bytes: &[u8]) -> Result<(), automerge::AutomergeError> {
        self.doc = Automerge::load(bytes)?;
        Ok(())
    }
}