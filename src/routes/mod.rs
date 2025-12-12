use crate::state::AppState;
use axum::Router;

mod auth;
mod health;

pub use auth::auth_routes;
pub use health::health_routes;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health_routes())
        .nest("/auth", auth_routes())
}
