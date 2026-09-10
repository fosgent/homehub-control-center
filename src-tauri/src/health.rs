/// Native Control API health probe and endpoint validation.
///
/// The network call runs in the native (Rust) layer and never in the WebView,
/// preserving the privileged/unprivileged boundary described in `ARCHITECTURE.md`.
/// The UI receives an already-classified `ConnectionTest`; it has no raw network
/// capability and no shell/subprocess surface.
///
/// Health interpretation: the Control API health contract (`GET /api/v1/health`)
/// returns `status`, `database`, `version` plus optional `timestamp`/`checks`.
/// A successful HTTP 2xx is NOT assumed to mean the API is healthy: the body must
/// parse into that shape and report `status == "ok"` before the client claims
/// "Connected". Any other well-formed response is treated as degraded.
///
/// Documented limitation: `http_error` vs `unreachable` are distinguished at the
/// transport level (an HTTP error status counts as a reachable-but-failing API).
/// The client relies on the existing health contract and does not define a new
/// contract.
use std::time::Duration;

use serde::Deserialize;
use url::Url;

use crate::server::{ConnectionTest, TestOutcome};

/// Control API health path. Kept identical to the backend contract.
pub const HEALTH_PATH: &str = "/api/v1/health";
/// Timeout applied to every health probe.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Shape required from the Control API health endpoint.
#[derive(Debug, Deserialize)]
struct HealthBody {
    status: String,
    database: String,
    version: String,
}

/// Validate a Control API endpoint URL.
///
/// Accepts only `http`/`https` URLs with a host. Rejects arbitrary protocols
/// (`file:`, `javascript:`, `data:`, `shell:`, …) and embedded credentials
/// (userinfo), because credentials must never live inside URLs.
///
/// Returns the normalized URL (leading/trailing whitespace removed, trailing
/// slashes trimmed) or a clear error message.
pub fn validate_url(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Endpoint URL is required.".to_string());
    }

    let parsed = Url::parse(trimmed).map_err(|_| "Endpoint URL is invalid.".to_string())?;

    match parsed.scheme() {
        "http" | "https" => {}
        other => {
            return Err(format!(
                "Endpoint URL uses unsupported protocol `{other}`; only http:// and https:// endpoints are allowed."
            ));
        }
    }

    if parsed.host_str().is_none() {
        return Err("Endpoint URL must include a host.".to_string());
    }

    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err("Endpoint URL must not embed credentials.".to_string());
    }

    Ok(trimmed.trim_end_matches('/').to_string())
}

fn health_url(base: &str) -> String {
    format!("{base}{HEALTH_PATH}")
}

/// Probe the Control API and classify the result into a user-facing outcome.
///
/// Never panics and never raises: transport errors, HTTP error statuses,
/// invalid URLs, and unparseable bodies are all classified into a
/// `ConnectionTest`. An invalid URL yields `TestOutcome::InvalidEndpoint`.
pub fn probe_url(server_url: &str, timeout: Duration) -> ConnectionTest {
    let base = match validate_url(server_url) {
        Ok(base) => base,
        Err(message) => {
            return ConnectionTest::invalid_endpoint(server_url.to_string(), message);
        }
    };
    let url = health_url(&base);
    let mut result = ConnectionTest::new(base, url);

    match ureq::get(&result.health_url).timeout(timeout).call() {
        Ok(response) => {
            let status = response.status();
            let body = response.into_string().unwrap_or_default();
            classify_response(status, &body, result)
        }
        Err(ureq::Error::Status(code, _)) => {
            result.outcome = TestOutcome::HttpError;
            result.message = format!("Control API returned HTTP {code}.");
            result
        }
        Err(ureq::Error::Transport(err)) => {
            result.outcome = TestOutcome::Unreachable;
            result.message = format!(
                "Control API is unreachable at {}. ({err})",
                result.server_url
            );
            result
        }
    }
}

