use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidBallValue(#[from] derive_more::TryFromReprError<usize>),
    #[error("无法找到执行器上下文属性：{0}.")]
    ContextAttrNotExist(String),
    #[error(transparent)]
    IoFailure(#[from] std::io::Error),
    #[error("其他错误: {0}")]
    OtherFailure(String),
}
