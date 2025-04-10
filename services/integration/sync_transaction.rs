// services/integration/sync_transaction.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

use crate::shared::logger::Logger;
use crate::shared::db::Database;

/// Sync Transaction Manager
///
/// Manages the lifecycle of synchronization transactions, including
/// tracking, commit/rollback, and persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTransaction {
    pub transaction_id: String,
    pub entity_type: String,
    pub operation: String,
    pub source_system: String,
    pub target_system: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
    pub steps: Vec<TransactionStep>,
    pub event: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStep {
    pub step_id: String,
    pub name: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl SyncTransaction {
    /// Create a new sync transaction
    pub fn new(event: serde_json::Value) -> Self {
        let transaction_id = event["transactionId"].as_str()
            .unwrap_or_else(|| Uuid::new_v4().to_string().as_str())
            .to_string();
            
        let entity_type = event["entityType"].as_str()
            .unwrap_or("unknown")
            .to_string();
            
        let operation = event["operation"].as_str()
            .unwrap_or("unknown")
            .to_string();
            
        let source_system = event["sourceSystem"].as_str()
            .unwrap_or("unknown")
            .to_string();
            
        let target_system = event["targetSystem"].as_str()
            .unwrap_or("unknown")
            .to_string();
        
        SyncTransaction {
            transaction_id,
            entity_type,
            operation,
            source_system,
            target_system,
            start_time: Utc::now(),
            end_time: None,
            status: "pending".to_string(),
            steps: Vec::new(),
            event,
        }
    }
    
    /// Begin the transaction
    pub async fn begin(&mut self, db: &mut Database) -> Result<(), Box<dyn Error>> {
        let logger = Logger::new("SyncTransaction");
        logger.info(&format!("Beginning sync transaction: {}", self.transaction_id), None);
        
        // Record transaction start in database
        db.sync_transactions.create(serde_json::json!({
            "transactionId": self.transaction_id,
            "entityType": self.entity_type,
            "operation": self.operation,
            "sourceSystem": self.source_system,
            "targetSystem": self.target_system,
            "startTime": self.start_time,
            "status": self.status,
        })).await?;
        
        Ok(())
    }
    
    /// Add a step to the transaction
    pub fn add_step(&mut self, name: &str, status: &str, data: Option<serde_json::Value>) -> TransactionStep {
        let step = TransactionStep {
            step_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            status: status.to_string(),
            timestamp: Utc::now(),
            error: None,
            data,
        };
        
        self.steps.push(step.clone());
        step
    }
    
    /// Mark a step as complete
    pub fn complete_step(&mut self, step_id: &str) -> Option<&mut TransactionStep> {
        if let Some(step) = self.steps.iter_mut().find(|s| s.step_id == step_id) {
            step.status = "completed".to_string();
            Some(step)
        } else {
            None
        }
    }
    
    /// Mark a step as failed
    pub fn fail_step(&mut self, step_id: &str, error: &str) -> Option<&mut TransactionStep> {
        if let Some(step) = self.steps.iter_mut().find(|s| s.step_id == step_id) {
            step.status = "failed".to_string();
            step.error = Some(error.to_string());
            Some(step)
        } else {
            None
        }
    }
    
    /// Commit the transaction
    pub async fn commit(&mut self, db: &mut Database) -> Result<(), Box<dyn Error>> {
        let logger = Logger::new("SyncTransaction");
        logger.info(&format!("Committing sync transaction: {}", self.transaction_id), None);
        
        self.status = "committed".to_string();
        self.end_time = Some(Utc::now());
        
        // Update transaction in database
        db.sync_transactions.update(
            self.transaction_id.clone(),
            serde_json::json!({
                "status": self.status,
                "endTime": self.end_time,
                "steps": self.steps,
            })
        ).await?;
        
        Ok(())
    }
    
    /// Rollback the transaction
    pub async fn rollback(&mut self, db: &mut Database, reason: &str) -> Result<(), Box<dyn Error>> {
        let logger = Logger::new("SyncTransaction");
        logger.warn(&format!("Rolling back sync transaction: {} - Reason: {}", 
            self.transaction_id, reason), None);
        
        self.status = "rolled_back".to_string();
        self.end_time = Some(Utc::now());
        
        // Add rollback step
        self.add_step("rollback", "completed", Some(serde_json::json!({
            "reason": reason
        })));
        
        // Update transaction in database
        db.sync_transactions.update(
            self.transaction_id.clone(),
            serde_json::json!({
                "status": self.status,
                "endTime": self.end_time,
                "steps": self.steps,
            })
        ).await?;
        
        Ok(())
    }
}
