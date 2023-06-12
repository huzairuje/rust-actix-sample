use serde::Deserialize;

const DEFAULT_SIZE: i32 = 10;

#[derive(Debug, Deserialize, Clone)]
pub struct PaginationQuery {
    sort_order: Option<String>,
    order_by: Option<String>,
    size: Option<i32>,
    page: Option<i32>,
}

pub fn get_total_pages(total_count: i32, page_size: i32) -> i32 {
    let d = f64::from(total_count) / f64::from(page_size);
    f64::ceil(d) as i32
}

pub fn get_size(req: PaginationQuery) -> i32 {
    return if let Some(req_size) = req.size.clone() {
        req_size
    } else {
        DEFAULT_SIZE
    };
}

pub fn get_page(req: PaginationQuery) -> i32 {
    return if let Some(req_page) = req.page.clone() {
        req_page
    } else {
        0
    };
}
#[allow(dead_code)]
pub fn get_order_by(req: PaginationQuery) -> String {
    return if let Some(req_order_by) = req.order_by.clone() {
        req_order_by
    } else {
        "created_at".to_string()
    };
}
#[allow(dead_code)]
pub fn get_sort_order(req: PaginationQuery) -> String {
    if let Some(req_sort_order) = req.sort_order.clone() {
        if req_sort_order.to_lowercase() == "asc" {
            return "asc".to_string();
        } else if req_sort_order.to_lowercase() == "desc" {
            return "desc".to_string();
        }
    }
    "desc".to_string()
}
