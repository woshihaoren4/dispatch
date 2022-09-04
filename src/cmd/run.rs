use crate::conf::{Config, DataSourceDriver};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use wd_run::{CmdInfo, Context};
use crate::infra::{DataSourceCenter, MongoClient};

pub struct AppRun {}

impl AppRun {
    pub fn new() -> Self {
        AppRun {}
    }
    pub fn args(&self) -> CmdInfo {
        wd_run::CmdInfo::new("run", "running application").add(
            "c",
            "./src/conf/config.toml",
            "config file path",
        )
    }

    pub async fn load_config_ctx(ctx: &Context) -> Config {
        let path = ctx.copy::<_, String>("c").await.unwrap();
        wd_log::res_panic!(Config::from_file_by_path(&path);"load config failed from:({})",path)
    }

    pub async fn init_database_source(cfg:Config)->anyhow::Result<Arc<DataSourceCenter>>{
        let mut dsc = DataSourceCenter::new();
        match cfg.data_source.driver {
            DataSourceDriver::Mysql => {}
            DataSourceDriver::Postgresql => {}
            DataSourceDriver::Mongo(m) => {
                let url = m.url.clone();
                dsc = dsc.register_mongo(MongoClient::new(cfg.server.name,m).await?);
                wd_log::log_info_ln!("init mongodb success url:{}",url);
            }
        }

        return Ok(Arc::new(dsc))
    }

}

impl wd_run::EventHandle for AppRun {
    fn handle(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        return Box::pin(async move {
            //加载配置文件
            let cfg = AppRun::load_config_ctx(&ctx).await;
            wd_log::log_info_ln!("config load success: {}", cfg.to_string());

            //初始化数据源
            AppRun::init_database_source(cfg.clone()).await.unwrap();
            //启动服务
            crate::app::application_run(ctx.clone(), cfg).await;

            return ctx;
        });
    }
}
