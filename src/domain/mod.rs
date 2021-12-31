use rbatis::DateTimeNative;

pub mod entity;
pub mod vo;

#[crud_table]
#[derive(Clone, Debug)]
pub struct BizActivity {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub create_time: Option<DateTimeNative>,
}