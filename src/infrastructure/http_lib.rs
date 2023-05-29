use crate::infrastructure::pagination;
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

    pub fn custom(
        status_code: StatusCode,
        message: &str,
        data: T,
        data_error: E,
    ) -> Response<T, E> {
        Response {
            status: status_code.to_string(),
            code: status_code.as_u16(),
            message: message.to_string(),
            data: Some(data),
            data_error: Some(data_error),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    status: String,
    code: u16,
    message: String,
    page: u16,
    size: u16,
    total_count: u16,
    total_pages: u16,
    data: Option<T>,
}

impl<T> Pagination<T> {
    pub fn success(
        pg: pagination::PaginationQuery,
        message: &str,
        data: T,
        count: u16,
    ) -> Pagination<T> {
        Pagination {
            status: StatusCode::OK.to_string(),
            code: StatusCode::OK.as_u16(),
            message: message.to_string(),
            page: pagination::get_page(pg.clone()) as u16,
            size: pagination::get_size(pg.clone()) as u16,
            total_count: count,
            total_pages: pagination::get_total_pages(
                count.clone() as i32,
                pagination::get_size(pg.clone()),
            ) as u16,
            data: Some(data),
        }
    }
}
