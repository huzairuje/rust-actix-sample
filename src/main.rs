mod configuration;
mod database;
mod infrastructure;
mod middleware;
mod modules;
mod routes;
mod utils;

use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use configuration::config::Config;
use infrastructure::http_lib::Response;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::{Arc, Mutex};

pub struct AppState {
    db: Pool<Postgres>,
    cfg: Config,
}

async fn not_found() -> HttpResponse {
    let response: Response<(), ()> = Response::error(StatusCode::NOT_FOUND, "Not Found Routes");
    return HttpResponse::NotFound().json(response);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file into the Config struct
    let config_from_env =
        Config::from_env().expect("⚠️⚠️⚠️ Failed to load .env file on the root project");

    // Set the config from env onto an Arc Mutex
    let config_arc: Arc<Mutex<Config>> = Arc::new(Mutex::new(config_from_env));

    // Clone the Config struct and clone the host field separately
    let config = config_arc.lock().unwrap().clone();
    let host_url: String = config.host.clone().unwrap_or("localhost".to_string());
    let port: u16 = config.port.unwrap_or(8080) as u16;

    // Enable log if it is enabled in the .env file
    if let Some(config_enable_log) = config.enable_log.as_ref() {
        if config_enable_log == "true" {
            if env::var_os("RUST_LOG").is_none() {
                env::set_var("RUST_LOG", "actix_web=info");
            }
            env_logger::init();
        }
    } else {
        println!("⚠️⚠️⚠️ Can't load logging on std, Failed to load env var ENABLE_LOG");
    }

    let pool = database::database::initiate_database(config.clone())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to initialize database: {:?}", err);
            std::process::exit(1);
        });

    println!("🚀🚀🚀 Server starting!");

    HttpServer::new(move || {
        let cors_enable = middleware::cors::enable_cors();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                cfg: config.clone(),
            }))
            .configure(routes::routes::initiate_routes)
            .default_service(web::route().to(not_found))
            .wrap(cors_enable)
            .wrap(Logger::default())
    })
    .bind((&*host_url, port))?
    .run()
    .await
}
