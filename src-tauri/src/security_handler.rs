use std::fs;

use log::debug;
use rusqlite::named_params;
use uuid::Uuid;

use crate::{
    config_store::ConfigStore,
    dto::{EmDataDir, ImgDecryptPayload, ImgEncryptPayload, DIR_ENC, KEY_PASS},
    error::{BackendError, BackendResult},
    img_encryptor::{decrypt_image, encrypt_image},
    keystore_handler::KeyStoreState,
};

#[tauri::command]
pub async fn handle_encrypt_data(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    key_state: tauri::State<'_, KeyStoreState>,
    payload: ImgEncryptPayload,
) -> BackendResult<(), BackendError> {
    let secret_key = {
        let keystore = &mut *key_state.keystore.lock().unwrap();
        keystore.get_secret_key()
    };

    if !payload.path.exists() {
        debug!("{}: bruh", payload.name);
        return Err(BackendError::GenericError(
            "failed to open source file".to_string(),
        ));
    };

    if let Some(thumbnail) = payload.thumbnail.clone() {
        if !thumbnail.exists() {
            debug!("{}: bruh", payload.name);
            return Err(BackendError::GenericError(
                "failed to open source file".to_string(),
            ));
        };
    }

    let app_dir = match app_handle.path_resolver().app_data_dir() {
        Some(v) => v,
        None => {
            debug!("{}: bruh", payload.name);
            return Err(BackendError::GenericError(
                "appdata dir should be exists".to_string(),
            ));
        }
    };

    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    let file_uid = Uuid::new_v4();

    let tx = conn.transaction().unwrap();
    tx.execute(
        "
        insert into em_data_dir (folder_id, name, file_uid, file_ext, encrypted_at, accessed_at) 
        values (:folder_id, :name, :file_uid, :file_ext, time(), time())
        ",
        named_params! {
            ":folder_id": payload.folder_id,
            ":name": payload.name,
            ":file_uid": file_uid.to_string(),
            ":file_ext": payload.path.extension().unwrap().to_str().unwrap(),
        },
    )
    .unwrap();

    debug!("ehe");

    match encrypt_image(
        payload.path,
        payload.thumbnail,
        secret_key,
        app_dir,
        format!("{}_{}", payload.folder_id, file_uid),
    ) {
        Ok(_) => (),
        Err(e) => {
            debug!("{}: bruh", e);
            return Err(BackendError::GenericError(format!(
                "failed to encrypt changes err: {}",
                e
            )));
        }
    };
    debug!("ehe");

    match tx.commit() {
        Ok(_) => (),
        Err(e) => {
            return Err(BackendError::GenericError(format!(
                "failed to commit changes err: {}",
                e
            )))
        }
    };
    debug!("ehe");

    Ok(())
}

#[tauri::command]
pub async fn handle_decrypt_data(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    key_state: tauri::State<'_, KeyStoreState>,
    payload: ImgDecryptPayload,
) -> BackendResult<(), BackendError> {
    let secret_key = {
        let keystore = &mut *key_state.keystore.lock().unwrap();
        keystore.get_secret_key()
    };

    let app_dir = match app_handle.path_resolver().app_data_dir() {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "appdata dir should be exists".to_string(),
            ))
        }
    };

    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    let tx = conn.transaction().unwrap();
    let res = match tx.query_row(
        "select folder_id, file_uid, file_ext from em_data_dir where file_uid = :id",
        &[(":id", &payload.file_id)],
        |r| {
            Ok(EmDataDir {
                id: 0,
                name: "".to_string(),
                accessed_at: chrono::Utc::now(),
                encrypted_at: chrono::Utc::now(),
                folder_id: r.get(0).unwrap(),
                file_uid: r.get(1).unwrap(),
                file_ext: r.get(2).unwrap(),
            })
        },
    ) {
        Ok(v) => v,
        Err(_) => {
            return Err(BackendError::DataIntegrityError(
                "data not found".to_string(),
            ))
        }
    };

    match fs::read(app_dir.join(DIR_ENC).join(format!(
        "{}_{}.{}",
        res.folder_id, res.file_uid, res.file_ext
    ))) {
        Ok(_) => (),
        Err(e) => {
            return Err(BackendError::DataIntegrityError(format!(
                "failed to open source file err: {}",
                e
            )))
        }
    };

    match decrypt_image(
        app_dir,
        format!("{}_{}.{}", res.folder_id, res.file_uid, res.file_ext),
        secret_key,
        payload.out_path,
    ) {
        Ok(v) => v,
        Err(e) => {
            return Err(BackendError::GenericError(format!(
                "failed to encrypt changes err: {}",
                e
            )))
        }
    };

    Ok(())
}
