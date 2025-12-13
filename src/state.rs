use crate::services::env::Env;
use crate::services::jwt::JwtService;
use redis::Client;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub env: Env,
    pub redis: Client,
    pub jwt: JwtService,
}

impl AppState {
    pub fn new(db: DatabaseConnection, env: Env, redis: Client, jwt: JwtService) -> Self {
        Self {
            db,
            env,
            redis,
            jwt,
        }
    }
}
