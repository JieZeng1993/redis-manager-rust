#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate serde;
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
use tracing::{debug, Level, span};
/*use tracing::info;
use tracing_subscriber::{filter::LevelFilter, prelude::*};*/
use tracing_subscriber::fmt;
use redis_manager_rust::config::auth::HeaderAuth;

use redis_manager_rust::config::log as rabit_log;
use redis_manager_rust::rest::user1_rest::User1Rest;
use redis_manager_rust::rest::user_rest::UserRest;

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
    debug!("开始初始化");
    rabit_log::init_log();
    debug!("开始初始化1");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let api = OpenApiService::new((User1Rest,UserRest), "api", "1.1")
        .server("http://localhost:3000/api");
    // let api_service =
    //     OpenApiService::new(Api::default(), "Users", "1.0").server("http://localhost:3000/api");
    let swagger_ui = api.swagger_ui();

    // let tracer = init_tracer();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(Route::new().nest("/api", api)
            .nest("/swagger_ui", swagger_ui)
                 .with( HeaderAuth{
                     header_key: "Authorization".to_string()
                 })
            // .data(tracer.clone())
            // .with(OpenTelemetryMetrics::new())
            // .with(OpenTelemetryTracing::new(tracer))
        )
        .await
}
