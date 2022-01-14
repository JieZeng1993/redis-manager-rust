use poem_openapi::{
    ApiResponse,
    Object,
    // types::{Email, Password},
    OpenApi, OpenApiService, param::Path, payload::Json, Tags,
};

use crate::config::auth::Session;
use crate::domain::dto::redis_info::{RedisConnectDto, RedisPageDto};
use crate::domain::vo::redis_info::RedisInfoVo;
use crate::domain::vo::redis_node_info::RedisNodeInfoVo;
use crate::domain::vo::RespVO;
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::SERVICE_CONTEXT;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    RedisInfo,
}


pub struct RedisInfoRest;

#[derive(ApiResponse)]
enum FindRedisInfoResponse {
    /// Return the redis info.
    #[oai(status = 200)]
    Ok(Json<RespVO<RedisInfoVo>>),
    /// Return when the specified redis info is not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}

#[derive(ApiResponse)]
enum PageRedisInfoResponse {
    /// Return the redis info.
    #[oai(status = 200)]
    Ok(Json<RespVO<Vec<RedisInfoVo>>>),
    #[oai(status = 500)]
    InnerError,
}

#[derive(ApiResponse)]
enum RedisInfoRelatedInfoRtResponse {
    /// Return the redis info.
    #[oai(status = 200)]
    Ok(Json<RespVO<Vec<RedisNodeInfoVo>>>),
    #[oai(status = 500)]
    InnerError,
}

#[derive(ApiResponse)]
enum RedisConnectTestResponse {
    /// Return the redis info.
    #[oai(status = 200)]
    Ok(Json<RespVO<String>>),
    #[oai(status = 500)]
    InnerError,
}

#[derive(ApiResponse)]
enum UpdateByConnectResponse {
    /// Return the redis info.
    #[oai(status = 200)]
    Ok(Json<RespVO<String>>),
    /// Return when the redis infois not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}

#[derive(ApiResponse)]
enum AddByConnectResponse {
    /// Return the redis info.
    #[oai(status = 200)]
    Ok(Json<RespVO<String>>),
    /// Return when the redis infois not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}


#[OpenApi]
impl RedisInfoRest {
    #[oai(path = "/redisInfo/:id", method = "get", tag = "ApiTags::RedisInfo")]
    async fn find_redis_info(&self, id: Path<i32>) -> FindRedisInfoResponse {
        let user = SERVICE_CONTEXT.redis_info_service.find_by_id(id.0).await;
        deal_find_redis_info(user)
    }

    #[oai(path = "/redisInfo/page", method = "post", tag = "ApiTags::RedisInfo")]
    async fn page_redis_info(&self, redis_info_page_dto: Json<RedisPageDto>) -> PageRedisInfoResponse {
        let redis_info_page_resp = SERVICE_CONTEXT.redis_info_service.page(redis_info_page_dto.0).await;
        match redis_info_page_resp {
            Ok(redis_info_page_resp) => PageRedisInfoResponse::Ok(Json(redis_info_page_resp)),
            Err(_) => {
                log::error!("server started");
                PageRedisInfoResponse::InnerError
            }
        }
    }

    ///连接测试
    #[oai(path = "/redisInfo/connectTest", method = "post", tag = "ApiTags::RedisInfo")]
    async fn connect_test(&self, redis_connect_dto: Json<RedisConnectDto>) -> RedisConnectTestResponse {
        let connect_result = SERVICE_CONTEXT.redis_info_service.connect_test(redis_connect_dto.0).await;
        match connect_result {
            Ok(redis_node_info_vo) => RedisConnectTestResponse::Ok(Json(RespVO::from(&redis_node_info_vo))),
            Err(error) => RedisConnectTestResponse::Ok(Json(RespVO::from_error_code(error)))
        }
    }

    ///实时查询节点相关信息
    #[oai(path = "/redisInfo/relatedInfoRt", method = "post", tag = "ApiTags::RedisInfo")]
    async fn related_info_rt(&self, redis_connect_dto: Json<RedisConnectDto>) -> RedisInfoRelatedInfoRtResponse {
        let redis_node_info_vo = SERVICE_CONTEXT.redis_info_service.related_info_rt(redis_connect_dto.0).await;
        match redis_node_info_vo {
            Ok(redis_node_info_vo) => RedisInfoRelatedInfoRtResponse::Ok(Json(RespVO::from(&redis_node_info_vo))),
            Err(error) => RedisInfoRelatedInfoRtResponse::Ok(Json(RespVO::from_error_code(error)))
        }
    }

    ///通过连接信息更新
    #[oai(path = "/redisInfo/updateByConnect", method = "post", tag = "ApiTags::RedisInfo")]
    async fn update_by_connect(&self, redis_connect_dto: Json<RedisConnectDto>, session: &Session) -> UpdateByConnectResponse {
        let result = SERVICE_CONTEXT.redis_info_service.update_by_connect(redis_connect_dto.0, session).await;
        match result {
            Ok(msg) => UpdateByConnectResponse::Ok(Json(RespVO::success_msg(msg.to_string()))),
            Err(error) => UpdateByConnectResponse::Ok(Json(RespVO::from_error_code(error)))
        }
    }

    ///实时查询节点相关信息
    #[oai(path = "/redisInfo/addByConnect", method = "post", tag = "ApiTags::RedisInfo")]
    async fn add_by_connect(&self, redis_connect_dto: Json<RedisConnectDto>, session: &Session) -> AddByConnectResponse {
        let result = SERVICE_CONTEXT.redis_info_service.add_by_connect(redis_connect_dto.0, session).await;
        match result {
            Ok(msg) => AddByConnectResponse::Ok(Json(RespVO::success_msg(msg.to_string()))),
            Err(error) => AddByConnectResponse::Ok(Json(RespVO::from_error_code(error)))
        }
    }

    // #[oai(path = "/redisInfo", method = "put", tag = "ApiTags::RedisInfo")]
    // async fn update_redis_info(&self, user_update_dto: Json<UserUpdateDto>, session: &Session) -> UpdateUserResponse {
    //     let user = CONTEXT.user_service.update(user_update_dto.0, session.id).await;
    //     match user {
    //         Ok(user) => UpdateUserResponse::Ok(Json(RespVO::from(&user))),
    //         Err(_) => {
    //             log::error!("update error");
    //             UpdateUserResponse::InnerError
    //         }
    //     }
    // }
}

fn deal_find_redis_info(redis_info: Result<Option<RedisInfoVo>>) -> FindRedisInfoResponse {
    match redis_info {
        Ok(redis_info) => match redis_info {
            Some(redis_info) => FindRedisInfoResponse::Ok(Json(RespVO::from(&redis_info))),
            None => FindRedisInfoResponse::NotFound,
        },
        Err(_) => {
            log::error!("server started");
            FindRedisInfoResponse::InnerError
        }
    }
}