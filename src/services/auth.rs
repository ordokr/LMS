use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub async fn authenticate(client_id: &str, client_secret: &str, redirect_uri: &str) -> Result<User, Box<dyn std::error::Error>> {
    let auth_url = AuthUrl::new("https://canvas.instructure.com/login/oauth2/auth".to_string())?;
    let token_url = TokenUrl::new("https://canvas.instructure.com/login/oauth2/token".to_string())?;

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client.authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("url:read".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Open this URL in your browser:\n{}", auth_url);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let url = reqwest::Url::parse(input.trim())?;

    let code = AuthorizationCode::new(url.query_pairs().find(|(k, _)| k == "code").unwrap().1.to_string());
    let state = CsrfToken::new(url.query_pairs().find(|(k, _)| k == "state").unwrap().1.to_string());

    assert_eq!(csrf_token.secret(), state.secret());

    let token_response = client.exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request(async_http_client)?;

    let id_token = token_response.id_token().expect("Server did not return an ID token");
    let claims: User = serde_json::from_str(&id_token.to_string())?;

    Ok(claims)
}
