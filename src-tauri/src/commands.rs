/// Allowlisted Tauri commands exposed to the unprivileged UI.
///
/// There is deliberately **no** `execute(command)`, `run(command)`, shell, or
/// arbitrary-subprocess command. Each command performs exactly one concrete,
/// named operation:
///
/// - `get_app_version`     – report the canonical application version.
/// - `list_servers`        – configured Control API servers + runtime state.
/// - `save_server`         – create or update a server configuration.
/// - `remove_server`       – delete a server configuration.
/// - `test_connection`     – probe a server's health without connecting.
/// - `connect_server`      – health-check, then claim Connected on success.
/// - `disconnect_server`   – drop the active connection.
/// - `check_for_updates`   – ask the official updater for a newer version.
///
/// Network access stays in the native layer (`health::probe_url`). The UI never
/// performs HTTP directly and never receives a raw command string.
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::health;
use crate::server::{ServerConfigInput, ServerManager, ServerView};
use crate::updater;

/// Managed application state shared with the commands.
pub struct AppState {
    pub manager: Mutex<ServerManager>,
}

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

/// List the configured servers with their runtime connection state.
#[tauri::command]
pub fn list_servers(state: State<'_, AppState>) -> Result<Vec<ServerView>, String> {
    lock_manager(&state)?.list()
}

/// Create or update a server configuration (native URL validation).
#[tauri::command]
pub fn save_server(
    state: State<'_, AppState>,
    input: ServerConfigInput,
) -> Result<ServerView, String> {
    lock_manager(&state)?.save(input)
}

/// Remove a server configuration.
#[tauri::command]
pub fn remove_server(
    state: State<'_, AppState>,
    server_id: String,
) -> Result<Vec<ServerView>, String> {
    lock_manager(&state)?.remove(&server_id)
}

/// Probe a server's health endpoint without changing the connection state.
#[tauri::command]
pub async fn test_connection(
    _app: AppHandle,
    state: State<'_, AppState>,
    server_id: String,
) -> Result<ServerView, String> {
    let url = {
        let manager = lock_manager(&state)?;
        let server = manager.config(&server_id)?;
        if !server.enabled {
            return Err(format!(
                "Cannot test connection: `{}` is disabled.",
                server.name
            ));
        }
        server.url
    };

    let result = run_probe(url).await?;
    lock_manager(&state)?.note_test(&server_id, result)
}

/// Health-check a server, then move it to Connected on success.
#[tauri::command]
pub async fn connect_server(
    _app: AppHandle,
    state: State<'_, AppState>,
    server_id: String,
) -> Result<ServerView, String> {
    let url = {
        let server = lock_manager(&state)?.config(&server_id)?;
        if !server.enabled {
            return Err(format!("Cannot connect: `{}` is disabled.", server.name));
        }
        server.url
    };

    lock_manager(&state)?.note_connecting(&server_id)?;
    let result = run_probe(url).await?;
    lock_manager(&state)?.note_connect(&server_id, result)
}

/// Explicitly disconnect a server.
#[tauri::command]
pub fn disconnect_server(
    state: State<'_, AppState>,
    server_id: String,
) -> Result<ServerView, String> {
    lock_manager(&state)?.disconnect(&server_id)
}

/// Ask the official Tauri updater for a newer version.
#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> crate::result::UpdateCheckResult {
    updater::check_for_updates(app).await
}

fn lock_manager<'a>(
    state: &'a State<'_, AppState>,
) -> Result<std::sync::MutexGuard<'a, ServerManager>, String> {
    state
        .manager
        .lock()
        .map_err(|_| "Internal state lock is poisoned.".to_string())
}

/// Run the native health probe off the async executor and map join/report
/// failures into user-friendly errors.
async fn run_probe(url: String) -> Result<crate::server::ConnectionTest, String> {
    tauri::async_runtime::spawn_blocking(move || health::probe_url(&url, health::REQUEST_TIMEOUT))
        .await
        .map_err(|_| "Health probe task failed unexpectedly.".to_string())
}
