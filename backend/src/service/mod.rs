use std::borrow::Borrow;
use std::io::stdout;

use log::{Level, log};
use rbatis::rbatis::Rbatis;
use tracing_subscriber::filter::{FilterFn, LevelFilter};
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use cache_service::CacheService;

pub use crate::config::app_config::ApplicationConfig;
use crate::config::app_config::CONFIG_CONTEXT;
use crate::service::redis_info_service::RedisInfoService;
use crate::service::redis_node_info_service::RedisNodeInfoService;

pub mod cache_service;
mod user1_service;
pub mod user_service;
pub mod cache_impl;
pub mod redis_info_service;
pub mod redis_node_info_service;

pub struct ServiceContext<'a> {
    pub config: &'a ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    pub user1_service: user1_service::User1Service,
    pub user_service: user_service::UserService,
    pub redis_info_service: redis_info_service::RedisInfoService,
    pub redis_node_info_service: redis_node_info_service::RedisNodeInfoService,
}

impl Default for ServiceContext<'static> {
    fn default() -> Self {
        println!("init");
        let config = &CONFIG_CONTEXT;

        if config.debug {
            log!(Level::Info,"debug_mode is enable!");
        } else {
            log!(Level::Info,"release_mode is enable!");
        }

        let rabits = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                crate::mapper::init_rbatis(config).await
            })
        });

        ServiceContext {
            rbatis: rabits,
            cache_service: CacheService::new(config),
            config,
            user1_service: user1_service::User1Service {},
            user_service: user_service::UserService {},
            redis_info_service: RedisInfoService {},
            redis_node_info_service: RedisNodeInfoService {},
        }
    }
}

lazy_static! {
    pub static ref SERVICE_CONTEXT: ServiceContext<'static> = ServiceContext::default();
}


pub fn str_to_log_level(arg: &str) -> LevelFilter {
    match arg.to_lowercase().borrow() {
        "warn" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        "trace" => LevelFilter::TRACE,
        "info" => LevelFilter::INFO,
        "debug" => LevelFilter::DEBUG,
        _ => LevelFilter::INFO,
    }
}