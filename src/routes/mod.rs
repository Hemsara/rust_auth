use axum::Router;
use crate::state::AppState;

mod health;
mod auth;

pub use health::health_routes;
pub use auth::auth_routes;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health_routes())
        .nest("/auth", auth_routes())
}
