//! Environment variable management for Ordo
//!
//! This module provides utilities for loading and accessing environment variables
//! using dotenvy. For detailed implementation guide, see docs/technical/environment_variables.md

use dotenvy::{dotenv, from_filename};
use std::env;
use tracing::{info, warn};

/// Environment enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Development environment
    Development,
    /// Test environment
    Test,
    /// Production environment
    Production,
}

impl Environment {
    /// Get the current environment from the ORDO_ENV environment variable
    pub fn current() -> Self {
        match env::var("ORDO_ENV").unwrap_or_else(|_| "development".to_string()).as_str() {
            "production" => Environment::Production,
            "test" => Environment::Test,
            _ => Environment::Development,
        }
    }

    /// Check if the current environment is development
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }

    /// Check if the current environment is test
    pub fn is_test(&self) -> bool {
        matches!(self, Environment::Test)
    }

    /// Check if the current environment is production
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

/// Initialize environment variables
///
/// This function loads environment variables from the appropriate .env file
/// based on the current environment.
pub fn init() {
    // First try to load from the environment-specific file
    let environment = Environment::current();
    let env_file = match environment {
        Environment::Development => ".env.development",
        Environment::Test => ".env.test",
        Environment::Production => ".env.production",
    };

    match from_filename(env_file) {
        Ok(_) => info!("Loaded environment variables from {}", env_file),
        Err(_) => {
            // If environment-specific file doesn't exist, try the default .env file
            match dotenv() {
                Ok(_) => info!("Loaded environment variables from .env file"),
                Err(e) => warn!("Could not load .env file: {}", e),
            }
        }
    }
}

/// Get an environment variable with a default value
pub fn get_env_or<T: std::str::FromStr>(key: &str, default: T) -> T
where
    T: std::fmt::Display,
{
    match env::var(key) {
        Ok(val) => match val.parse::<T>() {
            Ok(parsed) => parsed,
            Err(_) => {
                warn!("Could not parse environment variable {}, using default: {}", key, default);
                default
            }
        },
        Err(_) => default,
    }
}

/// Get an environment variable as a boolean
pub fn get_env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(val) => match val.to_lowercase().as_str() {
            "true" | "1" | "yes" | "y" => true,
            "false" | "0" | "no" | "n" => false,
            _ => {
                warn!("Could not parse boolean environment variable {}, using default: {}", key, default);
                default
            }
        },
        Err(_) => default,
    }
}

/// Get an environment variable that must be present
pub fn get_required_env(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("Required environment variable {} is not set", key))
}

/// Get an environment variable as an Option
pub fn get_optional_env(key: &str) -> Option<String> {
    env::var(key).ok()
}
