use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifoldError {
    #[error("JSON parse error")]
    ParseError(#[from] serde_json::Error),

    #[error("HTTP error")]
    HttpError(#[from] reqwest::Error),

    #[error("Unexpected schema error")]
    SchemaError(String),

    #[error("Other error")]
    Other(String),
}

pub type Result<T> = core::result::Result<T, ManifoldError>;
