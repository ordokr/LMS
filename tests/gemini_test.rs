use crate::services::gemini;
use tokio;

#[tokio::test]
async fn test_gemini() {
    // Example test logic for Gemini service
    let result = gemini::fetch_data().await.unwrap();
    assert_eq!(result, "expected_result");
}
