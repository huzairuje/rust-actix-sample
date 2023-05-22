use crate::modules::notes::handler;
use actix_web::web;

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/notes")
        .service(handler::health_checker_handler)
        .service(handler::note_list_handler)
        .service(handler::create_note_handler)
        .service(handler::get_note_handler)
        .service(handler::edit_note_handler)
        .service(handler::delete_note_handler);

    conf.service(scope);
}
