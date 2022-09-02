use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use hyper::Body;
use tonic::{Code, Request, Response, Status, transport::Server};
use tonic::body::BoxBody;
use wd_run::{CmdInfo, Context};
use crate::app::middle::{LayerHyperInterceptor, MyMiddlewareLayer};
use crate::pb::*;
use crate::pb::task_manager_services_server::{TaskManagerServices, TaskManagerServicesServer};

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
            let layer = tower::ServiceBuilder::new()
                .timeout(Duration::from_secs(3))
                // .layer(GrpcMiddleLayer::new(Middle))
                // .layer(tower::layer::layer_fn(|service|HelloWorld{inner:service,index:10}))
                // .layer(tonic::service::interceptor(intercept))
                .layer(MyMiddlewareLayer::new(TestMiddle))
                .into_inner();
            let server =TaskManagerServicesServer::new(TaskServerImpl {});
            Server::builder()
                .layer(layer)
                .add_service(server)
                .serve("127.0.0.1:6666".parse().unwrap())
                .await.unwrap();
            return ctx
        })
    }
}

pub struct TaskServerImpl{}

#[async_trait::async_trait]
impl TaskManagerServices for TaskServerImpl{
    async fn create_task(&self, _request: Request<CreateTaskRequest>) -> Result<Response<CreateTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

    async fn update_task(&self, _request: Request<UpdateTaskRequest>) -> Result<Response<UpdateTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

    async fn search_task(&self, _request: Request<SearchTaskRequest>) -> Result<Response<SearchTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

    async fn search_sub_task(&self, _request: Request<SearchSubTaskRequest>) -> Result<Response<SearchSubTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

}

pub struct TestMiddle;

#[async_trait::async_trait]
impl LayerHyperInterceptor for TestMiddle{
    async fn request(&self, ctx: Context, request: hyper::Request<Body>) -> Result<hyper::Request<Body>, hyper::Response<BoxBody>> {
        ctx.set("hello","world").await;
        return Ok(request)
    }

    async fn response(&self, ctx: Context, response: hyper::Response<BoxBody>) -> hyper::Response<BoxBody> {
        let world = ctx.copy::<_,&str>("hello").await.unwrap_or("default_world");
        wd_log::log_info_ln!("TestMiddle ---> {}",world);
        return response;
    }
}
