use std::num::ParseIntError;

use polars::error::PolarsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Polars(#[from] PolarsError),
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error("Context attribute not existing: {0}")]
    ContextAttrNotExist(String),
    #[error("Other error: {0}")]
    Other(String),
}
