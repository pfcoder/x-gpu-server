use crate::{handlers::WarpResult, service::host::HostService};

pub async fn ws_handler(ws: warp::ws::Ws, id: String) -> WarpResult<impl Reply> {
    // make sure id is db valid
    match HostService::find_by_id(&id).await {
        Ok(h) => {
            host = h;
        }
        Err(e) => {
            tracing::error!("Failed to find host with id {}: {}", id, e);
            return Err(warp::reject::not_found());
        }
    }
    ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))
}
