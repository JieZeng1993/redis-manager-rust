use rbatis::crud::CRUD;

use crate::domain::entity::user1::User1;
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::CONTEXT;

pub struct User1Service {}

impl User1Service {
    ///后台用户根据id查找
    pub async fn find(&self, id: i64) -> Result<Option<User1>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq(User1::id(), id);
        return Ok(CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }
}