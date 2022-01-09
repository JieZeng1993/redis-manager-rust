use std::time::Duration;

use async_trait::async_trait;
use log::{Level, log};
use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::mix::error::{Error, Result};
use crate::service::cache_service::ICacheService;

///Redis缓存服务
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(host: String, port: u16, db: i64, username: Option<String>, password: Option<String>) -> Self {
        log!(Level::Info,"connect redis start");
        let client = redis::Client::open(redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(host, port),
            redis: redis::RedisConnectionInfo {
                db,
                username,
                password,
            },
        }).unwrap();
        log!(Level::Info,"connect redis success");
        log!(Level::Info,"init cache finish");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<redis::aio::Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(crate::mix::error::Error::from(err));
        }
        return Ok(conn.unwrap());
    }
}

#[async_trait]
impl ICacheService for RedisService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        return self.set_string_ex(k, v, None).await;
    }

    async fn get_string(&self, k: &str) -> Result<String> {
        let mut conn = self.get_conn().await?;
        let result: redis::RedisResult<Option<String>> =
            redis::cmd("GET").arg(&[k]).query_async(&mut conn).await;
        match result {
            Ok(v) => {
                return Ok(v.unwrap_or(String::new()));
            }
            Err(e) => {
                return Err(Error::from(format!(
                    "RedisService get_string({}) fail:{}",
                    k,
                    e.to_string()
                )));
            }
        }
    }

    ///set_string 自动过期
    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        let mut conn = self.get_conn().await?;
        if ex.is_none() {
            return match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            };
        } else {
            return match redis::cmd("SET")
                .arg(&[k, v, "EX", &ex.unwrap().as_secs().to_string()])
                .query_async(&mut conn)
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            };
        }
    }

    ///set_string 自动过期
    async fn ttl(&self, k: &str) -> Result<i64> {
        let mut conn = self.get_conn().await?;
        return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisService ttl fail:{}",
                e.to_string()
            ))),
        };
    }
}
