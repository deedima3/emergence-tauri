// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod config_store;
mod database;
mod error;
mod mapper;
mod security;
mod dto;
mod keystore_handler;
mod auth_handler;

use config_store::ConfigStore;
use keystore_handler::{
    init_keystore, KeyStoreState,
    handle_is_registered, handle_register_client,
};
use auth_handler::handle_auth;
use tauri::{utils::config::AppUrl, Manager, State, WindowUrl};
use tauri_plugin_log::LogTarget;

#[tokio::main]
async fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");

    let mut context = tauri::generate_context!();
    let url = format!("https://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);

    context.config_mut().build.dist_dir = AppUrl::Url(window_url);

    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .build(),
        )
        .manage(ConfigStore {
            db: Default::default(),
        })
        .manage(KeyStoreState {
            keystore: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            handle_auth, handle_is_registered, handle_register_client
        ])
        .setup(|app| {
            let handle = app.handle();

            let cfg_state: State<ConfigStore> = handle.state();
            let db = database::init_db(&handle).expect("failed to init database");
            *cfg_state.db.lock().unwrap() = Some(db);

            let keystore_state: State<KeyStoreState> = handle.state();
            let keystore = init_keystore(&cfg_state).unwrap_or_default();
                            *keystore_state.keystore.lock().unwrap() = keystore;

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
