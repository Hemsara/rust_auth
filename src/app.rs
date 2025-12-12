use crate::routes;
use crate::services::env::Env;
use crate::state::AppState;
use axum::Router;

pub async fn create_app() -> Router {
    let env = Env::new().expect("Failed to load environment variables");

    let db = sea_orm::Database::connect(&env.database_url)
        .await
        .expect("Failed to connect to the database");

    let state = AppState::new(db, env);
    routes::routes().with_state(state)
}
