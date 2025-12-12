use crate::routes;
use crate::state::AppState;
use axum::Router;

pub async fn create_app() -> Router {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let state = AppState::new(db);

    routes::routes().with_state(state)
}
