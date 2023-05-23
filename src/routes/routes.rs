use crate::modules::auth::routes as auth_routes;
use crate::modules::notes::routes as note_routes;
use crate::modules::users::routes as user_routes;
use actix_web::web;

pub fn initiate_routes(conf: &mut web::ServiceConfig) {
    //register all the routes here
    let scope = web::scope("/api/v1")
        .configure(auth_routes::routes)
        .configure(user_routes::routes)
        .configure(note_routes::routes);

    conf.service(scope);
}
