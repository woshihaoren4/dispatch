use crate::app::middle::LogInterceptor;
use crate::infra::LayerHyperInterceptor;
use hyper::header::HeaderValue;
use hyper::{Body, Request, Response};
use std::str::FromStr;
use std::sync::atomic::{AtomicU16, Ordering};
use tonic::body::BoxBody;
use tonic::codegen::http::HeaderMap;
use wd_run::Context;

const CURRENT_COUNT: &'static str = "dispatch_grpc_current_count";

pub struct ConcurrentInterceptor {
    counter: AtomicU16,
    limit: u16,
}

impl ConcurrentInterceptor {
    pub fn new() -> Self {
        let counter = AtomicU16::new(0);
        let limit = 100u16;
        ConcurrentInterceptor { counter, limit }
    }
    fn add_counter(&self) -> u16 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
    fn sub_counter(&self) -> u16 {
        self.counter.fetch_sub(1, Ordering::Relaxed)
    }
    fn get_counter(&self) -> u16 {
        let cc = self.counter.load(Ordering::Relaxed);
        return cc;
    }
    fn set_current_count(&self, header: &mut HeaderMap) {
        let cc = self.get_counter();
        header.insert(CURRENT_COUNT, HeaderValue::from(cc));
    }
    pub fn get_current_count(headers: &HeaderMap) -> u16 {
        let result = match headers.get(CURRENT_COUNT) {
            None => return u16::MAX,
            Some(s) => s.to_str(),
        };
        if result.is_err() {
            return u16::MAX;
        }
        let cc = u16::from_str(result.unwrap());
        return cc.unwrap_or(u16::MAX);
    }
}

#[async_trait::async_trait]
impl LayerHyperInterceptor for ConcurrentInterceptor {
    async fn request(
        &self,
        ctx: Context,
        mut request: Request<Body>,
    ) -> Result<Request<Body>, Response<BoxBody>> {
        let rid = LogInterceptor::get_request_id_by_request(request.headers());
        let cc = self.add_counter();
        self.set_current_count(request.headers_mut());
        wd_log::log_info_ln!(
            "ConcurrentInterceptor-> counter + 1 = {} request id:({})",
            cc,
            rid
        );
        return Ok(request);
    }

    async fn response(&self, ctx: Context, response: Response<BoxBody>) -> Response<BoxBody> {
        let rid = LogInterceptor::get_request_id(ctx).await;
        wd_log::log_info_ln!(
            "ConcurrentInterceptor-> counter - 1 = {} request id:({})",
            self.sub_counter(),
            rid
        );
        return response;
    }
}
