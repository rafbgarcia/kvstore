use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("Key not found")]
    KeyNotFound,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetResponse {
    Ok(Option<String>),
}

pub type Result<T> = AnyResult<T, KvsError>;
