use axum::Router;
use crate::api::integration::integration_routes;
use crate::app_state::AppState;

pub async fn setup_test_app() -> (Router, AppState) {
    // Setup logic for the test application
    let router = Router::new().nest("/integration", integration_routes());
    let app_state = AppState::default(); // Example app state

    (router, app_state)
}