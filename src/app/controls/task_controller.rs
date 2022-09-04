use crate::pb::task_manager_services_server::TaskManagerServices;
use crate::pb::{
    CreateTaskRequest, CreateTaskResponse, SearchSubTaskRequest, SearchSubTaskResponse,
    SearchTaskRequest, SearchTaskResponse, UpdateTaskRequest, UpdateTaskResponse,
};
use tonic::{Code, Request, Response, Status};

pub struct TaskController {}

impl TaskController {
    pub fn new() -> TaskController {
        return TaskController {};
    }
}

#[async_trait::async_trait]
impl TaskManagerServices for TaskController {
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, Status> {
        let option = request.metadata().get("dispatch_grpc_current_count");
        if let Some(s) = option {
            match s.to_str() {
                Ok(o) => {
                    wd_log::log_info_ln!("---------> dispatch_grpc_current_count {}", o);
                }
                Err(e) => {
                    println!("{}", e.to_string())
                }
            }
        } else {
            println!("{:?}", request.metadata())
        }

        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn update_task(
        &self,
        request: Request<UpdateTaskRequest>,
    ) -> Result<Response<UpdateTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn search_task(
        &self,
        request: Request<SearchTaskRequest>,
    ) -> Result<Response<SearchTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn search_sub_task(
        &self,
        request: Request<SearchSubTaskRequest>,
    ) -> Result<Response<SearchSubTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }
}
