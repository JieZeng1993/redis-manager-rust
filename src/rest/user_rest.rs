use poem_openapi::{
    ApiResponse,
    Object,
    // types::{Email, Password},
    OpenApi, OpenApiService, param::Path, payload::Json, Tags,
};

use crate::domain::dto::user::UserLoginDto;
use crate::domain::vo::RespVO;
use crate::domain::vo::user1::User1Vo;
use crate::domain::vo::user::LoginVo;
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
                    Some(user)=>{
                        LoginResponse::Ok(Json(RespVO::from( &user)))
                    }
                    None=>{
                        log::error!("user not found");
                        LoginResponse::NotFound
                    }
                }
            },
            Err(_) => {
                log::error!("user find error");
                LoginResponse::InnerError
            }
        }
    }
}