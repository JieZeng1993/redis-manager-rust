use poem_openapi::Object;

#[derive(Debug, Object, Clone, Eq, PartialEq, Default)]
pub struct User1Vo {
    pub id: Option<i32>,
    pub name: Option<String>,
}

