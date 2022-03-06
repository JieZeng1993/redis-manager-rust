use poem::{IntoResponse, Response};
use poem::http::{header, StatusCode};
use poem_openapi::{Object,ApiResponse};
use poem_openapi::payload::Payload;
use poem_openapi::registry::MetaSchemaRef;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use serde::{Deserialize, Serialize};

use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::rest::RespError;
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
    //error display type： 0 silent; 1 message.warn; 2 message.error; 4 notification; 9 page
    pub show_type: Option<i16>,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub current: Option<u64>,
    pub page_size: Option<u64>,
    pub total: Option<u64>,
}

impl<T> RespVO<T> where T: Sync + Send + Clone + poem_openapi::types::Type + ParseFromJSON + ToJSON
{
    pub fn from(arg: &T) -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: Some(arg.clone()),
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn from_move(arg: T) -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: Some(arg),
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn success_msg(msg: String) -> Self {
        Self {
            success: true,
            msg: Some(msg),
            error_code: None,
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
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
            show_type: None,
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
            show_type: None,
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
            show_type: None,
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
            show_type: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(ApiResponse)]
pub struct Resp<T> where T: Serialize + Send {
    pub success: bool,
    pub msg: Option<String>,
    //error display type： 0 silent; 1 message.warn; 2 message.error; 4 notification; 9 page
    pub show_type: Option<i16>,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub current: Option<u64>,
    pub page_size: Option<u64>,
    pub total: Option<u64>,
}

impl<T> Resp<T> where T: Serialize + Send {
    pub fn from(arg: T) -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: Some(arg),
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn not_found() -> Self {
        Self {
            success: false,
            msg: StatusCode::NOT_FOUND.canonical_reason().map(|reason| { reason.to_string() }),
            error_code: Some(StatusCode::NOT_FOUND.as_u16().to_string()),
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn error(error: Error) -> Self {
        Self {
            success: false,
            msg: Some(error.to_string()),
            error_code: Some(StatusCode::BAD_REQUEST.as_u16().to_string()),
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn from_result(arg: Result<Option<T>>) -> Self {
        match arg {
            Ok(arg) => match arg {
                Some(user) => Resp::from(user),
                None => Resp::not_found(),
            },
            Err(error) => {
                Resp::error(error)
            }
        }
    }
}

impl<T> IntoResponse for Resp<T> where T: Serialize + Send {
    fn into_response(self) -> Response {
        if self.error_code.is_some() && self.error_code.unwrap().eq(&RespError::UNKNOWN.to_string()) {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("unknown");
        }

        let data = match serde_json::to_vec(&self) {
            Ok(data) => data,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string());
            }
        };

        Response::builder()
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(data)
    }
}

#[test]
fn test() {
    let resp_vo = RespVO::from(&1);

    let json = serde_json::to_string_pretty(&resp_vo).unwrap();

    println!("{}", json);
}