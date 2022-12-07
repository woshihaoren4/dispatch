use crate::app::controls::Server;
use crate::pb::workers_scheduling_services_server::WorkersSchedulingServices;
use crate::pb::{
    CompleteSubTaskRequest, CompleteSubTaskResponse, FoundTaskRequest, FoundTaskResponse,
    PingRequest, PingResponse, PullTaskRequest, PullTaskResponse,
};
use tonic::{Code, Request, Response, Status};

#[async_trait::async_trait]
impl WorkersSchedulingServices for Server {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            result: Self::response_success(),
        }))
    }

    async fn found_task(
        &self,
        request: Request<FoundTaskRequest>,
    ) -> Result<Response<FoundTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "todo"));
    }

    async fn pull_task(
        &self,
        request: Request<PullTaskRequest>,
    ) -> Result<Response<PullTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "todo"));
    }

    async fn complete_sub_task(
        &self,
        request: Request<CompleteSubTaskRequest>,
    ) -> Result<Response<CompleteSubTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "todo"));
    }
}
