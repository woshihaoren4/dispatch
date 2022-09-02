
use wd_run::{Context, CmdInfo};
use std::pin::Pin;
use std::future::Future;


use tonic::{Status, Response, Request, transport::Server, Code};
use crate::pb::*;
use tonic::service::Interceptor;
use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use tower::{Layer, Service};
use hyper::Body;
use tonic::body::BoxBody;
use std::task::Poll;
use std::time::Duration;
// use crate::app::middle::{GrpcMiddleInterface, GrpcMiddleLayer};
use crate::pb::task_manager_server::{TaskManagerServer, TaskManager};


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
                .into_inner();
            let server =TaskManagerServer::new(TaskServerImpl {});
            Server::builder()
                .layer(layer)
                .add_service(server)
                .serve("127.0.0.1:666".parse().unwrap())
                .await.unwrap();
            return ctx
        })
    }
}

pub struct TaskServerImpl{}

#[async_trait::async_trait]
impl TaskManager for TaskServerImpl{
    async fn create_task(&self, request: Request<CreateTaskRequest>) -> Result<Response<CreateTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

    async fn update_task(&self, request: Request<UpdateTaskRequest>) -> Result<Response<UpdateTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

    async fn search_task(&self, request: Request<SearchTaskRequest>) -> Result<Response<SearchTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }

    async fn search_sub_task(&self, request: Request<SearchSubTaskRequest>) -> Result<Response<SearchSubTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown,"not found"))
    }
}

fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    Ok(req)
}