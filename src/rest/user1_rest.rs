use poem_openapi::{
    param::Path,
    payload::Json,
    // types::{Email, Password},
    ApiResponse, Object, OpenApi, OpenApiService, Tags,
};
use crate::domain::entity::user1::User1;

use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User1,
}


#[derive(Default)]
pub struct User1Rest {}

#[derive(ApiResponse)]
enum FindUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<RespVO<User1>>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[OpenApi]
impl User1Rest {
    /// Find user by id
    #[oai(path = "/user1/:user_id", method = "get", tag = "ApiTags::User1")]
    async fn find_user(&self, user_id: Path<i64>) -> FindUserResponse {
        let user = CONTEXT.user1_service.find(user_id.0).await;
        match user {
            Ok(user) => match user {
                Some(user) => FindUserResponse::Ok(Json(RespVO::from(&user))),
                None => FindUserResponse::NotFound,
            },
            Err(_) => FindUserResponse::NotFound,
        }
    }
}