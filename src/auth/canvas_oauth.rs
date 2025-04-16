// src/auth/canvas_oauth.rs

use std::error::Error;
use oauth2::{
    AuthorizationCode,
    ClientId,
    ClientSecret,
    AuthUrl,
    TokenUrl,
    RedirectUrl,
    CsrfToken,
    PkceCodeChallenge,
    PkceCodeVerifier,
    OAuth2BasicErrorResponseType,
    TokenResponse,
    reqwest::async_http_client,
    AuthorizationRequest,
    RevocationUrl,
    EmptyExtraTokenFields,
    StandardTokenResponse,
    StandardRevocableToken,
    StandardErrorResponse,
};
use oauth2::basic::{
    BasicClient,
    BasicTokenType,
    BasicTokenResponse,
    BasicErrorResponse,
};
use crate::config::get_config;

pub fn get_canvas_oauth_client() -> BasicClient {
    let config = get_config();

    let client_id = ClientId::new(config.canvas_oauth_client_id.clone());
    let client_secret = ClientSecret::new(config.canvas_oauth_client_secret.clone());
    let authorize_url = AuthUrl::new(config.canvas_oauth_authorize_url.clone()).unwrap();
    let token_url = TokenUrl::new(config.canvas_oauth_token_url.clone()).unwrap();

    BasicClient::new(
        client_id,
        Some(client_secret),
        authorize_url,
        Some(token_url)
    )
    .set_redirect_uri(RedirectUrl::new(config.canvas_oauth_redirect_url.clone()).unwrap())
}

pub async fn handle_canvas_oauth_callback(code: String, state: String) -> Result<BasicTokenResponse, Box<dyn Error>> {
    let client = get_canvas_oauth_client();

    // Exchange the code for a token.
    let token_response = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?;

    Ok(token_response)
}