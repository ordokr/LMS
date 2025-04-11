use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};
use anyhow::{Result, Context, anyhow};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;
use log::{info, warn, error};
use std::env;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Options for the Gemini Analyzer
#[derive(Debug, Clone)]
pub struct GeminiOptions {
    pub base_dir: Option<PathBuf>,
    pub gemini_api_key: Option<String>,
    pub min_time_between_calls: Duration,
    pub use_cache: bool,
    pub skip_gemini_25: bool,
}

impl Default for GeminiOptions {
    fn default() -> Self {
        Self {
            base_dir: None,
            gemini_api_key: None,
            min_time_between_calls: Duration::from_millis(1000), // 1 second minimum
            use_cache: true,
            skip_gemini_25: false,
        }
    }
}

/// Request payload for Gemini API
#[derive(Debug, Serialize, Deserialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

/// Response from Gemini API
#[derive(Debug, Serialize, Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PartResponse {
    text: String,
}

/// Module for Google Gemini AI integration
pub struct GeminiAnalyzer<M> {
    metrics: M,
    options: GeminiOptions,
    client: Client,
    cache_dir: PathBuf,
    last_api_call_time: Arc<Mutex<Instant>>,
}

impl<M> GeminiAnalyzer<M> {
    /// Create a new Gemini Analyzer with the given metrics and options
    pub fn new(metrics: M, options: Option<GeminiOptions>) -> Result<Self> {
        let options = options.unwrap_or_default();
        
        // Get API key from options or environment variable
        let _api_key = options.gemini_api_key.clone()
            .or_else(|| env::var("GEMINI_API_KEY").ok())
            .context("Gemini API key not provided in options or environment variables")?;
        
        // Create HTTP client
        let client = Client::new();
        
        // Set up cache directory
        let base_dir = options.base_dir.clone().unwrap_or_else(|| PathBuf::from("."));
        let cache_dir = base_dir.join(".analysis_cache").join("gemini");
        fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
        
        Ok(Self {
            metrics,
            options,
            client,
            cache_dir,
            last_api_call_time: Arc::new(Mutex::new(Instant::now())),
        })
    }
    
    /// Execute model with fallback capability, quota management and caching
    pub async fn execute_with_fallback(&self, prompt: &str, cache_key: Option<&str>) -> Result<String> {
        // Apply rate limiting
        let now = Instant::now();
        let time_since_last_call = {
            let last_call = self.last_api_call_time.lock().unwrap();
            now.saturating_duration_since(*last_call)
        };
        
        if time_since_last_call < self.options.min_time_between_calls {
            let delay_needed = self.options.min_time_between_calls - time_since_last_call;
            info!("Rate limiting: waiting {:?} before next API call", delay_needed);
            sleep(delay_needed).await;
        }
        
        // Update last API call time
        {
            let mut last_call_time = self.last_api_call_time.lock().unwrap();
            *last_call_time = Instant::now();
        }
        
        // Use cache if available and requested
        if let Some(key) = cache_key {
            if self.options.use_cache {
                if let Some(cached_result) = self.get_from_cache(key).await? {
                    info!("Using cached result for {}", key);
                    return Ok(cached_result);
                }
            }
        }
        
        // Check if we should skip Gemini 2.5 due to quota issues
        let skip_gemini_25 = self.options.skip_gemini_25 || 
                             env::var("SKIP_GEMINI_25").unwrap_or_default() == "true";
        
        // Try the primary model if not skipping
        if !skip_gemini_25 {
            info!("Using Gemini 2.5 model...");
            match self.call_gemini_model("gemini-2.5-pro-exp-03-25", prompt).await {
                Ok(response) => {
                    // Cache the successful result if we have a cache key
                    if let Some(key) = cache_key {
                        if let Err(e) = self.save_to_cache(key, &response).await {
                            warn!("Failed to cache result: {}", e);
                        }
                    }
                    
                    return Ok(response);
                }
                Err(error) => {
                    // If we hit a quota limit, set a flag to skip this model for future requests
                    if error.to_string().contains("429") && error.to_string().contains("quota") {
                        warn!("Gemini 2.5 quota exceeded, will skip for future requests");
                        
                        // Set environment variable to persist across runs
                        env::set_var("SKIP_GEMINI_25", "true");
                    }
                    
                    warn!("Gemini 2.5 failed, falling back to Gemini 2.0: {}", error);
                }
            }
        } else {
            info!("Skipping Gemini 2.5 due to previous quota limit, using Gemini 2.0...");
        }
        
        // Use fallback model
        match self.call_gemini_model("gemini-2.0-flash", prompt).await {
            Ok(response) => {
                // Cache the successful result if we have a cache key
                if let Some(key) = cache_key {
                    if let Err(e) = self.save_to_cache(key, &response).await {
                        warn!("Failed to cache result: {}", e);
                    }
                }
                
                Ok(response)
            }
            Err(fallback_error) => {
                // If both models have quota issues, we need to use local fallbacks
                if fallback_error.to_string().contains("429") && 
                   fallback_error.to_string().contains("quota") {
                    error!("All Gemini models have reached quota limits, using local fallbacks");
                    
                    // If it's a quota error, try to use cached similar requests
                    if let Some(key) = cache_key {
                        match self.find_similar_cached_request(key).await {
                            Ok(Some(similar_response)) => {
                                info!("Found similar cached response for {}", key);
                                return Ok(similar_response);
                            }
                            _ => {
                                // No similar cache found, return a default response
                                warn!("No similar cached responses found");
                                return Ok(self.generate_local_fallback_response(prompt).await?);
                            }
                        }
                    } else {
                        // No cache key provided, use local fallback
                        return Ok(self.generate_local_fallback_response(prompt).await?);
                    }
                }
                
                // Return the error if it's not handled by the fallbacks
                Err(fallback_error)
            }
        }
    }
    
