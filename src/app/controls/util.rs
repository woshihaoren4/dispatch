use crate::pb::CommonResult;

impl super::Server {
    pub fn response_success() -> Option<CommonResult> {
        Some(CommonResult {
            code: 0,
            message: "success".to_string(),
            payload: None,
        })
    }
    pub fn response_err_result<S: ToString>(code: i32, s: S) -> Option<CommonResult> {
        Some(CommonResult {
            code,
            message: s.to_string(),
            payload: None,
        })
    }
}

#[macro_export]
macro_rules! bad_request {
    ($eq:expr,$($arg:tt)*) => {
        if $eq {
            return Err(
                crate::pb::CommonResult{
                    code: 400,
                    message: format!($($arg)*),
                    payload: None
                })
        }
    };
}

#[macro_export]
macro_rules! server_error {
    ($($arg:tt)*) => {
            return Err(
                crate::pb::CommonResult{
                    code: 500,
                    message: format!($($arg)*),
                    payload: None
                })
    };
}
