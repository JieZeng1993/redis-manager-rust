use chrono::{DateTime, Local, LocalResult, TimeZone};
use rbatis::DateTimeNative;

use crate::util::time_util;

use super::super::vo::user::*;

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub create_time: Option<DateTimeNative>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTimeNative>,
    pub update_id: Option<i32>,
}

impl_field_name_method!(User{id,name,password,create_time,create_id,update_time,update_id});

impl User {
    pub fn convert2vo(self) -> UserVo {
        let create_time = time_util::convert(self.create_time);
        let update_time = time_util::convert(self.update_time);
        UserVo {
            id: self.id,
            name: self.name,
            create_time,
            create_id: self.create_id,
            update_time,
            update_id: self.update_id,
        }
    }

    pub fn convert2login_vo(self) -> LoginVo {
        let create_time = time_util::convert(self.create_time);
        let update_time = time_util::convert(self.update_time);
        LoginVo {
            id: self.id,
            name: self.name,
            authorization: None,
            create_time,
            create_id: self.create_id,
            update_time,
            update_id: self.update_id,
        }
    }
}
