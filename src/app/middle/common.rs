use tonic::{Request, Status};
use tonic::codegen::Service;
use hyper::{Body, Response};
use tonic::body::BoxBody;
use std::task::Poll;
use std::sync::Arc;
use tower::Layer;


#[async_trait::async_trait]
pub trait GrpcMiddleInterface{
    async fn request(&self,ctx:wd_run::Context,req: hyper::Request<Body>) -> Result<hyper::Request<Body>, hyper::Response<BoxBody>>;
    async fn response(&self,ctx:wd_run::Context,resp:Option<Response<BoxBody>>)-> Option<hyper::Response<BoxBody>>;
}



fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    Ok(req)
}

#[derive(Debug, Clone)]
pub struct GrpcMiddleLayer{
    handle:Arc<dyn GrpcMiddleInterface + Sync + Send + 'static>
}
impl GrpcMiddleLayer{
    pub fn new<M:GrpcMiddleInterface+ Sync + Send + 'static>(m:M)->Self{
        let handle = Arc::new(m);
        GrpcMiddleLayer{handle}
    }
}

impl<S> Layer<S> for GrpcMiddleLayer {
    type Service = GrpcMiddle<S>;

    fn layer(&self, service: S) -> Self::Service {
        let handle = self.handle.clone();
        GrpcMiddle { inner: service,handle }
    }
}

#[derive(Debug, Clone)]
struct GrpcMiddle<S> {
    inner: S,
    handle:Arc<dyn GrpcMiddleInterface + Send + Sync + 'static>
}

impl<S> Service<hyper::Request<Body>> for GrpcMiddle<S>
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
        let handle = self.handle.clone();
        Box::pin(async move {
            let ctx = wd_run::Context::new();
            let req = match handle.request(ctx.clone(),req).await {
              Ok(o)=>o,
              Err(e)=>return Ok(e)
            };
            // Do extra async work here...
            let result = inner.call(req).await;
            if result.is_ok(){
                return Ok(handle.response(ctx,Some(o.unwrap())).await.unwrap())
            }
            if let Some(o) = handle.response(ctx, None).await {
                return Ok(o)
            }
            result?;
            panic!("fuck")
        })
    }
}