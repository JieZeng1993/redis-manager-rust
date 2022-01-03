use chrono::{DateTime, Local};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Object, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct UserVo {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTime<Local>>,
    pub update_id: Option<i32>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(inline, rename_all = "camelCase")]
pub struct LoginVo {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub authorization: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_id: Option<i32>,
    pub update_time: Option<DateTime<Local>>,
    pub update_id: Option<i32>,
}
