use crate::config::errors::{MzResp, MzResult};
use actix_web::HttpResponse;
use actix_web::error::JsonPayloadError;
use anyhow::anyhow;
use tracing::info;

pub async fn health_check() -> MzResult<&'static str> {
    info!("Health check");
    let out = MzResp::<&str>::ok("healthy");
    Ok(out)
}

pub async fn not_found() -> MzResult<()> {
    Err(anyhow!("Not found!!!").into())
}

pub fn json_error(err: JsonPayloadError, _: &actix_web::HttpRequest) -> actix_web::Error {
    let error = format!("JSON error: {}", err);
    let resp = MzResp::<()>::ng(500, &error, ());
    actix_web::error::InternalError::from_response(err, HttpResponse::Ok().json(resp)).into()
}
