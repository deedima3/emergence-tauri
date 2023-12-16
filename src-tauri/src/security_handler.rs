use std::{fs, default};

use rusqlite::named_params;
use uuid::Uuid;

use crate::{
    config_store::ConfigStore,
    dto::{ImgDecryptPayload, ImgEncryptPayload, KEY_PASS, EmDataDir, DIR_ENC, ImgDecryptResponse},
    error::{BackendError, BackendResult},
    img_encryptor::{encrypt_image, decrypt_image},
};

#[tauri::command]
pub async fn handle_encrypt_data(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: ImgEncryptPayload,
) -> BackendResult<(), BackendError> {
    match fs::read(payload.path.clone()) {
        Ok(_) => (),
        Err(e) => {
            return Err(BackendError::GenericError(format!(
                "failed to open source file err: {}",
                e
            )))
        }
    };

    if let Some(thumbnail) = payload.thumbnail.clone() {
        match fs::read(thumbnail) {
            Ok(_) => (),
            Err(e) => {
                return Err(BackendError::GenericError(format!(
                    "failed to open source file err: {}",
                    e
                )))
            }
        };
    }

    let app_dir = match app_handle.path_resolver().app_data_dir() {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(format!(
                "appdata dir should be exists"
            )))
        }
    };

    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => return Err(BackendError::GenericError(format!("cannot connect to db"))),
    };

    let file_uid = Uuid::new_v4();

    let tx = conn.transaction().unwrap();
    tx.execute(
        "
        insert into em_data_dir (folder_id, name, file_uid, file_ext, encrypted_at, accessed_at) 
        values (:folder_id, :name, :file_uid, :file_ext, now(), now())
    ",
        named_params! {
            ":folder_id": payload.folder_id,
            ":name": payload.name,
            ":file_uid": file_uid.to_string(),
            ":file_ext": payload.path.extension().unwrap().to_str().unwrap(),
        },
    )
    .unwrap();

    let pass = cfg_state.get(KEY_PASS.to_string()).unwrap().unwrap();

    match encrypt_image(
        payload.path,
        payload.thumbnail,
        pass,
        app_dir,
        format!("{}_{}", payload.folder_id, file_uid),
    ) {
        Ok(_) => (),
        Err(e) => {
            return Err(BackendError::GenericError(format!(
                "failed to encrypt changes err: {}",
                e
            )))
        }
    };

    match tx.commit() {
        Ok(_) => (),
        Err(e) => {
            return Err(BackendError::GenericError(format!(
                "failed to commit changes err: {}",
                e
            )))
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn handle_decrypt_data(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: ImgDecryptPayload,
) -> BackendResult<ImgDecryptResponse, BackendError> {
    let app_dir = match app_handle.path_resolver().app_data_dir() {
        Some(v) => v,
        None => return Err(BackendError::GenericError(format!("appdata dir should be exists")))
    };

    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => return Err(BackendError::GenericError(format!("cannot connect to db"))),
    };

    let tx = conn.transaction().unwrap();
    let res = match tx.query_row(
        "select folder_id, file_uid, file_ext from em_data_dir where file_uid = :id",
        &[(":id", &payload.file_id)],
        |r| Ok(EmDataDir{
            id: 0,
            name: "".to_string(),
            accessed_at: chrono::Utc::now(),
            encrypted_at: chrono::Utc::now(),
            folder_id: r.get(0).unwrap(),
            file_uid: r.get(1).unwrap(),
            file_ext: r.get(2).unwrap(),
        }),
    ) {
        Ok(v) => v, 
        Err(_) => return Err(BackendError::DataIntegrityError("data not found".to_string()))
    };

    match fs::read(app_dir.join(DIR_ENC).join(format!("{}_{}.{}", res.folder_id, res.file_uid, res.file_ext))) {
        Ok(_) => (),
        Err(e) => return Err(BackendError::DataIntegrityError(format!("failed to open source file err: {}", e)))
    };

    let pass = cfg_state.get(KEY_PASS.to_string()).unwrap().unwrap();

    let data = match decrypt_image(app_dir, format!("{}_{}.{}", res.folder_id, res.file_uid, res.file_ext), pass) {
        Ok(v) => v,
        Err(e) => return Err(BackendError::GenericError(format!("failed to encrypt changes err: {}", e)))
    };

    Ok(ImgDecryptResponse {data})
}
