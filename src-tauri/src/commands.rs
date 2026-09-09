/// Allowlisted Tauri commands exposed to the unprivileged UI.
///
/// There is deliberately **no** `execute(command)`, `run(command)`, shell, or
/// arbitrary-subprocess command. Each command performs exactly one concrete,
/// named operation:
///
/// - `get_app_version`     – report the canonical application version.
/// - `check_api_health`    – probe the Control API health endpoint.
/// - `check_for_updates`   – ask the official updater for a newer version.
///
/// The Tauri layer never receives an arbitrary command string from the UI.

use serde::Serialize;
use tauri::AppHandle;

use crate::config;
use crate::health;
use crate::updater;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVersion {
    pub version: String,
    pub identifier: String,
    pub name: String,
}

/// Return the canonical application version from the native package metadata.
#[tauri::command]
pub fn get_app_version(app: AppHandle) -> AppVersion {
    let package = app.package_info();
    AppVersion {
        version: package.version.to_string(),
        identifier: app.config().identifier.clone(),
        name: package.name.clone(),
    }
}

/// Probe the Control API health endpoint from the native layer.
#[tauri::command]
pub async fn check_api_health(_app: AppHandle) -> crate::result::ApiHealth {
    // ureq is blocking; run it off the async executor.
    tauri::async_runtime::spawn_blocking(health::check_health)
        .await
        .unwrap_or_else(|_| health::check_health())
}

/// Ask the official Tauri updater for a newer version.
#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> crate::result::UpdateCheckResult {
    updater::check_for_updates(app).await
}

/// Expose connectivity configuration for display (non-secret).
#[tauri::command]
pub fn get_api_url() -> String {
    config::describe()
}
