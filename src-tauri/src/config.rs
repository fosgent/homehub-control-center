/// Native-layer configuration boundary.
///
/// The Control API base URL is centralized here (mirroring the frontend config)
/// so connectivity is not scattered across the codebase. No credentials live in
/// this module. The default points at the local development Control API and can
/// be overridden via the `HOMEHUB_CONTROL_API_URL` environment variable.

const DEFAULT_CONTROL_API_URL: &str = "http://127.0.0.1:8000";

/// Return the configured Control API base URL.
pub fn control_api_url() -> String {
    std::env::var("HOMEHUB_CONTROL_API_URL")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_CONTROL_API_URL.to_string())
}

/// Return the intact (non-secret) connectivity configuration for display.
pub fn describe() -> String {
    control_api_url()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_url_is_localhost_8000() {
        // When the env var is absent the default should be the local dev endpoint.
        // We only assert when it is actually unset to avoid clobbering an
        // environment that has already set it.
        if std::env::var("HOMEHUB_CONTROL_API_URL").is_err() {
            assert_eq!(control_api_url(), "http://127.0.0.1:8000");
        }
    }

    #[test]
    fn describe_matches_control_api_url() {
        assert_eq!(describe(), control_api_url());
    }
}
