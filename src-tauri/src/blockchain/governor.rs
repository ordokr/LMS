use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("memory budget exceeded")]
    MemoryBudgetExceeded,
    
    #[error("transaction rate limit exceeded")]
    TxRateLimitExceeded,
    
    #[error("CPU budget exceeded")]
    CpuBudgetExceeded,
}

pub struct ResourceGovernor {
    // Memory tracking
    mem_budget: Arc<AtomicUsize>,
    mem_current: Arc<AtomicUsize>,
    
    // Transaction rate limiting
    tx_counter: Arc<AtomicUsize>,
    tx_limit: usize,
    tx_reset_interval: tokio::time::Duration,
    
    // CPU budget tracking
    cpu_budget_ns: Arc<AtomicUsize>,
    cpu_current_ns: Arc<AtomicUsize>,
}

impl ResourceGovernor {
    pub fn new(
        memory_limit_bytes: usize,
        tx_limit_per_interval: usize,
        tx_interval_seconds: u64,
        cpu_budget_ns: usize,
    ) -> Self {
        let mem_budget = Arc::new(AtomicUsize::new(memory_limit_bytes));
        let mem_current = Arc::new(AtomicUsize::new(0));
        
        let tx_counter = Arc::new(AtomicUsize::new(0));
        let tx_reset_interval = tokio::time::Duration::from_secs(tx_interval_seconds);
        
        let cpu_budget = Arc::new(AtomicUsize::new(cpu_budget_ns));
        let cpu_current = Arc::new(AtomicUsize::new(0));
        
        // Start reset task for transaction counter
        let tx_counter_clone = Arc::clone(&tx_counter);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tx_reset_interval);
            loop {
                interval.tick().await;
                tx_counter_clone.store(0, Ordering::SeqCst);
            }
        });
        
        Self {
            mem_budget,
            mem_current,
            tx_counter,
            tx_limit: tx_limit_per_interval,
            tx_reset_interval,
            cpu_budget_ns: cpu_budget,
            cpu_current_ns: cpu_current,
        }
    }
    
    pub fn check_tx(&self) -> Result<TransactionGuard, ResourceError> {
        // Check memory budget
        let current_mem = self.mem_current.load(Ordering::SeqCst);
        let budget = self.mem_budget.load(Ordering::SeqCst);
        
        if current_mem >= budget {
            return Err(ResourceError::MemoryBudgetExceeded);
        }
        
        // Check transaction rate
        let tx_count = self.tx_counter.fetch_add(1, Ordering::SeqCst);
        if tx_count >= self.tx_limit {
            // Rollback counter increment
            self.tx_counter.fetch_sub(1, Ordering::SeqCst);
            return Err(ResourceError::TxRateLimitExceeded);
        }
        
        // Return guard to track resource usage
        Ok(TransactionGuard {
            governor: self,
            start_time: std::time::Instant::now(),
            memory_allocated: 0,
        })
    }
    
    pub fn allocate_memory(&self, bytes: usize) -> Result<(), ResourceError> {
        let current = self.mem_current.fetch_add(bytes, Ordering::SeqCst);
        let budget = self.mem_budget.load(Ordering::SeqCst);
        
        if current + bytes > budget {
            // Rollback allocation
            self.mem_current.fetch_sub(bytes, Ordering::SeqCst);
            return Err(ResourceError::MemoryBudgetExceeded);
        }
        
        Ok(())
    }
    
    pub fn release_memory(&self, bytes: usize) {
        let current = self.mem_current.load(Ordering::SeqCst);
        let to_release = std::cmp::min(current, bytes);
        self.mem_current.fetch_sub(to_release, Ordering::SeqCst);
    }
    
    pub fn record_cpu_usage(&self, duration_ns: usize) -> Result<(), ResourceError> {
        let current = self.cpu_current_ns.fetch_add(duration_ns, Ordering::SeqCst);
        let budget = self.cpu_budget_ns.load(Ordering::SeqCst);
        
        if current + duration_ns > budget {
            return Err(ResourceError::CpuBudgetExceeded);
        }
        
        Ok(())
    }
    
    // Reset CPU usage counters (called periodically)
    pub fn reset_cpu_usage(&self) {
        self.cpu_current_ns.store(0, Ordering::SeqCst);
    }
    
    // Get current memory usage as a percentage of budget
    pub fn memory_usage_percent(&self) -> f64 {
        let current = self.mem_current.load(Ordering::SeqCst) as f64;
        let budget = self.mem_budget.load(Ordering::SeqCst) as f64;
        
        if budget == 0.0 {
            return 0.0;
        }
        
        (current / budget) * 100.0
    }
    
    // Get current transaction usage as a percentage of limit
    pub fn tx_usage_percent(&self) -> f64 {
        let current = self.tx_counter.load(Ordering::SeqCst) as f64;
        let limit = self.tx_limit as f64;
        
        if limit == 0.0 {
            return 0.0;
        }
        
        (current / limit) * 100.0
    }
}

pub struct TransactionGuard<'a> {
    governor: &'a ResourceGovernor,
    start_time: std::time::Instant,
    memory_allocated: usize,
}

impl<'a> TransactionGuard<'a> {
    pub fn allocate_memory(&mut self, bytes: usize) -> Result<(), ResourceError> {
        self.governor.allocate_memory(bytes)?;
        self.memory_allocated += bytes;
        Ok(())
    }
}

impl<'a> Drop for TransactionGuard<'a> {
    fn drop(&mut self) {
        // Release allocated memory
        self.governor.release_memory(self.memory_allocated);
        
        // Record CPU usage
        let elapsed = self.start_time.elapsed();
        let ns = elapsed.as_nanos() as usize;
        let _ = self.governor.record_cpu_usage(ns);
    }
}