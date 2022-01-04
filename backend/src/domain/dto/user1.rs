use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use poem_openapi::Object;
use rbatis::DateTimeNative;

use crate::domain::entity::user1::User1;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct User1UpdateDto {
    pub id: i32,
    pub name: Option<String>,
    pub create_date: Option<DateTime<Local>>,
}

impl User1UpdateDto {
    pub fn convert2entity(self) -> User1 {
        let create_date = match self.create_date {
            Some(create_date) => {
                Some(DateTimeNative::from(create_date))
            }
            None => None
        };
        User1 {
            id: Some(self.id),
            name: self.name,
            create_date,
        }
    }
}