use poem_openapi::Object;
use poem_openapi::payload::Payload;
use poem_openapi::registry::MetaSchemaRef;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use serde::{Deserialize, Serialize};

use crate::mix::error::Error;
use crate::service::SERVICE_CONTEXT;

pub mod user1;
pub mod user;
pub mod redis_info;
pub mod redis_node_info;

pub const CODE_COMMON_FAIL: &str = "COMMON_FAIL";

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct RespVO<T> where T: Sync + Send + Clone + poem_openapi::types::Type + ParseFromJSON + ToJSON {
    pub success: bool,
    pub msg: Option<String>,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub current: Option<u64>,
    pub page_size: Option<u64>,
    pub total: Option<u64>,
}

impl<T> RespVO<T> where T: Sync + Send + Clone + poem_openapi::types::Type + ParseFromJSON + ToJSON
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                success: true,
                msg: None,
                error_code: None,
                data: arg.clone().ok(),
                current: None,
                page_size: None,
                total: None,
            }
        } else {
            Self {
                success: false,
                msg: Some(arg.clone().err().unwrap().to_string()),
                error_code: None,
                data: None,
                current: None,
                page_size: None,
                total: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: Some(arg.clone()),
            current: None,
            page_size: None,
            total: None,
        }
    }

    pub fn no_data() -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: None,
            current: None,
            page_size: None,
            total: None,
        }
    }

    pub fn from_error(code: &str, arg: &Error) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = CODE_COMMON_FAIL.to_string();
        }
        Self {
            success: false,
            msg: Some(arg.to_string()),
            error_code: Some(code_str),
            data: None,
            current: None,
            page_size: None,
            total: None,
        }
    }

    pub fn from_error_code(arg: Error) -> Self {
        let mut code_str = arg.to_string();
        Self {
            success: false,
            //这个地方后续需要国际化
            msg: Some(code_str.clone()),
            error_code: Some(code_str),
            data: None,
            current: None,
            page_size: None,
            total: None,
        }
    }

    pub fn from_error_info(code: &str, info: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = CODE_COMMON_FAIL.to_string();
        }
        Self {
            success: false,
            msg: Some(info.to_string()),
            error_code: Some(code_str),
            data: None,
            current: None,
            page_size: None,
            total: None,
        }
    }
}

#[test]
fn test() {
    let resp_vo = RespVO::from(&1);

    let json = serde_json::to_string_pretty(&resp_vo).unwrap();

    println!("{}", json);
}