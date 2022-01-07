use std::borrow::Borrow;

use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use rbatis::plugin::page::{Page, PageRequest};

use crate::config::auth;
use crate::domain::dto::{convert_rbatis_page_request, convert_rbatis_page_resp_and_convert};
use crate::domain::dto::redis_info::{RedisInfoRelatedInfoRtDto, RedisPageDto};
use crate::domain::dto::user1::User1UpdateDto;
use crate::domain::dto::user::UserUpdateDto;
use crate::domain::entity::redis_info::*;
use crate::domain::vo::redis_info::*;
use crate::domain::vo::redis_node_info::RedisNodeInfoVo;
use crate::domain::vo::RespVO;
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::CONTEXT;
use crate::util::string::IsEmpty;

pub struct RedisInfoService {}

impl RedisInfoService {
    ///根据id查找vo
    pub async fn find_by_id(&self, id: i32) -> Result<Option<RedisInfoVo>> {
        let redis_info = self.do_find_by_id(id).await?;
        match redis_info {
            Some(redis_info) => {
                let mut redis_info_vo = convert_redis_info2redis_info_vo(redis_info);
                let redis_node_info_vos = CONTEXT.redis_node_info_service.find_by_redis_info_id(redis_info_vo.id.unwrap()).await?;
                redis_info_vo.redis_node_infos = Some(redis_node_info_vos);
                Ok(Some(redis_info_vo))
            }
            None => Ok(None)
        }
    }

    ///redis info分页
    pub async fn page(&self, redis_page_dto: RedisPageDto) -> Result<RespVO<Vec<RedisInfoVo>>> {
        let wrapper = CONTEXT
            .rbatis
            .new_wrapper()
            .do_if(!redis_page_dto.name.is_empty(), |w| w.like(RedisInfo::name(), &redis_page_dto.name))
            .do_if(!redis_page_dto.host.is_empty(), |w| w.like(RedisInfo::host(), &redis_page_dto.host))
            .do_if(redis_page_dto.port.is_some(), |w| w.eq(RedisInfo::port(), &redis_page_dto.port))
            .do_if(redis_page_dto.cluster_type.is_some(), |w| w.eq(RedisInfo::cluster_type(), &redis_page_dto.cluster_type))
            .do_if(redis_page_dto.id.is_some(), |w| w.eq(RedisInfo::id(), &redis_page_dto.id))
            .do_if(redis_page_dto.update_time_begin.is_some(), |w| w.ge(RedisInfo::update_time(), &redis_page_dto.update_time_begin))
            .do_if(redis_page_dto.update_time_end.is_some(), |w| w.le(RedisInfo::update_time(), &redis_page_dto.update_time_end))
            .order_by(false, &[RedisInfo::update_time()]);

        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<RedisInfo>(wrapper, &convert_rbatis_page_request(redis_page_dto))
            .await?;

        Ok(convert_rbatis_page_resp_and_convert(data, convert_redis_info2redis_info_vo))
    }

    /// 根据id查找entity
    pub async fn do_find_by_id(&self, id: i32) -> Result<Option<RedisInfo>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq(RedisInfo::id(), id);
        return Ok(CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }

    ///实时查询节点相关信息
    pub async fn related_info_rt(&self, redis_info_related_info_rt_dto: RedisInfoRelatedInfoRtDto) ->  Result<Vec<RedisNodeInfoVo>> {
        return Ok(vec![RedisNodeInfoVo{
            id: None,
            redis_info_id: None,
            node_id: Some("查询出来的node id".to_string()),
            master_id: Some("查询出来的master id".to_string()),
            host: Some("查询出来的host".to_string()),
            port:  Some(6380),
            node_role: Some("MASTER".to_string()),
            node_status: Some("CONNECTED".to_string()),
            slot_from: Some(0),
            slot_to: Some(155),
            create_time: None,
            create_id: None,
            update_time: None,
            update_id: None
        }])
    }

    // ///后台用户根据name查找
    // pub async fn login(&self, name: &String, password: &String) -> Result<Option<LoginVo>> {
    //     let user = self.do_find_by_name(name).await?;
    //     match user {
    //         Some(user) => {
    //             if user.password.as_ref().unwrap().eq(password) {
    //                 //登录，账号和密码都能对上
    //                 let mut login_vo = user.convert2login_vo();
    //                 let session_id = auth::get_session_id(&login_vo).await;
    //                 login_vo.authorization = Some(session_id);
    //                 Ok(Some(login_vo))
    //             } else {
    //                 Ok(None)
    //             }
    //         }
    //         None => Ok(None)
    //     }
    // }
    //
    // ///后台用户根据name查找
    // pub async fn find_by_name(&self, name: &String) -> Result<Option<UserVo>> {
    //     let user = self.do_find_by_name(name).await?;
    //     match user {
    //         Some(user) => Ok(Some(user.convert2vo())),
    //         None => Ok(None)
    //     }
    // }
    //
    // ///后台用户根据id查找
    // pub async fn update(&self, user_update_dto: UserUpdateDto, update_id: Option<i32>) -> Result<u64> {
    //     let mut user_update_entity = user_update_dto.convert2entity();
    //     user_update_entity.update_id = update_id;
    //     user_update_entity.update_time = Some(DateTimeNative::now());
    //     Ok(CONTEXT.rbatis.update_by_column("id", &user_update_entity).await?)
    // }
    //
    //
    // /// 内部查询使用entity，到rest层再转为Vo
    // pub async fn do_find_by_name(&self, name: &String) -> Result<Option<User>> {
    //     let wrapper = CONTEXT.rbatis.new_wrapper().eq(User::name(), name);
    //     return Ok(CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    // }
}