use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    collections::VecDeque,
};
use tokio::sync::oneshot;
use log::{debug, error, info};
use crossbeam_channel::{bounded, Sender, Receiver};

pub type TaskFn = Box<dyn FnOnce() + Send + 'static>;

enum WorkerMessage {
    Task(TaskFn),
    Shutdown,
}

pub struct ThreadPool {
    sender: Sender<WorkerMessage>,
    workers: Vec<thread::JoinHandle<()>>,
    size: usize,
    task_count: Arc<Mutex<usize>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = bounded::<WorkerMessage>(size * 2);
        let receiver = Arc::new(receiver);
        let mut workers = Vec::with_capacity(size);
        let task_count = Arc::new(Mutex::new(0));
        
        for id in 0..size {
            let worker_receiver = receiver.clone();
            let worker_count = task_count.clone();
            
            let handle = thread::Builder::new()
                .name(format!("worker-{}", id))
                .spawn(move || {
                    debug!("Worker {} started", id);
                    Self::worker_loop(id, worker_receiver, worker_count);
                    debug!("Worker {} stopped", id);
                })
                .expect("Failed to spawn worker thread");
                
            workers.push(handle);
        }
        
        Self {
            sender,
            workers,
            size,
            task_count,
        }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut count = self.task_count.lock().unwrap();
        *count += 1;
        
        if let Err(_) = self.sender.send(WorkerMessage::Task(Box::new(f))) {
            error!("Failed to send task to worker pool - channel closed");
        }
    }
    
    pub fn execute_with_callback<F, T>(&self, f: F) -> oneshot::Receiver<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        
        self.execute(move || {
            let result = f();
            if tx.send(result).is_err() {
                debug!("Failed to send result - receiver dropped");
            }
        });
        
        rx
    }
    
    pub fn task_count(&self) -> usize {
        *self.task_count.lock().unwrap()
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
    
    fn worker_loop(id: usize, receiver: Arc<Receiver<WorkerMessage>>, task_count: Arc<Mutex<usize>>) {
        while let Ok(message) = receiver.recv() {
            match message {
                WorkerMessage::Task(task) => {
                    debug!("Worker {} executing task", id);
                    
                    // Execute the task
                    task();
                    
                    // Update task count
                    let mut count = task_count.lock().unwrap();
                    *count -= 1;
                    
                    debug!("Worker {} completed task, {} remaining", id, *count);
                },
                WorkerMessage::Shutdown => {
                    debug!("Worker {} shutting down", id);
                    break;
                }
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("Shutting down thread pool with {} workers", self.workers.len());
        
        // Send shutdown message to all workers
        for _ in 0..self.workers.len() {
            if let Err(_) = self.sender.send(WorkerMessage::Shutdown) {
                // Channel might be closed if workers have already panicked
                break;
            }
        }
        
        // Wait for all workers to finish
        for worker in self.workers.drain(..) {
            if let Err(e) = worker.join() {
                error!("Error joining worker thread: {:?}", e);
            }
        }
        
        debug!("Thread pool shutdown complete");
    }
}

// Function to create optimized thread pool based on CPU cores