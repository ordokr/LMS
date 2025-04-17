#[cfg(test)]
mod tests {
    use crate::db::redb_transaction::{RedbTransactionManager, TransactionOptions, IsolationLevel, TransactionHook};
    use crate::db::redb_error::{RedbError, Result};
    use crate::db::redb_transaction_log::{TransactionLogger, TransactionType, TransactionStatus};
    use redb::{Database, TableDefinition};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tempfile::tempdir;
    use tokio::time::sleep;

    // Helper function to create a temporary database
    async fn setup_test_db() -> (Database, RedbTransactionManager) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::create(db_path).unwrap();
        let tx_manager = RedbTransactionManager::new(db.clone());
        (db, tx_manager)
    }

    #[tokio::test]
    async fn test_nested_transactions() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Start a parent transaction
        let result = tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(table_def)?;
            table.insert("parent_key", "parent_value")?;
            Ok(())
        }).await;
        
        assert!(result.is_ok());
        
        // Start a parent transaction that will contain a nested transaction
        let parent_result = tx_manager.execute_write_transaction_with_options(|txn| {
            // Insert in parent
            let mut table = txn.open_table(table_def)?;
            table.insert("parent_key2", "parent_value2")?;
            
            // Begin a nested transaction
            let nested_id = tx_manager.begin_nested_transaction().await?;
            
            // Execute nested transaction logic
            let nested_result = tx_manager.execute_write_transaction_with_options(|nested_txn| {
                let mut nested_table = nested_txn.open_table(table_def)?;
                nested_table.insert("nested_key", "nested_value")?;
                Ok(())
            }, TransactionOptions::default()).await;
            
            assert!(nested_result.is_ok());
            
            // Commit the nested transaction
            tx_manager.commit_nested_transaction(nested_id).await?;
            
            Ok(())
        }, TransactionOptions::default()).await;
        
        assert!(parent_result.is_ok());
        
        // Verify both parent and nested transaction data was saved
        let parent_value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("parent_key2")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        let nested_value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("nested_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        assert_eq!(parent_value, Some("parent_value2".to_string()));
        assert_eq!(nested_value, Some("nested_value".to_string()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_nested_transaction_rollback() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Start a parent transaction that will contain a nested transaction
        let parent_result = tx_manager.execute_write_transaction_with_options(|txn| {
            // Insert in parent
            let mut table = txn.open_table(table_def)?;
            table.insert("parent_key", "parent_value")?;
            
            // Begin a nested transaction
            let nested_id = tx_manager.begin_nested_transaction().await?;
            
            // Execute nested transaction logic
            let nested_result = tx_manager.execute_write_transaction_with_options(|nested_txn| {
                let mut nested_table = nested_txn.open_table(table_def)?;
                nested_table.insert("nested_key", "nested_value")?;
                Ok(())
            }, TransactionOptions::default()).await;
            
            assert!(nested_result.is_ok());
            
            // Rollback the nested transaction
            tx_manager.rollback_nested_transaction(nested_id).await?;
            
            Ok(())
        }, TransactionOptions::default()).await;
        
        assert!(parent_result.is_ok());
        
        // Verify parent transaction data was saved but nested was not
        let parent_value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("parent_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        let nested_value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("nested_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        assert_eq!(parent_value, Some("parent_value".to_string()));
        assert_eq!(nested_value, None); // Nested transaction was rolled back
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_recovery() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Create options with recovery enabled
        let options = TransactionOptions {
            enable_recovery: true,
            ..TransactionOptions::default()
        };
        
        // Start a transaction that will be recovered
        let transaction_id = tx_manager.start_transaction(options.clone());
        
        // Add some batch operations
        let batch_op = crate::db::redb_transaction::BatchOperation::Insert {
            table: "test_table".to_string(),
            key: "recovery_key".as_bytes().to_vec(),
            value: "recovery_value".as_bytes().to_vec(),
        };
        
        tx_manager.add_batch_operation(batch_op)?;
        
        // Create a savepoint for recovery
        let nested_id = tx_manager.begin_nested_transaction().await?;
        
        // Simulate a failure
        tx_manager.rollback_nested_transaction(nested_id).await?;
        
        // Recover the transaction
        tx_manager.recover_transaction(transaction_id).await?;
        
        // Execute the recovered transaction
        let result = tx_manager.execute_write_transaction_with_options(|txn| {
            // The batch operations should be executed automatically
            Ok(())
        }, options).await;
        
        assert!(result.is_ok());
        
        // Verify the recovered data was saved
        let recovery_value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("recovery_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        assert_eq!(recovery_value, Some("recovery_value".to_string()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_batching() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Create options with batching enabled
        let options = TransactionOptions {
            enable_batching: true,
            max_batch_size: Some(10),
            ..TransactionOptions::default()
        };
        
        // Start a transaction with batching
        let transaction_id = tx_manager.start_transaction(options.clone());
        
        // Add batch operations
        for i in 0..5 {
            let key = format!("batch_key_{}", i);
            let value = format!("batch_value_{}", i);
            
            let batch_op = crate::db::redb_transaction::BatchOperation::Insert {
                table: "test_table".to_string(),
                key: key.as_bytes().to_vec(),
                value: value.as_bytes().to_vec(),
            };
            
            tx_manager.add_batch_operation(batch_op)?;
        }
        
        // Execute the transaction with batching
        let result = tx_manager.execute_write_transaction_with_options(|_| {
            // The batch operations should be executed automatically
            Ok(())
        }, options).await;
        
        assert!(result.is_ok());
        
        // Verify all batch operations were executed
        for i in 0..5 {
            let key = format!("batch_key_{}", i);
            let expected_value = format!("batch_value_{}", i);
            
            let value = tx_manager.execute_read_transaction(|txn| {
                let table = txn.open_table(table_def)?;
                match table.get(key.as_str())? {
                    Some(value) => Ok(Some(value.value().to_string())),
                    None => Ok(None),
                }
            }).await?;
            
            assert_eq!(value, Some(expected_value));
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_logging() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Create options with logging enabled
        let options = TransactionOptions {
            enable_logging: true,
            ..TransactionOptions::default()
        };
        
        // Execute a few transactions
        for i in 0..3 {
            let key = format!("log_key_{}", i);
            let value = format!("log_value_{}", i);
            
            let result = tx_manager.execute_write_transaction_with_options(|txn| {
                let mut table = txn.open_table(table_def)?;
                table.insert(key.as_str(), value.as_str())?;
                Ok(())
            }, options.clone()).await;
            
            assert!(result.is_ok());
        }
        
        // Get transaction metrics
        let metrics = tx_manager.get_transaction_metrics();
        assert!(metrics.is_some());
        
        let metrics = metrics.unwrap();
        assert_eq!(metrics.write_transactions, 3);
        assert_eq!(metrics.successful_transactions, 3);
        
        // Get transaction log
        let log = tx_manager.get_transaction_log();
        assert!(log.is_some());
        
        let log = log.unwrap();
        assert_eq!(log.len(), 3);
        
        // Verify log entries
        for entry in log {
            assert_eq!(entry.transaction_type, TransactionType::Write);
            assert_eq!(entry.status, TransactionStatus::Committed);
            assert!(entry.tables_accessed.contains(&"test_table".to_string()));
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_timeout() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        
        // Create options with a very short timeout
        let options = TransactionOptions {
            timeout_ms: Some(10), // 10ms timeout
            enable_logging: true,
            ..TransactionOptions::default()
        };
        
        // Execute a transaction that will timeout
        let result = tx_manager.execute_write_transaction_with_options(|_| async {
            // Sleep for longer than the timeout
            sleep(Duration::from_millis(100)).await;
            Ok(())
        }, options).await;
        
        // Verify that we got a timeout error
        match result {
            Err(RedbError::Timeout(_)) => {
                // Check that the timeout was logged
                let log = tx_manager.get_transaction_log().unwrap();
                let timeout_entries = log.iter()
                    .filter(|entry| entry.status == TransactionStatus::TimedOut)
                    .count();
                
                assert_eq!(timeout_entries, 1);
                Ok(())
            },
            _ => panic!("Expected timeout error, got: {:?}", result),
        }
    }
}
