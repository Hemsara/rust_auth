use crate::handlers::auth_handler;
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth_handler::login))
        .route("/register", post(auth_handler::register))
}
