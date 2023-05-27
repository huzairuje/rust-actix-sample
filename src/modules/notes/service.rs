use crate::modules::notes::constants;
use crate::modules::notes::model::{NoteModel, NoteSaveModel, NoteUpdateModel};
use crate::modules::notes::repository;
use crate::modules::notes::schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn get_notes_service(
    pool: &PgPool,
    filter: &FilterOptions,
    user_id: Uuid,
) -> Result<Vec<NoteModel>, String> {
    //unwrap the filter value
    let filter_option: &FilterOptions = filter;
    let limit = filter_option.limit.unwrap_or(10);
    let offset = (filter_option.page.unwrap_or(1) - 1) * limit.clone();

    // Build the query conditionally
    let mut query = String::new();

    // Title condition
    if let Some(title) = &filter_option.title {
        if !title.is_empty() {
            let query_with_param = format!("AND title ILIKE '%{}%'", title);
            query.push_str(&*query_with_param);
        }
    }

    // Content condition
    if let Some(content) = &filter_option.content {
        if !content.is_empty() {
            let query_with_param = format!("AND content ILIKE '%{}%'", content);
            query.push_str(&*query_with_param);
        }
    }

    // Sort condition
    let mut query_order = String::new();
    if let (Some(sort_by), Some(sort_order)) = (&filter_option.sort_by, &filter_option.sort_order) {
        if !sort_by.is_empty() && !sort_order.is_empty() {
            query_order.push_str(" ORDER BY ");
            query_order.push_str(sort_by);
            if sort_order.to_uppercase() == "ASC" || sort_order.to_uppercase() == "DESC" {
                query_order.push(' ');
                query_order.push_str(sort_order);
            }
        }
    } else {
        // Default sorting if sort_by is empty
        query_order.push_str(" ORDER BY created_at DESC");
    }

    let result_note: Result<Vec<NoteModel>, Error> = repository::get_notes_user(
        pool,
        limit as i32,
        offset as i32,
        query,
        query_order,
        user_id,
    )
    .await;

    match result_note {
        Ok(notes) => Ok(notes),
        Err(err) => {
            // Handle the error
            eprintln!("error get list notes: {:?}", err);
            let error_message = constants::NOTE_CANT_BE_FETCHED;
            Err(error_message.parse().unwrap())
        }
    }
}

pub async fn get_note_detail_service(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<NoteModel, String> {
    match repository::get_note_user_by_id(pool, note_id, user_id).await {
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
    body: &CreateNoteSchema,
    user_id: Uuid,
) -> Result<NoteModel, String> {
    let existing_notes: Result<Vec<NoteModel>, Error> =
        repository::get_notes_by_title(pool, body.title.to_string()).await;
    if let Err(err) = existing_notes {
        // Handle the error
        eprintln!("Error getting existing notes: {:?}", err);
        let error_message = constants::EXISTING_NOTE_CANT_BE_FETCHED;
        return Err(error_message.parse().unwrap());
    }
    if let Ok(notes) = existing_notes {
        if !notes.is_empty() {
            // Title already exists, handle the error
            eprintln!("Note with title {:?} already exists", body.title);
            let error_message = constants::NOTE_TITLE_ALREADY_EXIST;
            return Err(error_message.parse().unwrap());
        }
    }

    let note_save_model = NoteSaveModel {
        title: body.title.to_string(),
        content: body.content.to_string(),
        category: body.category.as_ref().map(|c| c.to_string()),
        published: body.published.map(|c| c),
        created_by: Option::from(user_id),
    };

    match repository::save_note(pool, note_save_model.clone()).await {
        Ok(note) => Ok(note),
        Err(err) => {
            // Handle the error
            eprintln!("Error saving note: {:?}", err);
            let error_message = constants::NOTE_CANT_BE_SAVED;
            Err(error_message.parse().unwrap())
        }
    }
}

pub async fn update_note_service(
    pool: &PgPool,
    note_id: Uuid,
    body: &UpdateNoteSchema,
    user_id: Uuid,
) -> Result<NoteModel, String> {
    let note_exist: Result<NoteModel, Error> =
        repository::get_note_user_by_id(pool, note_id, user_id).await;
    let existing_note: NoteModel = match note_exist {
        Ok(notes) => notes.clone(),
        Err(err) => {
            return match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(constants::NOTE_NOT_FOUND.to_string())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(constants::DETAIL_NOTE_CANT_BE_FETCHED.to_string())
                }
            };
        }
    };

    let note_update_model = NoteUpdateModel {
        title: body.title.as_ref().unwrap().to_string(),
        content: body.content.as_ref().unwrap().to_string(),
        category: body.category.as_ref().map(|c| c.to_string()),
        published: body.published.map(|c| c),
        updated_by: Option::from(user_id),
    };

    let note = existing_note;
    match repository::update_note(pool, note_id, note_update_model, note).await {
        Ok(note) => Ok(note),
        Err(err) => {
            // Handle the error
            eprintln!("Error update or patch note: {:?}", err);
            Err(constants::NOTE_CANT_BE_PATCHED.to_string())
        }
    }
}

pub async fn delete_note_service(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<i32, String> {
    let note_exist: Result<NoteModel, Error> =
        repository::get_note_user_by_id(pool, note_id, user_id).await;
    match note_exist {
        Ok(notes) => notes.clone(),
        Err(err) => {
            return match err {
                Error::RowNotFound => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(constants::NOTE_NOT_FOUND.to_string())
                }
                _ => {
                    // Handle the error
                    eprintln!("error get detail notes {:?}", err);
                    Err(constants::DETAIL_NOTE_CANT_BE_FETCHED.to_string())
                }
            };
        }
    };

    match repository::delete_note_user_by_id(pool, note_id, user_id).await {
        Ok(note) => Ok(note),
        Err(err) => {
            // Handle the error
            eprintln!("Error delete note: {:?}", err);
            Err(constants::NOTE_CANT_BE_DELETE.to_string())
        }
    }
}
