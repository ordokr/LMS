use async_trait::async_trait;
use reqwest::{Client, Response, StatusCode, Method, RequestBuilder, header};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;
use std::sync::Arc;
use std::fmt::Debug;

/// Error type for API operations
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// HTTP client error
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// API error with status code and message
    #[error("API error: {status_code} - {message}")]
    ApiError {
        status_code: StatusCode,
        message: String,
    },
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Rate limit error
    #[error("Rate limit exceeded: retry after {retry_after} seconds")]
    RateLimitError {
        retry_after: u64,
    },
    
    /// Timeout error
    #[error("Request timed out")]
    TimeoutError,
    
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Unexpected error
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

/// Type alias for result with ApiError
pub type Result<T> = std::result::Result<T, ApiError>;

/// Configuration for API clients
#[derive(Debug, Clone)]
pub struct ApiClientConfig {
    /// Base URL for the API
    pub base_url: String,
    
    /// API key or token
    pub api_key: String,
    
    /// API username (for APIs that require it)
    pub api_username: Option<String>,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// Maximum number of retries
    pub max_retries: u32,
    
    /// Whether to use exponential backoff for retries
    pub use_exponential_backoff: bool,
    
    /// Whether to enable circuit breaker
    pub enable_circuit_breaker: bool,
    
    /// Maximum number of connections per host
    pub max_connections_per_host: usize,
    
    /// Whether to enable compression
    pub enable_compression: bool,
    
    /// Additional headers
    pub additional_headers: Vec<(String, String)>,
}

impl Default for ApiClientConfig {
    fn default() -> Self {
        Self {
            base_url: "".to_string(),
            api_key: "".to_string(),
            api_username: None,
            timeout_seconds: 30,
            max_retries: 3,
            use_exponential_backoff: true,
            enable_circuit_breaker: true,
            max_connections_per_host: 10,
            enable_compression: true,
            additional_headers: Vec::new(),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    pub page: Option<u32>,
    
    /// Number of items per page
    pub per_page: Option<u32>,
    
    /// Cursor for cursor-based pagination
    pub cursor: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(50),
            cursor: None,
        }
    }
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Items in the current page
    pub items: Vec<T>,
    
    /// Total number of items
    pub total: Option<u64>,
    
    /// Current page number
    pub page: Option<u32>,
    
    /// Number of items per page
    pub per_page: Option<u32>,
    
    /// Total number of pages
    pub total_pages: Option<u32>,
    
    /// Next page cursor (for cursor-based pagination)
    pub next_cursor: Option<String>,
    
    /// Previous page cursor (for cursor-based pagination)
    pub prev_cursor: Option<String>,
    
    /// Whether there is a next page
    pub has_next: bool,
    
    /// Whether there is a previous page
    pub has_prev: bool,
}

/// Base API client trait
#[async_trait]
pub trait ApiClient: Send + Sync + Debug {
    /// Get the client configuration
    fn get_config(&self) -> &ApiClientConfig;
    
    /// Get the HTTP client
    fn get_http_client(&self) -> &Client;
    
