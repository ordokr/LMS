use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    analysis: Analysis,
}

#[derive(Deserialize, Debug)]
struct Analysis {
    quality: String,
    conflicts: Vec<String>,
    architecture_adherence: String,
    next_steps: Vec<String>,
}

pub async fn analyze_code() -> Result<Analysis, reqwest::Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.gemini.com/analyze")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "project_path": "./src",
            "language": "rust"
        }))
        .send()
        .await?;

    let gemini_response: GeminiResponse = response.json().await?;
    Ok(gemini_response.analysis)
}