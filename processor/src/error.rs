use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidBallValue(#[from] derive_more::TryFromReprError<usize>),
    #[error("processor context attribute [{0}] not exist.")]
    ContextAttrNotExist(String),
    #[error(transparent)]
    IoFailure(#[from] std::io::Error),
}
