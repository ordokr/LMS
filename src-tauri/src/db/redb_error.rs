use std::fmt;
use thiserror::Error;

/// Custom error type for Redb database operations
#[derive(Error, Debug)]
pub enum RedbError {
    /// Wraps the native Redb error
    #[error("Redb error: {0}")]
    Native(#[from] redb::Error),

    /// Transaction was aborted
    #[error("Transaction aborted: {0}")]
    Aborted(String),

    /// Transaction timed out
    #[error("Transaction timed out after {0} ms")]
    Timeout(u64),

    /// Transaction deadlock detected
    #[error("Transaction deadlock detected")]
    Deadlock,

    /// Nested transaction error
    #[error("Nested transaction error: {0}")]
    NestedTransaction(String),

    /// Savepoint error
    #[error("Savepoint error: {0}")]
    Savepoint(String),

    /// Transaction already committed
    #[error("Transaction already committed")]
    AlreadyCommitted,

    /// Transaction already rolled back
    #[error("Transaction already rolled back")]
    AlreadyRolledBack,

    /// Invalid savepoint name
    #[error("Invalid savepoint name: {0}")]
    InvalidSavepoint(String),

    /// No active transaction
    #[error("No active transaction")]
    NoActiveTransaction,

    /// Recovery error
    #[error("Recovery error: {0}")]
    Recovery(String),

    /// Table error
    #[error("Table error: {0}")]
    Table(String),

    /// Data serialization/deserialization error
    #[error("Data error: {0}")]
    Data(String),

    /// Constraint violation
    #[error("Constraint violation: {0}")]
    Constraint(String),

    /// Other errors
    #[error("Database error: {0}")]
    Other(String),
}

impl RedbError {
    /// Returns true if the error is transient and the operation might succeed if retried
    pub fn is_transient(&self) -> bool {
        match self {
            RedbError::Native(e) => {
                // Check if the native error is transient
                // This would need to be updated based on redb's error types
                matches!(e, redb::Error::Io(_))
            }
            RedbError::Timeout(_) | RedbError::Deadlock => true,
            _ => false,
        }
    }

    /// Returns true if the error indicates a conflict with another transaction
    pub fn is_conflict(&self) -> bool {
        match self {
            RedbError::Native(e) => {
                // Check if the native error indicates a conflict
                // This would need to be updated based on redb's error types
                matches!(e, redb::Error::TableAlreadyOpen)
            }
            RedbError::Deadlock => true,
            _ => false,
        }
    }
}

/// Result type alias for Redb operations
pub type Result<T> = std::result::Result<T, RedbError>;
