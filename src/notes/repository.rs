use crate::notes::model::NoteModel;
use crate::notes::schema::{CreateNoteSchema, UpdateNoteSchema};
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn get_notes(
    pool: &PgPool,
    limit: i32,
    offset: i32,
    query: String,
    query_order: String,
) -> Result<Vec<NoteModel>, Error> {
    //build all the query
    let query_with_order = format!(
        "SELECT * FROM notes WHERE deleted_at IS NULL {} {} LIMIT $1 OFFSET $2",
        query, query_order
    );

    let query_result = sqlx::query_as::<_, NoteModel>(&query_with_order)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await;

    query_result
}

pub async fn save_note(pool: &PgPool, body: &CreateNoteSchema) -> Result<NoteModel, Error> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title,
        body.content,
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(pool)
    .await;
    query_result
}

pub async fn update_note(
    pool: &PgPool,
    note_id: Uuid,
    body: &UpdateNoteSchema,
    note: NoteModel,
) -> Result<NoteModel, Error> {
    let now = Utc::now();
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
        .fetch_one(pool)
        .await;
    query_result
}

pub async fn get_note_by_id(pool: &PgPool, note_id: Uuid) -> Result<NoteModel, Error> {
    return sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes n where n.deleted_at is null and n.id = $1",
        note_id
    )
    .fetch_one(pool)
    .await;
}

pub async fn get_notes_by_title(pool: &PgPool, title: String) -> Result<Vec<NoteModel>, Error> {
    return sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes n where n.deleted_at is not null and n.title ILIKE $1",
        title
    )
    .fetch_all(pool)
    .await;
}

pub async fn delete_note_by_id(pool: &PgPool, note_id: Uuid) -> Result<i32, Error> {
    let now = Utc::now();
    let rows_affected = sqlx::query!(
        "UPDATE notes SET deleted_at = $1 WHERE id = $2",
        now,
        note_id,
    )
    .execute(pool)
    .await?;

    Ok(rows_affected.rows_affected() as i32)
}
