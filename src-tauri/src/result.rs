/// Error types for the HomeHub Control Center native layer.
///
/// Tauri commands return typed, serializable results. Internal errors are
/// mapped to safe, user-facing messages without leaking internals.
use serde::Serialize;

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
    fn update_check_state_not_configured_serializes() {
        assert_eq!(
            serde_json::to_string(&UpdateCheckState::NotConfigured).unwrap(),
            "\"not_configured\""
        );
    }
}
