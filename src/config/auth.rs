use poem::{Endpoint, Error, Middleware, Request};
use poem::http::header::ToStrError;
use poem::http::StatusCode;
use poem::web::headers;
use poem::web::headers::authorization::Basic;
use poem::web::headers::HeaderMapExt;
use crate::service::CONTEXT;

struct HeaderAuth {
    header_key: String,
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

struct HeaderAuthEndpoint<E> {
    ep: E,
    header_key: String,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for HeaderAuthEndpoint<E> {
    type Output = E::Output;

    async fn call(self, req: Request) -> poem::Result<Self::Output> {
        let auth = req.headers().get(&self.header_key);
        match auth {
            Some(auth) => {
                match auth.to_str() {
                    Ok(auth) => {
                        let a = CONTEXT.cache_service.get_json( format!("{}", auth));
                        if auth.eq(&"sdcard".to_string()) {
                            self.ep.call(req).await
                        } else {
                            Err(Error::from_status(StatusCode::UNAUTHORIZED))
                        }
                    }
                    Err(_) => { Err(Error::from_status(StatusCode::UNAUTHORIZED)) }
                }
            }
            None => Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }
}