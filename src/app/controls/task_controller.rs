use serde_json::{Number, Value};
use crate::pb::task_manager_services_server::TaskManagerServices;
use crate::pb::{CommonResult, CreateTaskRequest, CreateTaskResponse, SearchSubTaskRequest, SearchSubTaskResponse, SearchTaskRequest, SearchTaskResponse, UpdateTaskRequest, UpdateTaskResponse};
use tonic::{Code, Request, Response, Status};
use wd_log::log_info_ln;
use crate::app::controls::Server;
use crate::app::entity;
use crate::app::entity::{SubTask, Task};
use crate::infra::client::QueryOption;
use crate::pb::update_task_request::UpdateContent;


#[async_trait::async_trait]
impl TaskManagerServices for super::Server{
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, Status> {
        //参数校验
        if let Err(e) = Self::create_task_request_check(&request){
            return Ok(Response::new(CreateTaskResponse {
                task_code: String::new(),
                create_time: 0,
                result: Some(e)
            }))
        }
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
        request: Request<UpdateTaskRequest>,
    ) -> Result<Response<UpdateTaskResponse>, Status> {
        //验参
        let result = self.update_task_request_check(&request).await;
        let t = match result {
            Ok(t) => {t}
            Err(e) => {
                return Ok(Response::new(UpdateTaskResponse{ result: Some(e) }))
            }
        };
        log_info_ln!("find task success:{:?}",t);
        //更新任务
        let req = request.into_inner();

        match req.action {
            1=>{ //UpdateTaskAction::UpdateStatus
                let status = match req.update_content.unwrap() {
                    UpdateContent::Status(s) => {s}
                    _ =>{0}
                };
                let old = Task::number_to_task_status(t.status);
                let status = Task::number_to_task_status(status as u8);
                if !Self::fsm(old,status) {
                    return Ok(Response::new(UpdateTaskResponse{ result: Server::response_err_result(400,format!("task status({:?}) can not to status({:?})",old,status)) }))
                }
                let dao = self.dsc.get_dao().await;
                let mut update_task = Task::default();
                update_task.task_code = t.task_code.clone();
                update_task.status = status as u8;
                let result = dao.update_by_code(update_task).await;
                return match result {
                    Ok(_) => {
                        Ok(Response::new(UpdateTaskResponse { result: Self::response_success() }))
                    }
                    Err(e) => {
                        Ok(Response::new(UpdateTaskResponse { result: Self::response_err_result(500, e) }))
                    }
                }
            }
            2=>{} //UpdateTaskAction::UpdateTaskInfo
            3=>{ //UpdateTaskAction::AppendSubtasks
                let ast = match req.update_content.unwrap(){
                    UpdateContent::SubTasks(subs) => subs,
                    _ => {
                        return Ok(Response::new(UpdateTaskResponse{ result: Server::response_err_result(400,format!("active is append sub task,but content is not AppendSubTask")) }))
                    }
                };
                let sub_task_list = SubTask::from(ast,req.task_code);
                let dao = self.dsc.get_dao().await;
                let result = dao.insert_many(sub_task_list).await;
                match result {
                    Ok(_) => {
                        return Ok(Response::new(UpdateTaskResponse{ result: Self::response_success() }))
                    }
                    Err(e) => {
                        return Ok(Response::new(UpdateTaskResponse{ result: Server::response_err_result(500,format!("append sub task error:({})",e)) }))
                    }
                }
            }
            4=>{} //UpdateTaskAction::UpdateSubtaskInfo
            _=>{}
        }
        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn search_task(
        &self,
        request: Request<SearchTaskRequest>,
    ) -> Result<Response<SearchTaskResponse>, Status> {
        //todo 验参
        let req = request.into_inner();
        let mut query = vec![];
        query.push(("task_code".to_string(),QueryOption::Equal(Value::String(req.task_code))));
        query.push(("task_name".to_string(),QueryOption::Like(Value::String(req.name))));
        query.push(("type".to_string(),QueryOption::Equal(Value::Number(Number::from(req.r#type)))));
        query.push(("status".to_string(),QueryOption::Equal(Value::Number(Number::from(req.status)))));
        query.push(("create_time".to_string(),QueryOption::BetweenAnd(Value::Number(Number::from(req.start_time)), Value::Number(Number::from(req.end_time)))));
        let mut tag_query = vec![];
        for i in req.contain_tags.into_iter(){
            tag_query.push(Value::String(i));
        }
        query.push(("tags".to_string(),QueryOption::Contain(tag_query)));
        // query.push(("offset".to_string(),QueryOption::Limit(req.size as i64,req.page as i64)));
        // query.push(("sort".to_string(),QueryOption::Sort(req.sort,-1)));

        let dao = self.dsc.get_dao::<Task>().await;
        let result = dao.find(query,req.page as i64,req.size as i64).await;
        match result {
            Ok(item) => {
                let mut list = vec![];
                for x in item.0.into_iter() {
                    list.push(x.to_pb_task())
                }
                return Ok(Response::new(SearchTaskResponse{
                    tasks:  list,
                    total: item.1 as i32,
                    result: Self::response_success()
                }))
            }
            Err(e) => {
                return Ok(Response::new(SearchTaskResponse{
                    tasks: vec![],
                    total: 0,
                    result: Self::response_err_result(500,e)
                }))
            }
        }

        return Err(Status::new(Code::Unknown, "not found"));
    }

    async fn search_sub_task(
        &self,
        _request: Request<SearchSubTaskRequest>,
    ) -> Result<Response<SearchSubTaskResponse>, Status> {
        return Err(Status::new(Code::Unknown, "not found"));
    }
}

impl super::Server{
    fn create_task_request_check(req:& Request<CreateTaskRequest>)->Result<(), CommonResult>{
        let req = req.get_ref();
        bad_request!(req.task_name.len()>64,"name len > 64");
        bad_request!(req.description.len()>512,"description len > 521");
        bad_request!(req.end_time==0,"end time is 0");
        match req.r#type {
            #[allow(ellipsis_inclusive_range_patterns)]
            1 ... 3 =>{}
            _=>bad_request!(true,"unknown task type:{}",req.r#type),
        }
        Ok(())
    }
    async fn update_task_request_check(&self,req:&Request<UpdateTaskRequest>)->Result<entity::Task,CommonResult>{
        bad_request!(req.get_ref().task_code.is_empty(),"request code is empty");
        bad_request!(req.get_ref().action <= 0 || req.get_ref().action > 4,"unknown action");
        bad_request!(req.get_ref().update_content.is_none(),"content is nil");
        let dao = self.dsc.get_dao::<Task>().await;
        let result = dao.find_by_code(req.get_ref().task_code.clone()).await;
        let opt = match result {
            Ok(o)=>o,
            Err(e)=>{
                server_error!("{}",e);
            }
        };
        if opt.is_none(){
            bad_request!(true,"have not task by the code");
        }
        return Ok(opt.unwrap());
    }
}