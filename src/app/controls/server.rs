use std::sync::Arc;
use crate::infra::client::DataSourceCenter;
use crate::pb::CommonResult;

pub struct Server {
    pub dsc : Arc<DataSourceCenter>
}

impl Server {
    pub fn new(dsc : Arc<DataSourceCenter>) -> Self {
        return Self {dsc};
    }
    pub fn response_success() ->Option<CommonResult>{
        Some(CommonResult{
            code: 0,
            message: "success".to_string(),
            payload: None
        })
    }
}