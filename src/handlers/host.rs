use crate::{handlers::WarpResult, service::host::HostService, CLIENTS};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Debug, Deserialize)]
pub struct HostRegisterRequest {
    pub id: Option<String>,
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

#[derive(Deserialize, Debug)]
pub struct PublishEvent {
    message: String,
}

pub async fn register_handler(body: HostRegisterRequest) -> WarpResult<impl Reply> {
    match HostService::register(body).await {
        Ok(id) => Ok(json(&HostRegisterResponse { id })),
        Err(_e) => Err(warp::reject::not_found()), // TODO: fix this
    }
}

pub async fn publish_handler(body: PublishEvent) -> WarpResult<impl Reply> {
    CLIENTS.read().await.iter().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(body.message.clone())));
        }
    });

    Ok(StatusCode::OK)
}
