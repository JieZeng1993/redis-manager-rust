use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use poem_openapi::Object;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

use crate::domain::dto::IPageRequest;
use crate::domain::entity::redis_info::*;

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct RedisPageDto {
    pub id: Option<i32>,
    pub current: u64,
    /// page有值表示分页查询
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub cluster_type: Option<String>,
}

impl IPageRequest for RedisPageDto {
    fn page_size(&self) -> Option<u64> {
        self.page_size
    }

    fn page_no(&self) -> u64 {
        self.current
    }
}
// impl UserUpdateDto {
//     pub fn convert2entity(self) -> User {
//         User {
//             id: Some(self.id),
//             name: self.name,
//             password: None,
//             create_time: None,
//             create_id: None,
//             update_time: None,
//             update_id: None,
//         }
//     }
// }