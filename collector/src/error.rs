use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoFailure(#[from] std::io::Error),
    #[error(transparent)]
    FailToSerde(#[from] serde_json::Error),
    #[error(transparent)]
    InvalidBallValue(#[from] derive_more::TryFromReprError<usize>),
    #[error(transparent)]
    ReqwestFailure(#[from] reqwest::Error),
}
