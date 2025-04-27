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
