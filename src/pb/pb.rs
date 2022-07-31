#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchContent {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    ///是否是异步 不关心放回结果
    #[prost(bool, tag = "3")]
    pub r#async: bool,
    #[prost(string, tag = "4")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub end_time: ::prost::alloc::string::String,
    ///超过这个时间后会重试任务
    #[prost(int64, tag = "6")]
    pub retry_interval: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimingContent {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    ///是否是异步 不关心放回结果
    #[prost(bool, tag = "3")]
    pub r#async: bool,
    ///每次执行的最长时间
    #[prost(int64, tag = "4")]
    pub max_duration: i64,
    ///循环间隔 linux表示法"* * * * *"
    #[prost(string, tag = "5")]
    pub interval: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub retry_interval: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElectionContent {
    ///分组
    #[prost(uint32, tag = "1")]
    pub total: u32,
    ///周期
    #[prost(int64, tag = "2")]
    pub cycle_interval: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub task_name: ::prost::alloc::string::String,
    ///描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub create_time: ::prost::alloc::string::String,
    #[prost(enumeration = "TaskType", tag = "5")]
    pub r#type: i32,
    #[prost(enumeration = "TaskStatus", tag = "6")]
    pub status: i32,
    ///任务触发的时候会传递给执行者
    #[prost(bytes = "vec", tag = "7")]
    pub config: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///如果创建任务时指定子任务，则直接初始化完成
    #[prost(message, repeated, tag = "9")]
    pub sub_tasks: ::prost::alloc::vec::Vec<SubTask>,
    #[prost(oneof = "task::Content", tags = "100, 101, 102")]
    pub content: ::core::option::Option<task::Content>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "100")]
        BatchCtx(super::BatchContent),
        #[prost(message, tag = "101")]
        TimingCtx(super::TimingContent),
        #[prost(message, tag = "102")]
        ElectionCtx(super::ElectionContent),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubTask {
    #[prost(string, tag = "1")]
    pub sub_task_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sub_task_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "SubTaskStatus", tag = "4")]
    pub status: i32,
    ///序号
    #[prost(uint32, tag = "5")]
    pub serial: u32,
    #[prost(string, tag = "6")]
    pub create_time: ::prost::alloc::string::String,
    ///上次被调度的时间
    #[prost(string, tag = "7")]
    pub last_dispatch_time: ::prost::alloc::string::String,
    ///版本号，随着调度次数增加
    #[prost(uint32, tag = "8")]
    pub version: u32,
    ///扩展信息
    #[prost(string, tag = "9")]
    pub extern_: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worker {
    ///工人的唯一标识
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
//----------------interface common struct-----------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonResult {
    ///200 ：success
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    ///批处理任务
    Batch = 0,
    ///定时任务
    Timing = 1,
    ///状态集群任务
    Election = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskStatus {
    Created = 0,
    Initialized = 1,
    Launching = 2,
    Stop = 3,
    Over = 4,
    Close = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubTaskStatus {
    Create = 0,
    Dispatching = 1,
    Complete = 2,
    ///失败  不再重新调度
    Failed = 3,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskSubTask {
    #[prost(string, tag = "2")]
    pub sub_task_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    ///扩展信息
    #[prost(string, tag = "9")]
    pub extern_: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    #[prost(string, tag = "2")]
    pub task_name: ::prost::alloc::string::String,
    ///描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "TaskType", tag = "5")]
    pub r#type: i32,
    ///任务触发的时候会传递给执行者
    #[prost(bytes = "vec", tag = "7")]
    pub config: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "9")]
    pub sub_tasks: ::prost::alloc::vec::Vec<CreateTaskSubTask>,
    #[prost(oneof = "create_task_request::Content", tags = "100, 101, 102")]
    pub content: ::core::option::Option<create_task_request::Content>,
}
/// Nested message and enum types in `CreateTaskRequest`.
pub mod create_task_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "100")]
        BatchCtx(super::BatchContent),
        #[prost(message, tag = "101")]
        TimingCtx(super::TimingContent),
        #[prost(message, tag = "102")]
        ElectionCtx(super::ElectionContent),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskResponse {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub create_time: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskInfoValue {
    ///name
    ///description
    ///config
    #[prost(map = "string, message", tag = "1")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendSubTask {
    #[prost(message, repeated, tag = "1")]
    pub sub_tasks: ::prost::alloc::vec::Vec<CreateTaskSubTask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubTaskInfoValue {
    #[prost(string, tag = "1")]
    pub sub_task_id: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(enumeration = "UpdateTaskAction", tag = "2")]
    pub action: i32,
    #[prost(
        oneof = "update_task_request::UpdateContent",
        tags = "100, 101, 102, 103"
    )]
    pub update_content: ::core::option::Option<update_task_request::UpdateContent>,
}
/// Nested message and enum types in `UpdateTaskRequest`.
pub mod update_task_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UpdateContent {
        #[prost(enumeration = "super::TaskStatus", tag = "100")]
        Status(i32),
        #[prost(message, tag = "101")]
        TaskValues(super::UpdateTaskInfoValue),
        #[prost(message, tag = "102")]
        SubTasks(super::AppendSubTask),
        #[prost(message, tag = "103")]
        SubTaskValues(super::UpdateSubTaskInfoValue),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskResponse {
    #[prost(message, optional, tag = "100")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTaskRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "TaskType", tag = "3")]
    pub r#type: i32,
    #[prost(enumeration = "TaskStatus", tag = "4")]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub size: i32,
    #[prost(int32, tag = "8")]
    pub page: i32,
    ///default:"create_time desc"
    #[prost(string, tag = "9")]
    pub sort: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTaskResponse {
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    #[prost(int32, tag = "2")]
    pub total: i32,
    #[prost(message, optional, tag = "100")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSubTaskRequest {
    #[prost(string, tag = "1")]
    pub sub_task_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sub_name: ::prost::alloc::string::String,
    ///父任务的id
    #[prost(string, tag = "3")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(enumeration = "SubTaskStatus", tag = "4")]
    pub status: i32,
    ///  string start_time = 5;
    ///  string end_time = 6;
    #[prost(int32, tag = "7")]
    pub size: i32,
    #[prost(int32, tag = "8")]
    pub page: i32,
    ///default:"create_time desc"
    #[prost(string, tag = "9")]
    pub sort: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSubTaskResponse {
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<SubTask>,
    #[prost(int32, tag = "2")]
    pub total: i32,
    #[prost(message, optional, tag = "100")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateTaskAction {
    UpdateStatus = 0,
    UpdateTaskInfo = 1,
    AppendSubtasks = 2,
    UpdateSubtaskInfo = 3,
}
#[doc = r" Generated client implementations."]
pub mod task_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TaskManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TaskManagerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TaskManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TaskManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TaskManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "创建任务"]
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
        ) -> Result<tonic::Response<super::CreateTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManager/CreateTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "修改任务"]
        pub async fn update_task(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskRequest>,
        ) -> Result<tonic::Response<super::UpdateTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManager/UpdateTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "查询任务"]
        pub async fn search_task(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTaskRequest>,
        ) -> Result<tonic::Response<super::SearchTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManager/SearchTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "查询子任务"]
        pub async fn search_sub_task(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchSubTaskRequest>,
        ) -> Result<tonic::Response<super::SearchSubTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManager/SearchSubTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod task_manager_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TaskManagerServer."]
    #[async_trait]
    pub trait TaskManager: Send + Sync + 'static {
        #[doc = "创建任务"]
        async fn create_task(
            &self,
            request: tonic::Request<super::CreateTaskRequest>,
        ) -> Result<tonic::Response<super::CreateTaskResponse>, tonic::Status>;
        #[doc = "修改任务"]
        async fn update_task(
            &self,
            request: tonic::Request<super::UpdateTaskRequest>,
        ) -> Result<tonic::Response<super::UpdateTaskResponse>, tonic::Status>;
        #[doc = "查询任务"]
        async fn search_task(
            &self,
            request: tonic::Request<super::SearchTaskRequest>,
        ) -> Result<tonic::Response<super::SearchTaskResponse>, tonic::Status>;
        #[doc = "查询子任务"]
        async fn search_sub_task(
            &self,
            request: tonic::Request<super::SearchSubTaskRequest>,
        ) -> Result<tonic::Response<super::SearchSubTaskResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TaskManagerServer<T: TaskManager> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TaskManager> TaskManagerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TaskManagerServer<T>
    where
        T: TaskManager,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pb.TaskManager/CreateTask" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTaskSvc<T: TaskManager>(pub Arc<T>);
                    impl<T: TaskManager> tonic::server::UnaryService<super::CreateTaskRequest> for CreateTaskSvc<T> {
                        type Response = super::CreateTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TaskManager/UpdateTask" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTaskSvc<T: TaskManager>(pub Arc<T>);
                    impl<T: TaskManager> tonic::server::UnaryService<super::UpdateTaskRequest> for UpdateTaskSvc<T> {
                        type Response = super::UpdateTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TaskManager/SearchTask" => {
                    #[allow(non_camel_case_types)]
                    struct SearchTaskSvc<T: TaskManager>(pub Arc<T>);
                    impl<T: TaskManager> tonic::server::UnaryService<super::SearchTaskRequest> for SearchTaskSvc<T> {
                        type Response = super::SearchTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TaskManager/SearchSubTask" => {
                    #[allow(non_camel_case_types)]
                    struct SearchSubTaskSvc<T: TaskManager>(pub Arc<T>);
                    impl<T: TaskManager> tonic::server::UnaryService<super::SearchSubTaskRequest>
                        for SearchSubTaskSvc<T>
                    {
                        type Response = super::SearchSubTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchSubTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_sub_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchSubTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TaskManager> Clone for TaskManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TaskManager> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TaskManager> tonic::transport::NamedService for TaskManagerServer<T> {
        const NAME: &'static str = "pb.TaskManager";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FoundTaskRequest {
    #[prost(string, tag = "1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub size: i32,
    #[prost(int32, tag = "3")]
    pub page: i32,
    ///default:"create_time desc"
    #[prost(string, tag = "4")]
    pub sort: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FoundTaskResponse {
    #[prost(int32, tag = "1")]
    pub total: i32,
    #[prost(message, repeated, tag = "2")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    #[prost(message, optional, tag = "1")]
    pub worker: ::core::option::Option<Worker>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(message, optional, tag = "255")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullTaskRequest {
    #[prost(message, optional, tag = "1")]
    pub worker: ::core::option::Option<Worker>,
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullTaskResponse {
    #[prost(message, optional, tag = "1")]
    pub sub_task: ::core::option::Option<SubTask>,
    #[prost(message, optional, tag = "255")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteSubTaskRequest {
    #[prost(string, tag = "1")]
    pub sub_task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteSubTaskResponse {
    #[prost(message, optional, tag = "255")]
    pub result: ::core::option::Option<CommonResult>,
}
#[doc = r" Generated client implementations."]
pub mod workers_scheduling_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WorkersSchedulingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkersSchedulingClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WorkersSchedulingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WorkersSchedulingClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            WorkersSchedulingClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "心跳"]
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::PingRequest>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.WorkersScheduling/Ping");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "发现任务"]
        pub async fn found_task(
            &mut self,
            request: impl tonic::IntoRequest<super::FoundTaskRequest>,
        ) -> Result<tonic::Response<super::FoundTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.WorkersScheduling/FoundTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "拉取任务"]
        pub async fn pull_task(
            &mut self,
            request: impl tonic::IntoRequest<super::PullTaskRequest>,
        ) -> Result<tonic::Response<super::PullTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.WorkersScheduling/PullTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "完成任务"]
        pub async fn complete_sub_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteSubTaskRequest>,
        ) -> Result<tonic::Response<super::CompleteSubTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/pb.WorkersScheduling/CompleteSubTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod workers_scheduling_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WorkersSchedulingServer."]
    #[async_trait]
    pub trait WorkersScheduling: Send + Sync + 'static {
        #[doc = "心跳"]
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status>;
        #[doc = "发现任务"]
        async fn found_task(
            &self,
            request: tonic::Request<super::FoundTaskRequest>,
        ) -> Result<tonic::Response<super::FoundTaskResponse>, tonic::Status>;
        #[doc = "拉取任务"]
        async fn pull_task(
            &self,
            request: tonic::Request<super::PullTaskRequest>,
        ) -> Result<tonic::Response<super::PullTaskResponse>, tonic::Status>;
        #[doc = "完成任务"]
        async fn complete_sub_task(
            &self,
            request: tonic::Request<super::CompleteSubTaskRequest>,
        ) -> Result<tonic::Response<super::CompleteSubTaskResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WorkersSchedulingServer<T: WorkersScheduling> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WorkersScheduling> WorkersSchedulingServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkersSchedulingServer<T>
    where
        T: WorkersScheduling,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pb.WorkersScheduling/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: WorkersScheduling>(pub Arc<T>);
                    impl<T: WorkersScheduling> tonic::server::UnaryService<super::PingRequest> for PingSvc<T> {
                        type Response = super::PingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.WorkersScheduling/FoundTask" => {
                    #[allow(non_camel_case_types)]
                    struct FoundTaskSvc<T: WorkersScheduling>(pub Arc<T>);
                    impl<T: WorkersScheduling> tonic::server::UnaryService<super::FoundTaskRequest>
                        for FoundTaskSvc<T>
                    {
                        type Response = super::FoundTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FoundTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).found_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FoundTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.WorkersScheduling/PullTask" => {
                    #[allow(non_camel_case_types)]
                    struct PullTaskSvc<T: WorkersScheduling>(pub Arc<T>);
                    impl<T: WorkersScheduling> tonic::server::UnaryService<super::PullTaskRequest> for PullTaskSvc<T> {
                        type Response = super::PullTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pull_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PullTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.WorkersScheduling/CompleteSubTask" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteSubTaskSvc<T: WorkersScheduling>(pub Arc<T>);
                    impl<T: WorkersScheduling>
                        tonic::server::UnaryService<super::CompleteSubTaskRequest>
                        for CompleteSubTaskSvc<T>
                    {
                        type Response = super::CompleteSubTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteSubTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).complete_sub_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompleteSubTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: WorkersScheduling> Clone for WorkersSchedulingServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WorkersScheduling> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WorkersScheduling> tonic::transport::NamedService for WorkersSchedulingServer<T> {
        const NAME: &'static str = "pb.WorkersScheduling";
    }
}
