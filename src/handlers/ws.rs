use crate::{handlers::WarpResult, service::ws::client_connection, CLIENTS};
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, id: String) -> WarpResult<impl Reply> {
    // make sure id is db valid
    let client = CLIENTS.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, c))),
        None => Err(warp::reject::not_found()),
    }
}
