use serde_json::Error as SerdeJsonError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    CommandError(#[from] io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error(transparent)]
    StringConvertUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    WrapError(#[from] warp::Error),
    #[error(transparent)]
    UuidParseError(#[from] uuid::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
