use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Serialize, Deserialize};
use reqwest;
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;

/// LLM integration for generating AI insights
pub struct LlmIntegration {
    /// Base directory for analysis
    base_dir: PathBuf,

    /// LM Studio API endpoint
    api_endpoint: String,

    /// Model to use
    model: String,
}

/// LLM request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    /// Model to use
    pub model: String,

    /// Prompt to send to the model
    pub prompt: String,

    /// Maximum number of tokens to generate
    #[serde(rename = "max_tokens")]
    pub max_tokens: usize,

    /// Temperature for generation
    pub temperature: f32,

    /// Top-p for generation
    #[serde(rename = "top_p")]
    pub top_p: f32,
}

/// LLM response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// Generated text
    pub text: String,
}

impl LlmIntegration {
    /// Create a new LLM integration
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            api_endpoint: "http://localhost:1234/v1/completions".to_string(),
            model: "qwen2.5-coder-32b-instruct".to_string(),
        }
    }

    /// Set the API endpoint
    pub fn set_api_endpoint(&mut self, api_endpoint: String) {
        self.api_endpoint = api_endpoint;
    }

    /// Set the model
    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }

    /// Generate insights using LLM
    pub async fn generate_insights(&self, result: &AnalysisResult) -> Result<String, String> {
        println!("Generating insights using LLM...");

        // Create a prompt for the LLM
        let prompt = self.create_prompt(result)?;

        // Send the prompt to the LLM
        let insights = self.send_prompt(prompt).await?;

        Ok(insights)
    }

    /// Create a prompt for the LLM
    fn create_prompt(&self, result: &AnalysisResult) -> Result<String, String> {
        let mut prompt = String::new();

        // Add system instructions
        prompt.push_str("You are an expert software development advisor analyzing a codebase. Based on the following analysis results, provide insightful recommendations and identify potential issues.\n\n");

        // Add project summary
        prompt.push_str("# Project Summary\n");
        prompt.push_str(&format!("- Total Files: {}\n", result.summary.total_files));
        prompt.push_str(&format!("- Lines of Code: {}\n", result.summary.lines_of_code));
        prompt.push_str(&format!("- Rust Files: {}\n", result.summary.rust_files));
        prompt.push_str(&format!("- Haskell Files: {}\n", result.summary.haskell_files));
        prompt.push_str(&format!("- Overall Progress: {:.1}%\n\n", result.overall_progress));

        // Add component progress
        prompt.push_str("# Component Progress\n");
        prompt.push_str(&format!("- Models: {:.1}% ({}/{})\n",
            result.models.implementation_percentage,
            result.models.implemented,
            result.models.total));
        prompt.push_str(&format!("- API Endpoints: {:.1}% ({}/{})\n",
            result.api_endpoints.implementation_percentage,
            result.api_endpoints.implemented,
            result.api_endpoints.total));
        prompt.push_str(&format!("- UI Components: {:.1}% ({}/{})\n\n",
            result.ui_components.implementation_percentage,
            result.ui_components.implemented,
            result.ui_components.total));

        // Add feature areas
        prompt.push_str("# Feature Areas\n");
        for (area, metrics) in &result.feature_areas {
            prompt.push_str(&format!("- {}: {:.1}% ({}/{})\n",
                area,
                metrics.implementation_percentage,
                metrics.implemented,
                metrics.total));
        }
        prompt.push_str("\n");

        // Add technical debt
        prompt.push_str("# Technical Debt\n");
        prompt.push_str(&format!("- Total Issues: {}\n", result.tech_debt_metrics.total_issues));
        prompt.push_str(&format!("- Critical Issues: {}\n", result.tech_debt_metrics.critical_issues));
        prompt.push_str(&format!("- High Issues: {}\n", result.tech_debt_metrics.high_issues));
        prompt.push_str(&format!("- Medium Issues: {}\n", result.tech_debt_metrics.medium_issues));
        prompt.push_str(&format!("- Low Issues: {}\n\n", result.tech_debt_metrics.low_issues));

        // Add recent changes
        prompt.push_str("# Recent Changes\n");
        for change in &result.recent_changes {
            prompt.push_str(&format!("- {}\n", change));
        }
        prompt.push_str("\n");

        // Add next steps
        prompt.push_str("# Next Steps\n");
        for step in &result.next_steps {
            prompt.push_str(&format!("- {}\n", step));
        }
        prompt.push_str("\n");

        // Add instructions for the response format
        prompt.push_str("Based on the above information, provide insights in the following format:\n\n");
        prompt.push_str("# AI Insights\n\n");
        prompt.push_str("## Top Insights\n\n");
        prompt.push_str("1. **[Insight Title]**\n");
        prompt.push_str("   - **Category**: [Category]\n");
        prompt.push_str("   - **Priority**: [Critical/High/Medium/Low]\n");
        prompt.push_str("   - **Description**: [Description]\n");
        prompt.push_str("   - **Recommendations**:\n");
        prompt.push_str("     - [Recommendation 1]\n");
        prompt.push_str("     - [Recommendation 2]\n\n");
        prompt.push_str("2. **[Insight Title]**\n");
        prompt.push_str("   ...\n\n");
        prompt.push_str("## Detailed Analysis\n\n");
        prompt.push_str("### Technical Debt\n");
        prompt.push_str("[Analysis of technical debt issues and recommendations]\n\n");
        prompt.push_str("### Code Quality\n");
        prompt.push_str("[Analysis of code quality issues and recommendations]\n\n");
        prompt.push_str("### Architecture\n");
        prompt.push_str("[Analysis of architectural issues and recommendations]\n\n");
        prompt.push_str("### Project Progress\n");
        prompt.push_str("[Analysis of project progress and recommendations]\n\n");
        prompt.push_str("## Summary\n");
        prompt.push_str("[Overall summary and key recommendations]\n\n");

        Ok(prompt)
    }

    /// Send a prompt to the LLM
    async fn send_prompt(&self, prompt: String) -> Result<String, String> {
        // Create the request
        let request = LlmRequest {
            model: self.model.clone(),
            prompt,
            max_tokens: 2000,
            temperature: 0.7,
            top_p: 0.9,
        };

        // Send the request to the LLM
        let client = reqwest::Client::new();
        let response = client.post(&self.api_endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to LLM: {}", e))?;

        // Parse the response
        let response: LlmResponse = response.json()
            .await
            .map_err(|e| format!("Failed to parse LLM response: {}", e))?;

        Ok(response.text)
    }

    /// Check if LM Studio is running
    pub fn check_lm_studio_running(&self) -> bool {
        // Try to make a simple request to the API endpoint
        let client = reqwest::blocking::Client::new();
        let response = client.get("http://localhost:1234/v1/models")
            .send();

        response.is_ok()
    }

    /// Start LM Studio
    pub fn start_lm_studio(&self) -> Result<(), String> {
        // This is a placeholder for starting LM Studio
        // In a real implementation, we would need to know the path to the LM Studio executable

        println!("Please start LM Studio manually and ensure it's running on http://localhost:1234");

        Ok(())
    }
}
