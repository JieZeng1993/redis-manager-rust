use poem_openapi::{
    ApiResponse,
    Object,
    // types::{Email, Password},
    OpenApi, OpenApiService, param::Path, payload::Json, Tags,
};

use crate::domain::dto::user1::User1UpdateDto;
use crate::domain::vo::RespVO;
use crate::domain::vo::user1::User1Vo;
use crate::service::SERVICE_CONTEXT;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User1,
}


pub struct User1Rest;

#[derive(ApiResponse)]
enum FindUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<RespVO<User1Vo>>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}

#[derive(ApiResponse)]
enum UpdateUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<RespVO<u64>>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}

#[OpenApi]
impl User1Rest {
    /// Find user by id
    #[oai(path = "/user1/:user_id", method = "get", tag = "ApiTags::User1")]
    async fn find_user(&self, user_id: Path<i64>) -> FindUserResponse {
        let user = SERVICE_CONTEXT.user1_service.find(user_id.0).await;
        match user {
            Ok(user) => match user {
                Some(user) => FindUserResponse::Ok(Json(RespVO::from(&user))),
                None => FindUserResponse::NotFound,
            },
            Err(_) => {
                log::error!("server started");
                FindUserResponse::InnerError
            }
        }
    }

    #[oai(path = "/user1", method = "put", tag = "ApiTags::User1")]
    async fn update_user(&self, user1_update_dto: Json<User1UpdateDto>) -> UpdateUserResponse {
        let user = SERVICE_CONTEXT.user1_service.update(user1_update_dto.0).await;
        match user {
            Ok(user) => UpdateUserResponse::Ok(Json(RespVO::from(&user))),
            Err(_) => {
                log::error!("update error");
                UpdateUserResponse::InnerError
            }
        }
    }
}