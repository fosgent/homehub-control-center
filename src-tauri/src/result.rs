/// Error types for the HomeHub Control Center native layer.
///
/// Tauri commands return typed, serializable results. Internal errors are
/// mapped to safe, untrusted-user-facing messages without leaking internals.

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiHealthState {
    Ok,
    Degraded,
    Unreachable,
    Error,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiHealth {
    pub state: ApiHealthState,
    pub connected: bool,
    pub status: String,
    pub database: String,
    pub version: String,
    pub api_url: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum UpdateCheckState {
    NotConfigured,
    CheckFailed,
    UpToDate,
    UpdateAvailable,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckResult {
    pub state: UpdateCheckState,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_health_state_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&ApiHealthState::Ok).unwrap(),
            "\"ok\""
        );
        assert_eq!(
            serde_json::to_string(&ApiHealthState::Unreachable).unwrap(),
            "\"unreachable\""
        );
        assert_eq!(
            serde_json::to_string(&ApiHealthState::Degraded).unwrap(),
            "\"degraded\""
        );
        assert_eq!(
            serde_json::to_string(&ApiHealthState::Error).unwrap(),
            "\"error\""
        );
    }

    #[test]
    fn update_check_state_not_configured_serializes() {
        assert_eq!(
            serde_json::to_string(&UpdateCheckState::NotConfigured).unwrap(),
            "\"not_configured\""
        );
    }
}
