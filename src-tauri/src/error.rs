use serde::{ser::{Serializer, SerializeStruct}, Serialize};
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
    #[error("entity not found")]
    EntityNotFound,
    #[error("no access")]
    NoAccess,    
}

impl Serialize for BackendError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ErrorResponse", 2)?;
        match self {
            BackendError::ConfigMalformedError(_) => {
                state.serialize_field("code", "401001")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
            BackendError::DataIntegrityError(_) => {
                state.serialize_field("code", "403002")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
            BackendError::EntityNotFound => {
                state.serialize_field("code", "404003")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
            BackendError::GenericError(_) => {
                state.serialize_field("code", "500004")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
            BackendError::InvalidPayload => {
                state.serialize_field("code", "400005")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
            BackendError::KeyNotFound => {
                state.serialize_field("code", "403006")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
            BackendError::NoAccess => {
                state.serialize_field("code", "403007")?;
                state.serialize_field::<str>("msg", self.to_string().as_ref())?;
            },
        }
        state.end()
    }
}

pub type BackendResult<T, E = BackendError> = anyhow::Result<T, E>;