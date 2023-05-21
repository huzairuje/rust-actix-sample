use crate::infrastructure::http_lib::Response;
use crate::notes::constants;
use crate::{
    notes::model::NoteModel,
    notes::schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    notes::service,
    AppState,
};
use actix_web::http::StatusCode;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/health")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";
    let resp: Response<(), ()> = Response::success(StatusCode::OK, (), MESSAGE);
    HttpResponse::Ok().json(resp)
}

#[get("/notes")]
pub async fn note_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.clone().unwrap_or(10).clone();
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let final_limit = limit as i32;

    let notes: Result<Vec<NoteModel>, String> =
        service::get_notes_service(&data.db, final_limit, offset as i32).await;
    if let Err(err) = notes {
        let resp: Response<(), ()> =
            Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
        return HttpResponse::InternalServerError().json(resp);
    }

    let list_notes: Vec<NoteModel> = notes.unwrap_or_else(|err| {
        // This closure is not executed when result is Ok
        eprintln!("An error occurred: {:?}", err);
        let msg = constants::NOTE_CANT_BE_FETCHED;
        let resp: Response<Vec<NoteModel>, ()> =
            Response::error(StatusCode::INTERNAL_SERVER_ERROR, msg);
        HttpResponse::InternalServerError().json(resp);
        Vec::new() // Fallback value
    });
    let msg = constants::NOTE_FOUND;
    let resp: Response<Vec<NoteModel>, ()> = Response::success(StatusCode::OK, list_notes, msg);
    HttpResponse::Ok().json(resp)
}

#[post("/notes")]
pub async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    // save the notes
    let result_note: Result<NoteModel, String> = service::save_note_service(&data.db, body).await;
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
                HttpResponse::InternalServerError().json(resp)
            }
        }
    };

    let msg = constants::NOTE_SUCCESS_SAVED;
    let resp: Response<NoteModel, ()> = Response::success(StatusCode::OK, note, msg);
    HttpResponse::Ok().json(resp)
}

#[get("/notes/{id}")]
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
                HttpResponse::InternalServerError().json(resp)
            }
        }
    };

    let resp: Response<NoteModel, ()> =
        Response::success(StatusCode::OK, note_detail, constants::NOTE_FOUND);
    HttpResponse::Ok().json(resp)
}

#[patch("/notes/{id}")]
pub async fn edit_note_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", note_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Note with ID: {} not found", note_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    let now = Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        note_id
    )
    .fetch_one(&data.db)
    .await
    ;

    return match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error","message": message}))
        }
    };
}

#[delete("/notes/{id}")]
pub async fn delete_note_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM notes  WHERE id = $1", note_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Note with ID: {} not found", note_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}
