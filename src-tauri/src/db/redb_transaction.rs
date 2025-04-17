use redb::{Database, ReadTransaction, WriteTransaction, TableDefinition};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::timeout;
use tracing::{info, error, warn, debug, instrument};
use async_trait::async_trait;

use crate::db::redb_error::{RedbError, Result};

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
enum BatchOperation {
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

/// Manager for Redb database transactions
pub struct RedbTransactionManager {
    /// The Redb database instance
    db: Arc<Database>,
    /// Transaction hooks
    hooks: Mutex<Vec<TransactionHook>>,
    /// Active transaction count for debugging
    active_transactions: Mutex<u64>,
    /// Transaction state map for tracking nested transactions
    transaction_states: Mutex<HashMap<u64, TransactionState>>,
    /// Transaction logger
    logger: Option<Arc<TransactionLogger>>,
    /// Current transaction ID for thread-local storage
    current_transaction: Mutex<HashMap<std::thread::ThreadId, u64>>,
    /// Recovery manager
    recovery_manager: Mutex<HashMap<u64, RecoveryData>>,
    /// Batch queue for transaction batching
    batch_queue: Mutex<Vec<BatchOperation>>,
}

use crate::db::redb_transaction_log::{TransactionLogger, TransactionType as LogTransactionType, TransactionStatus};

impl RedbTransactionManager {
    /// Create a new transaction manager
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            hooks: Mutex::new(Vec::new()),
            active_transactions: Mutex::new(0),
            transaction_states: Mutex::new(HashMap::new()),
            logger: Some(Arc::new(TransactionLogger::new(1000))), // Keep last 1000 transactions
            current_transaction: Mutex::new(HashMap::new()),
            recovery_manager: Mutex::new(HashMap::new()),
            batch_queue: Mutex::new(Vec::new()),
        }
    }

    /// Create a new transaction manager with custom options
    pub fn new_with_options(db: Database, max_log_size: usize, enable_logging: bool) -> Self {
        Self {
            db: Arc::new(db),
            hooks: Mutex::new(Vec::new()),
            active_transactions: Mutex::new(0),
            transaction_states: Mutex::new(HashMap::new()),
            logger: if enable_logging {
                Some(Arc::new(TransactionLogger::new(max_log_size)))
            } else {
                None
            },
            current_transaction: Mutex::new(HashMap::new()),
            recovery_manager: Mutex::new(HashMap::new()),
            batch_queue: Mutex::new(Vec::new()),
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

    /// Execute a write transaction with default options
    #[instrument(skip(self, f))]
    pub async fn execute_write_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut WriteTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.execute_write_transaction_with_options(f, TransactionOptions::default()).await
    }

    /// Execute a write transaction with custom options
    #[instrument(skip(self, f))]
    pub async fn execute_write_transaction_with_options<F, T>(
        &self,
        f: F,
        options: TransactionOptions
    ) -> Result<T>
    where
        F: FnOnce(&mut WriteTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let transaction_id = self.start_transaction(options.clone());
        let mut retries = 0;

        // Execute transaction hooks
        self.execute_before_begin_hooks();

        // Update transaction type in state
        {
            let mut transaction_states = self.transaction_states.lock().unwrap();
            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                state.transaction_type = TransactionType::Write;

                // Log transaction type update
                if options.enable_logging {
                    if let Some(logger) = &self.logger {
                        let log_transaction_type = LogTransactionType::Write;
                        let parent_id = state.parent_id;
                        let nested_depth = state.nested_depth;

                        logger.log_transaction_start(
                            transaction_id,
                            log_transaction_type,
                            parent_id,
                            nested_depth,
                        );
                    }
                }
            }
        }

        loop {
            let start_time = Instant::now();
            info!("Beginning write transaction (id: {}, retry: {})", transaction_id, retries);

            // Start the transaction
            let write_txn_result = self.db.begin_write();
            let mut write_txn = match write_txn_result {
                Ok(txn) => txn,
                Err(e) => {
                    let redb_error = RedbError::Native(e);
                    if redb_error.is_transient() && retries < options.max_retries {
                        retries += 1;
                        warn!("Transient error starting transaction, retrying ({}/{}): {}",
                              retries, options.max_retries, redb_error);
                        continue;
                    }
                    self.end_transaction(transaction_id, false);
                    return Err(redb_error);
                }
            };

            // Apply isolation level if needed
            self.apply_isolation_level(&mut write_txn, options.isolation_level);

            // Execute batch operations if batching is enabled
            if options.enable_batching {
                if let Err(e) = self.execute_batch_operations(transaction_id, &mut write_txn).await {
                    error!("Failed to execute batch operations: {}", e);
                    write_txn.abort();
                    self.end_transaction(transaction_id, false);
                    self.execute_after_rollback_hooks();
                    return Err(e);
                }
            }

            // Execute the transaction function with timeout if specified
            let result = if let Some(timeout_ms) = options.timeout_ms {
                match timeout(Duration::from_millis(timeout_ms), async {
                    f(&mut write_txn)
                }).await {
                    Ok(result) => result,
                    Err(_) => {
                        write_txn.abort();
                        self.end_transaction(transaction_id, false);
                        self.execute_after_rollback_hooks();

                        // Log timeout
                        if options.enable_logging {
                            if let Some(logger) = &self.logger {
                                let mut transaction_states = self.transaction_states.lock().unwrap();
                                if let Some(state) = transaction_states.get(&transaction_id) {
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

                        return Err(RedbError::Timeout(timeout_ms));
                    }
                }
            } else {
                f(&mut write_txn)
            };

            // Handle the result
            match result {
                Ok(result) => {
                    // Commit the transaction
                    match write_txn.commit() {
                        Ok(_) => {
                            let duration = start_time.elapsed().as_millis();
                            info!("Write transaction committed successfully (id: {}, duration: {}ms)",
                                  transaction_id, duration);

                            // Update transaction metrics
                            if options.enable_logging {
                                if let Some(logger) = &self.logger {
                                    let mut transaction_states = self.transaction_states.lock().unwrap();
                                    if let Some(state) = transaction_states.get(&transaction_id) {
                                        logger.log_transaction_end(
                                            transaction_id,
                                            TransactionStatus::Committed,
                                            retries,
                                            None,
                                            state.tables_accessed.clone(),
                                            state.operation_count,
                                        );
                                    }
                                }
                            }

                            self.end_transaction(transaction_id, true);
                            self.execute_after_commit_hooks();
                            return Ok(result);
                        }
                        Err(e) => {
                            let redb_error = RedbError::Native(e);
                            if redb_error.is_transient() && retries < options.max_retries {
                                retries += 1;
                                warn!("Transient error committing transaction, retrying ({}/{}): {}",
                                      retries, options.max_retries, redb_error);
                                continue;
                            }
                            error!("Failed to commit transaction (id: {}): {}", transaction_id, redb_error);

                            // Log failure
                            if options.enable_logging {
                                if let Some(logger) = &self.logger {
                                    let mut transaction_states = self.transaction_states.lock().unwrap();
                                    if let Some(state) = transaction_states.get(&transaction_id) {
                                        logger.log_transaction_end(
                                            transaction_id,
                                            TransactionStatus::Failed,
                                            retries,
                                            Some(format!("Commit failed: {}", redb_error)),
                                            state.tables_accessed.clone(),
                                            state.operation_count,
                                        );
                                    }
                                }
                            }

                            self.end_transaction(transaction_id, false);
                            self.execute_after_rollback_hooks();

                            // Try to recover if recovery is enabled
                            if options.enable_recovery {
                                if let Err(recovery_err) = self.recover_transaction(transaction_id).await {
                                    warn!("Failed to recover transaction: {}", recovery_err);
                                }
                            }

                            return Err(redb_error);
                        }
                    }
                }
                Err(e) => {
                    error!("Write transaction failed (id: {}): {}", transaction_id, e);
                    write_txn.abort();

                    // Log failure
                    if options.enable_logging {
                        if let Some(logger) = &self.logger {
                            let mut transaction_states = self.transaction_states.lock().unwrap();
                            if let Some(state) = transaction_states.get(&transaction_id) {
                                logger.log_transaction_end(
                                    transaction_id,
                                    TransactionStatus::Failed,
                                    retries,
                                    Some(format!("Transaction function failed: {}", e)),
                                    state.tables_accessed.clone(),
                                    state.operation_count,
                                );
                            }
                        }
                    }

                    self.end_transaction(transaction_id, false);
                    self.execute_after_rollback_hooks();

                    // Try to recover if recovery is enabled
                    if options.enable_recovery {
                        if let Err(recovery_err) = self.recover_transaction(transaction_id).await {
                            warn!("Failed to recover transaction: {}", recovery_err);
                        }
                    }

                    // Retry if it's a transient error
                    if e.is_transient() && retries < options.max_retries {
                        retries += 1;
                        warn!("Transient error in transaction, retrying ({}/{}): {}",
                              retries, options.max_retries, e);
                        continue;
                    }

                    return Err(e);
                }
            }
        }
    }

    /// Execute a read transaction with default options
    #[instrument(skip(self, f))]
    pub async fn execute_read_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&ReadTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.execute_read_transaction_with_options(f, TransactionOptions::default()).await
    }

    /// Execute a read transaction with custom options
    #[instrument(skip(self, f))]
    pub async fn execute_read_transaction_with_options<F, T>(
        &self,
        f: F,
        options: TransactionOptions
    ) -> Result<T>
    where
        F: FnOnce(&ReadTransaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let transaction_id = self.start_transaction(options.clone());
        let mut retries = 0;

        // Execute transaction hooks
        self.execute_before_begin_hooks();

        // Update transaction type in state
        {
            let mut transaction_states = self.transaction_states.lock().unwrap();
            if let Some(state) = transaction_states.get_mut(&transaction_id) {
                state.transaction_type = TransactionType::Read;

                // Log transaction type update
                if options.enable_logging {
                    if let Some(logger) = &self.logger {
                        let log_transaction_type = LogTransactionType::Read;
                        let parent_id = state.parent_id;
                        let nested_depth = state.nested_depth;

                        logger.log_transaction_start(
                            transaction_id,
                            log_transaction_type,
                            parent_id,
                            nested_depth,
                        );
                    }
                }
            }
        }

        loop {
            let start_time = Instant::now();
            info!("Beginning read transaction (id: {}, retry: {})", transaction_id, retries);

            // Start the transaction
            let read_txn_result = self.db.begin_read();
            let read_txn = match read_txn_result {
                Ok(txn) => txn,
                Err(e) => {
                    let redb_error = RedbError::Native(e);
                    if redb_error.is_transient() && retries < options.max_retries {
                        retries += 1;
                        warn!("Transient error starting transaction, retrying ({}/{}): {}",
                              retries, options.max_retries, redb_error);
                        continue;
                    }
                    self.end_transaction(transaction_id, false);
                    return Err(redb_error);
                }
            };

            // Execute the transaction function with timeout if specified
            let result = if let Some(timeout_ms) = options.timeout_ms {
                match timeout(Duration::from_millis(timeout_ms), async {
                    f(&read_txn)
                }).await {
                    Ok(result) => result,
                    Err(_) => {
                        self.end_transaction(transaction_id, false);

                        // Log timeout
                        if options.enable_logging {
                            if let Some(logger) = &self.logger {
                                let mut transaction_states = self.transaction_states.lock().unwrap();
                                if let Some(state) = transaction_states.get(&transaction_id) {
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

                        return Err(RedbError::Timeout(timeout_ms));
                    }
                }
            } else {
                f(&read_txn)
            };

            // Handle the result
            match result {
                Ok(result) => {
                    let duration = start_time.elapsed().as_millis();
                    info!("Read transaction completed successfully (id: {}, duration: {}ms)",
                          transaction_id, duration);

                    // Update transaction metrics
                    if options.enable_logging {
                        if let Some(logger) = &self.logger {
                            let mut transaction_states = self.transaction_states.lock().unwrap();
                            if let Some(state) = transaction_states.get(&transaction_id) {
                                logger.log_transaction_end(
                                    transaction_id,
                                    TransactionStatus::Committed,
                                    retries,
                                    None,
                                    state.tables_accessed.clone(),
                                    state.operation_count,
                                );
                            }
                        }
                    }

                    self.end_transaction(transaction_id, true);
                    return Ok(result);
                }
                Err(e) => {
                    error!("Read transaction failed (id: {}): {}", transaction_id, e);

                    // Log failure
                    if options.enable_logging {
                        if let Some(logger) = &self.logger {
                            let mut transaction_states = self.transaction_states.lock().unwrap();
                            if let Some(state) = transaction_states.get(&transaction_id) {
                                logger.log_transaction_end(
                                    transaction_id,
                                    TransactionStatus::Failed,
                                    retries,
                                    Some(format!("Transaction function failed: {}", e)),
                                    state.tables_accessed.clone(),
                                    state.operation_count,
                                );
                            }
                        }
                    }

                    self.end_transaction(transaction_id, false);

                    // Retry if it's a transient error
                    if e.is_transient() && retries < options.max_retries {
                        retries += 1;
                        warn!("Transient error in transaction, retrying ({}/{}): {}",
                              retries, options.max_retries, e);
                        continue;
                    }

                    return Err(e);
                }
            }
        }
    }

    /// Start a transaction and track its state
    fn start_transaction(&self, options: TransactionOptions) -> u64 {
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
    fn end_transaction(&self, transaction_id: u64, success: bool) {
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
                state.batch_operations.push(operation);
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
    async fn execute_batch_operations(&self, transaction_id: u64, txn: &mut WriteTransaction) -> Result<()> {
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
    pub fn get_transaction_metrics(&self) -> Option<crate::db::redb_transaction_log::TransactionMetrics> {
        if let Some(logger) = &self.logger {
            Some(logger.get_metrics())
        } else {
            None
        }
    }

    /// Get transaction log entries
    pub fn get_transaction_log(&self) -> Option<Vec<crate::db::redb_transaction_log::TransactionLogEntry>> {
        if let Some(logger) = &self.logger {
            Some(logger.get_log_entries())
        } else {
            None
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

    /// Execute before-begin hooks
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
}
}