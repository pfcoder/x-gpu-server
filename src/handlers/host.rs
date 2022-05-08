use crate::{handlers::WarpResult, service::host::HostService};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Debug, Deserialize)]
pub struct HostRegisterRequest {
    pub cpu_info: Option<Value>,
    pub gpu_info: Option<Value>,
    pub ram: Option<i32>,
    pub hd: Option<i32>,
    pub ip: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct HostRegisterResponse {
    id: String,
}

pub async fn register_handler(body: HostRegisterRequest) -> WarpResult<impl Reply> {
    match HostService::register(body).await {
        Ok(id) => Ok(json(&HostRegisterResponse { id })),
        Err(_e) => Err(warp::reject::not_found()), // TODO: fix this
    }
}
