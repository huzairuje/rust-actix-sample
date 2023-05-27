use crate::infrastructure::http_lib::Response;
use crate::modules::users::constants;
use crate::modules::users::schema::UserResponse;
use crate::utils::utils;
use crate::{
    modules::users::schema::{CreateUserRequest, UpdateUserRequest},
    modules::users::service,
    AppState,
};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use validator::Validate;

#[get("/health")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple REST API with Rust, SQLX, Postgres,and Actix Web";
    let resp: Response<(), ()> = Response::success(StatusCode::OK, (), MESSAGE);
    return HttpResponse::Ok().json(resp);
}

#[post("")]
pub async fn register_user_handler(
    body: web::Json<CreateUserRequest>,
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

    let req: &CreateUserRequest = &body.0;

    // save the notes
    let result_user: Result<UserResponse, String> =
        service::register_user_service(&data.db, req).await;
    let user = match result_user {
        Ok(user) => user,
        Err(err) => {
            return if err.contains(&constants::USERNAME_ALREADY_EXIST) {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::BAD_REQUEST, &constants::USERNAME_ALREADY_EXIST);
                HttpResponse::BadRequest().json(resp)
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
                return HttpResponse::InternalServerError().json(resp);
            }
        }
    };

    let msg = constants::USER_SUCCESS_SAVED;
    let resp: Response<UserResponse, ()> = Response::success(StatusCode::OK, user, msg);
    return HttpResponse::Ok().json(resp);
}

#[get("/detail")]
pub async fn get_user_detail_handler(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }
    let user_detail: UserResponse =
        match service::get_user_detail_service(&data.db, user_id.unwrap()).await {
            Ok(note) => note,
            Err(err) => {
                return if err.contains(constants::USER_NOT_FOUND) {
                    let resp: Response<(), ()> =
                        Response::error(StatusCode::NOT_FOUND, constants::USER_NOT_FOUND);
                    HttpResponse::NotFound().json(resp)
                } else {
                    let resp: Response<(), ()> = Response::error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        constants::DETAIL_USER_CANT_BE_FETCHED,
                    );
                    return HttpResponse::InternalServerError().json(resp);
                }
            }
        };

    let resp: Response<UserResponse, ()> =
        Response::success(StatusCode::OK, user_detail, constants::USER_FOUND);
    return HttpResponse::Ok().json(resp);
}

#[put("/detail")]
pub async fn update_user_handler(
    body: web::Json<UpdateUserRequest>,
    data: web::Data<AppState>,
    request_http: HttpRequest,
) -> impl Responder {
    //get user_id from authorization header
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), request_http);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }

    //validate the struct from body
    if let Err(errors) = body.validate() {
        let resp = Response::custom(
            StatusCode::BAD_REQUEST,
            StatusCode::BAD_REQUEST.as_str(),
            (),
            errors,
        );
        return HttpResponse::BadRequest().json(resp);
    }

    let req: &UpdateUserRequest = &body.0;
    //update the note
    let user_updated: Result<UserResponse, String> =
        service::update_user_service(&data.db, req, user_id.unwrap()).await;
    let user = match user_updated {
        Ok(note) => note,
        Err(err) => {
            return if err.contains(&constants::USERNAME_ALREADY_EXIST) {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::BAD_REQUEST, &constants::USERNAME_ALREADY_EXIST);
                HttpResponse::BadRequest().json(resp)
            } else if err.contains(&constants::USER_NOT_FOUND) {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::NOT_FOUND, &constants::USER_NOT_FOUND);
                HttpResponse::NotFound().json(resp)
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
                return HttpResponse::InternalServerError().json(resp);
            }
        }
    };

    let msg = constants::USER_SUCCESS_PATCHED;
    let resp: Response<UserResponse, ()> = Response::success(StatusCode::OK, user, msg);
    return HttpResponse::Ok().json(resp);
}

#[delete("/deactivate")]
pub async fn deactivate_user_handler(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    //get user_id from authorization header
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }
    let delete_user = service::deactivate_user_service(&data.db, user_id.unwrap()).await;
    match delete_user {
        Ok(_) => {}
        Err(err_delete_note) => {
            return if err_delete_note.contains(&constants::USER_NOT_FOUND) {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::NOT_FOUND, &constants::USER_NOT_FOUND);
                HttpResponse::NotFound().json(resp)
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err_delete_note.as_str());
                return HttpResponse::InternalServerError().json(resp);
            }
        }
    };

    let msg = constants::USER_SUCCESS_DELETED;
    let resp: Response<(), ()> = Response::success(StatusCode::OK, (), msg);
    return HttpResponse::Ok().json(resp);
}

#[get("/detail/{username}")]
pub async fn get_user_detail_handler_by_username(
    path: web::Path<String>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    //get user_id from authorization header
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }
    let username = path.into_inner();
    let user_detail: UserResponse =
        match service::get_user_by_username_with_user_response(&data.db, &username).await {
            Ok(user) => user,
            Err(err) => {
                return if err.contains(constants::USER_NOT_FOUND) {
                    let resp: Response<(), ()> =
                        Response::error(StatusCode::NOT_FOUND, constants::USER_NOT_FOUND);
                    HttpResponse::NotFound().json(resp)
                } else {
                    let resp: Response<(), ()> = Response::error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        constants::DETAIL_USER_CANT_BE_FETCHED,
                    );
                    return HttpResponse::InternalServerError().json(resp);
                }
            }
        };

    let resp: Response<UserResponse, ()> =
        Response::success(StatusCode::OK, user_detail, constants::USER_FOUND);
    return HttpResponse::Ok().json(resp);
}
