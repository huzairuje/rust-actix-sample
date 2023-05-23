use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct UserSaveModel {
    pub username: String,
    pub password: String,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct UserUpdateModel {
    pub username: Option<String>,
    pub password: Option<String>,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}
