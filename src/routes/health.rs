use axum::{routing::get, Router};
use crate::handlers::health_handler;
use crate::state::AppState;

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/", get(health_handler::health_check))
}
