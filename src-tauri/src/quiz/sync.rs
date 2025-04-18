use super::models::{Quiz, QuizAttempt};
use super::storage::HybridQuizStore;
use crate::sync::{SyncEngine, SyncOperation, SyncResult};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use log::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizSyncOperation {
    CreateQuiz(Quiz),
    UpdateQuiz(Quiz),
    DeleteQuiz(Uuid),
    RecordAttempt(QuizAttempt),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Create,
    Update,
    Delete,
    RecordAttempt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    id: Uuid,
    timestamp: DateTime<Utc>,
    operation_type: OperationType,
    entity_id: Uuid,
    vector_clock: HashMap<String, u64>,
    payload: Vec<u8>,
}

pub struct VectorClock {
    clocks: HashMap<String, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }

    pub fn increment(&mut self, node_id: &str) {
        let count = self.clocks.entry(node_id.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn to_map(&self) -> HashMap<String, u64> {
        self.clocks.clone()
    }
}

pub struct QuizSyncAdapter {
    store: Arc<HybridQuizStore>,
    vector_clock: Arc<Mutex<VectorClock>>,
    operation_queue: mpsc::UnboundedSender<SyncOperation>,
}

impl QuizSyncAdapter {
    pub fn new(store: Arc<HybridQuizStore>) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let vector_clock = Arc::new(Mutex::new(VectorClock::new()));
        let store_clone = store.clone();

        tokio::spawn(async move {
            while let Some(op) = rx.recv().await {
                if let Err(e) = store_clone.process_sync_operation(op).await {
                    error!("Failed to process sync operation: {}", e);
                }
            }
        });

        Self {
            store,
            vector_clock,
            operation_queue: tx,
        }
    }

    pub async fn queue_operation(&self, op_type: OperationType, entity_id: Uuid, payload: Vec<u8>) -> Result<(), String> {
        let mut clock = self.vector_clock.lock().unwrap();
        clock.increment("local");

        let op = SyncOperation {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            operation_type: op_type,
            entity_id,
            vector_clock: clock.to_map(),
            payload,
        };

        self.operation_queue.send(op).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn queue_create_quiz(&self, quiz: Quiz) -> Result<(), String> {
        let payload = serde_json::to_vec(&quiz).map_err(|e| e.to_string())?;
        self.queue_operation(OperationType::Create, quiz.id, payload).await
    }

    pub async fn queue_update_quiz(&self, quiz: Quiz) -> Result<(), String> {
        let payload = serde_json::to_vec(&quiz).map_err(|e| e.to_string())?;
        self.queue_operation(OperationType::Update, quiz.id, payload).await
    }

    pub async fn queue_delete_quiz(&self, quiz_id: Uuid) -> Result<(), String> {
        self.queue_operation(OperationType::Delete, quiz_id, Vec::new()).await
    }

    pub async fn queue_record_attempt(&self, attempt: QuizAttempt) -> Result<(), String> {
        let payload = serde_json::to_vec(&attempt).map_err(|e| e.to_string())?;
        self.queue_operation(OperationType::RecordAttempt, attempt.quiz_id, payload).await
    }
}