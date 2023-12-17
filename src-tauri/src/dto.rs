use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const KEY_SK: &str = "SECRET_KEY";
pub const KEY_PASS: &str = "PASS_KEY";

pub const DIR_ENC: &str = "enc_files";
pub const DIR_THUMBNAILS: &str = "thumbnails";

pub static MAGIC_STRING: [u8; 8] = [0x4e, 0x2d, 0x51, 0xfa, 0x30, 0x57, 0x30, 0x5f];
pub static PAT_START_MAGIC_STRING: [u8; 6] = [0x0a, 0x0a, 0x00, 0x00, 0x0a, 0x0a];
pub static PAT_END_MAGIC_STRING: [u8; 6] = [0x0a, 0x0a, 0xff, 0xff, 0x0a, 0x0a];
pub static APAT_START_MAGIC_STRING: [u8; 6] = [0x0a, 0x0b, 0x00, 0x00, 0x0a, 0x0b];
pub static APAT_END_MAGIC_STRING: [u8; 6] = [0x0a, 0x0b, 0xff, 0xff, 0x0a, 0x0b];



#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ErrorResponse {
    pub code: u32,
    pub msg: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ClientPayload {
    pub password: String,
    pub secret_key: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AuthPayload {
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ImgEncryptPayload {
    pub path: PathBuf,
    pub name: String,
    pub folder_id: u64,
    pub thumbnail: Option<PathBuf>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ImgDecryptPayload {
    pub out_path: PathBuf,
    pub file_id: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct EmDataDir {
    pub id: u64,
    pub folder_id: u64,
    pub name: String,
    pub file_uid: String,
    pub file_ext: String,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub encrypted_at: chrono::DateTime<chrono::Utc>
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct EmFolder {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FolderRequest {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FolderResponse {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListFolderResponse {
    pub folders: Vec<FolderResponse>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FileMetaResponse {
    pub id: u64,
    pub folder_id: u64,
    pub name: String,
    pub file_uid: String,
    pub file_ext: String,
    pub encrypted_at: String,
    pub accessed_at: String,
    pub thumbnail: PathBuf,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListFileMetaResponse {
    pub files: Vec<FileMetaResponse>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FileMetaRequest {
    pub folder_id: u64,
    pub id: String,
    pub name: String,
}

