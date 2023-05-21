use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T, E> {
    status: String,
    code: u16,
    message: String,
    data: Option<T>,
    data_error: Option<E>,
}

impl<T, E> Response<T, E> {
    pub fn success(status_code: StatusCode, data: T, message: &str) -> Response<T, E> {
        Response {
            status: status_code.to_string(),
            code: status_code.as_u16(),
            message: message.to_string(),
            data: Some(data),
            data_error: None,
        }
    }

    pub fn error(status_code: StatusCode, message: &str) -> Response<T, E> {
        Response {
            status: status_code.to_string(),
            code: status_code.as_u16(),
            message: message.to_string(),
            data: None,
            data_error: None,
        }
    }

    // pub fn custom(
    //     status_code: StatusCode,
    //     message: &str,
    //     data: T,
    //     data_error: E,
    // ) -> Response<T, E> {
    //     Response {
    //         status: status_code.to_string(),
    //         code: status_code.as_u16(),
    //         message: message.to_string(),
    //         data: Some(data),
    //         data_error: Some(data_error),
    //     }
    // }
}
