use std::future::Future;
use std::pin::Pin;
use tonic::transport::Server;
use wd_run::{CmdInfo, Context};


pub struct AppRun {}

impl AppRun{
    pub fn new()->Self{
        AppRun{}
    }
    pub fn args(&self)->CmdInfo{
        wd_run::CmdInfo::new("run","running application")
            .add("c","./src/config/config.toml","config file path")
    }
}

impl wd_run::EventHandle for AppRun{
    fn handle(&self, ctx: Context) -> Pin<Box<dyn Future<Output=Context> + Send>> {
        println!("start server ---> :666");
        return Box::pin(async move{
            crate::app::start(ctx.clone()).await;
            return ctx
        })
    }
}
