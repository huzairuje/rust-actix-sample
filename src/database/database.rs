use crate::configuration::config::Config;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub async fn initiate_database(cfg: Config) -> Result<Pool<Postgres>, Error> {
    let postgres_url = format!(
        "postgresql://{}:{}@{}:{}/{}?schema={}",
        cfg.postgres_user.unwrap_or("postgres".to_string()),
        cfg.postgres_password.unwrap_or("postgres".to_string()),
        cfg.postgres_host.unwrap_or("localhost".to_string()),
        cfg.postgres_port.unwrap_or("5432".to_string()),
        cfg.postgres_db.unwrap_or("actix_sample".to_string()),
        cfg.postgres_schema.unwrap_or("public".to_string()),
    );

    let max_conn: u32 = cfg
        .postgres_max_connection
        .unwrap_or("10".to_string())
        .parse()
        .expect("Failed to parse postgres max connection");

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&postgres_url)
        .await?;

    // Ping the database to check the connection
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => {
            println!("✅✅✅ Connection to the database is successful!");
            Ok(pool)
        }
        Err(err) => {
            println!("⚠️⚠️⚠️ Failed to connect to the database: {:?}", err);
            Err(err.into())
        }
    }
}
