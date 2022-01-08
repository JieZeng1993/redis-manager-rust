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
use tracing_subscriber::fmt;

use redis_manager_rust::config::auth::HeaderAuth;
use redis_manager_rust::rest::redis_info_rest::RedisInfoRest;
use redis_manager_rust::rest::user1_rest::User1Rest;
use redis_manager_rust::rest::user_rest::UserRest;
use redis_manager_rust::service::CONTEXT;

/*use tracing::info;
use tracing_subscriber::{filter::LevelFilter, prelude::*};*/
// fn init_tracer() -> Tracer {
//     global::set_text_map_propagator(TraceContextPropagator::new());
//     opentelemetry_jaeger::new_pipeline()
//         .with_service_name("redis-manager-rust")
//         .with_collector_endpoint("http://localhost:14268/api/traces")
//         .install_simple()
//         .unwrap()
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let log_dir = &CONTEXT.config.log_dir;
    log!(Level::Info,"log file path:{}", log_dir);

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
