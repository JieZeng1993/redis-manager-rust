use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use poem_openapi::Object;
use rbatis::DateTimeNative;
use crate::domain::entity::user::User;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserLoginDto {
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserUpdateDto {
    pub id: i32,
    pub name: Option<String>,
}

impl UserUpdateDto {
    pub fn convert2entity(self) -> User {
        User{
            id: Some(self.id),
            name: self.name,
            password: None,
            create_time: None,
            create_id: None,
            update_time: None,
            update_id: None
        }
    }
}