use rbatis::DateTimeNative;

use super::super::vo::user1::User1Vo;
use chrono::{DateTime, Local, LocalResult, TimeZone};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User1 {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub create_date: Option<DateTimeNative>,
}

impl_field_name_method!(User1{id,name});

impl User1 {
    pub fn convert2vo(self) -> User1Vo {
        let create_date = match self.create_date {
            Some(create_date) => {
                match Local.from_local_datetime(&*create_date) {
                    LocalResult::Single(create_date) => Some(create_date),
                    //可能是None,也可能是Ambiguous
                    _ => { None }
                }
            }
            None => None
        };
        User1Vo {
            id: self.id,
            name: self.name,
            create_date,
        }
    }
}
