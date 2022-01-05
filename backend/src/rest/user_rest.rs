use poem_openapi::{
    ApiResponse,
    Object,
    // types::{Email, Password},
    OpenApi, OpenApiService, param::Path, payload::Json, Tags,
};

use crate::config::auth::Session;
use crate::domain::dto::user::{UserLoginDto, UserUpdateDto};
use crate::domain::vo::RespVO;
use crate::domain::vo::user::{LoginVo, UserVo};
use crate::domain::vo::user1::User1Vo;
use crate::mix::error::Error;
use crate::mix::error::Result;
use crate::service::CONTEXT;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}


pub struct UserRest;

#[derive(ApiResponse)]
enum FindUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<RespVO<UserVo>>),
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

#[derive(ApiResponse)]
enum LoginResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<RespVO<LoginVo>>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}

#[OpenApi]
impl UserRest {
    /// 登录
    #[oai(path = "/user/login", method = "post", tag = "ApiTags::User")]
    async fn login(&self, login_dto: Json<UserLoginDto>) -> LoginResponse {
        let user = CONTEXT.user_service.login(login_dto.name.as_ref().unwrap(), login_dto.password.as_ref().unwrap()).await;
        match user {
            Ok(user) => {
                match user {
                    Some(user) => {
                        LoginResponse::Ok(Json(RespVO::from(&user)))
                    }
                    None => {
                        log::error!("user not found");
                        LoginResponse::NotFound
                    }
                }
            }
            Err(_) => {
                log::error!("user find error");
                LoginResponse::InnerError
            }
        }
    }

    #[oai(path = "/user/:id", method = "get", tag = "ApiTags::User")]
    async fn find_user(&self, id: Path<i32>) -> FindUserResponse {
        let user = CONTEXT.user_service.find_by_id(id.0).await;
        deal_find_user(user)
    }

    ///获取当前已登录的用户信息
    #[oai(path = "/user/loginUser", method = "get", tag = "ApiTags::User")]
    async fn login_user(&self, session: &Session) -> FindUserResponse {
        let user = CONTEXT.user_service.find_by_id(session.id.unwrap()).await;
        deal_find_user(user)
    }

    #[oai(path = "/user", method = "put", tag = "ApiTags::User")]
    async fn update_user(&self, user_update_dto: Json<UserUpdateDto>, session: &Session) -> UpdateUserResponse {
        let user = CONTEXT.user_service.update(user_update_dto.0, session.id).await;
        match user {
            Ok(user) => UpdateUserResponse::Ok(Json(RespVO::from(&user))),
            Err(_) => {
                log::error!("update error");
                UpdateUserResponse::InnerError
            }
        }
    }
}

fn deal_find_user(user: Result<Option<UserVo>>) -> FindUserResponse {
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