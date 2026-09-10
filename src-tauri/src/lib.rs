//! HomeHub Control Center native (Rust/Tauri) layer.
//!
//! Security boundary: the native layer exposes only a small allowlist of typed
//! commands (see `commands`). There is no arbitrary command/shell/subprocess
//! capability exposed to the UI. Network access (Control API health probes)
//! lives in the native layer (`health`), and server configuration persistence
//! lives behind the `server` manager.

mod commands;
mod config;
mod health;
mod result;
mod server;
mod updater;

use std::sync::Mutex;

use tauri::Manager;

/// Wire up the Tauri application: register plugins, managed state and handlers.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let config_dir = app
                .path()
                .app_config_dir()
                .map_err(|e| format!("Failed to resolve app config directory: {e}"))?;
            std::fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create app config directory: {e}"))?;
            app.manage(commands::AppState {
                manager: Mutex::new(server::ServerManager::new(config_dir)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_version,
            commands::check_for_updates,
            commands::list_servers,
            commands::save_server,
            commands::remove_server,
            commands::test_connection,
            commands::connect_server,
            commands::disconnect_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running HomeHub Control Center");
}
