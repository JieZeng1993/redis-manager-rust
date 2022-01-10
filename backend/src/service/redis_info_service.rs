use std::borrow::Borrow;
use std::error::Error;

use log::{Level, log};
use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use rbatis::plugin::page::{Page, PageRequest};
use redis::{AsyncCommands, ErrorKind, RedisFuture, RedisResult};

use crate::config::auth;
use crate::domain::dto::{convert_rbatis_page_request, convert_rbatis_page_resp_and_convert};
use crate::domain::dto::redis_info::{RedisInfoRelatedInfoRtDto, RedisPageDto};
use crate::domain::dto::user1::User1UpdateDto;
use crate::domain::dto::user::UserUpdateDto;
use crate::domain::entity::redis_info::*;
use crate::domain::vo::redis_info::*;
use crate::domain::vo::redis_node_info::RedisNodeInfoVo;
use crate::domain::vo::RespVO;
use crate::mix::error::Result;
use crate::service::SERVICE_CONTEXT;
use crate::util::string::IsEmpty;

pub struct RedisInfoService {}

///大key
lazy_static!(
    pub static  ref REDIS_ID2_REDIS_CLIENT_MAP_MAP: std::collections::HashMap<i32, std::collections::HashMap<String,redis::Client>> = std::collections::HashMap::new();
);

impl RedisInfoService {
    ///根据id查找vo
    pub async fn find_by_id(&self, id: i32) -> Result<Option<RedisInfoVo>> {
        let redis_info = self.do_find_by_id(id).await?;
        match redis_info {
            Some(redis_info) => {
                let mut redis_info_vo = convert_redis_info2redis_info_vo(redis_info);
                let redis_node_info_vos = SERVICE_CONTEXT.redis_node_info_service.find_by_redis_info_id(redis_info_vo.id.unwrap()).await?;
                redis_info_vo.redis_node_infos = Some(redis_node_info_vos);
                Ok(Some(redis_info_vo))
            }
            None => Ok(None)
        }
    }

    ///redis info分页
    pub async fn page(&self, redis_page_dto: RedisPageDto) -> Result<RespVO<Vec<RedisInfoVo>>> {
        let wrapper = SERVICE_CONTEXT
            .rbatis
            .new_wrapper()
            .do_if(!redis_page_dto.name.is_empty(), |w| w.like(RedisInfo::name(), &redis_page_dto.name))
            .do_if(!redis_page_dto.host.is_empty(), |w| w.like(RedisInfo::host(), &redis_page_dto.host))
            .do_if(redis_page_dto.port.is_some(), |w| w.eq(RedisInfo::port(), &redis_page_dto.port))
            .do_if(redis_page_dto.cluster_type.is_some(), |w| w.eq(RedisInfo::cluster_type(), &redis_page_dto.cluster_type))
            .do_if(redis_page_dto.id.is_some(), |w| w.eq(RedisInfo::id(), &redis_page_dto.id))
            .do_if(redis_page_dto.update_time_begin.is_some(), |w| w.ge(RedisInfo::update_time(), &redis_page_dto.update_time_begin))
            .do_if(redis_page_dto.update_time_end.is_some(), |w| w.le(RedisInfo::update_time(), &redis_page_dto.update_time_end))
            .order_by(false, &[RedisInfo::update_time()]);

        let data = SERVICE_CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<RedisInfo>(wrapper, &convert_rbatis_page_request(redis_page_dto))
            .await?;

        Ok(convert_rbatis_page_resp_and_convert(data, convert_redis_info2redis_info_vo))
    }

    /// 根据id查找entity
    pub async fn do_find_by_id(&self, id: i32) -> Result<Option<RedisInfo>> {
        let wrapper = SERVICE_CONTEXT.rbatis.new_wrapper().eq(RedisInfo::id(), id);
        return Ok(SERVICE_CONTEXT.rbatis.fetch_by_wrapper(wrapper).await?);
    }

    ///实时查询节点相关信息
    pub async fn related_info_rt(&self, redis_info_related_info_rt_dto: RedisInfoRelatedInfoRtDto) -> Result<Vec<RedisNodeInfoVo>> {
        let mut username = redis_info_related_info_rt_dto.username;
        let mut password = redis_info_related_info_rt_dto.password;
        let mut old_username = None;
        let mut old_password = None;

        if redis_info_related_info_rt_dto.id.is_some() {
            let redis_info_option = self.do_find_by_id(redis_info_related_info_rt_dto.id.unwrap()).await?;
            if redis_info_option.is_some() {
                let redis_info = redis_info_option.unwrap();
                old_username = redis_info.username;
                old_password = redis_info.password;
            }
        }

        if username.is_empty() {
            if old_username.is_empty() {
                //用户未输入，表示没有用户名
                username = None;
            } else {
                //用户未输入，则使用数据库中的用户名
                username = old_username;
            }
        }

        if password.is_empty() {
            if old_password.is_empty() {
                //用户未输入，表示没有密码
                password = None;
            } else {
                //用户未输入，则使用数据库中的密码信息
                password = old_password;
            }
        }
        log!(Level::Info, "username:{:?}, password:{:?}",username,password);
        let mut client = redis::Client::open(redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(redis_info_related_info_rt_dto.host.clone().unwrap(), redis_info_related_info_rt_dto.port.unwrap()),
            redis: redis::RedisConnectionInfo {
                db: 0,
                username,
                password,
            },
        }).unwrap();

