use crate::infrastructure::http_lib::Response;
use crate::modules::auth::constants as auth_constants;
use crate::modules::auth::model::AuthModel;
use crate::modules::auth::schema::LoginRequest;
use crate::modules::users::constants as user_constants;
use crate::{modules::auth::service as auth_service, AppState};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
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
            return if err.contains(&user_constants::USER_NOT_FOUND) {
                let resp: Response<(), ()> = Response::error(
                    StatusCode::BAD_REQUEST,
                    &user_constants::USERNAME_ALREADY_EXIST,
                );
                HttpResponse::BadRequest().json(resp)
            } else if err.contains(&auth_constants::USERNAME_AND_PASSWORD_FAILED) {
                let resp: Response<(), ()> = Response::error(
                    StatusCode::BAD_REQUEST,
                    &auth_constants::USERNAME_AND_PASSWORD_FAILED,
                );
                return HttpResponse::BadRequest().json(resp);
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
                return HttpResponse::InternalServerError().json(resp);
            };
        }
    };

    let msg = auth_constants::LOGIN_SUCCESS;
    let resp: Response<AuthModel, ()> = Response::success(StatusCode::OK, auth, msg);
    return HttpResponse::Ok().json(resp);
}

#[post("/refresh-token")]
pub async fn refresh_token_handler(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    // login the user
    let auth: Result<AuthModel, String> =
        auth_service::refresh_token_service(&data.db, data.cfg.clone(), req).await;
    let auth = match auth {
        Ok(auth) => auth,
        Err(err) => {
            return if err.contains(&user_constants::USER_NOT_FOUND) {
                let resp: Response<(), ()> = Response::error(
                    StatusCode::BAD_REQUEST,
                    &user_constants::USERNAME_ALREADY_EXIST,
                );
                HttpResponse::BadRequest().json(resp)
            } else if err.contains(&auth_constants::USERNAME_AND_PASSWORD_FAILED) {
                let resp: Response<(), ()> = Response::error(
                    StatusCode::BAD_REQUEST,
                    &auth_constants::USERNAME_AND_PASSWORD_FAILED,
                );
                return HttpResponse::BadRequest().json(resp);
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
                return HttpResponse::InternalServerError().json(resp);
            };
        }
    };

    let msg = auth_constants::LOGIN_SUCCESS;
    let resp: Response<AuthModel, ()> = Response::success(StatusCode::OK, auth, msg);
    return HttpResponse::Ok().json(resp);
}
