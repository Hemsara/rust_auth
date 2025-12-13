use axum::{routing::get, Router};

use crate::state::AppState;

pub fn protected_routes() -> Router<AppState> {
    Router::new().route("/me", get(auth))
}

async fn auth() -> &'static str {
    println!("Protected route accessed");
    "Protected route accessed"
}
