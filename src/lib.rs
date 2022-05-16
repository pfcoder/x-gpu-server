extern crate lazy_static;
#[macro_use]
extern crate serde;

use futures::Future;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use warp::Filter;

pub mod configuration;
mod error;
pub mod handlers;
pub mod model;
pub mod service;
pub mod sql;

use crate::{
    configuration::{get_configuration, Settings},
    service::ws::Clients,
};
use handlers::{
    health::health_handler,
    host::{publish_handler, register_handler},
    ws::ws_handler,
};
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;

lazy_static! {
    pub static ref CONFIG: Settings = get_configuration().expect("Failed to read configuration.");
    pub static ref PG_POOL: PgPool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(CONFIG.database.with_db());
    pub static ref CLIENTS: Clients = Arc::new(RwLock::new(HashMap::new()));
}

pub fn server() -> impl Future<Output = ()> {
    let health_route = warp::path!("health").and_then(health_handler);

    let register_route = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and_then(ws_handler);

    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and_then(publish_handler);

    let routes = health_route
        .or(register_route)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], CONFIG.server.port))
}
