use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub enable_log: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub postgres_host: Option<String>,
    pub postgres_port: Option<i32>,
    pub postgres_user: Option<String>,
    pub postgres_password: Option<String>,
    pub postgres_db: Option<String>,
    pub postgres_schema: Option<String>,
    pub postgres_max_connection: Option<i32>,
    pub jwt_secret_key: Option<String>,
    pub access_token_expiry: Option<i32>,
    pub access_token_expiry_unit: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Config, dotenv::Error> {
        dotenv::dotenv().ok();
        let enable_log = env::var("ENABLE_LOG").ok();
        let host = env::var("HOST").ok();
        let port = env::var("PORT")
            .ok()
            .map(|p| p.parse::<i32>().ok())
            .flatten();
        let postgres_host = env::var("POSTGRES_HOST").ok();
        let postgres_port = env::var("POSTGRES_PORT")
            .ok()
            .map(|p| p.parse::<i32>().ok())
            .flatten();
        let postgres_user = env::var("POSTGRES_USER").ok();
        let postgres_password = env::var("POSTGRES_PASSWORD").ok();
        let postgres_db = env::var("POSTGRES_DB").ok();
        let postgres_schema = env::var("POSTGRES_SCHEMA").ok();
        let postgres_max_connection = env::var("POSTGRES_MAX_CONNECTION")
            .ok()
            .map(|p| p.parse::<i32>().ok())
            .flatten();
        let jwt_secret_key = env::var("JWT_SECRET_KEY").ok();
        let access_token_expiry = env::var("ACCESS_TOKEN_EXPIRY")
            .ok()
            .map(|p| p.parse::<i32>().ok())
            .flatten();
        let access_token_expiry_unit = env::var("ACCESS_TOKEN_EXPIRY_UNIT").ok();

        Ok(Config {
            enable_log,
            host,
            port,
            postgres_host,
            postgres_port,
            postgres_user,
            postgres_password,
            postgres_db,
            postgres_schema,
            postgres_max_connection,
            jwt_secret_key,
            access_token_expiry,
            access_token_expiry_unit,
        })
    }
}
