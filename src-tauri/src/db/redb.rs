use super::redb_transaction::{RedbTransactionManager, TransactionOptions, IsolationLevel};
use super::redb_error::{RedbError, Result};
use redb::{Database, TableDefinition};
use std::path::Path;
use std::time::Duration;
use tracing::{instrument, info, error};

/// Redb database module for handling ephemeral state and sync metadata
pub mod redb {
    use super::*;

    /// Open a Redb database at the specified path
    ///
    /// # Arguments
    /// * `path` - Path to the database file
    ///
    /// # Returns
    /// A tuple containing the database instance and transaction manager
    #[instrument(err)]
    pub fn open_database(path: &str) -> Result<(Database, RedbTransactionManager)> {
        // Ensure the directory exists
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| RedbError::Other(format!("Failed to create directory: {}", e)))?;
            }
        }

        // Create or open the database
        let db = Database::create(path)
            .map_err(|e| RedbError::Native(e))?;

        // Create the transaction manager
        let tx_manager = RedbTransactionManager::new(db.clone());

        Ok((db, tx_manager))
    }

    /// Save a draft document
    ///
    /// # Arguments
    /// * `tx_manager` - Transaction manager
    /// * `user_id` - User ID
    /// * `content` - Draft content
    ///
    /// # Returns
    /// Result indicating success or failure
    #[instrument(skip(tx_manager), err)]
    pub async fn save_draft(
        tx_manager: &RedbTransactionManager,
        user_id: &str,
        content: &str
    ) -> Result<()> {
        let drafts_table = TableDefinition::<&str, &str>::new("drafts");

        // Use default transaction options
        tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(drafts_table)?;
            table.insert(user_id, content)?;
            Ok(())
        }).await
    }

    /// Save a draft document with custom transaction options
    ///
    /// # Arguments
    /// * `tx_manager` - Transaction manager
    /// * `user_id` - User ID
    /// * `content` - Draft content
    /// * `options` - Transaction options
    ///
    /// # Returns
    /// Result indicating success or failure
    #[instrument(skip(tx_manager), err)]
    pub async fn save_draft_with_options(
        tx_manager: &RedbTransactionManager,
        user_id: &str,
        content: &str,
        options: TransactionOptions
    ) -> Result<()> {
        let drafts_table = TableDefinition::<&str, &str>::new("drafts");

        tx_manager.execute_write_transaction_with_options(|txn| {
            let mut table = txn.open_table(drafts_table)?;
            table.insert(user_id, content)?;
            Ok(())
        }, options).await
    }

    /// Get a draft document
    ///
    /// # Arguments
    /// * `tx_manager` - Transaction manager
    /// * `user_id` - User ID
    ///
    /// # Returns
    /// The draft content if found
    #[instrument(skip(tx_manager), err)]
    pub async fn get_draft(
        tx_manager: &RedbTransactionManager,
        user_id: &str
    ) -> Result<Option<String>> {
        let drafts_table = TableDefinition::<&str, &str>::new("drafts");

        tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(drafts_table)?;
            match table.get(user_id)? {
                Some(value) => Ok(Some(value.value().to_string())),
                None => Ok(None),
            }
        }).await
    }

    /// Delete a draft document
    ///
    /// # Arguments
    /// * `tx_manager` - Transaction manager
    /// * `user_id` - User ID
    ///
    /// # Returns
    /// Result indicating success or failure
    #[instrument(skip(tx_manager), err)]
    pub async fn delete_draft(
        tx_manager: &RedbTransactionManager,
        user_id: &str
    ) -> Result<()> {
        let drafts_table = TableDefinition::<&str, &str>::new("drafts");

        tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(drafts_table)?;
            table.remove(user_id)?;
            Ok(())
        }).await
    }

    /// Store a key-value pair in a specified table
    ///
    /// # Arguments
    /// * `tx_manager` - Transaction manager
    /// * `table_name` - Name of the table
    /// * `key` - Key
    /// * `value` - Value
    ///
    /// # Returns
    /// Result indicating success or failure
    #[instrument(skip(tx_manager), err)]
    pub async fn store<K: AsRef<[u8]> + 'static, V: AsRef<[u8]> + 'static>(
        tx_manager: &RedbTransactionManager,
        table_name: &str,
        key: K,
        value: V
    ) -> Result<()> {
        let table_def = TableDefinition::<K, V>::new(table_name);

        tx_manager.execute_write_transaction(|txn| {
            let mut table = txn.open_table(table_def)?;
            table.insert(key, value)?;
            Ok(())
        }).await
    }

    /// Retrieve a value from a specified table
    ///
    /// # Arguments
    /// * `tx_manager` - Transaction manager
    /// * `table_name` - Name of the table
    /// * `key` - Key
    ///
    /// # Returns
    /// The value if found
    #[instrument(skip(tx_manager), err)]
    pub async fn retrieve<K: AsRef<[u8]> + 'static, V: AsRef<[u8]> + 'static>(
        tx_manager: &RedbTransactionManager,
        table_name: &str,
        key: K
    ) -> Result<Option<Vec<u8>>> {
        let table_def = TableDefinition::<K, V>::new(table_name);

        tx_manager.execute_read_transaction(|txn| {
            let table = txn.open_table(table_def)?;
            match table.get(key)? {
                Some(value) => {
                    let bytes = value.value().as_ref().to_vec();
                    Ok(Some(bytes))
                },
                None => Ok(None),
            }
        }).await
    }

    /// Create a new transaction options object with custom settings
    ///
    /// # Arguments
    /// * `timeout_ms` - Transaction timeout in milliseconds
    /// * `max_retries` - Maximum number of retries for transient errors
    /// * `isolation_level` - Transaction isolation level
    ///
    /// # Returns
    /// Transaction options object
    pub fn transaction_options(
        timeout_ms: Option<u64>,
        max_retries: u32,
        isolation_level: IsolationLevel
    ) -> TransactionOptions {
        TransactionOptions {
            timeout_ms,
            max_retries,
            isolation_level,
        }
    }
}