        let mut connection = get_conn(client).await?;
        //主要是测试密码是否正确
        let result: RedisResult<String> = connection.get("TEST-CONNECTION").await;

        match result {
            Err(result) => {
                match result.kind() {
                    redis::ErrorKind::AuthenticationFailed => { return Err(crate::mix::error::Error::from("redisInfo.connection.error.authFail")); }
                    redis::ErrorKind::IoError => { return Err(crate::mix::error::Error::from("redisInfo.connection.error.connection")); }
                    _ => {}
                }
            }
            _ => {}
        }

        //连接没有问题
        //查询集群信息
        let cluster_nodes: RedisResult<String> = redis::cmd("cluster").arg("nodes").query_async(&mut connection).await;

        let cluster_nodes_ref = cluster_nodes.as_ref();
        if cluster_nodes_ref.is_err() && cluster_nodes_ref.err().unwrap().detail().unwrap_or("").contains("cluster support disabled") {
            //单节点，这是正常的响应
            return Ok(vec![RedisNodeInfoVo {
                id: redis_info_related_info_rt_dto.id,
                redis_info_id: None,
                //单机，没有这个属性
                node_id: Some("master_id".to_string()),
                //单机，没有这个属性
                master_id: Some("master_id".to_string()),
                host: redis_info_related_info_rt_dto.host,
                port: redis_info_related_info_rt_dto.port,
                node_role: Some("MASTER".to_string()),
                node_status: Some("CONNECTED".to_string()),
                //总共1
                slot_from: Some(0),
                slot_to: Some(16383),
                create_time: None,
                create_id: None,
                update_time: None,
                update_id: None,
            }]);
        }
        //单节点
        let cluster_nodes = deal_redis_result(cluster_nodes)?;

        //只需要处理哨兵和cluster，目前只处理cluster
        log!(Level::Info,"cluster_nodes:{}",cluster_nodes);
        let cluster_nodes = cluster_nodes.split("\n").into_iter().map(|cluster_node_info|
            //转换
            RedisNodeInfoVo {
                id: None,
                redis_info_id: None,
                node_id: Some("查询出来的node id".to_string()),
                master_id: Some("查询出来的master id".to_string()),
                host: Some("查询出来的host".to_string()),
                port: Some(6380),
                node_role: Some("MASTER".to_string()),
                node_status: Some("CONNECTED".to_string()),
                slot_from: Some(0),
                slot_to: Some(155),
                create_time: None,
                create_id: None,
                update_time: None,
                update_id: None,
            }
        ).collect();
        return Ok(cluster_nodes);
    }
}

pub fn deal_redis_result<T>(result: RedisResult<T>) -> Result<T> {
    if result.is_ok() {
        return Ok(result.unwrap());
    }
    let mut code = "";
    //处理Error
    match result.err().unwrap().kind() {
        redis::ErrorKind::AuthenticationFailed => { code = "authError"; }
        redis::ErrorKind::ResponseError => { code = "responseError"; }
        redis::ErrorKind::TypeError => { code = "typeError"; }
        redis::ErrorKind::ExecAbortError => { code = "execAbortError"; }
        redis::ErrorKind::BusyLoadingError => { code = "busyLoadingError"; }
        redis::ErrorKind::NoScriptError => { code = "noScriptError"; }
        redis::ErrorKind::InvalidClientConfig => { code = "invalidClientConfig"; }
        redis::ErrorKind::Moved => { code = "moved"; }
        redis::ErrorKind::Ask => { code = "ask"; }
        redis::ErrorKind::TryAgain => { code = "tryAgain"; }
        redis::ErrorKind::ClusterDown => { code = "clusterDown"; }
        redis::ErrorKind::CrossSlot => { code = "crossSlot"; }
        redis::ErrorKind::MasterDown => { code = "masterDown"; }
        redis::ErrorKind::IoError => { code = "ioError"; }
        redis::ErrorKind::ClientError => { code = "clientError"; }
        redis::ErrorKind::ExtensionError => { code = "extensionError"; }
        redis::ErrorKind::ReadOnly => { code = "readOnly"; }
        _ => { return Err(crate::mix::error::Error::from("redisInfo.connection.error")); }
    }


    Err(crate::mix::error::Error::from(format!("redisInfo.connection.error.connection.{}", code)))
}

