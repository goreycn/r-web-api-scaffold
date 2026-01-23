use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

/// 定义内部错误
#[derive(Error, Debug)]
pub enum MzError {
    // 系统兜底错误
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl MzError {
    pub fn code(&self) -> u16 {
        match self {
            MzError::Anyhow(_) => 500,
        }
    }
}

impl ResponseError for MzError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(MzResp::<&str>::ng(self.code(), &self.to_string(), ""))
    }
}


#[derive(Serialize)]
pub struct MzResp<T> {
    pub(crate) code: u16,
    pub(crate) msg: String,
    pub(crate) data: T,
}
impl<T> MzResp<T> {
    pub fn new(code: u16, msg: &str, data: T) -> MzResp<T> {
        MzResp {
            code,
            msg: "".to_string(),
            data,
        }
    }
    pub fn ok(data: T) -> MzResp<T> {
        MzResp {
            code: 0,
            msg: "success".to_string(),
            data,
        }
    }
    pub fn ng(code: u16, msg: &str, data: T) -> MzResp<T> {
        MzResp {
            code,
            msg: msg.to_string(),
            data,
        }
    }
}

impl<T: Serialize> Responder for MzResp<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let resp = json!({
            "code": self.code,
            "message": self.msg,
            "data": self.data,
            "timestamp": chrono::Utc::now(),
        });

        HttpResponse::Ok().json(resp)
    }
}


/// 统一接口返回类型：
/// Ok( T)
/// Err 通过上面的 ResponseError() 转换为通用异常方法
pub type MzResult<T> = Result<MzResp<T>, MzError>;