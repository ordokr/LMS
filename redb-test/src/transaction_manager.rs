use crate::error::{RedbError, Result};
use crate::transaction_log::{TransactionLogger, TransactionType as LogTransactionType, TransactionStatus};
use redb::{Database, ReadTransaction, WriteTransaction, TableDefinition};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::timeout;
use tracing::{info, error, warn, debug, instrument};
use async_trait::async_trait;
use chrono::Utc;

/// Transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransactionType {
    /// Read transaction
    Read,
    /// Write transaction
    Write,
}

/// Batch operation
#[derive(Debug, Clone)]
pub enum BatchOperation {
    /// Insert operation
    Insert {
        /// Table name
        table: String,
        /// Key
        key: Vec<u8>,
        /// Value
        value: Vec<u8>,
    },
    /// Update operation
    Update {
        /// Table name
        table: String,
        /// Key
        key: Vec<u8>,
        /// Value
        value: Vec<u8>,
        /// Previous value
        previous_value: Option<Vec<u8>>,
    },
    /// Delete operation
    Delete {
        /// Table name
        table: String,
        /// Key
        key: Vec<u8>,
        /// Previous value
        previous_value: Option<Vec<u8>>,
    },
}

/// Recovery data
#[derive(Debug, Clone)]
struct RecoveryData {
    /// Transaction ID
    transaction_id: u64,
    /// Savepoint name
    savepoint: String,
    /// Batch operations before the savepoint
    operations: Vec<BatchOperation>,
    /// Recovery timestamp
    timestamp: Instant,
}

/// Transaction state for tracking nested transactions
#[derive(Debug)]
struct TransactionState {
    /// Stack of savepoints for nested transactions
    savepoints: Vec<String>,
    /// Transaction start time
    start_time: Instant,
    /// Transaction options
    options: TransactionOptions,
    /// Parent transaction ID (if nested)
    parent_id: Option<u64>,
    /// Nested transaction depth
    nested_depth: usize,
    /// Transaction type
    transaction_type: TransactionType,
    /// Tables accessed during the transaction
    tables_accessed: Vec<String>,
    /// Operation count
    operation_count: u64,
    /// Batch operations for transaction batching
    batch_operations: Vec<BatchOperation>,
    /// Recovery data
    recovery_data: Option<RecoveryData>,
    /// Is transaction active
    is_active: bool,
    /// Is transaction committed
    is_committed: bool,
    /// Is transaction rolled back
    is_rolled_back: bool,
}

/// Transaction isolation level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    /// Default isolation level provided by Redb
    Default,
    /// Read committed isolation level
    ReadCommitted,
    /// Serializable isolation level
    Serializable,
}

/// Transaction hook type
pub enum TransactionHook {
    /// Called before transaction begins
    BeforeBegin(Box<dyn Fn() + Send + Sync>),
    /// Called after transaction commits successfully
    AfterCommit(Box<dyn Fn() + Send + Sync>),
    /// Called after transaction is rolled back
    AfterRollback(Box<dyn Fn() + Send + Sync>),
}

/// Transaction options
#[derive(Debug, Clone)]
pub struct TransactionOptions {
    /// Transaction timeout in milliseconds
    pub timeout_ms: Option<u64>,
    /// Maximum number of retries for transient errors
    pub max_retries: u32,
    /// Isolation level
    pub isolation_level: IsolationLevel,
    /// Enable nested transactions
    pub enable_nested: bool,
    /// Enable transaction recovery
    pub enable_recovery: bool,
    /// Enable transaction logging
    pub enable_logging: bool,
    /// Enable transaction metrics
    pub enable_metrics: bool,
    /// Enable transaction batching
    pub enable_batching: bool,
    /// Maximum batch size
    pub max_batch_size: Option<usize>,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            timeout_ms: Some(30000), // 30 seconds default timeout
            max_retries: 3,
            isolation_level: IsolationLevel::Default,
            enable_nested: true,
            enable_recovery: true,
            enable_logging: true,
            enable_metrics: true,
            enable_batching: false,
            max_batch_size: Some(100),
        }
    }
}

