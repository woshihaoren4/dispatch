use std::sync::Arc;
use crate::app::controls::Server;
use crate::app::middle::{ConcurrentInterceptor, LogInterceptor};
use crate::conf::Config;
use crate::infra::*;
use crate::infra::middle::CustomInterceptor;
use crate::pb;

mod controls;
mod middle;
mod entity;
mod dao;

pub async fn application_run(_ctx: wd_run::Context, cfg: Config,dsc:Arc<client::DataSourceCenter>) {
    let layer = tower::ServiceBuilder::new()
        .timeout(std::time::Duration::from_secs(60))
        // .concurrency_limit(100)
        .layer(CustomInterceptor::new(LogInterceptor::new()))
        .layer(CustomInterceptor::new(ConcurrentInterceptor::new()))
        .into_inner();

    let task_service =
        pb::task_manager_services_server::TaskManagerServicesServer::new(Server::new(dsc));

    wd_log::log_info_ln!(
        "server[{}] start lister:({})",
        cfg.server.name,
        cfg.server.host_port
    );

    tonic::transport::Server::builder()
        .layer(layer)
        .add_service(task_service)
        .serve(cfg.server.host_port.parse().unwrap())
        .await
        .unwrap();
}
