use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiConfig {
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gemini-pro".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    generation_config: GeminiGenerationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiGenerationConfig {
    temperature: f32,
    max_output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
    finish_reason: String,
}

pub struct GeminiAnalyzer {
    config: GeminiConfig,
    client: Client,
}

impl GeminiAnalyzer {
    pub fn new(api_key: String) -> Self {
        Self {
            config: GeminiConfig {
                api_key,
                ..GeminiConfig::default()
            },
            client: Client::new(),
        }
    }
    
    pub fn with_config(config: GeminiConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
    
    pub async fn generate_insights(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.config.model,
            self.config.api_key
        );
        
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GeminiGenerationConfig {
                temperature: self.config.temperature,
                max_output_tokens: self.config.max_tokens,
            },
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json::<GeminiResponse>()
            .await?;
        
        if response.candidates.is_empty() {
            return Err("No response from Gemini API".into());
        }
        
        let content = &response.candidates[0].content;
        let mut result = String::new();
        
        for part in &content.parts {
            result.push_str(&part.text);
        }
        
        // Format as markdown
        let formatted = format!("# AI Code Insights\n\n_Generated on: {}_ by Gemini\n\n{}", 
            chrono::Utc::now().format("%Y-%m-%d"),
            result
        );
        
        Ok(formatted)
    }
    
    pub async fn analyze_code_architecture(&self, codebase_summary: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let prompt = format!(
            "Analyze this codebase architecture summary and provide insights about the design patterns, 
            architecture decisions, and potential improvements. Focus on Rust, Tauri, and Leptos best practices.
            Format your analysis as markdown with appropriate sections and code examples where needed:
            
            {}",
            codebase_summary
        );
        
        self.generate_insights(&prompt).await
    }
    
    pub async fn analyze_integration_points(&self, integration_summary: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let prompt = format!(
            "Analyze these Canvas-Discourse integration points and identify potential conflicts, 
            synchronization challenges, and best practices for offline-first implementation.
            Format your analysis as markdown with appropriate sections:
            
            {}",
            integration_summary
        );
        
        self.generate_insights(&prompt).await
    }
    
    pub async fn generate_next_steps(&self, project_status: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let prompt = format!(
            "Based on this project status summary, recommend the next development steps for the Canvas-Discourse 
            integration project. Focus on Rust/Tauri/Leptos implementation priorities and provide specific, 
            actionable recommendations. Include code examples where appropriate. Format as markdown:
            
            {}",
            project_status
        );
        
        self.generate_insights(&prompt).await
    }
}
