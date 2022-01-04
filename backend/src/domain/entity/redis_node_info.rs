use chrono::{DateTime, Local, LocalResult, TimeZone};
use rbatis::DateTimeNative;

use crate::util::time_util;

use super::super::vo::redis_node_info::*;

///redis节点信息表实体
#[crud_table]
#[derive(Clone, Debug)]
pub struct RedisNodeInfo {
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
    pub create_time: Option<DateTimeNative>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTimeNative>,
    pub update_id: Option<i32>,
}

impl_field_name_method!(RedisNodeInfo{id,redis_info_id,node_id,master_id,host,port,node_role,node_status,slot_from,slot_to,create_time,create_id,update_time,update_id});

impl RedisNodeInfo {
    pub fn convert2vo(self) -> RedisNodeInfoVo {
        let create_time = time_util::convert(self.create_time);
        let update_time = time_util::convert(self.update_time);
        RedisNodeInfoVo {
            id: self.id,
            redis_info_id: self.redis_info_id,
            node_id: self.node_id,
            master_id: self.master_id,
            host: self.host,
            port: self.port,
            node_role: self.node_role,
            node_status: self.node_status,
            slot_from: self.slot_from,
            slot_to: self.slot_to,
            create_time,
            create_id: self.create_id,
            update_time,
            update_id: self.update_id,
        }
    }
}
