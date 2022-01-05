use chrono::{DateTime, Local};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::domain::vo::redis_node_info::RedisNodeInfoVo;

///redis信息主表的响应实体
#[derive(Debug, Object, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct RedisInfoVo {
    pub id: Option<i32>,
    /// redis名称
    pub name: Option<String>,
    ///redis的主机地址，可以是域名，也可以是ip
    pub host: Option<String>,
    ///redis的端口
    pub port: Option<u16>,
    ///用户名（空表示无需用户名）
    pub username: Option<String>,
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    pub cluster_type: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTime<Local>>,
    pub update_id: Option<i32>,
    pub redis_node_infos: Option<Vec<RedisNodeInfoVo>>,
}
