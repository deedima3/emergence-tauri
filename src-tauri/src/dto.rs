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

pub(crate) static MAGIC_STRING: [u8; 8] = [0x4e, 0x2d, 0x51, 0xfa, 0x30, 0x57, 0x30, 0x5f];
pub(crate) static PAT_START_MAGIC_STRING: [u8; 6] = [0x0a, 0x0a, 0x00, 0x00, 0x0a, 0x0a];
pub(crate) static PAT_END_MAGIC_STRING: [u8; 6] = [0x0a, 0x0a, 0xff, 0xff, 0x0a, 0x0a];
pub(crate) static APAT_START_MAGIC_STRING: [u8; 6] = [0x0a, 0x0b, 0x00, 0x00, 0x0a, 0x0b];
pub(crate) static APAT_END_MAGIC_STRING: [u8; 6] = [0x0a, 0x0b, 0xff, 0xff, 0x0a, 0x0b];

