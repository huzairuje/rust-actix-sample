use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct AuthModel {
    pub access_token: String,
    pub refresh_token: String,
    pub status: Option<String>
}