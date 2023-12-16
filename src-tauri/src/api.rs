use serde::{Deserialize, Serialize};

macro_rules! env_or {
    ($name: expr, $default: expr) => {
        if let Some(val) = option_env!($name) {
            val
        } else {
            $default
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    #[serde(rename = "msg")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PingResponse {
    message: String,
    timestamp: u64,
}
