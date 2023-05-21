use crate::notes::model::NoteModel;
use crate::notes::schema::CreateNoteSchema;
use actix_web::web;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn get_notes(pool: &PgPool, limit: i32, offset: i32) -> Result<Vec<NoteModel>, Error> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(pool)
    .await;

    query_result
}

pub async fn save_note(
    pool: &PgPool,
    body: web::Json<CreateNoteSchema>,
) -> Result<NoteModel, Error> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(pool)
    .await;
    query_result
}

pub async fn get_note_by_id(pool: &PgPool, note_id: Uuid) -> Result<NoteModel, Error> {
    return sqlx::query_as!(NoteModel, "SELECT * FROM notes n where n.id = $1", note_id)
        .fetch_one(pool)
        .await;
}

pub async fn get_notes_by_title(pool: &PgPool, title: String) -> Result<Vec<NoteModel>, Error> {
    return sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes n where n.title ILIKE $1",
        title
    )
    .fetch_all(pool)
    .await;
}
