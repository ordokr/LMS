use lms_lib::db::test_redb_transactions;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("Running Redb Transaction Tests...");
    
    if let Err(e) = test_redb_transactions::run_all_tests() {
        eprintln!("Tests failed: {}", e);
        std::process::exit(1);
    }
    
    println!("All tests passed successfully!");
}
