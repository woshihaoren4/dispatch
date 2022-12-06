use crate::infra::middle::LayerHyperInterceptor;
use hyper::http::HeaderValue;
use hyper::{Body, HeaderMap, Request, Response};
use std::collections::HashMap;
use std::str::FromStr;
use tonic::body::BoxBody;
use wd_run::Context;

const REQUEST_ID: &'static str = "dispatch_grpc_request_id";

pub struct LogInterceptor {
    id_generator: wd_sonyflake::SonyFlakeEntity,
}

impl LogInterceptor {
    pub fn new() -> Self {
        let id_generator = wd_sonyflake::SonyFlakeEntity::new_default();
        Self { id_generator }
    }
    fn generate_request_id(&self) -> i64 {
        self.id_generator.get_id()
    }
    async fn set_request_id(&self, headers: &mut HeaderMap) -> i64 {
        let rid = self.generate_request_id();
        headers.insert(REQUEST_ID, HeaderValue::from(rid));
        return rid;
    }
    pub async fn set_request_id_to_ctx(ctx: &mut Context, rid: i64) {
        ctx.set(REQUEST_ID, rid).await;
    }
    //从ctx中获取request id
    pub fn get_request_id_by_request(headers: &HeaderMap) -> i64 {
        let value = match headers.get(REQUEST_ID) {
            None => {
                return 0;
            }
            Some(s) => s.to_str(),
        };
        if value.is_err() {
            return 0;
        }
        let r = i64::from_str(value.unwrap());
        return r.unwrap_or(0);
    }
    pub async fn get_request_id(ctx: Context) -> i64 {
        return ctx.copy::<_, i64>(REQUEST_ID).await.unwrap_or(0);
    }
}

#[async_trait::async_trait]
impl LayerHyperInterceptor for LogInterceptor {
    async fn request(
        &self,
        mut ctx: Context,
        mut request: Request<Body>,
    ) -> Result<Request<Body>, Response<BoxBody>> {
        let rid = self.set_request_id(request.headers_mut()).await;
        LogInterceptor::set_request_id_to_ctx(&mut ctx, rid).await;
        let mut headers = HashMap::new();
        for (name, value) in request.headers().iter() {
            let vs = match value.to_str() {
                Ok(o) => o,
                Err(_) => {
                    continue;
                }
            };
            headers.insert(name.as_str(), vs);
        }
        wd_log::log_info_ln!(
            "request[{}] -> path:({}) header:({:?})",
            rid,
            request.uri().path(),
            headers
        );
        return Ok(request);
    }

    async fn response(&self, ctx: Context, response: Response<BoxBody>) -> Response<BoxBody> {
        let rid = LogInterceptor::get_request_id(ctx).await;
        wd_log::log_info_ln!(
            "response[{}] -> status:({})",
            rid,
            response.status().to_string()
        );
        return response;
    }
}
