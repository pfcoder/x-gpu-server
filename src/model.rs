use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Host {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub status: i32,
    pub cpu_info: Option<Value>,
    pub gpu_info: Option<Value>,
    pub ram: Option<i32>,
    pub hd: Option<i32>,
    pub ip: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Host {
    pub const TABLE: &'static str = "hosts";
}

#[derive(Debug)]
pub struct CreateHostData {
    pub status: i32,
    pub cpu_info: Option<Value>,
    pub gpu_info: Option<Value>,
    pub ram: Option<i32>,
    pub hd: Option<i32>,
    pub ip: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
