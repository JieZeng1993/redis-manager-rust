use log::{Level, log};
use poem::{Endpoint, Error, FromRequest, Middleware, Request, RequestBody};
use poem::http::header::ToStrError;
use poem::http::StatusCode;
use poem::web::headers;
use poem::web::headers::authorization::Basic;
use poem::web::headers::HeaderMapExt;
use poem_openapi::Object;
use rand::{distributions::Alphanumeric, Rng, rngs::OsRng};
use serde::{Deserialize, Serialize};

use crate::domain::vo::user::LoginVo;
use crate::service::CONTEXT;

const AUTHORIZATION_KEY: &'static str = "Authorization";

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[oai(rename_all = "camelCase")]
pub struct Session {
    pub id: Option<i32>,
    pub name: Option<String>,
}

pub struct HeaderAuth {
    pub header_key: String,
}

impl<E: Endpoint> Middleware<E> for HeaderAuth {
    type Output = HeaderAuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        HeaderAuthEndpoint {
            ep,
            header_key: self.header_key.clone(),
        }
    }
}

pub struct HeaderAuthEndpoint<E> {
    ep: E,
    header_key: String,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for HeaderAuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let uri = req.uri();
        log!(Level::Info,"request uri:{}", uri);

        if uri.eq("/api/user/login") || uri.eq("/favicon.ico") {
            //登录接口跳过鉴权
            return self.ep.call(req).await;
        } else if uri.to_string().starts_with("/swagger_ui") {
            return self.ep.call(req).await;
        }

        let auth = req.headers().get(&self.header_key);
        match auth {
            Some(auth) => {
                match auth.to_str() {
                    Ok(auth) => {
                        let authorization = CONTEXT.cache_service.get_json::<Session>(&format!("{}:{}", AUTHORIZATION_KEY, auth)).await;
                        match authorization {
                            Ok(authorization) => {
                                req.extensions_mut().insert(authorization);
                                self.ep.call(req).await
                            }
                            Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED))
                        }
                    }
                    Err(_) => { Err(Error::from_status(StatusCode::UNAUTHORIZED)) }
                }
            }
            None => Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }
}

#[async_trait::async_trait]
impl<'a> FromRequest<'a> for &'a Session {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        Ok(req
            .extensions()
            .get::<Session>()
            .expect("To use the `Session` extractor, the `CookieSession` middleware is required."))
    }
}

/// 获取唯一数
fn get_unique_id() -> String {
    let value = std::iter::repeat(())
        .map(|()| OsRng.sample(Alphanumeric))
        .take(32)
        .collect::<Vec<_>>();
    String::from_utf8(value).unwrap_or_default()
}

///返回token
pub async fn get_session_id(login_vo: &LoginVo) -> String {
    let session_id = get_unique_id();

    //存储token
    let set_result = CONTEXT.cache_service.set_json::<Session>(&format!("{}:{}", AUTHORIZATION_KEY, session_id),
                                                               &Session {
                                                                   id: login_vo.id,
                                                                   name: login_vo.name.clone(),
                                                               }).await;
    log!(Level::Info, "set_result: {:?}", set_result);
    return session_id;
}