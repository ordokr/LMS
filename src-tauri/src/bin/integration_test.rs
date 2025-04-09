use lms::db::DB;
use lms::tools::integration_tester::IntegrationTester;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments or use environment variables
    let canvas_api_url = env::var("CANVAS_API_URL")
        .unwrap_or_else(|_| "https://canvas.example.com".to_string());
    
    let canvas_api_token = env::var("CANVAS_API_TOKEN")
        .unwrap_or_else(|_| "canvas_token".to_string());
    
    let discourse_api_url = env::var("DISCOURSE_API_URL")
        .unwrap_or_else(|_| "https://discourse.example.com".to_string());
    
    let discourse_api_key = env::var("DISCOURSE_API_KEY")
        .unwrap_or_else(|_| "discourse_key".to_string());
    
    let discourse_api_username = env::var("DISCOURSE_API_USERNAME")
        .unwrap_or_else(|_| "system".to_string());
    
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:lms.db".to_string());
    
    // Connect to database
    let db = DB::connect(&db_url).await?;
    
    // Initialize database tables
    db.initialize_tables().await?;
    
    // Create tester
    let tester = IntegrationTester::new(
        db,
        canvas_api_url,
        canvas_api_token,
        discourse_api_url,
        discourse_api_key,
        discourse_api_username,
    );
    
    // Run tests
    tester.run_all_tests().await?;
    
    Ok(())
}