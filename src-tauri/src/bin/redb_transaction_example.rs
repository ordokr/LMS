use lms_lib::db::redb_error::Result;
use lms_lib::examples::redb_transaction_example;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Run the transaction example
    println!("Running Redb Transaction Example...");
    redb_transaction_example::run_transaction_example().await?;
    println!("Example completed successfully!");

    Ok(())
}
