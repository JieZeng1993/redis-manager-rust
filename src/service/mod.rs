use rbatis::rbatis::Rbatis;

use cache_service::CacheService;

pub use crate::config::app_config::ApplicationConfig;

pub mod cache_service;
pub mod mem_service;
pub mod redis_service;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,

    //service
    // pub sys_res_service: SysResService,
    // pub sys_user_service: SysUserService,
    // pub sys_role_service: SysRoleService,
    // pub sys_role_res_service: SysRoleResService,
    // pub sys_user_role_service: SysUserRoleService,
    // pub sys_dict_service: SysDictService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        match config.cache_type.as_str() {
            "mem" => {
                println!("[abs_admin] cache_type: mem");
            }
            "redis" => {
                println!("[abs_admin] cache_type: redis");
            }
            e => {
                panic!("[abs_admin] unsupport of cache_type: \"{}\"", e);
            }
        }
        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build();
        let rabits = tokio::task::block_in_place(||{
            tokio::runtime::Handle::current().block_on(async {
                crate::mapper::init_rbatis(&config).await
            })
        });

        ServiceContext {
            rbatis: rabits,
            cache_service: CacheService::new(&config),
            // sys_res_service: SysResService {},
            // sys_user_service: SysUserService {},
            // sys_role_service: SysRoleService {},
            // sys_role_res_service: SysRoleResService {},
            // sys_user_role_service: SysUserRoleService {},
            // sys_dict_service: SysDictService {},
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}


// /// rabtis初始化
// pub async fn init_rbatis() -> Rbatis {
//     fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
//
//     log::info!("linking database...");
//
//     let rb = Rbatis::new();
//     rb.link("mysql://root:123456@mubuntu:3306/test")
//         .await
//         .unwrap();
//
//     log::info!("linked database...");
//     return rb;
// }
