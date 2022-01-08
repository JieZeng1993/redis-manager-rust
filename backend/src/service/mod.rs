use std::io;
use std::io::stdout;

use log::{Level, log};
use rbatis::rbatis::Rbatis;
use tracing_subscriber::{fmt, Layer, Registry};
use tracing_subscriber::filter::FilterFn;
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use cache_service::CacheService;

pub use crate::config::app_config::ApplicationConfig;
use crate::service::redis_info_service::RedisInfoService;
use crate::service::redis_node_info_service::RedisNodeInfoService;

pub mod cache_service;
mod user1_service;
pub mod user_service;
pub mod cache_impl;
pub mod redis_info_service;
pub mod redis_node_info_service;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    pub user1_service: user1_service::User1Service,
    pub user_service: user_service::UserService,
    pub redis_info_service: redis_info_service::RedisInfoService,
    pub redis_node_info_service: redis_node_info_service::RedisNodeInfoService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        println!("init");
        let config = ApplicationConfig::default();

        //init log
        std::fs::create_dir_all(&config.log_dir);

        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
        let file_appender = tracing_appender::rolling::hourly(&config.log_dir, "prefix.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // tracing_subscriber::fmt::Subscriber::builder()
        //     .with_max_level(str_to_log_level(&config.log_level))
        //     .finish()
        //     .with(tracing_subscriber::fmt::Layer::default()
        //         .with_writer(non_blocking))
        //     .init();

        // let collector =  tracing_subscriber::registry()
        //     .with( Subscriber::new()
        //         .with_writer(stdout())
        //         .with_target(false))
        //     .with( Subscriber::new()
        //         .with_writer(non_blocking)
        //         .with_target(false));
        //
        // tracing::collect::set_global_default(collector).expect("unable to set tracing collector");

        //过滤
        let err_filter = FilterFn::new(|metadata| {
            true
        });
        let info_filter = FilterFn::new(|metadata| true);

        let err = fmt::Layer::new()
            .with_writer(std::io::stdout());
        let info = fmt::Layer::new()
            .with_writer(non_blocking);

        let subscriber = Registry::default()
            .with(info.with_filter(info_filter))
            .with(err.with_filter(err_filter));

        tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");

        log!(Level::Info,"init log finish");

        if config.debug {
            log!(Level::Info,"debug_mode is enable!");
        } else {
            log!(Level::Info,"release_mode is enable!");
        }

        let rabits = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                crate::mapper::init_rbatis(&config).await
            })
        });

        ServiceContext {
            rbatis: rabits,
            cache_service: CacheService::new(&config),
            config,
            user1_service: user1_service::User1Service {},
            user_service: user_service::UserService {},
            redis_info_service: RedisInfoService {},
            redis_node_info_service: RedisNodeInfoService {},
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}


fn str_to_log_level(arg: &str) -> tracing::Level {
    return match arg {
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        "trace" => tracing::Level::TRACE,
        "info" => tracing::Level::INFO,
        "debug" => tracing::Level::DEBUG,
        _ => tracing::Level::INFO,
    };
}