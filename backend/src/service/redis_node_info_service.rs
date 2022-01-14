use std::borrow::{Borrow, BorrowMut};

use itertools::Itertools;
use rbatis::crud::CRUD;
use rbatis::DateTimeNative;

use crate::config::auth;
use crate::domain::dto::user1::User1UpdateDto;
use crate::domain::dto::user::UserUpdateDto;
use crate::domain::entity::redis_node_info::*;
use crate::domain::vo::redis_node_info::*;
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::SERVICE_CONTEXT;

pub struct RedisNodeInfoService {}

impl RedisNodeInfoService {
    ///后台用户根据id查找
    pub async fn find_by_id(&self, id: i32) -> Result<Option<RedisNodeInfoVo>> {
        let redis_node_info = self.do_find_by_id(id).await?;
        match redis_node_info {
            Some(redis_node_info) => Ok(Some(convert_redis_info2redis_info_vo(redis_node_info))),
            None => Ok(None)
        }
    }

    /// 内部查询使用entity，到rest层再转为Vo
    pub async fn do_find_by_id(&self, id: i32) -> Result<Option<RedisNodeInfo>> {
        let wrapper = SERVICE_CONTEXT.rbatis.new_wrapper().eq(RedisNodeInfo::id(), id);
        return Ok(SERVICE_CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }

    ///后台用户根据id查找
    pub async fn find_by_redis_info_id(&self, redis_info_id: i32) -> Result<Vec<RedisNodeInfoVo>> {
        let redis_node_infos = self.do_find_by_redis_info_id(redis_info_id).await?;
        Ok(redis_node_infos.into_iter().map(convert_redis_info2redis_info_vo).collect())
    }

    /// 根据redis_info_id查询所有node信息
    pub async fn do_find_by_redis_info_id(&self, redis_info_id: i32) -> Result<Vec<RedisNodeInfo>> {
        let wrapper = SERVICE_CONTEXT.rbatis.new_wrapper().eq(RedisNodeInfo::redis_info_id(), redis_info_id);
        return Ok(SERVICE_CONTEXT.rbatis.fetch_list_by_wrapper(wrapper).await?);
    }

    /// * `redis_info_id`
    /// * `redis_node_infos` 最新的相关信息
    /// 根据redis_info_id查出数据库中的数据，对比redis_node_infos，进行增加、更新、删除动作
    pub async fn update_by_redis_info_id(&self, redis_info_id: i32, redis_node_infos: Vec<RedisNodeInfo>) -> Result<()> {
        let redis_node_infos_in_db = self.do_find_by_redis_info_id(redis_info_id).await?;
        let mut add_redis_node_infos = vec![];
        let mut update_redis_node_infos = vec![];
        let mut delete_redis_node_info_ids = vec![];

        for redis_node_info in &redis_node_infos {
            let redis_node_info_in_db = redis_node_infos_in_db.iter().cloned().find(|redis_node_info_in_db| redis_node_info_in_db.host.eq(&redis_node_info.host) && redis_node_info_in_db.port.eq(&redis_node_info.port));
            if redis_node_info_in_db.is_some() {
                //数据库中已经有这个集合，放到更新集合中
                let mut redis_node_info_in_db = redis_node_info_in_db.unwrap();
                redis_node_info_in_db.node_id = redis_node_info.node_id.clone();
                redis_node_info_in_db.master_id = redis_node_info.master_id.clone();
                redis_node_info_in_db.node_role = redis_node_info.node_role.clone();
                redis_node_info_in_db.node_status = redis_node_info.node_status.clone();
                redis_node_info_in_db.slot_from = match redis_node_info.slot_from {
                    Some(_) => { redis_node_info.slot_from }
                    None => { Some(crate::config::constant::INVALID_NUM as u16) }
                };
                redis_node_info_in_db.slot_to = match redis_node_info.slot_to {
                    Some(_) => { redis_node_info.slot_to }
                    None => { Some(crate::config::constant::INVALID_NUM as u16) }
                };
                redis_node_info_in_db.update_time = redis_node_info.update_time;
                redis_node_info_in_db.update_id = redis_node_info.update_id;
                update_redis_node_infos.push(redis_node_info_in_db);
            } else {
                add_redis_node_infos.push(RedisNodeInfo {
                    id: None,
                    redis_info_id: Some(redis_info_id),
                    node_id: redis_node_info.node_id.clone(),
                    master_id: redis_node_info.master_id.clone(),
                    host: redis_node_info.host.clone(),
                    port: redis_node_info.port.clone(),
                    node_role: redis_node_info.node_role.clone(),
                    node_status: redis_node_info.node_status.clone(),
                    slot_from: match redis_node_info.slot_from {
                        Some(_) => { redis_node_info.slot_from }
                        None => { Some(crate::config::constant::INVALID_NUM as u16) }
                    },
                    slot_to: match redis_node_info.slot_to {
                        Some(_) => { redis_node_info.slot_to }
                        None => { Some(crate::config::constant::INVALID_NUM as u16) }
                    },
                    create_time: redis_node_info.create_time,
                    create_id: redis_node_info.create_id,
                    update_time: redis_node_info.update_time,
                    update_id: redis_node_info.update_id,
                });
            }
        }

        for redis_node_info_in_db in redis_node_infos_in_db {
            let redis_node_info = redis_node_infos.iter().cloned().find(|redis_node_info| redis_node_info.host.eq(&redis_node_info_in_db.host) && redis_node_info.port.eq(&redis_node_info_in_db.port));
            if redis_node_info.is_some() {
                delete_redis_node_info_ids.push(redis_node_info.unwrap().id);
            }
        }

        SERVICE_CONTEXT.rbatis.save_batch(&add_redis_node_infos, &[]).await?;
        SERVICE_CONTEXT.rbatis.update_batch_by_column(RedisNodeInfo::id(), &update_redis_node_infos).await?;
        SERVICE_CONTEXT.rbatis.remove_batch_by_column::<RedisNodeInfo, _>(RedisNodeInfo::id(), &delete_redis_node_info_ids).await?;

        Ok(())
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

mod test {
    use itertools::Itertools;
    use log::{Level, log};

    #[derive(Debug, Clone)]
    struct TestStruct {
        pub id: i32,
        pub name: String,
    }

    #[test]
    fn test_collection() {
        let a = vec![1, 0];
        let b = vec![TestStruct {
            id: 0,
            name: "0".to_string(),
        }, TestStruct {
            id: 0,
            name: "1".to_string(),
        }];


        for x in a {
            let find_val = b.iter().find(|y| y.id == x);
            println!("find_val:{:?}", find_val);
        }

        println!("b:{:?}", b);
    }
}