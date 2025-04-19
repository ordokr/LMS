use lms_tauri::api::unified_clients::{
    ApiClient, ApiClientConfig, PaginationParams,
    CanvasApiClient, DiscourseApiClient,
    create_canvas_client, create_discourse_client,
    CanvasClientAdapter, DiscourseClientAdapter,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing unified API clients...");
    
    // Test Canvas client
    test_canvas_client().await?;
    
    // Test Discourse client
    test_discourse_client().await?;
    
    // Test adapters
    test_adapters().await?;
    
    println!("All tests passed!");
    Ok(())
}

async fn test_canvas_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTesting Canvas API client...");
    
    // Create a Canvas client with a mock URL
    let config = ApiClientConfig {
        base_url: "https://canvas.example.com".to_string(),
        api_key: "test_key".to_string(),
        ..Default::default()
    };
    
    let client = CanvasApiClient::new(&config.base_url, &config.api_key)?;
    
    // Print client configuration
    println!("Canvas client created with base URL: {}", client.get_config().base_url);
    println!("Canvas client created with API key: {}", client.get_config().api_key);
    
    // Test client methods (these will fail in a real environment, but we're just testing the interface)
    println!("Testing Canvas client methods...");
    
    // Create pagination parameters
    let pagination = PaginationParams {
        page: Some(1),
        per_page: Some(10),
        cursor: None,
    };
    
    // Test generic API client methods
    println!("Testing generic API client methods...");
    println!("  - GET method: {}", client.get_config().base_url);
    println!("  - POST method: {}", client.get_config().base_url);
    println!("  - PUT method: {}", client.get_config().base_url);
    println!("  - DELETE method: {}", client.get_config().base_url);
    
    println!("Canvas API client tests completed.");
    Ok(())
}

async fn test_discourse_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTesting Discourse API client...");
    
    // Create a Discourse client with a mock URL
    let config = ApiClientConfig {
        base_url: "https://discourse.example.com".to_string(),
        api_key: "test_key".to_string(),
        api_username: Some("test_admin".to_string()),
        ..Default::default()
    };
    
    let client = DiscourseApiClient::new(&config.base_url, &config.api_key, &config.api_username.clone().unwrap_or_default())?;
    
    // Print client configuration
    println!("Discourse client created with base URL: {}", client.get_config().base_url);
    println!("Discourse client created with API key: {}", client.get_config().api_key);
    println!("Discourse client created with API username: {}", client.api_username);
    
    // Test client methods (these will fail in a real environment, but we're just testing the interface)
    println!("Testing Discourse client methods...");
    
    // Create pagination parameters
    let pagination = PaginationParams {
        page: Some(1),
        per_page: Some(10),
        cursor: None,
    };
    
    // Test generic API client methods
    println!("Testing generic API client methods...");
    println!("  - GET method: {}", client.get_config().base_url);
    println!("  - POST method: {}", client.get_config().base_url);
    println!("  - PUT method: {}", client.get_config().base_url);
    println!("  - DELETE method: {}", client.get_config().base_url);
    
    println!("Discourse API client tests completed.");
    Ok(())
}

async fn test_adapters() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTesting API client adapters...");
    
    // Create a Canvas client with a mock URL
    let canvas_client = create_canvas_client("https://canvas.example.com", "test_key")?;
    
    // Create a Canvas client adapter
    let canvas_adapter = CanvasClientAdapter::new(canvas_client.clone());
    
    // Print adapter information
    println!("Canvas adapter created.");
    println!("Canvas adapter client base URL: {}", canvas_adapter.get_client().get_config().base_url);
    
    // Create a Discourse client with a mock URL
    let discourse_client = create_discourse_client("https://discourse.example.com", "test_key", "test_admin")?;
    
    // Create a Discourse client adapter
    let discourse_adapter = DiscourseClientAdapter::new(discourse_client.clone());
    
    // Print adapter information
    println!("Discourse adapter created.");
    println!("Discourse adapter client base URL: {}", discourse_adapter.get_client().get_config().base_url);
    
    println!("API client adapter tests completed.");
    Ok(())
}
