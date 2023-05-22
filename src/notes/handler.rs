use crate::infrastructure::http_lib::Response;
use crate::infrastructure::validator;
use crate::infrastructure::validator::ErrorResponse;
use crate::notes::constants;
use crate::{
    notes::model::NoteModel,
    notes::schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    notes::service,
    AppState,
};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

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
) -> impl Responder {
    //get list note
    let notes: Result<Vec<NoteModel>, String> =
        service::get_notes_service(&data.db, &filter.into_inner()).await;
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
    let resp: Response<Vec<NoteModel>, ()> = Response::success(StatusCode::OK, list_notes, msg);
    return HttpResponse::Ok().json(resp);
}

#[post("")]
pub async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    //validate the struct from body
    let req: &CreateNoteSchema = &body.0;
    let error_validate: Vec<ErrorResponse> = validator::validate_struct(req);
    if !error_validate.is_empty() {
        let resp: Response<(), Vec<ErrorResponse>> = Response::custom(
            StatusCode::BAD_REQUEST,
            StatusCode::BAD_REQUEST.as_str(),
            (),
            error_validate,
        );
        return HttpResponse::BadRequest().json(resp);
    }
    // save the notes
    let result_note: Result<NoteModel, String> = service::save_note_service(&data.db, req).await;
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
) -> impl Responder {
    let note_id_str = path.into_inner();

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

    let note_detail: NoteModel = match service::get_note_detail_service(&data.db, note_id).await {
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
) -> impl Responder {
    let note_id_str = path.into_inner();

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
    let req: &UpdateNoteSchema = &body.0;
    let error_validate: Vec<ErrorResponse> = validator::validate_struct(req);
    if !error_validate.is_empty() {
        let resp: Response<(), Vec<ErrorResponse>> = Response::custom(
            StatusCode::BAD_REQUEST,
            StatusCode::BAD_REQUEST.as_str(),
            (),
            error_validate,
        );
        return HttpResponse::BadRequest().json(resp);
    }

    let note_updated: Result<NoteModel, String> =
        service::update_note_service(&data.db, note_id, req).await;
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
) -> impl Responder {
    let note_id = path.into_inner();
    let delete_note = service::delete_note_service(&data.db, note_id).await;
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
    return HttpResponse::Ok().json(resp)
}
