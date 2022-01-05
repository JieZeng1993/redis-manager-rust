use chrono::{DateTime, Local, LocalResult, TimeZone};
use rbatis::DateTimeNative;

use crate::util::time_util;

use super::super::vo::redis_info::*;

///redis信息主表实体
#[crud_table]
#[derive(Clone, Debug)]
pub struct RedisInfo {
    pub id: Option<i32>,
    ///redis的主机地址，可以是域名，也可以是ip
    pub host: Option<String>,
    ///redis的端口
    pub port: Option<u16>,
    ///用户名（空表示无需用户名）
    pub username: Option<String>,
    ///密码（空表示无需密码）
    pub password: Option<String>,
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    pub cluster_type: Option<String>,
    pub create_time: Option<DateTimeNative>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTimeNative>,
    pub update_id: Option<i32>,
}

impl_field_name_method!(RedisInfo{id,host,port,username,password,cluster_type,create_time,create_id,update_time,update_id});

impl RedisInfo {
    pub fn convert2vo(self) -> RedisInfoVo {
        let create_time = time_util::convert(self.create_time);
        let update_time = time_util::convert(self.update_time);
        RedisInfoVo {
            id: self.id,
            host: self.host,
            port: self.port,
            username: self.username,
            cluster_type: self.cluster_type,
            create_time,
            create_id: self.create_id,
            update_time,
            update_id: self.update_id,
            redis_node_infos: None
        }
    }
}
