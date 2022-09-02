use crate::app::controls::TaskController;
use crate::pb;

mod controls;

pub async fn start(ctx: wd_run::Context){
    let layer = tower::ServiceBuilder::new()
        .timeout(std::time::Duration::from_secs(60))
        .concurrency_limit(100)
        .into_inner();

    let task_service = pb::task_manager_services_server::TaskManagerServicesServer::new(TaskController::new());

    tonic::transport::Server::builder().
        layer(layer).
        add_service(task_service).
        serve("127.0.0.1:6666".parse().unwrap())
        .await.unwrap();
}
