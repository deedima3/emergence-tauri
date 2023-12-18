use std::{fs, default};

use log::debug;

use crate::{
    config_store::ConfigStore,
    dto::{
        EmDataDir, FileMetaRequest, FileMetaResponse, FolderResponse, ListFileMetaResponse,
        ListFolderResponse, DIR_THUMBNAILS, DIR_ENC,
    },
    error::{BackendError, BackendResult},
};

#[tauri::command]
pub async fn handle_get_all_folder(
    cfg_state: tauri::State<'_, ConfigStore>,
) -> BackendResult<ListFolderResponse, BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    let mut stmt = conn
        .prepare("select id, name from em_folders")
        .map_err(|e| BackendError::GenericError(e.to_string()))?;

    let mut rows = stmt
        .query([])
        .map_err(|e| BackendError::GenericError(e.to_string()))?;

    let mut res: Vec<FolderResponse> = Vec::new();
    while let Some(row) = rows
        .next()
        .map_err(|e| BackendError::GenericError(e.to_string()))?
    {
        res.push(FolderResponse {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        })
    }

    Ok(ListFolderResponse { folders: res })
}

#[tauri::command]
pub async fn handle_get_all_file(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<ListFileMetaResponse, BackendError> {
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

    let mut stmt = conn
        .prepare("select id, folder_id, name, encrypted_at, accessed_at, file_uid, file_ext from em_data_dir where folder_id = :folder_id")
        .map_err(|e| BackendError::GenericError(e.to_string()))?;

    let mut rows = stmt
        .query(&[(":folder_id", &payload.folder_id)])
        .map_err(|e| BackendError::GenericError(e.to_string()))?;

    let mut data: Vec<EmDataDir> = Vec::new();
    while let Some(row) = rows
        .next()
        .map_err(|e| BackendError::GenericError(e.to_string()))?
    {
        data.push(EmDataDir {
            id: row.get(0).unwrap(),
            folder_id: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            encrypted_at: row.get(3).unwrap(),
            accessed_at: row.get(4).unwrap(),
            file_uid: row.get(5).unwrap(),
            file_ext: row.get(6).unwrap(),
        })
    }

    let mut res: Vec<FileMetaResponse> = Vec::new();
    for d in data {
        res.push(FileMetaResponse {
            id: d.id,
            folder_id: d.folder_id,
            name: d.name,
            encrypted_at: d.encrypted_at,
            accessed_at: d.accessed_at,
            file_uid: d.file_uid.clone(),
            file_ext: d.file_ext.clone(),
            thumbnail: app_dir
                .join(DIR_THUMBNAILS)
                .join(format!("{}_{}.{}", d.folder_id, d.file_uid, d.file_ext)),
        })
    }

    Ok(ListFileMetaResponse { files: res })
}

#[tauri::command]
pub async fn handle_get_file(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<FileMetaResponse, BackendError> {
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

    let res = match conn.query_row(
        "select folder_id, file_uid, file_ext from em_data_dir where file_uid = :id",
        &[(":id", &payload.id)],
        |row| {
            Ok(EmDataDir {
                id: row.get(0).unwrap(),
                folder_id: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                encrypted_at: row.get(3).unwrap(),
                accessed_at: row.get(4).unwrap(),
                file_uid: row.get(5).unwrap(),
                file_ext: row.get(6).unwrap(),
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

    Ok(FileMetaResponse {
        id: res.id,
        folder_id: res.folder_id,
        name: res.name,
        encrypted_at: res.encrypted_at,
        accessed_at: res.accessed_at,
        file_uid: res.file_uid.clone(),
        file_ext: res.file_ext.clone(),
        thumbnail: app_dir.join(DIR_THUMBNAILS).join(format!(
            "{}_{}.{}",
            res.folder_id, res.file_uid, res.file_ext
        )),
    })
}

#[tauri::command]
pub async fn handle_create_folder(
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<(), BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    match conn.execute(
        "insert into em_folders(name) values (:name)",
        &[(":name", &payload.name)],
    ) {
        Ok(v) => v,
        Err(_) => {
            return Err(BackendError::DataIntegrityError(
                "data not found".to_string(),
            ))
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn handle_update_folder(
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<(), BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    debug!("{} {}", payload.folder_id, payload.name);

    match conn.execute(
        "update em_folders set name = :name where id = :id",
        &[
            (":name", &payload.name),
            (":id", &payload.folder_id.to_string()),
        ],
    ) {
        Ok(v) => v,
        Err(e) => {
            return Err(BackendError::DataIntegrityError(format!(
                "data not found err: {}",
                e
            )))
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn handle_update_file(
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<(), BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    match conn.execute(
        "update em_data_dir set name = :name where file_uid = :file_id",
        &[(":name", &payload.name), (":file_id", &payload.id)],
    ) {
        Ok(v) => v,
        Err(e) => {
            return Err(BackendError::DataIntegrityError(format!(
                "data not found err: {}",
                e
            )))
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn handle_delete_folder(
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<(), BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    debug!("{} {}", payload.folder_id, payload.name);

    match conn.execute(
        "delete from em_folders where id = :id",
        &[(":id", &payload.folder_id.to_string())],
    ) {
        Ok(v) => v,
        Err(e) => {
            return Err(BackendError::DataIntegrityError(format!(
                "data not found err: {}",
                e
            )))
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn handle_delete_file(
    app_handle: tauri::AppHandle,
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FileMetaRequest,
) -> BackendResult<(), BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "cannot connect to db".to_string(),
            ))
        }
    };

    let app_dir = match app_handle.path_resolver().app_data_dir() {
        Some(v) => v,
        None => {
            return Err(BackendError::GenericError(
                "appdata dir should be exists".to_string(),
            ))
        }
    };

    let res = match conn.query_row(
        "select folder_id, file_uid, file_ext from em_data_dir where file_uid = :id",
        &[(":id", &payload.id)],
        |row| {
            Ok(EmDataDir {
                id: 0,
                folder_id: row.get(0).unwrap(),
                name: "".to_string(),
                encrypted_at: "".to_string(),
                accessed_at: "".to_string(),
                file_uid: row.get(1).unwrap(),
                file_ext: row.get(2).unwrap(),
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

    match fs::remove_file(app_dir.join(DIR_ENC).join(format!("{}_{}.{}", res.folder_id, res.file_uid, res.file_ext))) {
        Ok(v) => v,
        Err(_) => {
            return Err(BackendError::DataIntegrityError(
                "data not found".to_string(),
            ))
        }
    };

    match fs::remove_file(app_dir.join(DIR_THUMBNAILS).join(format!("{}_{}.{}", res.folder_id, res.file_uid, res.file_ext))) {
        Ok(v) => v,
        Err(_) => {
            return Err(BackendError::DataIntegrityError(
                "data not found".to_string(),
            ))
        }
    };

    match conn.execute(
        "delete from em_data_dir where file_uid = :id",
        &[(":id", &payload.id)],
    ) {
        Ok(v) => v,
        Err(e) => {
            return Err(BackendError::DataIntegrityError(format!(
                "data not found err: {}",
                e
            )))
        }
    };

    Ok(())
}
