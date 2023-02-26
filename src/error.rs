use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifoldError {
    #[error("JSON parse error")]
    ParseError(#[from] serde_json::Error),

    #[error("HTTP error")]
    HttpError(#[from] reqwest::Error),

    #[error("Unexpected schema error: {0} {1:?}")]
    SchemaError(String, Option<Value>),

    #[error("Other error")]
    Other(String),
}

pub type Result<T> = core::result::Result<T, ManifoldError>;
