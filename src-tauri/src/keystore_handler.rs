use std::sync::{Arc, Mutex};

use data_encoding::BASE64;

use crate::{
    config_store::ConfigStore,
    error::{BackendError, BackendResult}, dto::{ClientPayload, KEY_SK, KEY_PASS}, security::{hash_password, expand_secret_key, generate_initial_secret_key},
};

pub struct KeyStore {
    secret_key: Vec<u8>,
}

impl Default for KeyStore {
    fn default() -> Self {
        Self { secret_key: vec![] }
    }
}

pub struct KeyStoreState {
    pub keystore: Arc<Mutex<KeyStore>>,
}

pub fn init_keystore(config: &ConfigStore) -> Result<KeyStore, BackendError> {
    let sk = config
        .get(KEY_SK.to_string())
        .expect("failed to fetch secret_key")
        .unwrap_or("".to_string());

    if sk.is_empty() {
        return Err(BackendError::ConfigMalformedError("SecretKey".to_string()));
    }

    let sk = BASE64.decode(sk.as_bytes()).unwrap();
    Ok(KeyStore { secret_key: sk })
}

impl KeyStore {
    pub fn get_secret_key(&self) -> Vec<u8> {
        self.secret_key.clone()
    }

    pub fn set_secret_key(&mut self, secret_key: Vec<u8>) {
        self.secret_key = secret_key;
    }
}

#[tauri::command]
pub async fn handle_is_registered(state: tauri::State<'_, KeyStoreState>) -> BackendResult<bool, ()> {
    let ks = &mut *state.keystore.lock().unwrap();

    Ok(!ks.get_secret_key().is_empty())
}

#[tauri::command]
pub async fn handle_register_client(key_state: tauri::State<'_, KeyStoreState>, cfg_state: tauri::State<'_, ConfigStore>, payload: ClientPayload ) -> BackendResult<(), BackendError> {
    let secret = generate_initial_secret_key(payload.secret_key.into_bytes())?;
    let password = hash_password(payload.password)?;
    
    let encded_secret = BASE64.encode(&secret);

    match cfg_state.save(KEY_SK.to_string(), Some(encded_secret)) {
        Ok(_) => (),
        Err(e) => {
           return Err(BackendError::GenericError(format!(
              "failed to save config for secret_key err: {}",
              e
           )))
        }
    }

    let ks = &mut *key_state.keystore.lock().unwrap();
    ks.set_secret_key(secret);

    match cfg_state.save(KEY_PASS.to_string(), Some(password)) {
        Ok(_) => (),
        Err(e) => {
           return Err(BackendError::GenericError(format!(
              "failed to save config for passkey err: {}",
              e
           )))
        }
    }

    Ok(())
}
