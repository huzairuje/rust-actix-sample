use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub async fn initiate_database() -> Result<Pool<Postgres>, Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    // Ping the database to check the connection
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => {
            println!("✅ Connection to the database is successful!");
            Ok(pool)
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            Err(err.into())
        }
    }
}
