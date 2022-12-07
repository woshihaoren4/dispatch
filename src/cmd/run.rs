use crate::app::schedule::Allocation;
use crate::conf::{Config, DataSourceDriver};
use crate::infra::client::{DataSourceCenter, MongoClient, Redis};
use crate::infra::election::{ElectionManager, MasterAndWorker};
use async_channel::{Receiver, Sender};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use wd_run::{CmdInfo, Context};

pub struct AppRun {
    sender: Sender<Box<dyn wd_run::EventHandle + Sync + Send + 'static>>,
}

impl AppRun {
    pub fn new() -> (
        Self,
        Receiver<Box<dyn wd_run::EventHandle + Sync + Send + 'static>>,
    ) {
        let (sender, receiver) = async_channel::unbounded();
        (AppRun { sender }, receiver)
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

    pub async fn init_database_source(cfg: Config) -> anyhow::Result<Arc<DataSourceCenter>> {
        let mut dsc = DataSourceCenter::new();
        //初始化数据库
        match cfg.data_source.driver {
            DataSourceDriver::Mysql => {}
            DataSourceDriver::Postgresql => {}
            DataSourceDriver::Mongo(m) => {
                let url = m.url.clone();
                dsc = dsc.register_mongo(MongoClient::new(cfg.server.name, m).await?);
                wd_log::log_info_ln!("init mongodb success url:{}", url);
            }
        }
        //初始化redis缓存
        let client = Redis::new(cfg.cache).await?;
        dsc = dsc.register_redis(client);

        return Ok(Arc::new(dsc));
    }
    pub fn init_schedule_entity(
        dsc: Arc<DataSourceCenter>,
    ) -> (impl MasterAndWorker, Arc<Allocation>) {
        let maw = crate::app::schedule::TaskDispatch::new(dsc).listen();
        let alloc = maw.alloc.clone();
        (maw, alloc)
    }
    // pub async fn add_exit_task<F:Future<Output=()>+Send+Sync+ 'static>(send:Sender<Box<dyn wd_run::EventHandle + Sync + Send+ 'static>>,f:F)->anyhow::Result<()>{
    //     send.send(Box::new(|x: Context| -> Pin<Box<dyn Future<Output = Context> + Send>> {
    //         Box::pin(async move {
    //             // f.await;
    //             wd_log::log_info_ln!("关闭功能待 wd_run 服务包升级");
    //             return x;
    //         })
    //     })).await?;Ok(())
    // }
    pub fn start_election_listen(
        maw: impl MasterAndWorker + 'static,
        dsc: Arc<DataSourceCenter>,
        cluster_name: String,
    ) -> ElectionManager {
        let e = dsc.get_election_impl(cluster_name);
        ElectionManager::build(e, maw)
    }
}

impl wd_run::EventHandle for AppRun {
    fn handle(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        let _exit_channel = self.sender.clone();
        return Box::pin(async move {
            //加载配置文件
            let cfg = AppRun::load_config_ctx(&ctx).await;
            wd_log::log_info_ln!("config load success: {}", cfg.to_string());
            //初始化数据源
            let dsc = AppRun::init_database_source(cfg.clone()).await.unwrap();
            //生成工作实体
            let (sch_entity, alloc) = AppRun::init_schedule_entity(dsc.clone());
            //初始化
            let election =
                AppRun::start_election_listen(sch_entity, dsc.clone(), cfg.server.name.clone());
            let _close = election.start().await;
            // wd_log::res_panic!(AppRun::add_exit_task(exit_channel,async move{
            //     //停止
            //     if let Err(e) = close.stop(std::time::Duration::from_secs(3)).await{
            //         wd_log::log_error_ln!("close election error:{}",e);
            //     }
            // }).await);
            //启动服务
            crate::app::application_run(ctx.clone(), cfg, dsc, alloc).await;

            return ctx;
        });
    }
}
