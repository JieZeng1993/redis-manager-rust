#[macro_use]
extern crate rbatis;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    ApiResponse,
    Object,
    OpenApi,
    OpenApiService, param::Path, payload::Json, Tags, types::{Email, Password},
};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use slab::Slab;
use tokio::sync::Mutex;
use tracing::{Level, span};
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};
use redis_manager_rust::config::log;
use redis_manager_rust::rest::user1_rest::User1Rest;
use redis_manager_rust::rest::user2_rest::User2Rest;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

/// Create user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct User {
    /// Id
    #[oai(read_only)]
    id: i64,
    /// Name
    #[oai(validator(max_length = 64))]
    name: String,
    /// Password
    #[oai(validator(max_length = 32))]
    password: Password,
    email: Email,
}

/// Update user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UpdateUser {
    /// Name
    name: Option<String>,
    /// Password
    password: Option<Password>,
}

#[derive(ApiResponse)]
enum CreateUserResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<i64>),
}

#[derive(ApiResponse)]
enum FindUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<User>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum DeleteUserResponse {
    /// Returns when the user is successfully deleted.
    #[oai(status = 200)]
    Ok,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum UpdateUserResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(Default)]
struct Api {
    users: Mutex<Slab<User>>,
}

#[OpenApi]
impl Api {
    /// Create a new user
    #[oai(path = "/users", method = "post", tag = "ApiTags::User")]
    async fn create_user(&self, user: Json<User>) -> CreateUserResponse {
        let mut users = self.users.lock().await;
        let id = users.insert(user.0) as i64;

        CreateUserResponse::Ok(Json(id))
    }

    /// Find user by id
    #[oai(path = "/users/:user_id", method = "get", tag = "ApiTags::User")]
    async fn find_user(&self, user_id: Path<i64>) -> FindUserResponse {
        let users = self.users.lock().await;
        match users.get(user_id.0 as usize) {
            Some(user) => FindUserResponse::Ok(Json(user.clone())),
            None => FindUserResponse::NotFound,
        }
    }

    /// Delete user by id
    #[oai(path = "/users/:user_id", method = "delete", tag = "ApiTags::User")]
    async fn delete_user(&self, user_id: Path<i64>) -> DeleteUserResponse {
        let mut users = self.users.lock().await;
        let user_id = user_id.0 as usize;
        if users.contains(user_id) {
            users.remove(user_id);
            DeleteUserResponse::Ok
        } else {
            DeleteUserResponse::NotFound
        }
    }

    /// Update user by id
    #[oai(path = "/users/:user_id", method = "put", tag = "ApiTags::User")]
    async fn put_user(&self, user_id: Path<i64>, update: Json<UpdateUser>) -> UpdateUserResponse {
        let mut users = self.users.lock().await;
        match users.get_mut(user_id.0 as usize) {
            Some(user) => {
                if let Some(name) = update.0.name {
                    user.name = name;
                }
                if let Some(password) = update.0.password {
                    user.password = password;
                }
                UpdateUserResponse::Ok
            }
            None => UpdateUserResponse::NotFound,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("开始初始化");
    log::init_log();
    println!("开始初始化1");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    //发送链路数据
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("redis-manager-rust")
        .install_simple().unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(opentelemetry)
        .try_init().unwrap();

    let api = OpenApiService::new((User1Rest, User2Rest), "api", "1.1")
        .server("http://localhost:3000/api");
    // let api_service =
    //     OpenApiService::new(Api::default(), "Users", "1.0").server("http://localhost:3000/api");
    let swagger_ui = api.swagger_ui();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(Route::new().nest("/api", api)
            .nest("/swagger_ui", swagger_ui))
        .await
}