/// Manager for Redb database transactions
#[derive(Clone)]
pub struct RedbTransactionManager {
    /// The Redb database instance
    db: Arc<Database>,
    /// Transaction hooks
    hooks: Arc<Mutex<Vec<TransactionHook>>>,
    /// Active transaction count for debugging
    active_transactions: Arc<Mutex<u64>>,
    /// Transaction state map for tracking nested transactions
    transaction_states: Arc<Mutex<HashMap<u64, TransactionState>>>,
    /// Transaction logger
    logger: Option<Arc<TransactionLogger>>,
    /// Current transaction ID for thread-local storage
    current_transaction: Arc<Mutex<HashMap<std::thread::ThreadId, u64>>>,
    /// Recovery manager
    recovery_manager: Arc<Mutex<HashMap<u64, RecoveryData>>>,
    /// Batch queue for transaction batching
    batch_queue: Arc<Mutex<Vec<BatchOperation>>>,
}

impl RedbTransactionManager {
    /// Create a new transaction manager
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            hooks: Arc::new(Mutex::new(Vec::new())),
            active_transactions: Arc::new(Mutex::new(0)),
            transaction_states: Arc::new(Mutex::new(HashMap::new())),
            logger: Some(Arc::new(TransactionLogger::new(1000))), // Keep last 1000 transactions
            current_transaction: Arc::new(Mutex::new(HashMap::new())),
            recovery_manager: Arc::new(Mutex::new(HashMap::new())),
            batch_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create a new transaction manager with custom options
    pub fn new_with_options(db: Database, max_log_size: usize, enable_logging: bool) -> Self {
        Self {
            db: Arc::new(db),
            hooks: Arc::new(Mutex::new(Vec::new())),
            active_transactions: Arc::new(Mutex::new(0)),
            transaction_states: Arc::new(Mutex::new(HashMap::new())),
            logger: if enable_logging {
                Some(Arc::new(TransactionLogger::new(max_log_size)))
            } else {
                None
            },
            current_transaction: Arc::new(Mutex::new(HashMap::new())),
            recovery_manager: Arc::new(Mutex::new(HashMap::new())),
            batch_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a transaction hook
    pub fn add_hook(&self, hook: TransactionHook) {
        let mut hooks = self.hooks.lock().unwrap();
        hooks.push(hook);
    }

    /// Get the database instance
    pub fn database(&self) -> Arc<Database> {
        self.db.clone()
    }

    /// Get the number of active transactions
    pub fn active_transaction_count(&self) -> u64 {
        *self.active_transactions.lock().unwrap()
    }

    /// Execute transaction hooks
    fn execute_before_begin_hooks(&self) {
        let hooks = self.hooks.lock().unwrap();
        for hook in hooks.iter() {
            if let TransactionHook::BeforeBegin(callback) = hook {
                callback();
            }
        }
    }

    /// Execute after-commit hooks
    fn execute_after_commit_hooks(&self) {
        let hooks = self.hooks.lock().unwrap();
        for hook in hooks.iter() {
            if let TransactionHook::AfterCommit(callback) = hook {
                callback();
            }
        }
    }

    /// Execute after-rollback hooks
    fn execute_after_rollback_hooks(&self) {
        let hooks = self.hooks.lock().unwrap();
        for hook in hooks.iter() {
            if let TransactionHook::AfterRollback(callback) = hook {
                callback();
            }
        }
    }

    /// Apply isolation level to a transaction
    fn apply_isolation_level(&self, _txn: &mut WriteTransaction, level: IsolationLevel) {
        // Redb doesn't directly support isolation levels yet, but this is where
        // we would implement it when it becomes available
        match level {
            IsolationLevel::Default => {}
            IsolationLevel::ReadCommitted => {
                // Would set read committed isolation level
            }
            IsolationLevel::Serializable => {
                // Would set serializable isolation level
            }
        }
    }

    /// Start a transaction and track its state
    pub fn start_transaction(&self, options: TransactionOptions) -> u64 {
        let mut active_transactions = self.active_transactions.lock().unwrap();
        *active_transactions += 1;
        let transaction_id = *active_transactions;

        // Check if this is a nested transaction
        let parent_id = if options.enable_nested {
            let current = self.current_transaction.lock().unwrap();
            current.get(&std::thread::current().id()).cloned()
        } else {
            None
        };

        let nested_depth = if let Some(parent_id) = parent_id {
            let transaction_states = self.transaction_states.lock().unwrap();
            if let Some(parent_state) = transaction_states.get(&parent_id) {
                parent_state.nested_depth + 1
            } else {
                0
            }
        } else {
            0
        };

        // Create transaction state
        let transaction_type = TransactionType::Write; // Will be updated later
        let mut transaction_states = self.transaction_states.lock().unwrap();
        transaction_states.insert(transaction_id, TransactionState {
            savepoints: Vec::new(),
            start_time: Instant::now(),
            options: options.clone(),
            parent_id,
            nested_depth,
            transaction_type,
            tables_accessed: Vec::new(),
            operation_count: 0,
            batch_operations: Vec::new(),
            recovery_data: None,
            is_active: true,
            is_committed: false,
            is_rolled_back: false,
        });

        // Set current transaction for this thread
        if options.enable_nested {
            let mut current = self.current_transaction.lock().unwrap();
            current.insert(std::thread::current().id(), transaction_id);
        }

        // Log transaction start
        if options.enable_logging {
            if let Some(logger) = &self.logger {
                let log_transaction_type = match transaction_type {
                    TransactionType::Read => LogTransactionType::Read,
                    TransactionType::Write => LogTransactionType::Write,
                };

                logger.log_transaction_start(
                    transaction_id,
                    log_transaction_type,
                    parent_id,
                    nested_depth,
                );
            }
        }

        transaction_id
    }

    /// End a transaction and clean up its state
    pub fn end_transaction(&self, transaction_id: u64, success: bool) {
        let mut transaction_states = self.transaction_states.lock().unwrap();

        if let Some(state) = transaction_states.get(&transaction_id) {
            let options = state.options.clone();
            let parent_id = state.parent_id;
            let tables_accessed = state.tables_accessed.clone();
            let operation_count = state.operation_count;
            let transaction_type = state.transaction_type;
            let nested_depth = state.nested_depth;

            // Log transaction end
            if options.enable_logging {
                if let Some(logger) = &self.logger {
                    let status = if success {
                        TransactionStatus::Committed
                    } else {
                        TransactionStatus::RolledBack
                    };

                    logger.log_transaction_end(
                        transaction_id,
                        status,
                        0, // retries
                        None, // error
                        tables_accessed.clone(),
                        operation_count,
                    );
                }
            }

            // If this is a nested transaction, restore parent as current
            if options.enable_nested && nested_depth > 0 {
                if let Some(parent_id) = parent_id {
                    let mut current = self.current_transaction.lock().unwrap();
                    current.insert(std::thread::current().id(), parent_id);
                }
            } else {
                // Remove current transaction for this thread
                let mut current = self.current_transaction.lock().unwrap();
                current.remove(&std::thread::current().id());
            }

            // Clean up recovery data if successful
            if success && options.enable_recovery {
                let mut recovery_manager = self.recovery_manager.lock().unwrap();
                recovery_manager.remove(&transaction_id);
            }
        }

        // Remove transaction state
        if let Some(state) = transaction_states.remove(&transaction_id) {
            let duration = state.start_time.elapsed();
            debug!("Transaction {} {} after {:?}",
                   transaction_id,
                   if success { "succeeded" } else { "failed" },
                   duration);
        }
    }

    /// Begin a nested transaction
    pub async fn begin_nested_transaction(&self) -> Result<u64> {
        let thread_id = std::thread::current().id();
        let current_transaction_id = {
            let current = self.current_transaction.lock().unwrap();
            current.get(&thread_id).cloned()
        };

        if let Some(parent_id) = current_transaction_id {
            // Create a savepoint for the nested transaction
            let savepoint_name = format!("sp_{}", Utc::now().timestamp_nanos());

            // Add savepoint to parent transaction
            let mut transaction_states = self.transaction_states.lock().unwrap();
            if let Some(parent_state) = transaction_states.get_mut(&parent_id) {
                if !parent_state.options.enable_nested {
                    return Err(RedbError::NestedTransaction(
                        "Nested transactions not enabled for parent transaction".to_string(),
                    ));
                }

                // Create recovery data if enabled
                if parent_state.options.enable_recovery {
                    let recovery_data = RecoveryData {
                        transaction_id: parent_id,
                        savepoint: savepoint_name.clone(),
                        operations: parent_state.batch_operations.clone(),
                        timestamp: Instant::now(),
                    };

                    parent_state.recovery_data = Some(recovery_data.clone());

                    // Store in recovery manager
                    let mut recovery_manager = self.recovery_manager.lock().unwrap();
                    recovery_manager.insert(parent_id, recovery_data);
                }

                parent_state.savepoints.push(savepoint_name);

                // Start a new transaction with the same options
                let options = parent_state.options.clone();
                drop(transaction_states); // Release lock before starting new transaction

                let transaction_id = self.start_transaction(options);
                return Ok(transaction_id);
            }
        }

        Err(RedbError::NoActiveTransaction)
    }

    /// Commit a nested transaction
    pub async fn commit_nested_transaction(&self, transaction_id: u64) -> Result<()> {
        let mut transaction_states = self.transaction_states.lock().unwrap();

        if let Some(state) = transaction_states.get_mut(&transaction_id) {
            if !state.is_active {
                return Err(RedbError::AlreadyCommitted);
            }

            if state.nested_depth == 0 {
                return Err(RedbError::NestedTransaction(
                    "Not a nested transaction".to_string(),
                ));
            }

            if let Some(parent_id) = state.parent_id {
                if let Some(parent_state) = transaction_states.get_mut(&parent_id) {
                    // Merge batch operations from child to parent
                    parent_state.batch_operations.extend(state.batch_operations.clone());
                    parent_state.operation_count += state.operation_count;

                    // Merge tables accessed
                    for table in &state.tables_accessed {
                        if !parent_state.tables_accessed.contains(table) {
                            parent_state.tables_accessed.push(table.clone());
                        }
                    }

                    // Remove savepoint from parent
                    if !parent_state.savepoints.is_empty() {
                        parent_state.savepoints.pop();
                    }

                    // Mark transaction as committed
                    state.is_active = false;
                    state.is_committed = true;

                    // Log transaction commit
                    if state.options.enable_logging {
                        if let Some(logger) = &self.logger {
                            logger.log_transaction_end(
                                transaction_id,
                                TransactionStatus::Committed,
                                0, // retries
                                None, // error
                                state.tables_accessed.clone(),
                                state.operation_count,
                            );
                        }
                    }

                    // Set parent as current transaction
                    let mut current = self.current_transaction.lock().unwrap();
                    current.insert(std::thread::current().id(), parent_id);

                    return Ok(());
                }
            }

            return Err(RedbError::NestedTransaction(
                "Parent transaction not found".to_string(),
            ));
        }

        Err(RedbError::NoActiveTransaction)
    }

    /// Rollback a nested transaction
    pub async fn rollback_nested_transaction(&self, transaction_id: u64) -> Result<()> {
        let mut transaction_states = self.transaction_states.lock().unwrap();

        if let Some(state) = transaction_states.get_mut(&transaction_id) {
            if !state.is_active {
                return Err(RedbError::AlreadyRolledBack);
            }

            if state.nested_depth == 0 {
                return Err(RedbError::NestedTransaction(
                    "Not a nested transaction".to_string(),
                ));
            }

            if let Some(parent_id) = state.parent_id {
                if let Some(parent_state) = transaction_states.get_mut(&parent_id) {
                    // Remove savepoint from parent
                    if !parent_state.savepoints.is_empty() {
                        parent_state.savepoints.pop();
                    }

                    // Mark transaction as rolled back
                    state.is_active = false;
                    state.is_rolled_back = true;

                    // Log transaction rollback
                    if state.options.enable_logging {
                        if let Some(logger) = &self.logger {
                            logger.log_transaction_end(
                                transaction_id,
                                TransactionStatus::RolledBack,
                                0, // retries
                                None, // error
                                state.tables_accessed.clone(),
                                state.operation_count,
                            );
                        }
                    }

                    // Set parent as current transaction
                    let mut current = self.current_transaction.lock().unwrap();
                    current.insert(std::thread::current().id(), parent_id);

                    return Ok(());
                }
            }

            return Err(RedbError::NestedTransaction(
                "Parent transaction not found".to_string(),
            ));
        }

        Err(RedbError::NoActiveTransaction)
    }

    /// Add a batch operation to the current transaction
    pub fn add_batch_operation(&self, operation: BatchOperation) -> Result<()> {
        let thread_id = std::thread::current().id();
        let current_transaction_id = {
            let current = self.current_transaction.lock().unwrap();
            current.get(&thread_id).cloned()
        };

        if let Some(transaction_id) = current_transaction_id {
            let mut transaction_states = self.transaction_states.lock().unwrap();
            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                if !state.options.enable_batching {
                    return Err(RedbError::Other(
                        "Transaction batching not enabled".to_string(),
                    ));
                }

                // Check if we've reached the batch size limit
                if let Some(max_batch_size) = state.options.max_batch_size {
                    if state.batch_operations.len() >= max_batch_size {
                        return Err(RedbError::Other(
                            format!("Batch size limit reached ({})", max_batch_size),
                        ));
                    }
                }

                // Add operation to batch
                state.batch_operations.push(operation.clone());
                state.operation_count += 1;

                // Extract table name from operation
                let table_name = match &operation {
                    BatchOperation::Insert { table, .. } => table.clone(),
                    BatchOperation::Update { table, .. } => table.clone(),
                    BatchOperation::Delete { table, .. } => table.clone(),
                };

                // Add to tables accessed if not already there
                if !state.tables_accessed.contains(&table_name) {
                    state.tables_accessed.push(table_name);
                }

                return Ok(());
            }
        }

        Err(RedbError::NoActiveTransaction)
    }

    /// Execute batch operations for a transaction
    pub async fn execute_batch_operations(&self, transaction_id: u64, txn: &mut WriteTransaction) -> Result<()> {
        let mut transaction_states = self.transaction_states.lock().unwrap();

        if let Some(state) = transaction_states.get_mut(&transaction_id) {
            if state.batch_operations.is_empty() {
                return Ok(());
            }

            // Process each operation in the batch
            for operation in &state.batch_operations {
                match operation {
                    BatchOperation::Insert { table, key, value } => {
                        let table_def = TableDefinition::<&[u8], &[u8]>::new(table);
                        let mut table = txn.open_table(table_def)
                            .map_err(|e| RedbError::Native(e))?;

                        table.insert(key.as_slice(), value.as_slice())
                            .map_err(|e| RedbError::Native(e))?;
                    },
                    BatchOperation::Update { table, key, value, .. } => {
                        let table_def = TableDefinition::<&[u8], &[u8]>::new(table);
                        let mut table = txn.open_table(table_def)
                            .map_err(|e| RedbError::Native(e))?;

                        table.insert(key.as_slice(), value.as_slice())
                            .map_err(|e| RedbError::Native(e))?;
                    },
                    BatchOperation::Delete { table, key, .. } => {
                        let table_def = TableDefinition::<&[u8], &[u8]>::new(table);
                        let mut table = txn.open_table(table_def)
                            .map_err(|e| RedbError::Native(e))?;

                        table.remove(key.as_slice())
                            .map_err(|e| RedbError::Native(e))?;
                    },
                }
            }

            // Clear batch operations after execution
            state.batch_operations.clear();

            Ok(())
        } else {
            Err(RedbError::NoActiveTransaction)
        }
    }

    /// Recover a transaction from a savepoint
    pub async fn recover_transaction(&self, transaction_id: u64) -> Result<()> {
        let recovery_data = {
            let recovery_manager = self.recovery_manager.lock().unwrap();
            recovery_manager.get(&transaction_id).cloned()
        };

        if let Some(recovery_data) = recovery_data {
            let mut transaction_states = self.transaction_states.lock().unwrap();

            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                if !state.options.enable_recovery {
                    return Err(RedbError::Recovery(
                        "Transaction recovery not enabled".to_string(),
                    ));
                }

                // Restore batch operations from recovery data
                state.batch_operations = recovery_data.operations.clone();

                // Log recovery
                if state.options.enable_logging {
                    if let Some(logger) = &self.logger {
                        logger.log_transaction_end(
                            transaction_id,
                            TransactionStatus::RolledBack,
                            0, // retries
                            Some("Transaction recovered from savepoint".to_string()),
                            state.tables_accessed.clone(),
                            state.operation_count,
                        );
                    }
                }

                return Ok(());
            }
        }

        Err(RedbError::Recovery(
            "No recovery data found for transaction".to_string(),
        ))
    }

