use std::borrow::Borrow;

use rbatis::crud::CRUD;
use rbatis::DateTimeNative;

use crate::config::auth;
use crate::domain::dto::user1::User1UpdateDto;
use crate::domain::dto::user::UserUpdateDto;
use crate::domain::entity::user::User;
use crate::domain::vo::user::{LoginVo, UserVo};
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::CONTEXT;

pub struct UserService {}

impl UserService {
    ///后台用户根据id查找
    pub async fn find_by_id(&self, id: i32) -> Result<Option<UserVo>> {
        let user = self.do_find_by_id(id).await?;
        match user {
            Some(user) => Ok(Some(user.convert2vo())),
            None => Ok(None)
        }
    }

    /// 内部查询使用entity，到rest层再转为Vo
    pub async fn do_find_by_id(&self, id: i32) -> Result<Option<User>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq(User::id(), id);
        return Ok(CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }

    ///后台用户根据name查找
    pub async fn login(&self, name: &String, password: &String) -> Result<Option<LoginVo>> {
        let user = self.do_find_by_name(name).await?;
        match user {
            Some(user) => {
                if user.password.as_ref().unwrap().eq(password) {
                    //登录，账号和密码都能对上
                    let mut login_vo = user.convert2login_vo();
                    let session_id = auth::get_session_id(&login_vo).await;
                    login_vo.authorization = Some(session_id);
                    Ok(Some(login_vo))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None)
        }
    }

    ///后台用户根据name查找
    pub async fn find_by_name(&self, name: &String) -> Result<Option<UserVo>> {
        let user = self.do_find_by_name(name).await?;
        match user {
            Some(user) => Ok(Some(user.convert2vo())),
            None => Ok(None)
        }
    }

    ///后台用户根据id查找
    pub async fn update(&self, user_update_dto: UserUpdateDto, update_id: Option<i32>) -> Result<u64> {
        let mut user_update_entity = user_update_dto.convert2entity();
        user_update_entity.update_id = update_id;
        user_update_entity.update_time = Some(DateTimeNative::now());
        Ok(CONTEXT.rbatis.update_by_column("id", &user_update_entity).await?)
    }


    /// 内部查询使用entity，到rest层再转为Vo
    pub async fn do_find_by_name(&self, name: &String) -> Result<Option<User>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq(User::name(), name);
        return Ok(CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }
}