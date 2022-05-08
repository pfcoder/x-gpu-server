use warp::Rejection;

pub mod health;
pub mod host;

pub type WarpResult<T> = std::result::Result<T, Rejection>;
