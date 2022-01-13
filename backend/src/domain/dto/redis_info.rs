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
    pub current: u64,
    /// page有值表示分页查询
    pub page_size: Option<u64>,
    pub id: Option<i32>,
    /// redis名称
    pub name: Option<String>,
    ///redis的主机地址，可以是域名，也可以是ip
    pub host: Option<String>,
    ///redis的端口
    pub port: Option<u16>,
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    pub cluster_type: Option<String>,
    ///更新时间范围-开始时间
    pub update_time_begin: Option<String>,
    ///更新时间范围-结束时间
    pub update_time_end: Option<String>,
    pub update_id: Option<i32>,
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

///redis节点连接相关信息请求
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct RedisConnectDto {
    pub id: Option<i32>,
    ///name校验重复（校验时，如果）
    pub name: Option<String>,
    ///redis的主机地址，可以是域名，也可以是ip
    pub host: Option<String>,
    ///redis的端口
    pub port: Option<u16>,
    ///用户名（空表示无需用户名）
    pub username: Option<String>,
    ///密码（空表示无密码）
    pub password: Option<String>,
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    pub cluster_type: Option<String>,
}