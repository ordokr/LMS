pub mod handlers;
pub mod jwt;
pub mod jwt_service;
pub mod middleware;
pub mod refresh_handler;
pub mod routes;
pub mod sso;
pub mod sso_handler;

// Re-export important items for easier imports
pub use routes::auth_routes;
pub use jwt_service::{JwtService, Claims};
pub use middleware::auth_middleware;
