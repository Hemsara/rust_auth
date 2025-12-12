use axum::Router;
use crate::state::AppState;
use crate::routes;

pub fn create_app() -> Router {
    let state = AppState::new();

    routes::routes()
        .with_state(state)
}
