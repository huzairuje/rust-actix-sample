use crate::modules::notes::routes as note_routes;
use actix_web::web;

pub fn initiate_routes(conf: &mut web::ServiceConfig) {
    //register all the routes here
    //example
    //.configure(user_route::routes);
    //.configure(auth_route::routes);

    //register note routes
    let scope = web::scope("/api/v1").configure(note_routes::routes);

    conf.service(scope);
}
