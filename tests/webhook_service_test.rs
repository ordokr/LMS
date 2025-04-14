#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;
    use serde_json::{json, Value};
    
    // Import the webhook service
    use crate::services::webhook_service::{self, handle_canvas_webhook, handle_discourse_webhook};
    use tokio;
    
    #[tokio::test]
    async fn test_handle_canvas_webhook() {
        // Example test logic for handling Canvas webhook
        let payload = "{}";
        handle_canvas_webhook(payload).await;
    }
    
    #[tokio::test]
    async fn test_handle_discourse_webhook() {
        // Example test logic for handling Discourse webhook
        let payload = "{}";
        handle_discourse_webhook(payload).await;
    }
}
