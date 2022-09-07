use crate::pb::task_manager_services_server::TaskManagerServices;
use crate::pb::{ CreateTaskRequest, CreateTaskResponse, SearchSubTaskRequest, SearchSubTaskResponse, SearchTaskRequest, SearchTaskResponse, UpdateTaskRequest, UpdateTaskResponse};
use tonic::{Code, Request, Response, Status};
use crate::app::entity;


#[async_trait::async_trait]
impl TaskManagerServices for super::Server{
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, Status> {
        //todo 参数校验
        //创建task
        let t = entity::Task::from(request.into_inner());
        let dao = self.dsc.get_dao::<entity::Task>().await;
        let result = dao.insert(t).await;
        match result {
            Ok(o) => {
                Ok(Response::new(CreateTaskResponse {
                    task_code: o.task_code,
                    create_time: o.create_time,
                    result: Self::response_success()
                }))
            }
            Err(e) => {
                Err(Status::new(Code::Unknown, e.to_string()))
            }
        }
    }

    async fn update_task(
        &self,
        _request: Request<UpdateTaskRequest>,
    ) -> Result<Response<UpdateTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn search_task(
        &self,
        _request: Request<SearchTaskRequest>,
    ) -> Result<Response<SearchTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn search_sub_task(
        &self,
        _request: Request<SearchSubTaskRequest>,
    ) -> Result<Response<SearchSubTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }
}