    /// Make a GET request
    async fn get<T>(&self, path: &str, query_params: Option<&[(&str, &str)]>) -> Result<T>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.request::<(), T>(Method::GET, path, query_params, None).await
    }
    
    /// Make a GET request with pagination
    async fn get_paginated<T>(&self, path: &str, pagination: &PaginationParams, query_params: Option<&[(&str, &str)]>) -> Result<PaginatedResponse<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        // Convert pagination params to query params
        let mut all_params = Vec::new();
        
        if let Some(page) = pagination.page {
            all_params.push(("page", page.to_string()));
        }
        
        if let Some(per_page) = pagination.per_page {
            all_params.push(("per_page", per_page.to_string()));
        }
        
        if let Some(cursor) = &pagination.cursor {
            all_params.push(("cursor", cursor.clone()));
        }
        
        // Add additional query params
        if let Some(params) = query_params {
            for (key, value) in params {
                all_params.push((key.to_string(), value.to_string()));
            }
        }
        
        // Convert to &[(&str, &str)]
        let params: Vec<(&str, &str)> = all_params.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        
        self.request::<(), PaginatedResponse<T>>(Method::GET, path, Some(&params), None).await
    }
    
    /// Make a POST request
    async fn post<D, T>(&self, path: &str, data: Option<&D>, query_params: Option<&[(&str, &str)]>) -> Result<T>
    where
        D: Serialize + Send + Sync + ?Sized,
        T: DeserializeOwned + Send + 'static,
    {
        self.request::<D, T>(Method::POST, path, query_params, data).await
    }
    
    /// Make a PUT request
    async fn put<D, T>(&self, path: &str, data: Option<&D>, query_params: Option<&[(&str, &str)]>) -> Result<T>
    where
        D: Serialize + Send + Sync + ?Sized,
        T: DeserializeOwned + Send + 'static,
    {
        self.request::<D, T>(Method::PUT, path, query_params, data).await
    }
    
    /// Make a PATCH request
    async fn patch<D, T>(&self, path: &str, data: Option<&D>, query_params: Option<&[(&str, &str)]>) -> Result<T>
    where
        D: Serialize + Send + Sync + ?Sized,
        T: DeserializeOwned + Send + 'static,
    {
        self.request::<D, T>(Method::PATCH, path, query_params, data).await
    }
    
    /// Make a DELETE request
    async fn delete<T>(&self, path: &str, query_params: Option<&[(&str, &str)]>) -> Result<T>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.request::<(), T>(Method::DELETE, path, query_params, None).await
    }
    
    /// Make a request with the specified method, path, query parameters, and data
    async fn request<D, T>(&self, method: Method, path: &str, query_params: Option<&[(&str, &str)]>, data: Option<&D>) -> Result<T>
    where
        D: Serialize + Send + Sync + ?Sized,
        T: DeserializeOwned + Send + 'static,
    {
        let config = self.get_config();
        let url = format!("{}{}", config.base_url, path);
        
        let mut builder = self.get_http_client().request(method, &url);
        
        // Add query parameters
        if let Some(params) = query_params {
            builder = builder.query(params);
        }
        
        // Add request body
        if let Some(body) = data {
            builder = builder.json(body);
        }
        
        // Execute request with retry logic
        self.execute_with_retry(builder).await
    }
    
    /// Execute a request with retry logic
    async fn execute_with_retry(&self, builder: RequestBuilder) -> Result<Response> {
        let config = self.get_config();
        let mut retries = 0;
        
        loop {
            let response = builder.try_clone()
                .ok_or_else(|| ApiError::UnexpectedError("Failed to clone request".to_string()))?
                .send()
                .await;
                
            match response {
                Ok(resp) => {
                    // Check for rate limiting
                    if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                        let retry_after = resp.headers()
                            .get(header::RETRY_AFTER)
                            .and_then(|h| h.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(1);
                            
                        if retries < config.max_retries {
                            retries += 1;
                            tokio::time::sleep(Duration::from_secs(retry_after)).await;
                            continue;
                        } else {
                            return Err(ApiError::RateLimitError { retry_after });
                        }
                    }
                    
                    // Check for server errors (5xx)
                    if resp.status().is_server_error() {
                        if retries < config.max_retries {
                            retries += 1;
                            
                            // Calculate backoff time
                            let backoff_time = if config.use_exponential_backoff {
                                // Exponential backoff: 2^retries seconds
                                2u64.pow(retries)
                            } else {
                                // Linear backoff: retries seconds
                                retries as u64
                            };
                            
                            tokio::time::sleep(Duration::from_secs(backoff_time)).await;
                            continue;
                        }
                    }
                    
                    return Ok(resp);
                },
                Err(err) => {
                    // Retry on timeout or connection errors
                    if err.is_timeout() || err.is_connect() {
                        if retries < config.max_retries {
                            retries += 1;
                            
                            // Calculate backoff time
                            let backoff_time = if config.use_exponential_backoff {
                                // Exponential backoff: 2^retries seconds
                                2u64.pow(retries)
                            } else {
                                // Linear backoff: retries seconds
                                retries as u64
                            };
                            
                            tokio::time::sleep(Duration::from_secs(backoff_time)).await;
                            continue;
                        } else if err.is_timeout() {
                            return Err(ApiError::TimeoutError);
                        } else {
                            return Err(ApiError::NetworkError(err.to_string()));
                        }
                    }
                    
                    return Err(ApiError::HttpError(err));
                }
            }
        }
    }
    
    /// Process a response
    async fn process_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned + Send + 'static,
    {
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
                
            return Err(ApiError::ApiError {
                status_code: status,
                message: text,
            });
        }
        
        // For empty responses (e.g., 204 No Content), return an empty object
        if response.status() == StatusCode::NO_CONTENT {
            // Create an empty JSON object and try to deserialize it
            return serde_json::from_value::<T>(serde_json::json!({}))
                .map_err(ApiError::SerializationError);
        }
        
        // Parse the response body
        response.json::<T>().await
            .map_err(ApiError::SerializationError)
    }
}

