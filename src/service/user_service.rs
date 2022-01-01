use rbatis::crud::CRUD;
use crate::domain::dto::user1::User1UpdateDto;

use crate::domain::entity::user::User;
use crate::domain::vo::user::UserVo;
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::CONTEXT;

pub struct UserService {}

impl UserService {
    ///后台用户根据id查找
    pub async fn find(&self, id: i64) -> Result<Option<UserVo>> {
        let user = self.do_find(id).await?;
        match user {
            Some(user) => Ok(Some(user.convert2vo())),
            None => Ok(None)
        }
    }

    /// 内部查询使用entity，到rest层再转为Vo
    pub async fn do_find(&self, id: i64) -> Result<Option<User>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq(User::id(), id);
        return Ok(CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }
}