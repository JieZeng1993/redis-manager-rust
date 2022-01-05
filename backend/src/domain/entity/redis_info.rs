use chrono::{DateTime, Local, LocalResult, TimeZone};
use rbatis::DateTimeNative;

use crate::util::time_util;

use super::super::vo::redis_info::*;

///redis信息主表实体
#[crud_table]
#[derive(Clone, Debug)]
pub struct RedisInfo {
    pub id: Option<i32>,
    /// redis名称
    pub name: Option<String>,
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

impl_field_name_method!(RedisInfo{id,name,host,port,username,password,cluster_type,create_time,create_id,update_time,update_id});

pub fn convert_redis_info2redis_info_vo(redis_info: RedisInfo) -> RedisInfoVo {
    let create_time = time_util::convert(redis_info.create_time);
    let update_time = time_util::convert(redis_info.update_time);
    RedisInfoVo {
        id: redis_info.id,
        name: redis_info.name,
        host: redis_info.host,
        port: redis_info.port,
        username: redis_info.username,
        cluster_type: redis_info.cluster_type,
        create_time,
        create_id: redis_info.create_id,
        update_time,
        update_id: redis_info.update_id,
        redis_node_infos: None,
    }
}

