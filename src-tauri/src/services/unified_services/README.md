# Unified Services

This directory contains the unified service implementations for the LMS application. These services replace the redundant service implementations that were scattered throughout the codebase.

## Overview

The unified services provide a consistent interface for business logic across the application. They are designed to be:

- **Consistent**: All services follow the same design patterns and conventions
- **Comprehensive**: All services include all methods needed for all use cases
- **Configurable**: All services support flexible configuration options
- **Testable**: All services are designed for easy testing

## Services

The following services are included:

- **BaseService**: Base implementation of the Service trait
- **AuthService**: Authentication and authorization service
- **NotificationService**: Notification management service

## Usage

### Creating a Service

```rust
// Create a service configuration
let config = ServiceConfig::new("auth")
    .with_db_pool(db_pool)
    .with_parameter("token_expiration", "3600");

// Create an auth service
let auth_service = AuthService::new(config, jwt_secret);

// Register the service
init_services(vec![auth_service]).await?;
```

### Using a Service

```rust
// Get a service
let auth_service = get_service::<AuthService>("auth").await?;

// Use the service
let (user, access_token, refresh_token) = auth_service.login("username", "password").await?;

// Create a notification
let notification_service = get_service::<NotificationService>("notification").await?;
let notification = notification_service.create_notification(
    "Title",
    "Message",
    NotificationType::Info,
    Some("user123"),
    None,
    None,
    None,
    None,
).await?;
```

### Service Health Checks

```rust
// Check if all services are healthy
let all_healthy = health_check_services().await?;

// Check if a specific service is healthy
let auth_service = get_service::<AuthService>("auth").await?;
let is_healthy = auth_service.health_check().await?;
```

### Shutting Down Services

```rust
// Shutdown all services
shutdown_services().await?;
```

## Creating a New Service

To create a new service, follow these steps:

1. Create a new file for your service (e.g., `my_service.rs`)
2. Implement the `Service` trait for your service
3. Add your service to the `mod.rs` file
4. Register your service with the service registry

Example:

```rust
use async_trait::async_trait;
use crate::errors::error::Result;
use super::base_service::{Service, ServiceConfig, BaseService};

#[derive(Debug)]
pub struct MyService {
    base: BaseService,
}

impl MyService {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            base: BaseService::new(config),
        }
    }
    
    pub async fn do_something(&self) -> Result<String> {
        // Implement your service logic here
        Ok("Something done".to_string())
    }
}

#[async_trait]
impl Service for MyService {
    fn name(&self) -> &str {
        self.base.name()
    }
    
    async fn init(&self) -> Result<()> {
        self.base.init().await
    }
    
    async fn shutdown(&self) -> Result<()> {
        self.base.shutdown().await
    }
    
    async fn health_check(&self) -> Result<bool> {
        self.base.health_check().await
    }
}
```

## Testing Services

Services are designed to be easily testable. Here's an example of how to test a service:

```rust
#[tokio::test]
async fn test_auth_service() {
    // Create a test database
    let db_pool = create_test_db().await;
    
    // Create a service configuration
    let config = ServiceConfig::new("auth")
        .with_db_pool(db_pool)
        .with_parameter("token_expiration", "3600");
    
    // Create an auth service
    let auth_service = AuthService::new(config, "test_secret".as_bytes().to_vec());
    
    // Register a test user
    let user = auth_service.register_user(
        "testuser",
        "test@example.com",
        "password123",
        Some("user"),
    ).await.unwrap();
    
    // Test login
    let (logged_in_user, access_token, refresh_token) = auth_service.login(
        "testuser",
        "password123",
    ).await.unwrap();
    
    // Verify user
    assert_eq!(logged_in_user.id, user.id);
    assert_eq!(logged_in_user.username, "testuser");
    
    // Verify token
    let claims = auth_service.verify_token(&access_token).unwrap();
    assert_eq!(claims.sub, user.id);
}
```

## Best Practices

1. **Use the Service trait**: All services should implement the `Service` trait
2. **Use the BaseService**: All services should use the `BaseService` for common functionality
3. **Use the ServiceConfig**: All services should use the `ServiceConfig` for configuration
4. **Use the ServiceRegistry**: All services should be registered with the `ServiceRegistry`
5. **Use the Result type**: All service methods should return a `Result` type
6. **Use async/await**: All service methods should be async
7. **Use proper error handling**: All service methods should handle errors properly
8. **Use proper logging**: All service methods should log important events
9. **Use proper validation**: All service methods should validate input
10. **Use proper documentation**: All service methods should be documented
