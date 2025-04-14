use crate::services::auth;

#[tokio::test]
async fn test_authentication() {
    let client_id = "your_client_id";
    let client_secret = "your_client_secret";
    let redirect_uri = "http://localhost:8080/callback";

    match auth::authenticate(client_id, client_secret, redirect_uri).await {
        Ok(user) => println!("User authenticated: {:?}", user),
        Err(e) => panic!("Authentication failed: {}", e),
    }
}
