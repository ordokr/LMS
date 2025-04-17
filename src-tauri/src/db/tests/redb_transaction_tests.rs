#[cfg(test)]
mod tests {
    use crate::db::redb_transaction::{RedbTransactionManager, TransactionOptions, IsolationLevel, TransactionHook};
    use crate::db::redb_error::{RedbError, Result};
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
    async fn test_basic_write_transaction() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Execute a write transaction
        let result = tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(table_def)?;
            table.insert("key1", "value1")?;
            Ok(())
        }).await;
        
        assert!(result.is_ok());
        
        // Verify the data was written
        let value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("key1")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        assert_eq!(value, Some("value1".to_string()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_with_options() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Create custom transaction options
        let options = TransactionOptions {
            timeout_ms: Some(5000),
            max_retries: 2,
            isolation_level: IsolationLevel::Default,
        };
        
        // Execute a write transaction with options
        let result = tx_manager.execute_write_transaction_with_options(|txn| {
            let mut table = txn.open_table(table_def)?;
            table.insert("key2", "value2")?;
            Ok(())
        }, options).await;
        
        assert!(result.is_ok());
        
        // Verify the data was written
        let value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("key2")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        assert_eq!(value, Some("value2".to_string()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_transaction_timeout() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        
        // Create options with a very short timeout
        let options = TransactionOptions {
            timeout_ms: Some(10), // 10ms timeout
            max_retries: 0,
            isolation_level: IsolationLevel::Default,
        };
        
        // Execute a transaction that will timeout
        let result = tx_manager.execute_write_transaction_with_options(|_| async {
            // Sleep for longer than the timeout
            sleep(Duration::from_millis(100)).await;
            Ok(())
        }, options).await;
        
        // Verify that we got a timeout error
        match result {
            Err(RedbError::Timeout(_)) => Ok(()),
            _ => panic!("Expected timeout error, got: {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_transaction_hooks() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Create counters for hooks
        let before_begin_count = Arc::new(Mutex::new(0));
        let after_commit_count = Arc::new(Mutex::new(0));
        
        // Add hooks
        let before_begin_count_clone = before_begin_count.clone();
        tx_manager.add_hook(TransactionHook::BeforeBegin(Box::new(move || {
            let mut count = before_begin_count_clone.lock().unwrap();
            *count += 1;
        })));
        
        let after_commit_count_clone = after_commit_count.clone();
        tx_manager.add_hook(TransactionHook::AfterCommit(Box::new(move || {
            let mut count = after_commit_count_clone.lock().unwrap();
            *count += 1;
        })));
        
        // Execute a transaction
        tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(table_def)?;
            table.insert("key3", "value3")?;
            Ok(())
        }).await?;
        
        // Verify hooks were called
        assert_eq!(*before_begin_count.lock().unwrap(), 1);
        assert_eq!(*after_commit_count.lock().unwrap(), 1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_transactions() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Execute multiple transactions in parallel
        let mut handles = Vec::new();
        
        for i in 0..10 {
            let tx_manager_clone = tx_manager.clone();
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            
            let handle = tokio::spawn(async move {
                tx_manager_clone.execute_write_transaction(|txn| {
                    let mut table = txn.open_table(table_def)?;
                    table.insert(key.as_str(), value.as_str())?;
                    Ok(())
                }).await
            });
            
            handles.push(handle);
        }
        
        // Wait for all transactions to complete
        for handle in handles {
            handle.await.unwrap()?;
        }
        
        // Verify all data was written
        for i in 0..10 {
            let key = format!("key{}", i);
            let expected_value = format!("value{}", i);
            
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
    async fn test_transaction_retry() -> Result<()> {
        let (_, tx_manager) = setup_test_db().await;
        let table_def = TableDefinition::<&str, &str>::new("test_table");
        
        // Counter for retry attempts
        let attempt_count = Arc::new(Mutex::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        // Create options with retries
        let options = TransactionOptions {
            timeout_ms: None,
            max_retries: 3,
            isolation_level: IsolationLevel::Default,
        };
        
        // Execute a transaction that will fail on the first two attempts
        let result = tx_manager.execute_write_transaction_with_options(|txn| {
            let mut count = attempt_count_clone.lock().unwrap();
            *count += 1;
            
            // Fail on the first two attempts
            if *count <= 2 {
                return Err(RedbError::Other("Simulated transient error".to_string()));
            }
            
            // Succeed on the third attempt
            let mut table = txn.open_table(table_def)?;
            table.insert("retry_key", "retry_value")?;
            Ok(())
        }, options).await;
        
        // Verify that the transaction succeeded after retries
        assert!(result.is_ok());
        assert_eq!(*attempt_count.lock().unwrap(), 3); // Initial attempt + 2 retries
        
        // Verify the data was written
        let value = tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get("retry_key")? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await?;
        
        assert_eq!(value, Some("retry_value".to_string()));
        
        Ok(())
    }
}
