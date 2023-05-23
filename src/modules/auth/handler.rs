use crate::infrastructure::http_lib::Response;
use crate::modules::auth::constants as auth_constants;
use crate::modules::auth::model::AuthModel;
use crate::modules::auth::schema::LoginRequest;
use crate::{modules::auth::service as auth_service, AppState};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};
use validator::Validate;

#[get("/health")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple REST API with Rust, SQLX, Postgres,and Actix Web";
    let resp: Response<(), ()> = Response::success(StatusCode::OK, (), MESSAGE);
    return HttpResponse::Ok().json(resp);
}

#[post("/login")]
pub async fn login_handler(
    body: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    //validate the struct from body
    if let Err(errors) = body.validate() {
        let resp = Response::custom(
            StatusCode::BAD_REQUEST,
            StatusCode::BAD_REQUEST.to_string().as_str(),
            (),
            errors,
        );
        return HttpResponse::BadRequest().json(resp);
    }

    let req: &LoginRequest = &body.0;

    // login the user
    let auth: Result<AuthModel, String> =
        auth_service::login_service(&data.db, data.cfg.clone(), req).await;
    let auth = match auth {
        Ok(auth) => auth,
        Err(err) => {
            // TODO handling error from the login service
            // return if err.contains(&constants::USERNAME_ALREADY_EXIST) {
            //     let resp: Response<(), ()> =
            //         Response::error(StatusCode::BAD_REQUEST, &constants::USERNAME_ALREADY_EXIST);
            //     HttpResponse::BadRequest().json(resp)
            // } else {
            // }
            let resp: Response<(), ()> =
                Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let msg = auth_constants::LOGIN_SUCCESS;
    let resp: Response<AuthModel, ()> = Response::success(StatusCode::OK, auth, msg);
    return HttpResponse::Ok().json(resp);
}