pub async fn get_conn(client: redis::Client) -> Result<redis::aio::MultiplexedConnection> {
    let conn = client.get_multiplexed_async_connection().await;
    if conn.is_ok() {
        return Ok(conn.unwrap());
    }
    //处理error
    let err = conn.err().unwrap();

    log!(Level::Error,"RedisService connect fail:{}", err);

    match err.kind() {
        redis::ErrorKind::IoError => { Err(crate::mix::error::Error::from("redisInfo.connection.error.connect")) }
        redis::ErrorKind::AuthenticationFailed => { Err(crate::mix::error::Error::from("redisInfo.connection.error.authFail")) }
        _ => { Err(crate::mix::error::Error::from(err.detail().unwrap_or("redisInfo.connection.error"))) }
    }
}

// let test_result = connection.get("1");
// log!(Level::Error,"RedisService connect test_result:{:?}", test_result);


mod test {
    use log::{Level, log};
    use redis::{AsyncCommands, ErrorKind, RedisResult};
    use tokio_test::block_on;

    use crate::service::redis_info_service::get_conn;

    #[test]
    fn test_split() {
        let str = "String1 1\r\nString2 2\r\nString3 3".to_string();

        for x in str.split("\n") {
            println!("{:?}", x);
        }
        let collect: Vec<String> = str.split("\n").into_iter().map(|str| str.to_owned() + "234").collect();
        println!("map: {:?}", collect);
        //
        // let strings = vec!["tofu", "93", "18"];
        // let numbers: Vec<_> = strings
        //     .into_iter()
        //     .map(|s| s.parse::<i32>())
        //     .collect();
        // println!("Results: {:?}", numbers);
    }

    //先解析这个字符串
    // cluster_nodes:814c27a6600b5ab69c6f79dc957268d0fec1da6c 172.31.157.81:6376@16376 slave 7ea3061c771a2ee507b0259a5641b646ea793806 0 1641829841000 2 connected
    // 7ea3061c771a2ee507b0259a5641b646ea793806 172.31.157.81:6372@16372 master - 0 1641829841627 2 connected 5461-10922
    // ed4f3ba1aa472b4452f038280ce96bede653549c 172.31.157.81:6373@16373 master - 0 1641829839000 3 connected 10923-16383
    // f353502c4e08b0aa11704b1ca11d2e780745d998 172.31.157.81:6374@16374 slave ed4f3ba1aa472b4452f038280ce96bede653549c 0 1641829840625 3 connected
    // a09d6f02e736f433664485f64ded693942eb80a7 172.31.157.81:6375@16375 myself,slave 2740fde9a37aca6e231e04b2f30653be48653adb 0 1641829840000 1 connected
    // 2740fde9a37aca6e231e04b2f30653be48653adb 172.31.157.81:6371@16371 master - 0 1641829837617 1 connected 0-5460
    #[test]
    fn test_connection() {
        block_on(async {
            let mut client = redis::Client::open(redis::ConnectionInfo {
                addr: redis::ConnectionAddr::Tcp("localhost".to_string(), 6379),
                redis: redis::RedisConnectionInfo {
                    db: 0,
                    username: None,
                    password: Some("123456".to_string()),
                },
            }).unwrap();

            let mut connection = get_conn(client).await;
            match connection {
                Ok(mut connection) => {
                    let result: RedisResult<String> = redis::cmd("get").arg("TEST-CONNECTION").query_async(&mut connection).await;
                    // let result: RedisResult<String> = connection.get("12").await;
                    println!("{:?}", result);
                    // match result {
                    //     Err(result) => {
                    //         println!("{}", result);
                    //         match result.kind() {
                    //             ErrorKind::AuthenticationFailed => { println!("auth"); }
                    //             ErrorKind::ResponseError => {}
                    //             ErrorKind::TypeError => { println!("TypeError"); }
                    //             ErrorKind::ExecAbortError => {}
                    //             ErrorKind::BusyLoadingError => {}
                    //             ErrorKind::NoScriptError => {}
                    //             ErrorKind::InvalidClientConfig => {}
                    //             ErrorKind::Moved => {}
                    //             ErrorKind::Ask => {}
                    //             ErrorKind::TryAgain => {}
                    //             ErrorKind::ClusterDown => {}
                    //             ErrorKind::CrossSlot => {}
                    //             ErrorKind::MasterDown => {}
                    //             ErrorKind::IoError => { println!("IoError"); }
                    //             ErrorKind::ClientError => {}
                    //             ErrorKind::ExtensionError => {}
                    //             ErrorKind::ReadOnly => {}
                    //             _ => {}
                    //         }
                    //     }
                    //     _ => {
                    let info: RedisResult<redis::InfoDict> = redis::cmd("info").query_async(&mut connection).await;
                    if (info.is_err()) {
                        println!("info error:{:?}", info.as_ref().err());
                    }
                    let info = info.unwrap();
                    println!("info result:{:?}", info);
                    //     }
                    // }
                }
                _ => {}
            }
        });
    }
}