/// Base API client implementation
#[derive(Debug, Clone)]
pub struct BaseApiClient {
    /// Client configuration
    config: ApiClientConfig,
    
    /// HTTP client
    client: Client,
}

impl BaseApiClient {
    /// Create a new BaseApiClient
    pub fn new(config: ApiClientConfig) -> Result<Self> {
        // Create a client builder
        let mut client_builder = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .pool_max_idle_per_host(config.max_connections_per_host)
            .pool_idle_timeout(Duration::from_secs(60))
            .tcp_keepalive(Duration::from_secs(60));
            
        // Enable compression if requested
        if config.enable_compression {
            client_builder = client_builder.gzip(true).brotli(true);
        }
        
        // Build the client
        let client = client_builder.build()
            .map_err(ApiError::HttpError)?;
            
        Ok(Self {
            config,
            client,
        })
    }
    
    /// Create a new BaseApiClient with default configuration
    pub fn new_with_defaults(base_url: &str, api_key: &str) -> Result<Self> {
        let config = ApiClientConfig {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            ..Default::default()
        };
        
        Self::new(config)
    }
}

impl ApiClient for BaseApiClient {
    fn get_config(&self) -> &ApiClientConfig {
        &self.config
    }
    
    fn get_http_client(&self) -> &Client {
        &self.client
    }
}

/// Create a new API client with the specified configuration
pub fn create_api_client(config: ApiClientConfig) -> Result<Arc<dyn ApiClient>> {
    let client = BaseApiClient::new(config)?;
    Ok(Arc::new(client))
}

/// Create a new API client with default configuration
pub fn create_default_api_client(base_url: &str, api_key: &str) -> Result<Arc<dyn ApiClient>> {
    let client = BaseApiClient::new_with_defaults(base_url, api_key)?;
    Ok(Arc::new(client))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    struct TestResponse {
        message: String,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    struct TestRequest {
        name: String,
    }
    
    #[tokio::test]
    async fn test_get_request() {
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message":"success"}"#)
            .create();
            
        let config = ApiClientConfig {
            base_url: server_url(),
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        
        let client = BaseApiClient::new(config).unwrap();
        let response: TestResponse = client.get("/test", None).await.unwrap();
        
        assert_eq!(response.message, "success");
    }
    
    #[tokio::test]
    async fn test_post_request() {
        let _m = mock("POST", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message":"created"}"#)
            .create();
            
        let config = ApiClientConfig {
            base_url: server_url(),
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        
        let client = BaseApiClient::new(config).unwrap();
        let request = TestRequest { name: "test".to_string() };
        let response: TestResponse = client.post("/test", Some(&request), None).await.unwrap();
        
        assert_eq!(response.message, "created");
    }
    
    #[tokio::test]
    async fn test_error_handling() {
        let _m = mock("GET", "/error")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"not found"}"#)
            .create();
            
        let config = ApiClientConfig {
            base_url: server_url(),
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        
        let client = BaseApiClient::new(config).unwrap();
        let result: Result<TestResponse> = client.get("/error", None).await;
        
        assert!(result.is_err());
        if let Err(ApiError::ApiError { status_code, .. }) = result {
            assert_eq!(status_code, StatusCode::NOT_FOUND);
        } else {
            panic!("Expected ApiError::ApiError");
        }
    }
    
    #[tokio::test]
    async fn test_query_params() {
        let _m = mock("GET", "/test?param1=value1&param2=value2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message":"success"}"#)
            .create();
            
        let config = ApiClientConfig {
            base_url: server_url(),
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        
        let client = BaseApiClient::new(config).unwrap();
        let params = [("param1", "value1"), ("param2", "value2")];
        let response: TestResponse = client.get("/test", Some(&params)).await.unwrap();
        
        assert_eq!(response.message, "success");
    }
}
