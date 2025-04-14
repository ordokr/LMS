use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub analysis: String,
}

pub async fn run_gemini_analysis(code: &str) -> Result<GeminiResponse, Error> {
    let client = reqwest::Client::new();
    let response = client.post("https://api.gemini.com/analyze")
        .json(&serde_json::json!({
            "code": code
        }))
        .send()
        .await?
        .json::<GeminiResponse>()
        .await?;

    Ok(response)
}
