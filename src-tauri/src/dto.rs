use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClientPayload {
    pub password: String,
    pub secret_key: String
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AuthPayload {
    pub password: String,
}

pub const KEY_SK: &str = "SECRET_KEY";
pub const KEY_PASS: &str = "PASS_KEY";


