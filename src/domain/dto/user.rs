use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use poem_openapi::Object;
use rbatis::DateTimeNative;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserLoginDto {
    pub name: Option<String>,
    pub password: Option<String>,
}