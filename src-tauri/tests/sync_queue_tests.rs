use crate::sync::sync_queue::{SyncQueue, SyncOperation, SyncStatus};
use crate::error::Error;
use sqlx::{Pool, Sqlite, SqlitePool};
use uuid::Uuid;
use chrono::Utc;
use std::time::Duration;
use tokio::time::sleep;

/// Test utilities and setup
async fn setup_test_db() -> Result<Pool<Sqlite>, Error> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    
    // Create the necessary tables for testing
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sync_queue (
            id TEXT PRIMARY KEY,
            entity_type TEXT NOT NULL,
            entity_id TEXT NOT NULL,
            operation TEXT NOT NULL,
            source_system TEXT NOT NULL,
            target_system TEXT NOT NULL,
            data TEXT,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            attempts INTEGER DEFAULT 0,
            error_message TEXT
        )
        "#
    )
    .execute(&pool)
    .await?;
    
    Ok(pool)
}

#[tokio::test]
async fn test_sync_queue_basic_operations() -> Result<(), Error> {
    let pool = setup_test_db().await?;
    let sync_queue = SyncQueue::new(pool);
    
    // Test adding an operation to the queue
    let operation_id = sync_queue.enqueue(
        "course",
        "course-123",
        "update",
        "canvas",
        "discourse",
        serde_json::json!({"title": "Test Course"})
    ).await?;
    
    // Test retrieving the operation
    let operation = sync_queue.get_operation(&operation_id).await?;
    assert!(operation.is_some());
    let operation = operation.unwrap();
    
    assert_eq!(operation.entity_type, "course");
    assert_eq!(operation.entity_id, "course-123");
    assert_eq!(operation.operation, "update");
    assert_eq!(operation.status, SyncStatus::Pending);
    
    // Test retrieving pending operations
    let pending_operations = sync_queue.get_pending_operations(10).await?;
    assert_eq!(pending_operations.len(), 1);
    
    // Test marking as in progress
    sync_queue.mark_in_progress(&operation_id).await?;
    let updated_op = sync_queue.get_operation(&operation_id).await?.unwrap();
    assert_eq!(updated_op.status, SyncStatus::InProgress);
    
    // Test marking as completed
    sync_queue.mark_completed(&operation_id).await?;
    let completed_op = sync_queue.get_operation(&operation_id).await?.unwrap();
    assert_eq!(completed_op.status, SyncStatus::Completed);
    
    // Test marking as failed
    let operation_id_2 = sync_queue.enqueue(
        "topic",
        "topic-456",
        "create",
        "canvas",
        "discourse",
        serde_json::json!({"title": "Test Topic"})
    ).await?;
    
    sync_queue.mark_failed(&operation_id_2, "Test error message").await?;
    let failed_op = sync_queue.get_operation(&operation_id_2).await?.unwrap();
    assert_eq!(failed_op.status, SyncStatus::Failed);
    assert_eq!(failed_op.error_message.unwrap(), "Test error message");
    assert_eq!(failed_op.attempts, 1);
    
    Ok(())
}

#[tokio::test]
async fn test_sync_queue_processing() -> Result<(), Error> {
    let pool = setup_test_db().await?;
    let sync_queue = SyncQueue::new(pool);
    
    // Add multiple operations
    for i in 0..5 {
        sync_queue.enqueue(
            "course",
            &format!("course-{}", i),
            "update",
            "canvas",
            "discourse",
            serde_json::json!({"title": format!("Test Course {}", i)})
        ).await?;
    }
    
    // Test batch processing using a mock processor function
    let processed = sync_queue.process_pending(3, |op| {
        Box::pin(async move {
            // Simulate processing
            sleep(Duration::from_millis(10)).await;
            
            // Simulate success for even IDs, failure for odd IDs
            let entity_id_num = op.entity_id.split('-').nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0);
            
            if entity_id_num % 2 == 0 {
                Ok(())
            } else {
                Err(Error::Custom(format!("Failed to process {}", op.entity_id)))
            }
        })
    }).await?;
    
    assert_eq!(processed, 3); // Should have processed 3 operations (batch size)
    
    // Check statuses
    let operations = sqlx::query_as::<_, SyncOperation>("SELECT * FROM sync_queue")
        .fetch_all(sync_queue.pool())
        .await?;
    
    let completed_count = operations.iter().filter(|op| op.status == SyncStatus::Completed).count();
    let failed_count = operations.iter().filter(|op| op.status == SyncStatus::Failed).count();
    let pending_count = operations.iter().filter(|op| op.status == SyncStatus::Pending).count();
    
    // Since we process even IDs as success and odd as failure, and we processed 3 items (0, 1, 2)
    // we should have 2 completed (0, 2) and 1 failed (1)
    assert_eq!(completed_count, 2);
    assert_eq!(failed_count, 1);
    assert_eq!(pending_count, 2); // 2 operations still pending
    
    // Test retry mechanism
    let failed_ops = sync_queue.get_failed_operations(3).await?;
    assert_eq!(failed_ops.len(), 1);
    
    for op in failed_ops {
        sync_queue.retry(&op.id).await?;
    }
    
    let retried_op = sync_queue.get_operation(&failed_ops[0].id).await?.unwrap();
    assert_eq!(retried_op.status, SyncStatus::Pending);
    assert_eq!(retried_op.attempts, 1); // Attempt count should be preserved
    
    Ok(())
}

