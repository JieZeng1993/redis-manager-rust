use rbatis::DateTimeNative;

use super::super::vo::user::UserVo;
use chrono::{DateTime, Local, LocalResult, TimeZone};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub passwd: Option<String>,
    pub create_time: Option<DateTimeNative>,
    pub create_id: Option<i32>,
}

impl_field_name_method!(User{id,name,passwd,create_time,create_id});

impl User {
    pub fn convert2vo(self) -> UserVo {
        let create_time = match self.create_time {
            Some(create_time) => {
                match Local.from_local_datetime(&*create_time) {
                    LocalResult::Single(create_time) => Some(create_time),
                    //可能是None,也可能是Ambiguous
                    _ => { None }
                }
            }
            None => None
        };
        UserVo {
            id: self.id,
            name: self.name,
            create_time,
        }
    }
}
