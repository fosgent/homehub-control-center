//! HomeHub Control Center native (Rust/Tauri) layer.
//!
//! Security boundary: the native layer exposes only a small allowlist of typed
//! commands (see `commands`). There is no arbitrary command/shell/subprocess
//! capability exposed to the UI.

mod commands;
mod config;
mod health;
mod result;
mod updater;

/// Wire up the Tauri application: register plugins and invoke handlers.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_app_version,
            commands::check_api_health,
            commands::check_for_updates,
            commands::get_api_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running HomeHub Control Center");
}
