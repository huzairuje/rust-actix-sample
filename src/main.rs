mod configuration;
mod cors;
mod database;
mod infrastructure;
mod modules;
mod routes;

use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use configuration::config::Config;
use infrastructure::http_lib::Response;
use sqlx::{Pool, Postgres};
use std::env;

pub struct AppState {
    db: Pool<Postgres>,
}

async fn not_found() -> HttpResponse {
    let response: Response<(), ()> = Response::error(StatusCode::NOT_FOUND, "Not Found Routes");
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load .env file into structs config
    let config = Config::from_env().expect("⚠️⚠️⚠️ Failed to load .env file on the root project");
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

    // parse port and host from config .env
    let port: u16 = config
        .port
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Failed to parse port");
    let host_url: String = config.host.unwrap_or("localhost".to_string());

    println!("🚀🚀🚀 Server starting!");

    HttpServer::new(move || {
        let cors_enable = cors::cors::enable_cors();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(routes::routes::initiate_routes)
            .default_service(web::route().to(not_found))
            .wrap(cors_enable)
            .wrap(Logger::default())
    })
    .bind((host_url, port))?
    .run()
    .await
}
