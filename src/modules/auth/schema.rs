use serde::{Deserialize, Serialize};
use validator::ValidationError;
use validator_derive::Validate;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    #[validate(custom = "validate_username")]
    pub username: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 1 {
        return Err(ValidationError::new("username is empty"));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 1 {
        return Err(ValidationError::new("password is empty"));
    }
    Ok(())
}
