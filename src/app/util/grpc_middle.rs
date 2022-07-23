use std::pin::Pin;
use std::future::Future;

use tonic::{Status, Response, Request, transport::Server, Code};
use crate::pb::*;
use crate::pb::task_manager_service_server::{TaskManagerService, TaskManagerServiceServer};
use tonic::service::Interceptor;
use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use tower::{Layer, Service};
use hyper::Body;
use tonic::body::BoxBody;
use std::task::Poll;
use std::time::Duration;

macro_rules! tonic_middle {
    ($name:ident,$function1:block,$function2:block) => {
        #[derive(Debug, Clone)]
struct $name<S> {
    inner: S,
}
impl<S> Service<hyper::Request<Body>> for $name<S>
    where
        S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
        S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: hyper::Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let response = inner.call(req).await?;
            $function2
            Ok(response)
        })
    }
}
    };
}
