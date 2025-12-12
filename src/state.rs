use crate::services::env::Env;
use redis::Client;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub env: Env,
    pub redis: Client,
}

impl AppState {
    pub fn new(db: DatabaseConnection, env: Env, redis: Client) -> Self {
        Self { db, env, redis }
    }
}
