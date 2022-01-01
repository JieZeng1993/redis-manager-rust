use chrono::{DateTime, Local};
use poem_openapi::Object;

#[derive(Debug, Object, Clone, Eq, PartialEq, Default)]
pub struct UserVo {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub create_time: Option<DateTime<Local>>,
}

