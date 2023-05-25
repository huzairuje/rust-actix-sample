use crate::modules::auth::handler;
use actix_web::web;

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(handler::health_checker_handler)
        .service(handler::login_handler)
        .service(handler::refresh_token_handler);

    conf.service(scope);
}
