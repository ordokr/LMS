use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use log::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    IndexContent { id: i64, content: String },
    SendNotification { user_id: i64, message: String },
    ProcessAttachment { id: i64, path: String },
    ExportData { query: String, format: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: uuid::Uuid,
    pub task_type: TaskType,
    pub priority: u8,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub struct TaskQueue {
    queue: Arc<Mutex<VecDeque<Task>>>,
    sender: mpsc::Sender<Task>,
    receiver: Arc<Mutex<mpsc::Receiver<Task>>>,
    workers: usize,
}

impl TaskQueue {
    pub fn new(workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            workers,
        }
    }
    
    pub async fn enqueue(&self, task_type: TaskType, priority: u8) -> uuid::Uuid {
        let task = Task {
            id: uuid::Uuid::new_v4(),
            task_type,
            priority,
            created_at: chrono::Utc::now(),
            status: TaskStatus::Pending,
        };
        
        let task_id = task.id;
        
        // Add to internal queue
        {
            let mut queue = self.queue.lock().await;
            
            // Find position based on priority (higher number = higher priority)
            let pos = queue.iter().position(|t| t.priority < priority)
                .unwrap_or(queue.len());
                
            queue.insert(pos, task.clone());
        }
        
        // Send to channel for processing
        if let Err(e) = self.sender.send(task).await {
            error!("Failed to enqueue task: {}", e);
        }
        
        task_id
    }
    
    pub async fn start_workers<F>(&self, handler: F)
    where
        F: Fn(Task) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>> + Send + Sync + Clone + 'static,
    {
        let handler = Arc::new(handler);
        
        for worker_id in 0..self.workers {
            let queue = self.queue.clone();
            let receiver = self.receiver.clone();
            let handler = handler.clone();
            
            tokio::spawn(async move {
                info!("Starting worker {}", worker_id);
                
                let mut receiver = receiver.lock().await;
                
                while let Some(task) = receiver.recv().await {
                    info!("Worker {} processing task {:?}", worker_id, task.id);
                    
                    // Update status
                    {
                        let mut queue = queue.lock().await;
                        if let Some(task_entry) = queue.iter_mut().find(|t| t.id == task.id) {
                            task_entry.status = TaskStatus::Processing;
                        }
                    }
                    
                    // Process task
                    let result = handler(task.clone()).await;
                    
                    // Update status based on result
                    {
                        let mut queue = queue.lock().await;
                        if let Some(task_entry) = queue.iter_mut().find(|t| t.id == task.id) {
                            task_entry.status = if result.is_ok() {
                                TaskStatus::Completed
                            } else {
                                error!("Task {:?} failed: {:?}", task.id, result.err());
                                TaskStatus::Failed
                            };
                        }
                    }
                }
            });
        }
    }
}