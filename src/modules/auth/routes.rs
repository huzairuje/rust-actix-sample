use crate::modules::auth::handler;
use actix_web::web;

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(handler::health_checker_handler)
        .service(handler::login_handler);

    conf.service(scope);
}
