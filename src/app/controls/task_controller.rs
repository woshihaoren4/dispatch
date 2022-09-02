use tonic::{Code, Request, Response, Status};
use crate::pb::{CreateTaskRequest, CreateTaskResponse, SearchSubTaskRequest, SearchSubTaskResponse, SearchTaskRequest, SearchTaskResponse, UpdateTaskRequest, UpdateTaskResponse};
use crate::pb::task_manager_services_server::TaskManagerServices;

pub struct TaskController{

}

impl TaskController {
    pub fn new()->TaskController{
        return TaskController{};
    }
}

#[async_trait::async_trait]
impl TaskManagerServices for TaskController{
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
