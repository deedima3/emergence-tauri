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
mod img_encryptor;
mod security_handler;
mod explorer_handler;

use config_store::ConfigStore;
use keystore_handler::{
    init_keystore, KeyStoreState,
    handle_is_registered, handle_register_client,
};
use security_handler::{
    handle_encrypt_data, handle_decrypt_data
};
use explorer_handler::{
    handle_get_all_folder,
    handle_get_all_file,
    handle_get_file,
    handle_create_folder,
    handle_update_file,
    handle_update_folder,
    handle_delete_file,
    handle_delete_folder
};
use auth_handler::handle_auth;
use tauri::{utils::config::AppUrl, Manager, State, WindowUrl, Window};
use tauri_plugin_log::LogTarget;

#[tauri::command]
async fn close_splashscreen(window: Window) {
  // Close splashscreen
  window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
  // Show main window
  window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

#[tauri::command]
async fn open_splashscreen(window: Window) {
  // Open splashscreen
  window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").show().unwrap();
}

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
            handle_auth, 
            handle_is_registered, 
            handle_register_client,
            handle_encrypt_data, 
            handle_decrypt_data, 
            handle_get_all_folder,
            handle_get_all_file,
            handle_get_file,
            handle_create_folder,
            handle_update_file,
            handle_update_folder,
            handle_delete_file,
            handle_delete_folder,
            open_splashscreen, 
            close_splashscreen
        ])
        .setup(|app| {
            // let splash_window = app.get_window("splashscreen").unwrap();
            // let main_window = app.get_window("main").unwrap();

            let handle = app.handle();

            let cfg_state: State<ConfigStore> = handle.state();
            let db = database::init_db(&handle).expect("failed to init database");
            *cfg_state.db.lock().unwrap() = Some(db);

            let keystore_state: State<KeyStoreState> = handle.state();
            let keystore = init_keystore(&cfg_state).unwrap_or_default();
                            *keystore_state.keystore.lock().unwrap() = keystore;

            // thread::sleep(std::time::Duration::from_secs(5));
            // splash_window.close().unwrap();
            // main_window.show().unwrap();
            
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
