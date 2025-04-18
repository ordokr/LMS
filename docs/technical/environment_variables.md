# Environment Variable Management: dotenvy

_Last updated: 2025-04-17_

This guide explains how to integrate and use the dotenvy crate for secure, flexible environment variable management in the Ordo project, following the modular, offline-first, and clean architecture principles.

## Table of Contents

1. [Overview](#overview)
2. [Dependencies](#dependencies)
3. [Creating Environment Files](#creating-environment-files)
4. [Loading Environment Variables](#loading-environment-variables)
5. [Using Environment Variables](#using-environment-variables)
6. [Modular Integration](#modular-integration)
7. [Best Practices](#best-practices)
8. [Implementation Examples](#implementation-examples)
9. [Summary](#summary)

## Overview

### Why Use dotenvy?

- **Security**: Keeps secrets (API keys, database URLs) out of code and version control
- **Flexibility**: Supports multiple environments (development, production) and overrides
- **Modern**: Actively maintained and recommended over the deprecated dotenv crate
- **Integration**: Works seamlessly with Rust, Tauri, async code, and CLI/server applications

dotenvy allows you to store configuration in environment variables, following the [twelve-factor app](https://12factor.net/config) methodology, while providing a convenient way to load these variables from files during development.

## Dependencies

Add dotenvy to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
dotenvy = "0.15"
```

## Creating Environment Files

At the root of your project, create a file named `.env`:

```
DATABASE_URL=sqlite://ordo.db
API_KEY=supersecretkey
PORT=3000
RUST_LOG=info
CANVAS_API_URL=https://canvas.example.com/api/v1
DISCOURSE_URL=https://discourse.example.com
```

### Multiple Environments

For different environments, create separate files:

- `.env.development` - Development environment settings
- `.env.test` - Test environment settings
- `.env.production` - Production environment settings

Example `.env.development`:
```
DATABASE_URL=sqlite://ordo_dev.db
API_KEY=dev_key
PORT=3000
RUST_LOG=debug
```

Example `.env.production`:
```
DATABASE_URL=sqlite://ordo_prod.db
API_KEY=${SECURE_API_KEY}
PORT=8080
RUST_LOG=info
```

> **Important**: Never commit `.env` files containing secrets to version control. Add them to your `.gitignore` file.

## Loading Environment Variables

### Basic Loading (Recommended for Most Use Cases)

In your main entrypoint (e.g., `src-tauri/src/main.rs`):

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    // Load .env file into process environment
    dotenv().ok();
    
    // Now you can access environment variables
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);
    
    // Continue with application initialization
    // ...
}
```

This loads variables from `.env` and preserves any set in the environment (e.g., from system or CI).

### Loading Specific Environment Files

To load environment-specific files:

```rust
use dotenvy::from_filename;
use std::env;

fn main() {
    // Determine which environment we're in
    let environment = env::var("ORDO_ENV").unwrap_or_else(|_| "development".to_string());
    
    // Load the appropriate .env file
    from_filename(format!(".env.{}", environment)).ok();
    
    // Continue with application initialization
    // ...
}
```

### Overriding Existing Variables

If you want `.env` values to override existing environment variables:

```rust
use dotenvy::dotenv_override;

fn main() {
    // .env values override existing ones
    dotenv_override().ok();
    
    // Continue with application initialization
    // ...
}
```

### Non-Modifying API (Advanced/Library Use)

If you want to read variables without modifying the global environment:

```rust
use dotenvy::vars;

fn main() {
    // Load variables into an iterator without modifying the process environment
    for (key, value) in vars() {
        println!("{}={}", key, value);
    }
    
    // Or use the EnvLoader for more control
    let env_map = dotenvy::EnvLoader::new().load().expect("Failed to load .env");
    println!("API_KEY={}", env_map.var("API_KEY").unwrap_or_default());
}
```

## Using Environment Variables

Once loaded, access environment variables anywhere in your Rust code:

```rust
use std::env;

fn get_database_connection() -> Result<SqlitePool, Error> {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
```

### With Default Values

For optional variables with defaults:

```rust
let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid number");
```

### Type Conversion

Convert string values to appropriate types:

```rust
// Convert to integer
let max_connections = env::var("MAX_CONNECTIONS")
    .unwrap_or_else(|_| "5".to_string())
    .parse::<u32>()
    .expect("MAX_CONNECTIONS must be a valid number");

// Convert to boolean
let debug_mode = env::var("DEBUG_MODE")
    .unwrap_or_else(|_| "false".to_string())
    .to_lowercase() == "true";
```

## Modular Integration

### Backend (Rust/Tauri)

Load environment variables in the main entrypoint before any configuration or service initialization:

```rust
// src-tauri/src/main.rs
fn main() {
    // Load environment variables first
    dotenvy::dotenv().ok();
    
    // Initialize logging with environment-based configuration
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    // Initialize database with environment-based configuration
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = initialize_database(&db_url).await;
    
    // Continue with application initialization
    // ...
}
```

### Configuration Module

Create a dedicated configuration module that loads from environment variables:

```rust
// src-tauri/src/config.rs
use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub database_url: String,
    pub api_key: String,
    pub port: u16,
    pub log_level: String,
    pub canvas_api_url: String,
    pub discourse_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Load environment variables if not already loaded
        dotenvy::dotenv().ok();
        
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            api_key: env::var("API_KEY")
                .expect("API_KEY must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            log_level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
            canvas_api_url: env::var("CANVAS_API_URL")
                .unwrap_or_else(|_| "https://canvas.example.com/api/v1".to_string()),
            discourse_url: env::var("DISCOURSE_URL")
                .unwrap_or_else(|_| "https://discourse.example.com".to_string()),
        }
    }
}

// Global configuration instance
pub static CONFIG: Lazy<Config> = Lazy::new(Config::from_env);
```

### Frontend (Leptos)

For frontend code, expose non-sensitive configuration via API endpoints or use Tauri's build system to inject environment variables at build time.

## Best Practices

1. **Security**
   - Never commit secrets in `.env` to version control
   - Use different `.env` files for different environments
   - Consider using a secrets manager for production environments

2. **Documentation**
   - Document all required environment variables in `docs/technical/implementation_details.md`
   - Include example values and descriptions for each variable
   - Provide a template `.env.example` file in the repository

3. **Loading Order**
   - Load environment variables before initializing any services that depend on them
   - Use a configuration module to centralize access to environment variables
   - Consider the precedence of environment variables (system > `.env` or vice versa)

4. **Error Handling**
   - Provide clear error messages when required variables are missing
   - Use default values for optional variables
   - Validate environment variables at startup

5. **Testing**
   - Use a separate `.env.test` file for testing
   - Mock environment variables in unit tests
   - Reset environment variables after tests

## Implementation Examples

### Basic Application Startup

```rust
// src-tauri/src/main.rs
use dotenvy::dotenv;
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    info!("Starting Ordo application");
    
    // Get database configuration from environment
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            error!("DATABASE_URL environment variable not set");
            return Err("DATABASE_URL environment variable not set".into());
        }
    };
    
    // Initialize database
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    // Continue with application initialization
    // ...
    
    Ok(())
}
```

### Configuration Module with Validation

```rust
// src-tauri/src/config.rs
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVariable(String),
    
    #[error("Invalid environment variable: {0}")]
    InvalidVariable(String),
}

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

pub struct ApiConfig {
    pub key: String,
    pub port: u16,
}

pub struct Config {
    pub database: DatabaseConfig,
    pub api: ApiConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // Load environment variables
        dotenvy::dotenv().ok();
        
        // Load and validate database configuration
        let db_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVariable("DATABASE_URL".to_string()))?;
        
        let max_connections = env::var("MAX_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .map_err(|_| ConfigError::InvalidVariable("MAX_CONNECTIONS".to_string()))?;
        
        // Load and validate API configuration
        let api_key = env::var("API_KEY")
            .map_err(|_| ConfigError::MissingVariable("API_KEY".to_string()))?;
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidVariable("PORT".to_string()))?;
        
        Ok(Self {
            database: DatabaseConfig {
                url: db_url,
                max_connections,
            },
            api: ApiConfig {
                key: api_key,
                port,
            },
        })
    }
}
```

### Environment-Specific Configuration

```rust
// src-tauri/src/config.rs
use std::env;

pub enum Environment {
    Development,
    Test,
    Production,
}

impl Environment {
    pub fn from_env() -> Self {
        match env::var("ORDO_ENV").unwrap_or_else(|_| "development".to_string()).as_str() {
            "production" => Environment::Production,
            "test" => Environment::Test,
            _ => Environment::Development,
        }
    }
    
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }
    
    pub fn is_test(&self) -> bool {
        matches!(self, Environment::Test)
    }
    
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

pub fn load_environment_config() -> Result<(), Box<dyn std::error::Error>> {
    let environment = Environment::from_env();
    
    // Load the appropriate .env file
    match environment {
        Environment::Development => dotenvy::from_filename(".env.development").ok(),
        Environment::Test => dotenvy::from_filename(".env.test").ok(),
        Environment::Production => dotenvy::from_filename(".env.production").ok(),
    };
    
    Ok(())
}
```

## Summary

| Step | What to Do | Where/How |
|------|------------|-----------|
| 1 | Add dependency | `dotenvy = "0.15"` in Cargo.toml |
| 2 | Create .env file | Add key-value pairs in project root |
| 3 | Load in code | `dotenv().ok();` or `dotenv_override().ok();` in main entrypoint |
| 4 | Access variables | `env::var("KEY")` anywhere in Rust code |
| 5 | Document variables | List required vars and usage in docs/technical/ |

Implementing dotenvy ensures Ordo's configuration is secure, modular, and environment-agnostic, supporting the project's offline-first and extensible goals. By keeping configuration in environment variables, you maintain flexibility across different deployment environments while keeping sensitive information out of your codebase.
