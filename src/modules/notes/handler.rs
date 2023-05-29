use crate::infrastructure::http_lib::{Pagination, Response};
use crate::infrastructure::pagination::PaginationQuery;
use crate::modules::notes::constants;
use crate::utils::utils;
use crate::{
    modules::notes::model::NoteModel,
    modules::notes::schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    modules::notes::service,
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

#[get("")]
pub async fn note_list_handler(
    filter: web::Query<FilterOptions>,
    data: web::Data<AppState>,
    req: HttpRequest,
    paginated: web::Query<PaginationQuery>,
) -> impl Responder {
    //get user_id from authorization token
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }
    //get list note
    let (notes, total_count) =
        service::get_notes_service(&data.db, &filter.into_inner(), user_id.unwrap()).await;
    if let Err(err) = notes {
        let resp: Response<(), ()> =
            Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
        return HttpResponse::InternalServerError().json(resp);
    }

    let list_notes: Vec<NoteModel> = notes.unwrap_or_else(|err| {
        // This closure is not executed when the result is Ok
        eprintln!("An error occurred: {:?}", err);
        let msg = constants::NOTE_CANT_BE_FETCHED;
        let resp: Response<Vec<NoteModel>, ()> =
            Response::error(StatusCode::INTERNAL_SERVER_ERROR, msg);
        HttpResponse::InternalServerError().json(resp);
        Vec::new() // Fallback value
    });
    let msg = constants::NOTE_FOUND;
    let pg: PaginationQuery = paginated.0;
    let resp: Pagination<Vec<NoteModel>> = Pagination::success(pg, msg, list_notes, total_count);
    return HttpResponse::Ok().json(resp);
}

#[post("")]
pub async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    //get user_id from authorization token
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
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

    let req: &CreateNoteSchema = &body.0;

    // save the notes
    let result_note: Result<NoteModel, String> =
        service::save_note_service(&data.db, req, user_id.unwrap()).await;
    let note = match result_note {
        Ok(note) => note,
        Err(err) => {
            return if err.contains(&constants::NOTE_TITLE_ALREADY_EXIST) {
                let resp: Response<(), ()> = Response::error(
                    StatusCode::BAD_REQUEST,
                    &constants::NOTE_TITLE_ALREADY_EXIST,
                );
                HttpResponse::BadRequest().json(resp)
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
                return HttpResponse::InternalServerError().json(resp);
            }
        }
    };

    let msg = constants::NOTE_SUCCESS_SAVED;
    let resp: Response<NoteModel, ()> = Response::success(StatusCode::OK, note, msg);
    return HttpResponse::Ok().json(resp);
}

#[get("/{id}")]
pub async fn get_note_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let note_id_str = path.into_inner();

    //get user_id from authorization token
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }

    // Attempt to parse the UUID from the path
    let note_id = match uuid::Uuid::parse_str(&note_id_str) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("match uuid::Uuid::parse_str, got err : {:?}", err);
            let msg = constants::PARAMETER_NOTE_ID_INVALID;
            let error_response: Response<(), ()> = Response::error(StatusCode::BAD_REQUEST, msg);
            return HttpResponse::BadRequest().json(error_response);
        }
    };

    let note_detail: NoteModel =
        match service::get_note_detail_service(&data.db, note_id, user_id.unwrap()).await {
            Ok(note) => note,
            Err(err) => {
                return if err.contains(constants::NOTE_NOT_FOUND) {
                    let resp: Response<(), ()> =
                        Response::error(StatusCode::NOT_FOUND, constants::NOTE_NOT_FOUND);
                    HttpResponse::NotFound().json(resp)
                } else {
                    let resp: Response<(), ()> = Response::error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        constants::DETAIL_NOTE_CANT_BE_FETCHED,
                    );
                    return HttpResponse::InternalServerError().json(resp);
                }
            }
        };

    let resp: Response<NoteModel, ()> =
        Response::success(StatusCode::OK, note_detail, constants::NOTE_FOUND);
    return HttpResponse::Ok().json(resp);
}

#[put("/{id}")]
pub async fn edit_note_handler(
    path: web::Path<String>,
    body: web::Json<UpdateNoteSchema>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let note_id_str = path.into_inner();

    //get user_id from authorization token
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }

    // Attempt to parse the UUID from the path
    let note_id = match uuid::Uuid::parse_str(&note_id_str) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("match uuid::Uuid::parse_str, got err : {:?}", err);
            let msg = constants::PARAMETER_NOTE_ID_INVALID;
            let error_response: Response<(), ()> = Response::error(StatusCode::BAD_REQUEST, msg);
            return HttpResponse::BadRequest().json(error_response);
        }
    };

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

    let req: &UpdateNoteSchema = &body.0;
    //update the note
    let note_updated: Result<NoteModel, String> =
        service::update_note_service(&data.db, note_id, req, user_id.unwrap()).await;
    let note = match note_updated {
        Ok(note) => note,
        Err(err) => {
            return if err.contains(&constants::NOTE_TITLE_ALREADY_EXIST) {
                let resp: Response<(), ()> = Response::error(
                    StatusCode::BAD_REQUEST,
                    &constants::NOTE_TITLE_ALREADY_EXIST,
                );
                HttpResponse::BadRequest().json(resp)
            } else if err.contains(&constants::NOTE_NOT_FOUND) {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::NOT_FOUND, &constants::NOTE_NOT_FOUND);
                HttpResponse::NotFound().json(resp)
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
                return HttpResponse::InternalServerError().json(resp);
            }
        }
    };

    let msg = constants::NOTE_SUCCESS_PATCHED;
    let resp: Response<NoteModel, ()> = Response::success(StatusCode::OK, note, msg);
    return HttpResponse::Ok().json(resp);
}

#[delete("/{id}")]
pub async fn delete_note_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let note_id = path.into_inner();

    //get user_id from authorization token
    let user_id = utils::get_current_user_uuid_from_jwt(&data.cfg.clone(), req);
    if let Err(err) = user_id {
        let resp: Response<(), ()> = Response::error(StatusCode::UNAUTHORIZED, err.as_str());
        return HttpResponse::Unauthorized().json(resp);
    }

    let delete_note = service::delete_note_service(&data.db, note_id, user_id.unwrap()).await;
    match delete_note {
        Ok(_) => {}
        Err(err_delete_note) => {
            return if err_delete_note.contains(&constants::NOTE_NOT_FOUND) {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::NOT_FOUND, &constants::NOTE_NOT_FOUND);
                HttpResponse::NotFound().json(resp)
            } else {
                let resp: Response<(), ()> =
                    Response::error(StatusCode::INTERNAL_SERVER_ERROR, err_delete_note.as_str());
                return HttpResponse::InternalServerError().json(resp);
            }
        }
    };

    let msg = constants::NOTE_SUCCESS_DELETED;
    let resp: Response<(), ()> = Response::success(StatusCode::OK, (), msg);
    return HttpResponse::Ok().json(resp);
}
