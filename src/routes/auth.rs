use axum::{routing::get, Router};
use crate::handlers::auth_handler;
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/", get(auth_handler::login))
}
