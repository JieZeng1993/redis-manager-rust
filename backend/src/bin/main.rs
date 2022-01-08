#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate serde;

use log::{Level, log};
// use opentelemetry::{
//     Context,
//     global,
//     KeyValue,
//     sdk::{propagation::TraceContextPropagator, trace::Tracer}, trace::{FutureExt, SpanKind, TraceContextExt, Tracer as _},
// };
use poem::{EndpointExt, listener::TcpListener,
           // middleware::{OpenTelemetryMetrics, OpenTelemetryTracing},
           Route, Server};
use poem_openapi::{
    ApiResponse,
    Object,
    OpenApi,
    OpenApiService, param::Path, payload::Json, Tags, types::{Email, Password},
};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use tokio::sync::Mutex;
use tracing::span;
use tracing_subscriber::{fmt, Layer};
use tracing_subscriber::filter::{FilterFn, LevelFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;
// fn init_tracer() -> Tracer {
//     global::set_text_map_propagator(TraceContextPropagator::new());
//     opentelemetry_jaeger::new_pipeline()
//         .with_service_name("redis-manager-rust")
//         .with_collector_endpoint("http://localhost:14268/api/traces")
//         .install_simple()
//         .unwrap()
// }
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use redis_manager_rust::config::app_config::CONFIG_CONTEXT;
use redis_manager_rust::config::auth::HeaderAuth;
use redis_manager_rust::rest::redis_info_rest::RedisInfoRest;
use redis_manager_rust::rest::user1_rest::User1Rest;
use redis_manager_rust::rest::user_rest::UserRest;
use redis_manager_rust::service::{SERVICE_CONTEXT, str_to_log_level};

/*use tracing::info;
use tracing_subscriber::{filter::LevelFilter, prelude::*};*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&CONFIG_CONTEXT.log_dir);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let level_filter = str_to_log_level(&CONFIG_CONTEXT.log_level);

    let file_appender = tracing_appender::rolling::hourly(&CONFIG_CONTEXT.log_dir, "redis-manager-rust.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    //这个方法需要在这个地方调用，不然日志会不全
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_filter(level_filter)
        )
        .with(tracing_subscriber::fmt::layer()
            .with_filter(level_filter)
        )
        .init();

    log!(Level::Info,"init log finish,log file path:{}", SERVICE_CONTEXT.config.log_dir);

    let api = OpenApiService::new((User1Rest, UserRest, RedisInfoRest), "api", "1.1")
        .server("http://localhost:3001/api");
    let swagger_ui = api.swagger_ui();

    // let tracer = init_tracer();

    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(Route::new().nest("/api", api)
                 .nest("/swagger_ui", swagger_ui)
                 .with(HeaderAuth {
                     header_key: "Authorization".to_string()
                 })
             // .data(tracer.clone())
             // .with(OpenTelemetryMetrics::new())
             // .with(OpenTelemetryTracing::new(tracer))
        )
        .await
}
