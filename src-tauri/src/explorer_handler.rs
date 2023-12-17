use crate::{
    config_store::ConfigStore,
    dto::{FolderResponse, ListFolderResponse, EmDataDir, FileMetaResponse, ListFileMetaResponse, DIR_THUMBNAILS, FileMetaRequest},
    error::{BackendError, BackendResult},
};

#[tauri::command]
pub async fn handle_get_all_folder(
    cfg_state: tauri::State<'_, ConfigStore>,
) -> BackendResult<ListFolderResponse, BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => return Err(BackendError::GenericError("cannot connect to db".to_string())),
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
        res.push(FolderResponse{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        })
    }

    Ok(ListFolderResponse{folders: res})
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
            return Err(BackendError::GenericError("appdata dir should be exists".to_string()))
        }
    };

    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => return Err(BackendError::GenericError("cannot connect to db".to_string())),
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
        data.push(EmDataDir{
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
            encrypted_at: d.encrypted_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            accessed_at: d.accessed_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            file_uid: d.file_uid.clone(),
            file_ext: d.file_ext.clone(),
            thumbnail: app_dir.join(DIR_THUMBNAILS).join(format!("{}_{}.{}", d.folder_id, d.file_uid, d.file_ext))
        })   
    }

    Ok(ListFileMetaResponse{files: res})
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
            return Err(BackendError::GenericError("appdata dir should be exists".to_string()))
        }
    };

    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => return Err(BackendError::GenericError("cannot connect to db".to_string())),
    };

    let res = match conn.query_row(
        "select folder_id, file_uid, file_ext from em_data_dir where file_uid = :id",
        &[(":id", &payload.id)],
        |row| Ok(EmDataDir{
            id: row.get(0).unwrap(),
            folder_id: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            encrypted_at: row.get(3).unwrap(),
            accessed_at: row.get(4).unwrap(),
            file_uid: row.get(5).unwrap(),
            file_ext: row.get(6).unwrap(),
        }),
    ) {
        Ok(v) => v, 
        Err(_) => return Err(BackendError::DataIntegrityError("data not found".to_string()))
    };


    Ok(FileMetaResponse {
        id: res.id,
        folder_id: res.folder_id,
        name: res.name,
        encrypted_at: res.encrypted_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        accessed_at: res.accessed_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        file_uid: res.file_uid.clone(),
        file_ext: res.file_ext.clone(),
        thumbnail: app_dir.join(DIR_THUMBNAILS).join(format!("{}_{}.{}", res.folder_id, res.file_uid, res.file_ext))
    })
}

#[tauri::command]
pub async fn handle_create_folder(
    cfg_state: tauri::State<'_, ConfigStore>,
    payload: FolderResponse,
) -> BackendResult<(), BackendError> {
    let db = &mut *cfg_state.db.lock().unwrap();
    let conn = match db {
        Some(v) => v,
        None => return Err(BackendError::GenericError("cannot connect to db".to_string())),
    };

    match conn.execute(
        "insert into em_folders(name) values (:name)",
        &[(":name", &payload.name)]) {
        Ok(v) => v, 
        Err(_) => return Err(BackendError::DataIntegrityError("data not found".to_string()))
    };


    Ok(())
}