    /// Get transaction metrics
    pub fn get_transaction_metrics(&self) -> Option<crate::transaction_log::TransactionMetrics> {
        if let Some(logger) = &self.logger {
            Some(logger.get_metrics())
        } else {
            None
        }
    }

    /// Get transaction log entries
    pub fn get_transaction_log(&self) -> Option<Vec<crate::transaction_log::TransactionLogEntry>> {
        if let Some(logger) = &self.logger {
            Some(logger.get_log_entries())
        } else {
            None
        }
    }

    /// Execute a read transaction
    pub async fn execute_read_transaction<F, T>(&self, callback: F) -> Result<T>
    where
        F: FnOnce(&ReadTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.execute_read_transaction_with_options(callback, TransactionOptions::default()).await
    }

    /// Execute a read transaction with options
    pub async fn execute_read_transaction_with_options<F, T>(
        &self,
        callback: F,
        options: TransactionOptions,
    ) -> Result<T>
    where
        F: FnOnce(&ReadTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        // Execute hooks
        self.execute_before_begin_hooks();

        // Start transaction
        let transaction_id = self.start_transaction(options.clone());

        // Update transaction type
        {
            let mut transaction_states = self.transaction_states.lock().unwrap();
            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                state.transaction_type = TransactionType::Read;
            }
        }

