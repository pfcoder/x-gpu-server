use crate::PG_POOL;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::Result,
    model::{CreateHostData, Host},
};

impl Host {
    pub async fn find_by_id(id: Uuid) -> Result<Host> {
        let sql = format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", Host::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_one(&*PG_POOL).await?)
    }

    pub async fn create(data: CreateHostData) -> Result<Host> {
        let sql = format!(
            "INSERT INTO {} (status, cpu_info, gpu_info, ram, hd, ip, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            Host::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.status)
            .bind(data.cpu_info)
            .bind(data.gpu_info)
            .bind(data.ram)
            .bind(data.hd)
            .bind(data.ip)
            .bind(data.created_at)
            .bind(data.updated_at)
            .fetch_one(&*PG_POOL)
            .await?)
    }
}
