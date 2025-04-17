use crate::db::redb_transaction::{RedbTransactionManager, TransactionOptions, IsolationLevel, TransactionHook};
use crate::db::redb_error::{RedbError, Result};
use crate::db::redb::redb;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;
use tracing::{info, error, warn};

/// Example demonstrating the use of enhanced Redb transaction handling
pub async fn run_transaction_example() -> Result<()> {
    // Open the database
    let db_path = "data/redb/example.db";
    let (_, tx_manager) = redb::open_database(db_path)?;

    // Add transaction hooks for monitoring
    let transaction_count = Arc::new(Mutex::new(0));
    let transaction_count_clone = transaction_count.clone();

    tx_manager.add_hook(TransactionHook::BeforeBegin(Box::new(move || {
        info!("Transaction is about to begin");
    })));

    tx_manager.add_hook(TransactionHook::AfterCommit(Box::new(move || {
        let mut count = transaction_count_clone.lock().unwrap();
        *count += 1;
        info!("Transaction committed successfully. Total successful transactions: {}", *count);
    })));

    // Example 1: Basic transaction
    info!("Example 1: Basic transaction");
    let result = redb::save_draft(&tx_manager, "user123", "Draft content").await;
    match result {
        Ok(_) => info!("Draft saved successfully"),
        Err(e) => error!("Failed to save draft: {}", e),
    }

    // Example 2: Transaction with custom options
    info!("Example 2: Transaction with custom options");
    let options = TransactionOptions {
        timeout_ms: Some(5000),
        max_retries: 2,
        isolation_level: IsolationLevel::Serializable,
        enable_logging: true,
        enable_recovery: true,
        enable_nested: true,
        enable_batching: false,
        enable_metrics: true,
        max_batch_size: Some(100),
    };

    let result = redb::save_draft_with_options(
        &tx_manager,
        "user456",
        "Draft with custom transaction options",
        options
    ).await;

    match result {
        Ok(_) => info!("Draft with custom options saved successfully"),
        Err(e) => error!("Failed to save draft with custom options: {}", e),
    }

    // Example 3: Reading data
    info!("Example 3: Reading data");
    let draft = redb::get_draft(&tx_manager, "user123").await?;
    match draft {
        Some(content) => info!("Retrieved draft: {}", content),
        None => info!("No draft found"),
    }

    // Example 4: Nested transactions
    info!("Example 4: Nested transactions");

    // Start a parent transaction
    let parent_result = tx_manager.execute_write_transaction_with_options(|txn| {
        // Create a table for our example
        let table_def = redb::TableDefinition::<&str, &str>::new("nested_example");
        let mut table = txn.open_table(table_def)?;

        // Insert data in parent transaction
        table.insert("parent_key", "parent_value")?;
        info!("Inserted data in parent transaction");

        // Begin a nested transaction
        let nested_id_future = tx_manager.begin_nested_transaction();
        let nested_id = tokio::runtime::Handle::current().block_on(nested_id_future)?;
        info!("Started nested transaction with ID: {}", nested_id);

        // Execute nested transaction logic
        let nested_options = TransactionOptions {
            enable_nested: true,
            enable_logging: true,
            ..TransactionOptions::default()
        };

        let nested_result_future = tx_manager.execute_write_transaction_with_options(|nested_txn| {
            let nested_table = nested_txn.open_table(table_def)?;
            nested_txn.insert("nested_key", "nested_value")?;
            info!("Inserted data in nested transaction");
            Ok(())
        }, nested_options);

        let nested_result = tokio::runtime::Handle::current().block_on(nested_result_future);

        if let Err(e) = nested_result {
            error!("Nested transaction failed: {}", e);
            return Err(e);
        }

        // Commit the nested transaction
        let commit_result = tokio::runtime::Handle::current().block_on(
            tx_manager.commit_nested_transaction(nested_id)
        );

        if let Err(e) = commit_result {
            error!("Failed to commit nested transaction: {}", e);
            return Err(e);
        }

        info!("Nested transaction committed successfully");
        Ok(())
    }, TransactionOptions {
        enable_nested: true,
        enable_logging: true,
        ..TransactionOptions::default()
    }).await;

    match parent_result {
        Ok(_) => info!("Parent transaction with nested transaction completed successfully"),
        Err(e) => error!("Parent transaction failed: {}", e),
    }

    // Example 5: Transaction batching
    info!("Example 5: Transaction batching");

    // Create options with batching enabled
    let batch_options = TransactionOptions {
        enable_batching: true,
        max_batch_size: Some(10),
        enable_logging: true,
        ..TransactionOptions::default()
    };

    // Start a transaction with batching
    let batch_result = tx_manager.execute_write_transaction_with_options(|_| {
        // Add batch operations
        for i in 0..5 {
            let key = format!("batch_key_{}", i);
            let value = format!("batch_value_{}", i);

            let batch_op = crate::db::redb_transaction::BatchOperation::Insert {
                table: "batch_example".to_string(),
                key: key.as_bytes().to_vec(),
                value: value.as_bytes().to_vec(),
            };

            if let Err(e) = tx_manager.add_batch_operation(batch_op) {
                error!("Failed to add batch operation: {}", e);
                return Err(e);
            }

            info!("Added batch operation for key: {}", key);
        }

        info!("All batch operations added successfully");
        Ok(())
    }, batch_options).await;

    match batch_result {
        Ok(_) => info!("Batch transaction completed successfully"),
        Err(e) => error!("Batch transaction failed: {}", e),
    }

    // Example 6: Transaction recovery
    info!("Example 6: Transaction recovery");

    // Create options with recovery enabled
    let recovery_options = TransactionOptions {
        enable_recovery: true,
        enable_nested: true,
        enable_logging: true,
        ..TransactionOptions::default()
    };

    // Start a transaction that will be recovered
    let transaction_id = tx_manager.start_transaction(recovery_options.clone());
    info!("Started transaction for recovery demo with ID: {}", transaction_id);

    // Add some batch operations
    let batch_op = crate::db::redb_transaction::BatchOperation::Insert {
        table: "recovery_example".to_string(),
        key: "recovery_key".as_bytes().to_vec(),
        value: "recovery_value".as_bytes().to_vec(),
    };

    if let Err(e) = tx_manager.add_batch_operation(batch_op) {
        error!("Failed to add batch operation for recovery: {}", e);
    } else {
        info!("Added batch operation for recovery");
    }

    // Create a savepoint for recovery
    match tx_manager.begin_nested_transaction().await {
        Ok(nested_id) => {
            info!("Created savepoint for recovery with nested transaction ID: {}", nested_id);

            // Simulate a failure
            if let Err(e) = tx_manager.rollback_nested_transaction(nested_id).await {
                error!("Failed to rollback nested transaction: {}", e);
            } else {
                info!("Rolled back nested transaction to simulate failure");
            }

            // Recover the transaction
            match tx_manager.recover_transaction(transaction_id).await {
                Ok(_) => info!("Transaction recovered successfully"),
                Err(e) => error!("Failed to recover transaction: {}", e),
            }

            // Execute the recovered transaction
            let recovery_result = tx_manager.execute_write_transaction_with_options(|_| {
                // The batch operations should be executed automatically
                info!("Executing recovered transaction");
                Ok(())
            }, recovery_options).await;

            match recovery_result {
                Ok(_) => info!("Recovered transaction executed successfully"),
                Err(e) => error!("Failed to execute recovered transaction: {}", e),
            }
        },
        Err(e) => error!("Failed to create nested transaction for recovery: {}", e),
    }

    // Example 7: Transaction metrics and logging
    info!("Example 7: Transaction metrics and logging");

    // Get transaction metrics
    if let Some(metrics) = tx_manager.get_transaction_metrics() {
        info!("Transaction Metrics:");
        info!("  Total transactions: {}", metrics.total_transactions);
        info!("  Read transactions: {}", metrics.read_transactions);
        info!("  Write transactions: {}", metrics.write_transactions);
        info!("  Successful transactions: {}", metrics.successful_transactions);
        info!("  Failed transactions: {}", metrics.failed_transactions);
        info!("  Average duration: {:.2} ms", metrics.avg_duration_ms);
        info!("  Max duration: {} ms", metrics.max_duration_ms);
        info!("  Nested transactions: {}", metrics.nested_transactions);
        info!("  Max nesting depth: {}", metrics.max_nesting_depth);
    } else {
        warn!("Transaction metrics not available");
    }

    // Get transaction log
    if let Some(log) = tx_manager.get_transaction_log() {
        info!("Transaction Log (last 5 entries):");
        for (i, entry) in log.iter().rev().take(5).enumerate() {
            info!("  [{}] Transaction {}: {:?} - {:?}",
                  i, entry.transaction_id, entry.transaction_type, entry.status);
        }
    } else {
        warn!("Transaction log not available");
    }

    info!("Transaction examples completed successfully");
    Ok(())
}
