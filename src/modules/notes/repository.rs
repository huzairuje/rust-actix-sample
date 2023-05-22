use crate::modules::notes::model::NoteModel;
use crate::modules::notes::schema::{CreateNoteSchema, UpdateNoteSchema};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;
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
    let query = "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *";
    let query_result = sqlx::query_as::<_, NoteModel>(&query)
        .bind(body.title.as_str())
        .bind(body.content.as_str())
        .bind(body.category.to_owned().unwrap_or("".to_string()))
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
    let query = "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *";
    let query_result = sqlx::query_as::<_, NoteModel>(query)
        .bind(body.title.to_owned().unwrap_or(note.title))
        .bind(body.content.to_owned().unwrap_or(note.content))
        .bind(body.category.to_owned().unwrap_or(note.category.unwrap()))
        .bind(body.published.unwrap_or(note.published.unwrap()))
        .bind(now)
        .bind(note_id)
        .fetch_one(pool)
        .await;
    query_result
}

pub async fn get_note_by_id(pool: &PgPool, note_id: Uuid) -> Result<NoteModel, Error> {
    return sqlx::query_as::<_, NoteModel>(
        "SELECT * FROM notes n where n.deleted_at is null and n.id = $1",
    )
    .bind(note_id)
    .fetch_one(pool)
    .await;
}

pub async fn get_notes_by_title(pool: &PgPool, title: String) -> Result<Vec<NoteModel>, Error> {
    return sqlx::query_as::<_, NoteModel>(
        "SELECT * FROM notes n where n.deleted_at is not null and n.title ILIKE $1",
    )
    .bind(title)
    .fetch_all(pool)
    .await;
}

pub async fn delete_note_by_id(pool: &PgPool, note_id: Uuid) -> Result<i32, Error> {
    let now: DateTime<Utc> = Utc::now();
    let rows_affected: PgQueryResult =
        sqlx::query::<_>("UPDATE notes SET deleted_at = $1 WHERE id = $2")
            .bind(now)
            .bind(note_id)
            .execute(pool)
            .await?;

    Ok(rows_affected.rows_affected() as i32)
}