/// Classify a raw health response (status code + body) into a `ConnectionTest`.
/// Pure function so every branch is unit-testable without a network.
fn classify_response(status: u16, body: &str, mut result: ConnectionTest) -> ConnectionTest {
    if !(200..300).contains(&status) {
        result.outcome = TestOutcome::HttpError;
        result.message = format!("Control API returned HTTP {status}.");
        return result;
    }

    match serde_json::from_str::<HealthBody>(body) {
        Ok(health) => {
            result.status = health.status.clone();
            result.database = health.database.clone();
            result.version = health.version.clone();
            match health.status.as_str() {
                "ok" => {
                    result.outcome = TestOutcome::Healthy;
                    result.connected = true;
                    result.degraded = false;
                    result.message = format!(
                        "Control API is healthy at {}. API v{}. Database {}.",
                        result.server_url, health.version, health.database
                    );
                }
                "degraded" | "unavailable" => {
                    result.outcome = TestOutcome::Degraded;
                    result.connected = true;
                    result.degraded = true;
                    result.message = format!(
                        "Control API responded at {}, but reports status `{}` (database {}).",
                        result.server_url, health.status, health.database
                    );
                }
                other => {
                    result.outcome = TestOutcome::Degraded;
                    result.connected = true;
                    result.degraded = true;
                    result.message = format!(
                        "Control API responded at {}, but reported unexpected status `{other}`.",
                        result.server_url
                    );
                }
            }
        }
        Err(_) => {
            result.outcome = TestOutcome::InvalidResponse;
            result.connected = false;
            result.degraded = false;
            result.message = format!(
                "Unexpected response from {}. Expected a HomeHub health response.",
                result.health_url
            );
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_http_url_is_accepted() {
        assert_eq!(
            validate_url("http://127.0.0.1:8000").unwrap(),
            "http://127.0.0.1:8000"
        );
    }

    #[test]
    fn valid_https_url_with_path_is_accepted() {
        assert_eq!(
            validate_url(" https://homehub.example.com/api ").unwrap(),
            "https://homehub.example.com/api"
        );
    }

    #[test]
    fn trailing_slashes_are_normalized() {
        assert_eq!(
            validate_url("http://localhost:8000///").unwrap(),
            "http://localhost:8000"
        );
    }

    #[test]
    fn empty_url_is_rejected() {
        let err = validate_url("").unwrap_err();
        assert_eq!(err, "Endpoint URL is required.");
        assert!(validate_url("   ").is_err());
    }

    #[test]
    fn arbitrary_protocols_are_rejected() {
        for bad in [
            "file:///etc/passwd",
            "javascript:alert(1)",
            "javascript://alert(1)",
            "data:text/html,<script>alert(1)</script>",
            "shell://host/run",
            "ftp://example.com/file",
            "ws://example.com/sock",
            "gopher://example.com",
        ] {
            let err = validate_url(bad).unwrap_err();
            assert!(err.contains("unsupported protocol"), "{bad} => {err}");
        }
    }

    #[test]
    fn credentials_in_url_are_rejected() {
        assert!(validate_url("http://user@example.com:8000").is_err());
        assert!(validate_url("http://user:pass@example.com:8000").is_err());
    }

    #[test]
    fn garbage_url_is_rejected() {
        assert!(validate_url("not a url").is_err());
        assert!(validate_url("http://").is_err());
    }

    fn classify(status: u16, body: &str) -> ConnectionTest {
        classify_response(
            status,
            body,
            ConnectionTest::new(
                "http://127.0.0.1:8000".to_string(),
                "http://127.0.0.1:8000/api/v1/health".to_string(),
            ),
        )
    }

    #[test]
    fn healthy_response_is_connected() {
        let r = classify(200, r#"{"status":"ok","database":"ok","version":"0.1.0"}"#);
        assert_eq!(r.outcome, TestOutcome::Healthy);
        assert!(r.connected);
        assert!(!r.degraded);
        assert_eq!(r.status, "ok");
        assert_eq!(r.version, "0.1.0");
    }

    #[test]
    fn degraded_response_is_connected_but_degraded() {
        let r = classify(
            200,
            r#"{"status":"degraded","database":"failed","version":"0.1.0"}"#,
        );
        assert_eq!(r.outcome, TestOutcome::Degraded);
        assert!(r.connected);
        assert!(r.degraded);
    }

    #[test]
    fn unavailable_status_is_treated_as_degraded_not_ok() {
        let r = classify(
            200,
            r#"{"status":"unavailable","database":"not_checked","version":"0.1.0"}"#,
        );
        assert_eq!(r.outcome, TestOutcome::Degraded);
        assert!(r.connected);
        assert!(r.degraded);
    }

    #[test]
    fn malformed_http_2xx_body_is_invalid_response() {
        let r = classify(200, "this is not json");
        assert_eq!(r.outcome, TestOutcome::InvalidResponse);
        assert!(!r.connected);
    }

    #[test]
    fn http_2xx_body_missing_fields_is_invalid_response() {
        let r = classify(200, r#"{"status":"ok"}"#);
        assert_eq!(r.outcome, TestOutcome::InvalidResponse);
        assert!(!r.connected);
    }

    #[test]
    fn http_error_status_is_classified() {
        let r = classify(500, "internal server error");
        assert_eq!(r.outcome, TestOutcome::HttpError);
        assert!(!r.connected);
        assert!(r.message.contains("HTTP 500"));
    }

    #[test]
    fn http_404_is_http_error_not_unreachable() {
        let r = classify(404, r#"{"detail":"not found"}"#);
        assert_eq!(r.outcome, TestOutcome::HttpError);
        assert!(r.message.contains("404"));
    }

    #[test]
    fn invalid_url_classifies_as_invalid_endpoint() {
        let r = probe_url("javascript:alert(1)", REQUEST_TIMEOUT);
        assert_eq!(r.outcome, TestOutcome::InvalidEndpoint);
        assert!(!r.connected);
        assert!(r.message.to_lowercase().contains("protocol"));
    }

    #[test]
    #[ignore = "requires a running Control API; run with: cargo test -- --ignored"]
    fn live_probe_against_running_control_api() {
        // Manual smoke test, excluded from the default suite. Start the Control
        // API (e.g. `uvicorn homehub.main:app --port 8000`) then run:
        //   cargo test -- --ignored
        let url = std::env::var("HOMEHUB_SMOKE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
        let result = probe_url(&url, REQUEST_TIMEOUT);
        assert_eq!(result.outcome, TestOutcome::Healthy, "{}", result.message);
    }
}
