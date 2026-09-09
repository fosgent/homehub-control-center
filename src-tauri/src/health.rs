/// Native Control API health probe.
///
/// Performs a real HTTP `GET /api/v1/health` from the native layer. Doing this
/// in Rust (rather than from the WebView) avoids browser CORS constraints and
/// keeps the privileged/unprivileged boundary clear: the UI is a pure view, and
/// native I/O performs the network call. Any failure is represented as an
/// unreachable/error state rather than a panic.

use std::time::Duration;

use serde::Deserialize;

use crate::config;
use crate::result::{ApiHealth, ApiHealthState};

const HEALTH_PATH: &str = "/api/v1/health";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Deserialize)]
struct HealthBody {
    status: String,
    database: String,
    version: String,
}

/// Return the state of the Control API health endpoint without panicking.
pub fn check_health() -> ApiHealth {
    let api_url = config::control_api_url();
    let url = format!("{api_url}{HEALTH_PATH}");

    let response = ureq::get(&url).timeout(REQUEST_TIMEOUT).call();

    match response {
        Ok(resp) => match resp.into_string() {
            Ok(text) => match serde_json::from_str::<HealthBody>(&text) {
                Ok(body) => {
                    let connected = body.status == "ok";
                    ApiHealth {
                        state: if connected {
                            ApiHealthState::Ok
                        } else {
                            ApiHealthState::Degraded
                        },
                        connected,
                        status: body.status,
                        database: body.database,
                        version: body.version,
                        api_url,
                        detail: "health check succeeded".to_string(),
                    }
                }
                Err(_) => ApiHealth {
                    state: ApiHealthState::Degraded,
                    connected: false,
                    status: "unknown".to_string(),
                    database: "unknown".to_string(),
                    version: "unknown".to_string(),
                    api_url,
                    detail: "unexpected health response".to_string(),
                },
            },
            Err(_) => ApiHealth {
                state: ApiHealthState::Error,
                connected: false,
                status: "unknown".to_string(),
                database: "unknown".to_string(),
                version: "unknown".to_string(),
                api_url,
                detail: "failed to read health response".to_string(),
            },
        },
        Err(_) => ApiHealth {
            state: ApiHealthState::Unreachable,
            connected: false,
            status: "unknown".to_string(),
            database: "unknown".to_string(),
            version: "unknown".to_string(),
            api_url,
            detail: "control api unreachable".to_string(),
        },
    }
}