#[tokio::test]
async fn test_sync_queue_cleanup() -> Result<(), Error> {
    let pool = setup_test_db().await?;
    let sync_queue = SyncQueue::new(pool);
    
    // Add operations with various timestamps
    let now = Utc::now();
    
    // Create some completed operations with different timestamps
    for i in 0..5 {
        let op_id = sync_queue.enqueue(
            "course",
            &format!("course-{}", i),
            "update",
            "canvas",
            "discourse",
            serde_json::json!({"title": format!("Test Course {}", i)})
        ).await?;
        
        sync_queue.mark_completed(&op_id).await?;
        
        // Modify timestamp directly in DB to simulate older entries
        let days_old = 10 + i as i64; // 10-14 days old
        let old_timestamp = now - chrono::Duration::days(days_old);
        
        sqlx::query(
            "UPDATE sync_queue SET updated_at = ? WHERE id = ?"
        )
        .bind(old_timestamp.to_rfc3339())
        .bind(&op_id)
        .execute(sync_queue.pool())
        .await?;
    }
    
    // Create some failed operations with different timestamps
    for i in 5..10 {
        let op_id = sync_queue.enqueue(
            "topic",
            &format!("topic-{}", i),
            "create",
            "canvas",
            "discourse",
            serde_json::json!({"title": format!("Test Topic {}", i)})
        ).await?;
        
        sync_queue.mark_failed(&op_id, &format!("Test error {}", i)).await?;
        
        // Modify timestamp directly in DB to simulate older entries
        let days_old = 20 + (i-5) as i64; // 20-24 days old
        let old_timestamp = now - chrono::Duration::days(days_old);
        
        sqlx::query(
            "UPDATE sync_queue SET updated_at = ? WHERE id = ?"
        )
        .bind(old_timestamp.to_rfc3339())
        .bind(&op_id)
        .execute(sync_queue.pool())
        .await?;
    }
    
    // Add recent operations that shouldn't be cleaned up
    for i in 10..15 {
        let op_id = sync_queue.enqueue(
            "assignment",
            &format!("assignment-{}", i),
            "update",
            "canvas",
            "discourse",
            serde_json::json!({"title": format!("Test Assignment {}", i)})
        ).await?;
        
        if i % 2 == 0 {
            sync_queue.mark_completed(&op_id).await?;
        } else {
            sync_queue.mark_failed(&op_id, &format!("Recent error {}", i)).await?;
        }
    }
    
    // Verify we have 15 operations before cleanup
    let before_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sync_queue")
        .fetch_one(sync_queue.pool())
        .await?;
    assert_eq!(before_count, 15);
    
    // Test cleanup of completed operations older than 7 days
    let completed_cleaned = sync_queue.cleanup_completed(7).await?;
    assert_eq!(completed_cleaned, 5); // All 5 old completed operations should be cleaned up
    
    // Test cleanup of failed operations older than 14 days
    let failed_cleaned = sync_queue.cleanup_failed(14).await?;
    assert_eq!(failed_cleaned, 5); // All 5 old failed operations should be cleaned up
    
    // Verify we now have 5 operations left (the recent ones)
    let after_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sync_queue")
        .fetch_one(sync_queue.pool())
        .await?;
    assert_eq!(after_count, 5);
    
    Ok(())
}