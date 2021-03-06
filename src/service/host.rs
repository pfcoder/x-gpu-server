use crate::{
    error::Result,
    handlers::host::HostRegisterRequest,
    model::{CreateHostData, Host},
    service::ws::Client,
    CLIENTS,
};

use uuid::Uuid;

#[derive(Debug)]
pub enum HostStatus {
    Idle,
    Renting,
    SelfWorking,
    Fault,
}

impl From<i32> for HostStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => HostStatus::Idle,
            1 => HostStatus::Renting,
            2 => HostStatus::SelfWorking,
            3 => HostStatus::Fault,
            _ => panic!("invalid host status"),
        }
    }
}

impl Into<i32> for HostStatus {
    fn into(self) -> i32 {
        match self {
            HostStatus::Idle => 0,
            HostStatus::Renting => 1,
            HostStatus::SelfWorking => 2,
            HostStatus::Fault => 3,
        }
    }
}

pub struct HostService;

impl HostService {
    pub async fn register(body: HostRegisterRequest) -> Result<String> {
        // db operation
        let host = match body.id {
            Some(id) => {
                // TODO: update
                Host::find_by_id(Uuid::parse_str(&id).unwrap()).await?
            }
            None => {
                Host::create(CreateHostData {
                    status: HostStatus::Idle.into(),
                    cpu_info: body.cpu_info,
                    gpu_info: body.gpu_info,
                    ram: body.ram,
                    hd: body.hd,
                    ip: body.ip,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
                .await?
            }
        };

        // update hash map
        let client = Client {
            id: host.id.to_string(),
            sender: None,
        };
        // insert client into hash map
        CLIENTS.write().await.insert(host.id.to_string(), client);

        Ok(host.id.to_string())
    }

    pub async fn find_by_id(id: &str) -> Result<Host> {
        let id = Uuid::parse_str(id)?;
        Host::find_by_id(id).await
    }
}
