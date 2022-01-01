use super::super::vo::user1::User1Vo;
use rbatis::DateTimeNative;

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
        User1Vo{
            id: self.id,
            name: self.name
        }
    }
}
