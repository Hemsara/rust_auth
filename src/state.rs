use sea_orm::DatabaseConnection;
use crate::services::env::Env;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub env: Env,
}

impl AppState {
    pub fn new(db: DatabaseConnection, env: Env) -> Self {
        Self { db , env}
    }
}
