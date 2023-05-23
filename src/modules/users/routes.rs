use crate::modules::users::handler;
use actix_web::web;

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(handler::health_checker_handler)
        .service(handler::register_user_handler)
        .service(handler::get_user_detail_handler)
        .service(handler::update_user_handler)
        .service(handler::deactivate_user_handler);

    conf.service(scope);
}
