pub mod schedule;

mod controls;
mod entity;
mod middle;

use crate::app::controls::Server;
use crate::app::middle::{ConcurrentInterceptor, LogInterceptor};
use crate::app::schedule::Allocation;
use crate::conf::Config;
use crate::infra::middle::CustomInterceptor;
use crate::infra::*;
use crate::pb;
use std::sync::Arc;

pub async fn application_run(
    _ctx: wd_run::Context,
    cfg: Config,
    dsc: Arc<client::DataSourceCenter>,
    alloc: Arc<Allocation>,
) {
    let layer = tower::ServiceBuilder::new()
        .timeout(std::time::Duration::from_secs(60))
        // .concurrency_limit(100)
        .layer(CustomInterceptor::new(LogInterceptor::new()))
        .layer(CustomInterceptor::new(ConcurrentInterceptor::new()))
        .into_inner();

    let server_entity = Server::new(dsc, alloc);
    let task_service =
        pb::task_manager_services_server::TaskManagerServicesServer::new(server_entity.clone());

    let worker_service =
        pb::workers_scheduling_services_server::WorkersSchedulingServicesServer::new(server_entity);

    wd_log::log_info_ln!(
        "server[{}] start lister:({})",
        cfg.server.name,
        cfg.server.host_port
    );

    tonic::transport::Server::builder()
        .layer(layer)
        .add_service(task_service)
        .add_service(worker_service)
        .serve(cfg.server.host_port.parse().unwrap())
        .await
        .unwrap();
}
