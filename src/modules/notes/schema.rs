use serde::{Deserialize, Serialize};
use validator::ValidationError;
use validator_derive::Validate;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateNoteSchema {
    #[validate(custom = "validate_title")]
    pub title: String,
    #[validate(custom = "validate_content")]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_category")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateNoteSchema {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_category")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

fn validate_title(title: &str) -> Result<(), ValidationError> {
    if title.len() < 1 {
        return Err(ValidationError::new(
            "title must have a minimum length of 1 characters",
        ));
    }
    Ok(())
}

fn validate_content(content: &str) -> Result<(), ValidationError> {
    if content.len() < 1 {
        return Err(ValidationError::new(
            "content must have minimum length of 1 characters",
        ));
    }
    Ok(())
}

fn validate_category(category: &str) -> Result<(), ValidationError> {
    if category.len() < 1 {
        return Err(ValidationError::new(
            "category must have a minimum length of 1 characters",
        ));
    }
    Ok(())
}
