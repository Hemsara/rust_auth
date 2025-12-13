use std::env;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Env {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub redis_url: String,
    pub jwt_expiration_days: i64,
    pub auth_session_key_prefix: String,
}

#[derive(Debug)]
pub enum EnvError {
    MissingVar(String),
    InvalidVar(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::MissingVar(var) => write!(f, "Missing environment variable: {}", var),
            EnvError::InvalidVar(var) => write!(f, "Invalid environment variable: {}", var),
        }
    }
}

impl std::error::Error for EnvError {}

impl Env {
    pub fn new() -> Result<Self, EnvError> {
        dotenvy::dotenv().ok();

        let get_var = |key: &str| -> Result<String, EnvError> {
            env::var(key).map_err(|_| EnvError::MissingVar(key.to_string()))
        };

        let database_url = get_var("DATABASE_URL")?;
        let jwt_secret = get_var("SECRET_KEY")?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| EnvError::InvalidVar("PORT".to_string()))?;

        let redis_url = get_var("REDIS_URL")?;
        let jwt_expiration_days = get_var("JWT_EXPIRATION_DAYS")?
            .parse::<i64>()
            .map_err(|_| EnvError::InvalidVar("JWT_EXPIRATION_DAYS".to_string()))?;
        let auth_session_key_prefix = get_var("AUTH_SESSION_KEY_PREFIX")?;

        Ok(Self {
            database_url,
            server_port,
            jwt_secret,
            redis_url,
            jwt_expiration_days,
            auth_session_key_prefix,
        })
    }
}
