use serde::Serialize;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    failed_field: String,
    tag: String,
    value: String,
}

pub fn validate_struct<T: Validate>(input: T) -> Vec<ErrorResponse> {
    let mut err_response: Vec<ErrorResponse> = Vec::new();
    let errors: Result<(), ValidationErrors> = input.validate();

    if let Err(validation_errors) = errors {
        for err in validation_errors.field_errors().values() {
            for field_err in err.iter() {
                let element = ErrorResponse {
                    failed_field: trim_string_from_dot(&field_err.to_string()),
                    tag: field_err.code.to_string(),
                    value: field_err
                        .params
                        .get("0")
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                };
                err_response.push(element);
            }
        }
    }

    err_response
}

fn trim_string_from_dot(string: &str) -> String {
    match string.find('.') {
        Some(dot_pos) => string[dot_pos + 1..].to_lowercase(),
        None => string.to_owned(),
    }
}
