use crate::modules::users::model::{UserModel, UserSaveModel, UserUpdateModel};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn save_user(pool: &PgPool, request: &UserSaveModel) -> Result<UserModel, Error> {
    let query = "INSERT INTO users (username,password,fullname,email,phone_number) VALUES ($1, $2, $3, $4, $5) RETURNING *";
    let query_result = sqlx::query_as::<_, UserModel>(&query)
        .bind(request.username.as_str())
        .bind(request.password.as_str())
        .bind(request.fullname.as_ref().unwrap_or(&"".to_string()))
        .bind(request.email.as_ref().unwrap_or(&"".to_string()))
        .bind(request.phone_number.as_ref().unwrap_or(&"".to_string()))
        .fetch_one(pool)
        .await;

    query_result
}

pub async fn update_user(
    pool: &PgPool,
    user_id: Uuid,
    request: &UserUpdateModel,
    user: UserModel,
) -> Result<UserModel, Error> {
    let now = Utc::now();
    let query = "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *";
    let query_result = sqlx::query_as::<_, UserModel>(query)
        .bind(request.username.as_ref().unwrap_or(&user.username))
        .bind(request.password.as_ref().unwrap_or(&user.username))
        .bind(
            request
                .fullname
                .as_ref()
                .unwrap_or(&user.fullname.as_ref().unwrap()),
        )
        .bind(
            request
                .email
                .as_ref()
                .unwrap_or(&user.email.as_ref().unwrap()),
        )
        .bind(
            request
                .phone_number
                .as_ref()
                .unwrap_or(&user.phone_number.as_ref().unwrap()),
        )
        .bind(now)
        .bind(user_id)
        .fetch_one(pool)
        .await;
    query_result
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<UserModel, Error> {
    let query = "SELECT * FROM users u where u.deleted_at is null and u.id = $1";
    return sqlx::query_as::<_, UserModel>(query)
        .bind(user_id)
        .fetch_one(pool)
        .await;
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: String,
) -> Result<Vec<UserModel>, Error> {
    let query = "SELECT * FROM users u where u.deleted_at is null and u.username = $1";
    return sqlx::query_as::<_, UserModel>(query)
        .bind(username)
        .fetch_all(pool)
        .await;
}

pub async fn get_user_single_by_username(
    pool: &PgPool,
    username: String,
) -> Result<UserModel, Error> {
    let query = "SELECT * FROM users u where u.deleted_at is null and u.username = $1";
    return sqlx::query_as::<_, UserModel>(query)
        .bind(username)
        .fetch_one(pool)
        .await;
}

pub async fn delete_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<i32, Error> {
    let now: DateTime<Utc> = Utc::now();
    let query = "UPDATE users SET deleted_at = $1 WHERE id = $2";
    let rows_affected: PgQueryResult = sqlx::query::<_>(query)
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(rows_affected.rows_affected() as i32)
}

pub async fn get_all_user(pool: &PgPool) -> Result<Vec<UserModel>, Error> {
    let query = "SELECT * FROM users u where u.deleted_at is null";
    return sqlx::query_as::<_, UserModel>(query).fetch_all(pool).await;
}
