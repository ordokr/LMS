fn main() {
    println!("Running minimal Redb test...");
    
    // Create a temporary database
    let db_path = "minimal_test.db";
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).unwrap();
    }
    
    let db = redb::Database::create(db_path).unwrap();
    
    // Define a table
    let table_def = redb::TableDefinition::<&str, &str>::new("test_table");
    
    // Execute a write transaction
    {
        let write_txn = db.begin_write().unwrap();
        {
            let mut table = write_txn.open_table(table_def).unwrap();
            table.insert("key1", "value1").unwrap();
        }
        write_txn.commit().unwrap();
        println!("Write transaction committed successfully");
    }
    
    // Execute a read transaction
    {
        let read_txn = db.begin_read().unwrap();
        let table = read_txn.open_table(table_def).unwrap();
        let value = table.get("key1").unwrap();
        println!("Read value: {:?}", value.map(|v| v.value()));
    }
    
    println!("Test completed successfully!");
    
    // Clean up
    std::fs::remove_file(db_path).unwrap();
}
