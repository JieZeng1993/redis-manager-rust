#[crud_table]
#[derive(Clone, Debug)]
pub struct User1 {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl_field_name_method!(User1{id,name});