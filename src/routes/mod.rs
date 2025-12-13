use crate::middleware::main::auth_middleware;
use crate::state::AppState;
use axum::Router;

use axum::middleware;

mod auth;
mod health;
mod protected;

pub use auth::auth_routes;
pub use health::health_routes;
pub use protected::protected_routes;

pub fn routes(state: AppState) -> Router {
    let protected = protected_routes().route_layer(middleware::from_fn_with_state(
        state.clone(),
        auth_middleware,
    ));

    Router::new()
        .nest("/health", health_routes())
        .nest("/auth", auth_routes())
        .nest("/api", protected)
        .with_state(state)
}
