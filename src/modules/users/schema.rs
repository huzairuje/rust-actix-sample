use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::ValidationError;
use validator_derive::Validate;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    #[validate(custom = "validate_username")]
    pub username: String,
    #[validate(custom = "validate_password")]
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_fullname")]
    pub fullname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_phone_number")]
    pub phone_number: Option<String>,
}

fn validate_fullname(fullname: &str) -> Result<(), ValidationError> {
    if fullname.len() < 3 {
        return Err(ValidationError::new(
            "fullname must have a minimum length of 3 characters",
        ));
    }
    Ok(())
}

fn validate_phone_number(phone_number: &str) -> Result<(), ValidationError> {
    if !phone_number.starts_with('0') {
        return Err(ValidationError::new("Phone number must start with '0'"));
    }
    Ok(())
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if !username.len() < 4 {
        return Err(ValidationError::new(
            "username must have a minimum length of 3 characters",
        ));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if !password.len() < 6 {
        return Err(ValidationError::new(
            "password must have a minimum length of 6 characters",
        ));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), ValidationError> {
    // Define the regular expression pattern for email validation
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    // Check if the email matches the pattern
    if !email_regex.is_match(email) {
        return Err(ValidationError::new(
            "email must be valid email like 'mail@mail.com'",
        ));
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_username")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_password")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_fullname")]
    pub fullname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_phone_number")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
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
