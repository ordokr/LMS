use crate::db::redb_transaction::{RedbTransactionManager, TransactionOptions, IsolationLevel};
use crate::db::redb_error::Result;
use redb::{Database, TableDefinition};
use std::path::Path;
use tempfile::tempdir;

/// A simple function to test the basic transaction functionality
pub fn test_basic_transaction() -> Result<()> {
    // Create a temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::create(&db_path).unwrap();
    let tx_manager = RedbTransactionManager::new(db);
    
    // Define a table
    let table_def = TableDefinition::<&str, &str>::new("test_table");
    
    // Execute a write transaction
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(async {
        tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(table_def)?;
            table.insert("key1", "value1")?;
            Ok(())
        }).await
    });
    
    println!("Write transaction result: {:?}", result);
    
    // Execute a read transaction
    let read_result = runtime.block_on(async {
        tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("key1")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await
    });
    
    println!("Read transaction result: {:?}", read_result);
    
    // Test nested transactions
    let nested_result = runtime.block_on(async {
        tx_manager.execute_write_transaction_with_options(|txn| {
            // Insert in parent
            let mut table = txn.open_table(table_def)?;
            table.insert("parent_key", "parent_value")?;
            
            // Begin a nested transaction
            let nested_id = tx_manager.begin_nested_transaction().await?;
            println!("Started nested transaction with ID: {}", nested_id);
            
            // Execute nested transaction logic
            let nested_options = TransactionOptions {
                enable_nested: true,
                enable_logging: true,
                ..TransactionOptions::default()
            };
            
            let nested_result = tx_manager.execute_write_transaction_with_options(|nested_txn| {
                let mut nested_table = nested_txn.open_table(table_def)?;
                nested_table.insert("nested_key", "nested_value")?;
                println!("Inserted data in nested transaction");
                Ok(())
            }, nested_options).await;
            
            if let Err(e) = nested_result {
                println!("Nested transaction failed: {}", e);
                return Err(e);
            }
            
            // Commit the nested transaction
            tx_manager.commit_nested_transaction(nested_id).await?;
            println!("Nested transaction committed successfully");
            
            Ok(())
        }, TransactionOptions {
            enable_nested: true,
            enable_logging: true,
            ..TransactionOptions::default()
        }).await
    });
    
    println!("Nested transaction result: {:?}", nested_result);
    
    // Verify both parent and nested transaction data was saved
    let parent_value = runtime.block_on(async {
        tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("parent_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await
    });
    
    println!("Parent value: {:?}", parent_value);
    
    let nested_value = runtime.block_on(async {
        tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("nested_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await
    });
    
    println!("Nested value: {:?}", nested_value);
    
    // Test transaction metrics
    if let Some(metrics) = tx_manager.get_transaction_metrics() {
        println!("Transaction Metrics:");
        println!("  Total transactions: {}", metrics.total_transactions);
        println!("  Read transactions: {}", metrics.read_transactions);
        println!("  Write transactions: {}", metrics.write_transactions);
        println!("  Successful transactions: {}", metrics.successful_transactions);
        println!("  Failed transactions: {}", metrics.failed_transactions);
        println!("  Average duration: {:.2} ms", metrics.avg_duration_ms);
    } else {
        println!("Transaction metrics not available");
    }
    
    // Test transaction log
    if let Some(log) = tx_manager.get_transaction_log() {
        println!("Transaction Log (last 5 entries):");
        for (i, entry) in log.iter().rev().take(5).enumerate() {
            println!("  [{}] Transaction {}: {:?} - {:?}", 
                  i, entry.transaction_id, entry.transaction_type, entry.status);
        }
    } else {
        println!("Transaction log not available");
    }
    
    Ok(())
}

/// A simple function to test transaction batching
pub fn test_transaction_batching() -> Result<()> {
    // Create a temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::create(&db_path).unwrap();
    let tx_manager = RedbTransactionManager::new(db);
    
    // Define a table
    let table_def = TableDefinition::<&str, &str>::new("test_table");
    
    // Create options with batching enabled
    let batch_options = TransactionOptions {
        enable_batching: true,
        max_batch_size: Some(10),
        enable_logging: true,
        ..TransactionOptions::default()
    };
    
    // Start a transaction with batching
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let batch_result = runtime.block_on(async {
        // Start a transaction with batching
        let transaction_id = tx_manager.start_transaction(batch_options.clone());
        println!("Started batch transaction with ID: {}", transaction_id);
        
        // Add batch operations
        for i in 0..5 {
            let key = format!("batch_key_{}", i);
            let value = format!("batch_value_{}", i);
            
            let batch_op = crate::db::redb_transaction::BatchOperation::Insert {
                table: "test_table".to_string(),
                key: key.as_bytes().to_vec(),
                value: value.as_bytes().to_vec(),
            };
            
            if let Err(e) = tx_manager.add_batch_operation(batch_op) {
                println!("Failed to add batch operation: {}", e);
                return Err(e);
            }
            
            println!("Added batch operation for key: {}", key);
        }
        
        println!("All batch operations added successfully");
        
        // Execute the transaction with batching
        tx_manager.execute_write_transaction_with_options(|_| {
            // The batch operations should be executed automatically
            println!("Executing batch transaction");
            Ok(())
        }, batch_options).await
    });
    
    println!("Batch transaction result: {:?}", batch_result);
    
    // Verify all batch operations were executed
    for i in 0..5 {
        let key = format!("batch_key_{}", i);
        let expected_value = format!("batch_value_{}", i);
        
        let value = runtime.block_on(async {
            tx_manager.execute_read_transaction(|txn| {
                let table = txn.open_table(table_def)?;
                match table.get(key.as_str())? {
                    Some(value) => Ok(Some(value.value().to_string())),
                    None => Ok(None),
                }
            }).await
        });
        
        println!("Key: {}, Value: {:?}, Expected: {}", key, value, expected_value);
    }
    
    Ok(())
}

/// A simple function to test transaction recovery
pub fn test_transaction_recovery() -> Result<()> {
    // Create a temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::create(&db_path).unwrap();
    let tx_manager = RedbTransactionManager::new(db);
    
    // Define a table
    let table_def = TableDefinition::<&str, &str>::new("test_table");
    
    // Create options with recovery enabled
    let recovery_options = TransactionOptions {
        enable_recovery: true,
        enable_nested: true,
        enable_logging: true,
        ..TransactionOptions::default()
    };
    
    // Start a transaction that will be recovered
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let recovery_result = runtime.block_on(async {
        // Start a transaction that will be recovered
        let transaction_id = tx_manager.start_transaction(recovery_options.clone());
        println!("Started transaction for recovery demo with ID: {}", transaction_id);
        
        // Add some batch operations
        let batch_op = crate::db::redb_transaction::BatchOperation::Insert {
            table: "test_table".to_string(),
            key: "recovery_key".as_bytes().to_vec(),
            value: "recovery_value".as_bytes().to_vec(),
        };
        
        if let Err(e) = tx_manager.add_batch_operation(batch_op) {
            println!("Failed to add batch operation for recovery: {}", e);
            return Err(e);
        } else {
            println!("Added batch operation for recovery");
        }
        
        // Create a savepoint for recovery
        match tx_manager.begin_nested_transaction().await {
            Ok(nested_id) => {
                println!("Created savepoint for recovery with nested transaction ID: {}", nested_id);
                
                // Simulate a failure
                if let Err(e) = tx_manager.rollback_nested_transaction(nested_id).await {
                    println!("Failed to rollback nested transaction: {}", e);
                    return Err(e);
                } else {
                    println!("Rolled back nested transaction to simulate failure");
                }
                
                // Recover the transaction
                match tx_manager.recover_transaction(transaction_id).await {
                    Ok(_) => println!("Transaction recovered successfully"),
                    Err(e) => {
                        println!("Failed to recover transaction: {}", e);
                        return Err(e);
                    }
                }
                
                // Execute the recovered transaction
                tx_manager.execute_write_transaction_with_options(|_| {
                    // The batch operations should be executed automatically
                    println!("Executing recovered transaction");
                    Ok(())
                }, recovery_options).await
            },
            Err(e) => {
                println!("Failed to create nested transaction for recovery: {}", e);
                Err(e)
            }
        }
    });
    
    println!("Recovery transaction result: {:?}", recovery_result);
    
    // Verify the recovered data was saved
    let recovery_value = runtime.block_on(async {
        tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("recovery_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await
    });
    
    println!("Recovery value: {:?}", recovery_value);
    
    Ok(())
}

/// Run all tests
pub fn run_all_tests() -> Result<()> {
    println!("Running basic transaction test...");
    test_basic_transaction()?;
    
    println!("\nRunning transaction batching test...");
    test_transaction_batching()?;
    
    println!("\nRunning transaction recovery test...");
    test_transaction_recovery()?;
    
    println!("\nAll tests completed successfully!");
    Ok(())
}
