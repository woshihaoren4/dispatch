#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchContent {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    #[prost(uint32, tag = "2")]
    pub group: u32,
    ///是否是异步 不关心放回结果
    #[prost(bool, tag = "3")]
    pub r#async: bool,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "6")]
    pub retry_interval: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimingContent {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    #[prost(uint32, tag = "2")]
    pub group: u32,
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
    pub group: u32,
    ///周期
    #[prost(int64, tag = "2")]
    pub cycle_interval: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "TaskType", tag = "5")]
    pub r#type: i32,
    #[prost(enumeration = "TaskStatus", tag = "6")]
    pub status: i32,
    ///任务触发的时候会传递给执行者
    #[prost(bytes = "vec", tag = "7")]
    pub config: ::prost::alloc::vec::Vec<u8>,
    ///如果创建任务时指定子任务，则直接初始化完成
    #[prost(message, repeated, tag = "8")]
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
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "SubTaskStatus", tag = "4")]
    pub status: i32,
    ///分组id
    #[prost(uint32, tag = "5")]
    pub group_id: u32,
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///上次被调度的时间
    #[prost(message, optional, tag = "7")]
    pub last_dispatch_time: ::core::option::Option<::prost_types::Timestamp>,
    ///版本号，随着调度次数增加
    #[prost(uint32, tag = "8")]
    pub version: u32,
    ///扩展信息
    #[prost(string, tag = "9")]
    pub extern_: ::prost::alloc::string::String,
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
    Batch = 0,
    Timing = 1,
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
    Dispatch = 1,
    Complete = 2,
    ///失败  不再重新调度
    Failed = 3,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    #[prost(message, optional, tag = "1")]
    pub task: ::core::option::Option<Task>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskResponse {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "100")]
    pub result: ::core::option::Option<CommonResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskInfoValue {
    #[prost(map = "string, message", tag = "1")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubTask {
    #[prost(message, repeated, tag = "1")]
    pub sub_tasks: ::prost::alloc::vec::Vec<SubTask>,
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
    #[prost(map = "string, int32", tag = "22")]
    pub aa: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
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
        SubTasks(super::UpdateSubTask),
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
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "7")]
    pub contain_child: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTaskResponse {
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
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
pub mod task_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TaskManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TaskManagerServiceClient<tonic::transport::Channel> {
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
    impl<T> TaskManagerServiceClient<T>
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
        ) -> TaskManagerServiceClient<InterceptedService<T, F>>
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
            TaskManagerServiceClient::new(InterceptedService::new(inner, interceptor))
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
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManagerService/CreateTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
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
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManagerService/UpdateTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
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
            let path = http::uri::PathAndQuery::from_static("/pb.TaskManagerService/SearchTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod task_manager_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TaskManagerServiceServer."]
    #[async_trait]
    pub trait TaskManagerService: Send + Sync + 'static {
        async fn create_task(
            &self,
            request: tonic::Request<super::CreateTaskRequest>,
        ) -> Result<tonic::Response<super::CreateTaskResponse>, tonic::Status>;
        async fn update_task(
            &self,
            request: tonic::Request<super::UpdateTaskRequest>,
        ) -> Result<tonic::Response<super::UpdateTaskResponse>, tonic::Status>;
        async fn search_task(
            &self,
            request: tonic::Request<super::SearchTaskRequest>,
        ) -> Result<tonic::Response<super::SearchTaskResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TaskManagerServiceServer<T: TaskManagerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TaskManagerService> TaskManagerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TaskManagerServiceServer<T>
    where
        T: TaskManagerService,
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
                "/pb.TaskManagerService/CreateTask" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTaskSvc<T: TaskManagerService>(pub Arc<T>);
                    impl<T: TaskManagerService>
                        tonic::server::UnaryService<super::CreateTaskRequest> for CreateTaskSvc<T>
                    {
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
                "/pb.TaskManagerService/UpdateTask" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTaskSvc<T: TaskManagerService>(pub Arc<T>);
                    impl<T: TaskManagerService>
                        tonic::server::UnaryService<super::UpdateTaskRequest> for UpdateTaskSvc<T>
                    {
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
                "/pb.TaskManagerService/SearchTask" => {
                    #[allow(non_camel_case_types)]
                    struct SearchTaskSvc<T: TaskManagerService>(pub Arc<T>);
                    impl<T: TaskManagerService>
                        tonic::server::UnaryService<super::SearchTaskRequest> for SearchTaskSvc<T>
                    {
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
    impl<T: TaskManagerService> Clone for TaskManagerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TaskManagerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TaskManagerService> tonic::transport::NamedService for TaskManagerServiceServer<T> {
        const NAME: &'static str = "pb.TaskManagerService";
    }
}
