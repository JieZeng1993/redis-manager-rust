use chrono::{DateTime, Local};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use super::super::vo::redis_info::*;

///redis节点信息响应实体
#[derive(Debug, Object, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct RedisNodeInfoVo {
    pub id: Option<i32>,
    ///redis_info表的主键
    pub redis_info_id: Option<i32>,
    ///redis集群中，redis的唯一标志
    pub node_id: Option<String>,
    ///redis集群或哨兵模式中，当前node为从节点，此字段表示master的id，否则此字段为空
    pub master_id: Option<String>,
    ///redis的主机地址，可以是域名，也可以是ip
    pub host: Option<String>,
    ///redis的端口
    pub port: Option<u16>,
    ///MASTER(单节点就是MASTER),SLAVE
    pub node_role: Option<String>,
    ///CONNECTED,UNKNOWN,UNCONNECTED
    pub node_status: Option<String>,
    ///集群模式中，slot开始，非集群为0
    pub slot_from: Option<u16>,
    ///集群模式中，slot结束，非集群为16383
    pub slot_to: Option<u16>,
    pub create_time: Option<DateTime<Local>>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTime<Local>>,
    pub update_id: Option<i32>,
}