    /// Call Gemini model with the given prompt
    async fn call_gemini_model(&self, model: &str, prompt: &str) -> Result<String> {
        let api_key = self.options.gemini_api_key.clone()
            .or_else(|| env::var("GEMINI_API_KEY").ok())
            .context("Gemini API key not found")?;
            
        let request_url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );
        
        let request_body = GenerateContentRequest {
            contents: vec![
                Content {
                    parts: vec![
                        Part {
                            text: prompt.to_string(),
                        },
                    ],
                },
            ],
        };
        
        let response = self.client
            .post(&request_url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;
            
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to read error response".to_string());
            return Err(anyhow!("Gemini API error ({}): {}", status, error_text));
        }
        
        let response_body: GenerateContentResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;
            
        // Extract text from response
        let text = response_body.candidates
            .into_iter()
            .flat_map(|candidate| {
                candidate.content.parts
                    .into_iter()
                    .map(|part| part.text)
            })
            .collect::<Vec<String>>()
            .join("\n");
            
        Ok(text)
    }
    
    /// Get response from cache
    async fn get_from_cache(&self, key: &str) -> Result<Option<String>> {
        let cache_file = self.cache_dir.join(format!("{}.json", key));
        
        if cache_file.exists() {
            let cached_content = fs::read_to_string(&cache_file)
                .context(format!("Failed to read cache file: {:?}", cache_file))?;
                
            Ok(Some(cached_content))
        } else {
            Ok(None)
        }
    }
    
    /// Save response to cache
    async fn save_to_cache(&self, key: &str, response: &str) -> Result<()> {
        let cache_file = self.cache_dir.join(format!("{}.json", key));
        
        fs::write(&cache_file, response)
            .context(format!("Failed to write cache file: {:?}", cache_file))?;
            
        Ok(())
    }
    
    /// Find a similar cached request when API quota is reached
    async fn find_similar_cached_request(&self, key: &str) -> Result<Option<String>> {
        // In a real implementation, this would search through cached files
        // to find one with similar content based on some similarity metric
        
        // For now, just return None to indicate no similar cache was found
        Ok(None)
    }
    
    /// Generate a local fallback response when API is unavailable
    async fn generate_local_fallback_response(&self, _prompt: &str) -> Result<String> {
        // This would implement a simpler local fallback when API is unavailable
        // For now, return a generic message
        Ok("API quota exceeded. This is a locally generated fallback response.".to_string())
    }
    
    // Additional methods for specific AI tasks would go here, such as:
    
    /// Analyze code for potential issues
    pub async fn analyze_code(&self, code: &str, file_path: &Path) -> Result<String> {
        let prompt = format!(
            "Analyze the following code from {} for potential issues, best practices, and optimization suggestions:\n\n```\n{}\n```",
            file_path.display(), code
        );
        
        let cache_key = format!("code_analysis_{}", self.hash_string(file_path.to_string_lossy().as_ref()));
        self.execute_with_fallback(&prompt, Some(&cache_key)).await
    }
    
    /// Generate documentation for code
    pub async fn generate_documentation(&self, code: &str, file_path: &Path) -> Result<String> {
        let prompt = format!(
            "Generate comprehensive documentation for the following code from {}:\n\n```\n{}\n```",
            file_path.display(), code
        );
        
        let cache_key = format!("documentation_{}", self.hash_string(file_path.to_string_lossy().as_ref()));
        self.execute_with_fallback(&prompt, Some(&cache_key)).await
    }
    
    /// Simple string hashing for cache keys
    fn hash_string(&self, input: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}
