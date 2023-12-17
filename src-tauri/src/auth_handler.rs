use crate::{config_store::ConfigStore, dto::{AuthPayload, KEY_PASS}, error::{BackendError, BackendResult}, security::verify_password};

#[tauri::command]
pub async fn handle_auth(cfg_state: tauri::State<'_, ConfigStore>, payload: AuthPayload ) -> BackendResult<(), BackendError> {
    let db_pass = cfg_state.get(KEY_PASS.to_string()).map_err(|e| BackendError::GenericError(e.to_string()))?;
    if db_pass.is_none() {
        return Err(BackendError::ConfigMalformedError("password not set".to_string()));
    }

    if !verify_password(payload.password, db_pass.unwrap())? {
        return Err(BackendError::ConfigMalformedError("password not set".to_string()));
    } 

    Ok(())
}
