use serde::{ser::Serializer, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("invalid payload")]
    InvalidPayload,
    #[error("key not found")]
    KeyNotFound,
    #[error("data integrity err: {0}")]
    DataIntegrityError(String),
    #[error("api error: {0}")]
    GenericError(String),
    #[error("missing config key: {0}")]
    ConfigMalformedError(String),
}

impl Serialize for BackendError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serializer.serialize_struct(name, len)
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type BackendResult<T, E = BackendError> = anyhow::Result<T, E>;