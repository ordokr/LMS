use redb::{Database, TableDefinition};
use std::path::Path;
use tempfile::tempdir;
use tracing::{info, error};

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("Running Redb Transaction Test");
    
    // Create a temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    
    // Create the database
    let db = match Database::create(&db_path) {
        Ok(db) => {
            println!("Database created successfully at {:?}", db_path);
            db
        },
        Err(e) => {
            println!("Failed to create database: {}", e);
            return;
        }
    };
    
    // Define a table
    let table_def = TableDefinition::<&str, &str>::new("test_table");
    
    // Test basic write transaction
    println!("\n=== Testing Basic Write Transaction ===");
    let write_result = match db.begin_write() {
        Ok(mut txn) => {
            match txn.open_table(table_def) {
                Ok(mut table) => {
                    match table.insert("key1", "value1") {
                        Ok(_) => {
                            println!("Inserted key1 = value1");
                            match txn.commit() {
                                Ok(_) => {
                                    println!("Transaction committed successfully");
                                    true
                                },
                                Err(e) => {
                                    println!("Failed to commit transaction: {}", e);
                                    false
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to insert data: {}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to open table: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            println!("Failed to begin write transaction: {}", e);
            false
        }
    };
    
    println!("Write transaction result: {}", write_result);
    
    // Test basic read transaction
    println!("\n=== Testing Basic Read Transaction ===");
    let read_result = match db.begin_read() {
        Ok(txn) => {
            match txn.open_table(table_def) {
                Ok(table) => {
                    match table.get("key1") {
                        Ok(Some(value)) => {
                            println!("Read key1 = {}", value.value());
                            true
                        },
                        Ok(None) => {
                            println!("Key1 not found");
                            false
                        },
                        Err(e) => {
                            println!("Failed to read data: {}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to open table: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            println!("Failed to begin read transaction: {}", e);
            false
        }
    };
    
    println!("Read transaction result: {}", read_result);
    
    // Test multiple transactions
    println!("\n=== Testing Multiple Transactions ===");
    
    // First transaction
    let first_result = match db.begin_write() {
        Ok(mut txn) => {
            match txn.open_table(table_def) {
                Ok(mut table) => {
                    match table.insert("multi_key1", "multi_value1") {
                        Ok(_) => {
                            println!("Inserted multi_key1 = multi_value1");
                            match txn.commit() {
                                Ok(_) => {
                                    println!("First transaction committed successfully");
                                    true
                                },
                                Err(e) => {
                                    println!("Failed to commit first transaction: {}", e);
                                    false
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to insert data in first transaction: {}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to open table in first transaction: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            println!("Failed to begin first write transaction: {}", e);
            false
        }
    };
    
    println!("First transaction result: {}", first_result);
    
    // Second transaction
    let second_result = match db.begin_write() {
        Ok(mut txn) => {
            match txn.open_table(table_def) {
                Ok(mut table) => {
                    match table.insert("multi_key2", "multi_value2") {
                        Ok(_) => {
                            println!("Inserted multi_key2 = multi_value2");
                            match txn.commit() {
                                Ok(_) => {
                                    println!("Second transaction committed successfully");
                                    true
                                },
                                Err(e) => {
                                    println!("Failed to commit second transaction: {}", e);
                                    false
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to insert data in second transaction: {}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to open table in second transaction: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            println!("Failed to begin second write transaction: {}", e);
            false
        }
    };
    
    println!("Second transaction result: {}", second_result);
    
    // Read both values
    let read_multi_result = match db.begin_read() {
        Ok(txn) => {
            match txn.open_table(table_def) {
                Ok(table) => {
                    let key1_result = match table.get("multi_key1") {
                        Ok(Some(value)) => {
                            println!("Read multi_key1 = {}", value.value());
                            true
                        },
                        Ok(None) => {
                            println!("multi_key1 not found");
                            false
                        },
                        Err(e) => {
                            println!("Failed to read multi_key1: {}", e);
                            false
                        }
                    };
                    
                    let key2_result = match table.get("multi_key2") {
                        Ok(Some(value)) => {
                            println!("Read multi_key2 = {}", value.value());
                            true
                        },
                        Ok(None) => {
                            println!("multi_key2 not found");
                            false
                        },
                        Err(e) => {
                            println!("Failed to read multi_key2: {}", e);
                            false
                        }
                    };
                    
                    key1_result && key2_result
                },
                Err(e) => {
                    println!("Failed to open table for reading multiple keys: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            println!("Failed to begin read transaction for multiple keys: {}", e);
            false
        }
    };
    
    println!("Read multiple keys result: {}", read_multi_result);
    
    println!("\nAll tests completed!");
}