        // Begin read transaction
        let txn = self.db.begin_read()
            .map_err(|e| RedbError::Native(e))?;

        // Execute callback with timeout if specified
        let result = if let Some(timeout_ms) = options.timeout_ms {
            match timeout(Duration::from_millis(timeout_ms), async {
                callback(&txn)
            }).await {
                Ok(result) => result,
                Err(_) => {
                    // Transaction timed out
                    let mut transaction_states = self.transaction_states.lock().unwrap();
                    if let Some(state) = transaction_states.get_mut(&transaction_id) {
                        if state.options.enable_logging {
                            if let Some(logger) = &self.logger {
                                logger.log_transaction_end(
                                    transaction_id,
                                    TransactionStatus::TimedOut,
                                    0, // retries
                                    Some(format!("Transaction timed out after {} ms", timeout_ms)),
                                    state.tables_accessed.clone(),
                                    state.operation_count,
                                );
                            }
                        }
                    }

                    Err(RedbError::Timeout(timeout_ms))
                }
            }
        } else {
            callback(&txn)
        };

        // End transaction
        self.end_transaction(transaction_id, result.is_ok());

        result
    }

    /// Execute a write transaction
    pub async fn execute_write_transaction<F, T>(&self, callback: F) -> Result<T>
    where
        F: FnOnce(&mut WriteTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.execute_write_transaction_with_options(callback, TransactionOptions::default()).await
    }

    /// Execute a write transaction with options
    pub async fn execute_write_transaction_with_options<F, T>(
        &self,
        callback: F,
        options: TransactionOptions,
    ) -> Result<T>
    where
        F: FnOnce(&mut WriteTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        // Execute hooks
        self.execute_before_begin_hooks();

        // Start transaction
        let transaction_id = self.start_transaction(options.clone());

        // Update transaction type
        {
            let mut transaction_states = self.transaction_states.lock().unwrap();
            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                state.transaction_type = TransactionType::Write;
            }
        }

        let mut retries = 0;
        let max_retries = options.max_retries;

        // Retry loop for transient errors
        loop {
            // Begin write transaction
            let mut txn = match self.db.begin_write() {
                Ok(txn) => txn,
                Err(e) => {
                    let error = RedbError::Native(e);

                    // Check if error is transient and we should retry
                    if error.is_transient() && retries < max_retries {
                        retries += 1;
                        continue;
                    }

                    // Log transaction failure
                    let mut transaction_states = self.transaction_states.lock().unwrap();
                    if let Some(state) = transaction_states.get_mut(&transaction_id) {
                        if state.options.enable_logging {
                            if let Some(logger) = &self.logger {
                                logger.log_transaction_end(
                                    transaction_id,
                                    TransactionStatus::Failed,
                                    retries,
                                    Some(format!("Failed to begin transaction: {}", error)),
                                    state.tables_accessed.clone(),
                                    state.operation_count,
                                );
                            }
                        }
                    }

                    self.end_transaction(transaction_id, false);
                    return Err(error);
                }
            };

            // Apply isolation level
            self.apply_isolation_level(&mut txn, options.isolation_level);

            // Execute batch operations if batching is enabled
            if options.enable_batching {
                if let Err(e) = self.execute_batch_operations(transaction_id, &mut txn).await {
                    self.end_transaction(transaction_id, false);
                    return Err(e);
                }
            }

            // Execute callback with timeout if specified
            let result = if let Some(timeout_ms) = options.timeout_ms {
                match timeout(Duration::from_millis(timeout_ms), async {
                    callback(&mut txn)
                }).await {
                    Ok(result) => result,
                    Err(_) => {
                        // Transaction timed out
                        let mut transaction_states = self.transaction_states.lock().unwrap();
                        if let Some(state) = transaction_states.get_mut(&transaction_id) {
                            if state.options.enable_logging {
                                if let Some(logger) = &self.logger {
                                    logger.log_transaction_end(
                                        transaction_id,
                                        TransactionStatus::TimedOut,
                                        retries,
                                        Some(format!("Transaction timed out after {} ms", timeout_ms)),
                                        state.tables_accessed.clone(),
                                        state.operation_count,
                                    );
                                }
                            }
                        }

                        self.end_transaction(transaction_id, false);
                        return Err(RedbError::Timeout(timeout_ms));
                    }
                }
            } else {
                callback(&mut txn)
            };

            // Handle result
            match result {
                Ok(value) => {
                    // Commit transaction
                    match txn.commit() {
                        Ok(_) => {
                            // Execute after-commit hooks
                            self.execute_after_commit_hooks();

                            // End transaction
                            self.end_transaction(transaction_id, true);

                            return Ok(value);
                        }
                        Err(e) => {
                            let error = RedbError::Native(e);

                            // Check if error is transient and we should retry
                            if error.is_transient() && retries < max_retries {
                                retries += 1;
                                continue;
                            }

                            // Log transaction failure
                            let mut transaction_states = self.transaction_states.lock().unwrap();
                            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                                if state.options.enable_logging {
                                    if let Some(logger) = &self.logger {
                                        logger.log_transaction_end(
                                            transaction_id,
                                            TransactionStatus::Failed,
                                            retries,
                                            Some(format!("Failed to commit transaction: {}", error)),
                                            state.tables_accessed.clone(),
                                            state.operation_count,
                                        );
                                    }
                                }
                            }

                            // Execute after-rollback hooks
                            self.execute_after_rollback_hooks();

                            // End transaction
                            self.end_transaction(transaction_id, false);

                            return Err(error);
                        }
                    }
                }
                Err(e) => {
                    // Check if error is transient and we should retry
                    if e.is_transient() && retries < max_retries {
                        retries += 1;
                        continue;
                    }

                    // Log transaction failure
                    let mut transaction_states = self.transaction_states.lock().unwrap();
                    if let Some(state) = transaction_states.get_mut(&transaction_id) {
                        if state.options.enable_logging {
                            if let Some(logger) = &self.logger {
                                logger.log_transaction_end(
                                    transaction_id,
                                    TransactionStatus::Failed,
                                    retries,
                                    Some(format!("Transaction callback failed: {}", e)),
                                    state.tables_accessed.clone(),
                                    state.operation_count,
                                );
                            }
                        }
                    }

                    // Execute after-rollback hooks
                    self.execute_after_rollback_hooks();

                    // End transaction
                    self.end_transaction(transaction_id, false);

                    return Err(e);
                }
            }
        }
    }
}
