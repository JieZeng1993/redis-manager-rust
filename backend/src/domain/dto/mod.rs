use poem_openapi::types::{ParseFromJSON, ToJSON};
use rbatis::{Page, PageRequest};

use crate::domain::vo::RespVO;

pub mod user1;
pub mod user;
pub mod redis_info;

pub trait IPageRequest {
    fn page_size(&self) -> Option<u64>;
    fn page_no(&self) -> u64;
}

pub fn convert_rbatis_page_request(page_request: impl IPageRequest) -> PageRequest {
    PageRequest::new(page_request.page_no(), page_request.page_size().unwrap_or(10))
}

pub fn convert_rbatis_page_resp_and_convert<T, F, R>(page_resp: Page<T>, f: F) -> RespVO<Vec<R>>
    where F: FnMut(T) -> R, R: Clone + poem_openapi::types::Type + ParseFromJSON + ToJSON {
    RespVO {
        success: true,
        msg: None,
        show_type: None,
        data: Some(page_resp.records.into_iter().map(f).collect()),
        error_code: None,
        current: Some(page_resp.page_no),
        page_size: Some(page_resp.page_size),
        total: Some(page_resp.total),
    }
}

pub fn convert_rbatis_page_resp<T>(page_resp: Page<T>) -> RespVO<Vec<T>>
    where T: Clone + poem_openapi::types::Type + ParseFromJSON + ToJSON {
    RespVO {
        success: true,
        msg: None,
        show_type: None,
        data: Some(page_resp.records),
        error_code: None,
        current: Some(page_resp.page_no),
        page_size: Some(page_resp.page_size),
        total: Some(page_resp.total),
    }
}