use crate::handlers::WarpResult;
use warp::{http::StatusCode, Reply};

pub async fn health_handler() -> WarpResult<impl Reply> {
    Ok(StatusCode::OK)
}
