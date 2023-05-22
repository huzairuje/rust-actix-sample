use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub enable_log: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub postgres_host: Option<String>,
    pub postgres_port: Option<String>,
    pub postgres_user: Option<String>,
    pub postgres_password: Option<String>,
    pub postgres_db: Option<String>,
    pub postgres_schema: Option<String>,
    pub postgres_max_connection: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Config, dotenv::Error> {
        dotenv::dotenv().ok();

        let enable_log = env::var("ENABLE_LOG").ok();
        let host = env::var("HOST").ok();
        let port = env::var("PORT").ok();
        let postgres_host = env::var("POSTGRES_HOST").ok();
        let postgres_port = env::var("POSTGRES_PORT").ok();
        let postgres_user = env::var("POSTGRES_USER").ok();
        let postgres_password = env::var("POSTGRES_PASSWORD").ok();
        let postgres_db = env::var("POSTGRES_DB").ok();
        let postgres_schema = env::var("POSTGRES_SCHEMA").ok();
        let postgres_max_connection = env::var("POSTGRES_MAX_CONNECTION").ok();

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
        })
    }
}
