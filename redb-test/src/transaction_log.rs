use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error, debug};

/// Transaction log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    /// Transaction ID
    pub transaction_id: u64,
    /// Transaction type (read or write)
    pub transaction_type: TransactionType,
    /// Transaction status
    pub status: TransactionStatus,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time (if completed)
    pub end_time: Option<DateTime<Utc>>,
    /// Duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Number of retries
    pub retries: u32,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Nested transaction depth
    pub nested_depth: usize,
    /// Parent transaction ID (if nested)
    pub parent_id: Option<u64>,
    /// Tables accessed
    pub tables_accessed: Vec<String>,
    /// Operation count
    pub operation_count: u64,
}

/// Transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Read transaction
    Read,
    /// Write transaction
    Write,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction is in progress
    InProgress,
    /// Transaction was committed successfully
    Committed,
    /// Transaction was rolled back
    RolledBack,
    /// Transaction failed
    Failed,
    /// Transaction timed out
    TimedOut,
}

/// Transaction metrics
#[derive(Debug, Default)]
pub struct TransactionMetrics {
    /// Total number of transactions
    pub total_transactions: u64,
    /// Number of read transactions
    pub read_transactions: u64,
    /// Number of write transactions
    pub write_transactions: u64,
    /// Number of successful transactions
    pub successful_transactions: u64,
    /// Number of failed transactions
    pub failed_transactions: u64,
    /// Number of timed out transactions
    pub timed_out_transactions: u64,
    /// Number of retried transactions
    pub retried_transactions: u64,
    /// Total number of retries
    pub total_retries: u64,
    /// Average transaction duration in milliseconds
    pub avg_duration_ms: f64,
    /// Maximum transaction duration in milliseconds
    pub max_duration_ms: u64,
    /// Number of nested transactions
    pub nested_transactions: u64,
    /// Maximum nesting depth
    pub max_nesting_depth: usize,
    /// Total operations performed
    pub total_operations: u64,
}

/// Transaction logger
#[derive(Debug, Clone)]
pub struct TransactionLogger {
    /// Transaction log entries
    log_entries: Arc<Mutex<Vec<TransactionLogEntry>>>,
    /// Transaction metrics
    metrics: Arc<Mutex<TransactionMetrics>>,
    /// Maximum log size
    max_log_size: usize,
}

impl TransactionLogger {
    /// Create a new transaction logger
    pub fn new(max_log_size: usize) -> Self {
        Self {
            log_entries: Arc::new(Mutex::new(Vec::new())),
            metrics: Arc::new(Mutex::new(TransactionMetrics::default())),
            max_log_size,
        }
    }

    /// Log a transaction start
    pub fn log_transaction_start(
        &self,
        transaction_id: u64,
        transaction_type: TransactionType,
        parent_id: Option<u64>,
        nested_depth: usize,
    ) {
        let entry = TransactionLogEntry {
            transaction_id,
            transaction_type,
            status: TransactionStatus::InProgress,
            start_time: Utc::now(),
            end_time: None,
            duration_ms: None,
            retries: 0,
            error: None,
            nested_depth,
            parent_id,
            tables_accessed: Vec::new(),
            operation_count: 0,
        };

        let mut log_entries = self.log_entries.lock().unwrap();
        log_entries.push(entry);

        // Trim log if it exceeds max size
        if log_entries.len() > self.max_log_size {
            let excess = log_entries.len() - self.max_log_size;
            log_entries.drain(0..excess);
        }
    }

    /// Log a transaction end
    pub fn log_transaction_end(
        &self,
        transaction_id: u64,
        status: TransactionStatus,
        retries: u32,
        error: Option<String>,
        tables_accessed: Vec<String>,
        operation_count: u64,
    ) {
        let mut log_entries = self.log_entries.lock().unwrap();
        let mut metrics = self.metrics.lock().unwrap();

        // Find the transaction entry
        if let Some(entry) = log_entries.iter_mut().rev().find(|e| e.transaction_id == transaction_id) {
            // Update the entry
            entry.status = status;
            entry.end_time = Some(Utc::now());
            entry.duration_ms = Some(
                (entry.end_time.unwrap() - entry.start_time)
                    .num_milliseconds()
                    .max(0) as u64,
            );
            entry.retries = retries;
            entry.error = error;
            entry.tables_accessed = tables_accessed;
            entry.operation_count = operation_count;

            // Update metrics
            metrics.total_transactions += 1;
            
            match entry.transaction_type {
                TransactionType::Read => metrics.read_transactions += 1,
                TransactionType::Write => metrics.write_transactions += 1,
            }

            match status {
                TransactionStatus::Committed => metrics.successful_transactions += 1,
                TransactionStatus::Failed => metrics.failed_transactions += 1,
                TransactionStatus::TimedOut => metrics.timed_out_transactions += 1,
                TransactionStatus::RolledBack => metrics.failed_transactions += 1,
                _ => {}
            }

            if retries > 0 {
                metrics.retried_transactions += 1;
                metrics.total_retries += retries as u64;
            }

            if let Some(duration) = entry.duration_ms {
                // Update average duration
                let total_duration = metrics.avg_duration_ms * (metrics.total_transactions - 1) as f64 + duration as f64;
                metrics.avg_duration_ms = total_duration / metrics.total_transactions as f64;
                
                // Update max duration
                if duration > metrics.max_duration_ms {
                    metrics.max_duration_ms = duration;
                }
            }

            if entry.nested_depth > 0 {
                metrics.nested_transactions += 1;
                if entry.nested_depth > metrics.max_nesting_depth {
                    metrics.max_nesting_depth = entry.nested_depth;
                }
            }

            metrics.total_operations += operation_count;
        }
    }

    /// Get transaction log entries
    pub fn get_log_entries(&self) -> Vec<TransactionLogEntry> {
        let log_entries = self.log_entries.lock().unwrap();
        log_entries.clone()
    }

    /// Get transaction metrics
    pub fn get_metrics(&self) -> TransactionMetrics {
        let metrics = self.metrics.lock().unwrap();
        metrics.clone()
    }

    /// Clear transaction log
    pub fn clear_log(&self) {
        let mut log_entries = self.log_entries.lock().unwrap();
        log_entries.clear();
    }

    /// Reset transaction metrics
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        *metrics = TransactionMetrics::default();
    }
}

impl Clone for TransactionMetrics {
    fn clone(&self) -> Self {
        Self {
            total_transactions: self.total_transactions,
            read_transactions: self.read_transactions,
            write_transactions: self.write_transactions,
            successful_transactions: self.successful_transactions,
            failed_transactions: self.failed_transactions,
            timed_out_transactions: self.timed_out_transactions,
            retried_transactions: self.retried_transactions,
            total_retries: self.total_retries,
            avg_duration_ms: self.avg_duration_ms,
            max_duration_ms: self.max_duration_ms,
            nested_transactions: self.nested_transactions,
            max_nesting_depth: self.max_nesting_depth,
            total_operations: self.total_operations,
        }
    }
}
