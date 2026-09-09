/// Update-check integration boundary.
///
/// Delegates to the official Tauri updater plugin, which uses a signed manifest
/// with cryptographic signature verification — never a "download any exe and
/// run it" mechanism. This is the integration boundary only: release endpoints
/// and a signing public key are not configured in the MVP, so the check reports
/// ``not_configured`` rather than executing fake update logic.

use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

use crate::result::{UpdateCheckResult, UpdateCheckState};

/// Ask the Tauri updater whether a newer version is available.
///
/// Returns a typed result. With no endpoints/pubkey configured this reports
/// `not_configured`; it never downloads or executes anything on its own.
pub async fn check_for_updates(app: AppHandle) -> UpdateCheckResult {
    let updater = match app.updater() {
        Ok(u) => u,
        Err(_) => {
            return UpdateCheckResult {
                state: UpdateCheckState::NotConfigured,
                message: "Updater is not configured".to_string(),
            }
        }
    };

    match updater.check().await {
        Ok(Some(_update)) => UpdateCheckResult {
            state: UpdateCheckState::UpdateAvailable,
            message: "An update is available".to_string(),
        },
        Ok(None) => UpdateCheckResult {
            state: UpdateCheckState::UpToDate,
            message: "You are up to date".to_string(),
        },
        Err(_) => UpdateCheckResult {
            state: UpdateCheckState::NotConfigured,
            message: "Update check requires release endpoints and a signing key".to_string(),
        },
    }
}
