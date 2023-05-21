mod database;
mod infrastructure;
mod notes;

use actix_cors::Cors;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use infrastructure::http_lib::Response;
use notes::routes as note_routes;
use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

async fn not_found() -> HttpResponse {
    let response: Response<(), ()> = Response::error(StatusCode::NOT_FOUND, "Not Found Routes");
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let pool = database::database::initiate_database()
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to initialize database: {:?}", err);
            std::process::exit(1);
        });

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(note_routes::routes)
            .default_service(web::route().to(not_found))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
