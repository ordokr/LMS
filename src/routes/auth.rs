// src/routes/auth.rs

use actix_web::{get, post, web, HttpResponse, Responder, Error, Router};
use serde::{Deserialize, Serialize};
use crate::auth::canvas_oauth::{get_canvas_oauth_client, handle_canvas_oauth_callback};
use crate::app_state::AppState;
use crate::services;
use crate::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    message: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(AuthResponse { message: "Hello from auth!".to_string() })
}

#[get("/canvas/login")]
pub async fn canvas_login() -> impl Responder {
    let client = get_canvas_oauth_client();

    let (authorize_url, csrf_state) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .into_login_url();

    HttpResponse::Found()
        .append_header(("Location", authorize_url.to_string()))
        .finish()
}

#[get("/canvas/callback")]
pub async fn canvas_callback(query: web::Query<CanvasCallbackParams>) -> Result<HttpResponse, Error> {
    let code = query.code.clone();
    let state = query.state.clone();

    match handle_canvas_oauth_callback(code, state).await {
        Ok(token_response) => {
            // TODO: Store the token and associate it with the user
            Ok(HttpResponse::Ok().json(AuthResponse { message: "Canvas OAuth successful!".to_string() }))
        }
        Err(e) => {
            // TODO: Handle the error appropriately
            Ok(HttpResponse::InternalServerError().json(AuthResponse { message: format!("Canvas OAuth failed: {}", e).to_string() }))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CanvasCallbackParams {
    code: String,
    state: String,
}

#[get("/discourse/sso")]
pub async fn discourse_sso(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // TODO: Get the current user from the session or database
    let user = models::user::User {
        id: "user123".to_string(),
        email: "test@example.com".to_string(),
        name: Some("Test User".to_string()),
        display_name: Some("Test User".to_string()),
        admin: Some(false),
        roles: Some(vec!["student".to_string()]),
    };

    let jwt_auth_provider = services::integration::auth::jwt_provider::JwtAuthProvider::new(None)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let sso_url = jwt_auth_provider.generate_sso_url(&services::integration::auth::jwt_provider::User {
        id: user.id,
        email: user.email,
        name: user.name,
        display_name: user.display_name,
        admin: user.admin,
        roles: user.roles,
    }, None).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Found()
        .append_header(("Location", sso_url))
        .finish())
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hello))
        .route("/canvas/login", get(canvas_login))
        .route("/canvas/callback", get(canvas_callback))
        .route("/discourse/sso", get(discourse_sso))
}