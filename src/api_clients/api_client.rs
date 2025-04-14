
// Define the ApiClient trait

// api_client.rs
use reqwest::blocking::{Client, Response};



pub trait ApiClient {

    fn get(&self, url: &str) -> Result<Response, reqwest::Error>;

    fn post(&self, url: &str, body: serde_json::Value) -> Result<Response, reqwest::Error>;

    fn put(&self, url: &str, body: serde_json::Value) -> Result<Response, reqwest::Error>;

    fn delete(&self, url: &str) -> Result<Response, reqwest::Error>;

}
