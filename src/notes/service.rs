use crate::notes::constants;
use crate::notes::model::NoteModel;
use crate::notes::repository;
use crate::notes::schema::CreateNoteSchema;
use actix_web::web;
use sqlx::{Error, PgPool};

pub async fn get_notes_service(
    pool: &PgPool,
    limit: i32,
    offset: i32,
) -> Result<Vec<NoteModel>, String> {
    match repository::get_notes(pool, limit, offset).await {
        Ok(notes) => Ok(notes),
        Err(err) => {
            // Handle the error
            eprintln!("error get list notes {:?}", err);
            let error_message = constants::NOTE_CANT_BE_FETCHED;
            Err(error_message.parse().unwrap())
        }
    }
}

pub async fn get_note_detail_service(
    pool: &PgPool,
    note_id: uuid::Uuid,
) -> Result<NoteModel, String> {
    match repository::get_note_by_id(pool, note_id).await {
        Ok(note) => Ok(note),
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = constants::NOTE_NOT_FOUND;
                    Err(error_message.parse().unwrap())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    let error_message = constants::DETAIL_NOTE_CANT_BE_FETCHED;
                    Err(error_message.parse().unwrap())
                }
            }
        }
    }
}

pub async fn save_note_service(
    pool: &PgPool,
    body: web::Json<CreateNoteSchema>,
) -> Result<NoteModel, String> {
    let existing_notes: Result<Vec<NoteModel>, Error> =
        repository::get_notes_by_title(pool, body.title.to_string()).await;

    let list_existing_note: Vec<NoteModel> = match existing_notes {
        Ok(notes) => notes.clone(),
        Err(err) => {
            // Handle the error
            eprintln!("Error getting existing notes: {:?}", err);
            let error_message = constants::EXISTING_NOTE_CANT_BE_FETCHED;
            return Err(error_message.parse().unwrap());
        }
    };

    if !list_existing_note.is_empty() {
        // Title already exists, handle the error
        eprintln!("Note with title {:?} already exists", body.title);
        let error_message = constants::NOTE_TITLE_ALREADY_EXIST;
        return Err(error_message.parse().unwrap());
    }

    match repository::save_note(pool, body).await {
        Ok(note) => Ok(note),
        Err(err) => {
            // Handle the error
            eprintln!("Error saving note: {:?}", err);
            let error_message = constants::NOTE_CANT_BE_SAVED;
            Err(error_message.parse().unwrap())
        }
    }
}
