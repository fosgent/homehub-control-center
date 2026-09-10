/// Server connection model and persistence.
///
/// A configured server is a first-class application concept, not a random UI
/// setting. The model is multi-server ready (a list of `ServerConfig` records
/// with a stable `server_id`) while this task only exposes single-active
/// connection behavior:
///
/// - local server identifier (`server_id`, stable, generated once);
/// - display name;
/// - Control API URL (validated to http/https only);
/// - enabled/disabled;
/// - runtime connection state (never persisted - it is derived, not configured).
///
/// Persistence: a small versioned JSON file (`servers.json`) in the Tauri app
/// config directory, written atomically (tmp file + rename). No database is
/// introduced into the Windows client, consistent with the backend-only scope
/// of ADR-009. No secrets are ever stored here; there is no credential in this
/// task.
use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config;
use crate::health;

/// Name given to the seeded default server.
pub const DEFAULT_SERVER_NAME: &str = "HomeHub Local";
/// Registry file schema version; bump only with a migration path.
const REGISTRY_VERSION: u32 = 1;
/// Registry filename inside the app config directory.
const REGISTRY_FILE: &str = "servers.json";

/// Persisted server configuration (no credentials).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub server_id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

/// Payload for creating or updating a server configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfigInput {
    pub server_id: Option<String>,
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

/// On-disk registry shape (versioned for future migration).
#[derive(Debug, Serialize, Deserialize)]
struct ServerRegistryFile {
    version: u32,
    servers: Vec<ServerConfig>,
}

/// Runtime (non-persisted) connection status.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
    Disabled,
}

/// Classification of a connection test / health probe.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestOutcome {
    Healthy,
    Degraded,
    Unreachable,
    InvalidEndpoint,
    HttpError,
    InvalidResponse,
}

/// Result of a health probe against a Control API endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTest {
    pub outcome: TestOutcome,
    pub connected: bool,
    pub degraded: bool,
    pub message: String,
    pub server_url: String,
    pub health_url: String,
    pub status: String,
    pub database: String,
    pub version: String,
}

impl ConnectionTest {
    /// A probe result that has not been classified yet (defaults to unknown).
    pub fn new(server_url: String, health_url: String) -> Self {
        Self {
            outcome: TestOutcome::Unreachable,
            connected: false,
            degraded: false,
            message: String::new(),
            server_url,
            health_url,
            status: "unknown".to_string(),
            database: "unknown".to_string(),
            version: "unknown".to_string(),
        }
    }

    /// A probe that failed URL validation (e.g. unsupported protocol).
    pub fn invalid_endpoint(server_url: String, message: String) -> Self {
        Self {
            outcome: TestOutcome::InvalidEndpoint,
            connected: false,
            degraded: false,
            message,
            server_url: server_url.trim_end_matches('/').to_string(),
            health_url: String::new(),
            status: "unknown".to_string(),
            database: "unknown".to_string(),
            version: "unknown".to_string(),
        }
    }
}

/// Runtime state attached to one configured server.
#[derive(Debug, Clone)]
struct ServerRuntime {
    state: ConnectionState,
    degraded: bool,
    message: Option<String>,
    last_check: Option<ConnectionTest>,
}

impl Default for ServerRuntime {
    fn default() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            degraded: false,
            message: None,
            last_check: None,
        }
    }
}

/// Stable view sent to the UI: configuration plus runtime state.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerView {
    pub server_id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub status: ConnectionState,
    pub degraded: bool,
    pub error: Option<String>,
    pub last_check: Option<ConnectionTest>,
}

/// Owns the configured servers and their runtime connection state.
pub struct ServerManager {
    config_dir: PathBuf,
    initialized: bool,
    servers: Vec<ServerConfig>,
    runtime: HashMap<String, ServerRuntime>,
    active_id: Option<String>,
}

impl ServerManager {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            initialized: false,
            servers: Vec::new(),
            runtime: HashMap::new(),
            active_id: None,
        }
    }

    fn registry_path(&self) -> PathBuf {
        self.config_dir.join(REGISTRY_FILE)
    }

    fn ensure_loaded(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }
        self.load()?;
        self.initialized = true;
        Ok(())
    }

    fn load(&mut self) -> Result<(), String> {
        let path = self.registry_path();
        if !path.exists() {
            // First launch: seed a sensible default endpoint, persisted so
            // restart restores it.
            self.servers = vec![ServerConfig {
                server_id: Uuid::new_v4().to_string(),
                name: DEFAULT_SERVER_NAME.to_string(),
                url: config::control_api_url(),
                enabled: true,
            }];
            return self.persist();
        }

        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read server configuration: {e}"))?;
        let file: ServerRegistryFile = serde_json::from_str(&text)
            .map_err(|e| format!("Server configuration is corrupted: {e}"))?;
        if file.version != REGISTRY_VERSION {
            return Err(format!(
                "Unsupported server configuration version {} (expected {REGISTRY_VERSION}).",
                file.version
            ));
        }
        self.servers = file.servers;
        Ok(())
    }

    fn persist(&self) -> Result<(), String> {
        let file = ServerRegistryFile {
            version: REGISTRY_VERSION,
            servers: self.servers.clone(),
        };
        let json = serde_json::to_string_pretty(&file)
            .map_err(|e| format!("Failed to serialize server configuration: {e}"))?;

        let tmp = self.config_dir.join(format!("{REGISTRY_FILE}.tmp"));
        std::fs::write(&tmp, json)
            .map_err(|e| format!("Failed to write server configuration: {e}"))?;
        std::fs::rename(&tmp, self.registry_path())
            .map_err(|e| format!("Failed to save server configuration: {e}"))
    }

    fn index_of(&self, id: &str) -> Option<usize> {
        self.servers.iter().position(|s| s.server_id == id)
    }

    /// Resolve a configured server (with its id) for probing/connection.
    pub fn config(&self, id: &str) -> Result<ServerConfig, String> {
        let index = self
            .index_of(id)
            .ok_or_else(|| "Server not found.".to_string())?;
        Ok(self.servers[index].clone())
    }

    fn view(&self, id: &str) -> Option<ServerView> {
        let server = self.servers.iter().find(|s| s.server_id == id)?;
        let runtime = self.runtime.get(id);
        let status = if !server.enabled {
            ConnectionState::Disabled
        } else {
            runtime
                .map(|r| r.state)
                .unwrap_or(ConnectionState::Disconnected)
        };
        Some(ServerView {
            server_id: server.server_id.clone(),
            name: server.name.clone(),
            url: server.url.clone(),
            enabled: server.enabled,
            status,
            degraded: runtime.map(|r| r.degraded).unwrap_or(false),
            error: runtime
                .and_then(|r| r.message.clone())
                .filter(|_| status == ConnectionState::Error),
            last_check: runtime.and_then(|r| r.last_check.clone()),
        })
    }

    fn all_views(&self) -> Vec<ServerView> {
        self.servers
            .iter()
            .map(|s| self.view(&s.server_id))
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default()
    }

    /// List configured servers with their runtime state.
    pub fn list(&mut self) -> Result<Vec<ServerView>, String> {
        self.ensure_loaded()?;
        Ok(self.all_views())
    }

    /// Create or update a server configuration. Validates the URL natively
    /// before persisting; never stores credentials.
    pub fn save(&mut self, input: ServerConfigInput) -> Result<ServerView, String> {
        self.ensure_loaded()?;

        let name = input.name.trim().to_string();
        if name.is_empty() {
            return Err("Server name is required.".to_string());
        }
        if name.chars().count() > 120 {
            return Err("Server name must be 120 characters or fewer.".to_string());
        }
        let url = health::validate_url(&input.url)?;

        match input.server_id.as_deref() {
            Some(id) => {
                let index = self
                    .index_of(id)
                    .ok_or_else(|| "Server not found.".to_string())?;
                self.servers[index].name = name;
                self.servers[index].url = url;
                self.servers[index].enabled = input.enabled;

                if !input.enabled && self.active_id.as_deref() == Some(id) {
                    self.active_id = None;
                    if let Some(rt) = self.runtime.get_mut(id) {
                        rt.state = ConnectionState::Disconnected;
                        rt.message = None;
                    }
                }
            }
            None => {
                let id = Uuid::new_v4().to_string();
                self.servers.push(ServerConfig {
                    server_id: id.clone(),
                    name,
                    url,
                    enabled: input.enabled,
                });
                self.runtime.insert(id.clone(), ServerRuntime::default());
            }
        }

        self.persist()?;
        let id = input.server_id.unwrap_or_else(|| {
            self.servers
                .last()
                .expect("just added a server")
                .server_id
                .clone()
        });
        Ok(self.view(&id).expect("just saved"))
    }

    /// Remove a configured server.
    pub fn remove(&mut self, id: &str) -> Result<Vec<ServerView>, String> {
        self.ensure_loaded()?;
        let index = self
            .index_of(id)
            .ok_or_else(|| "Server not found.".to_string())?;

        if self.active_id.as_deref() == Some(id) {
            self.active_id = None;
        }
        self.servers.remove(index);
        self.runtime.remove(id);
        self.persist()?;
        Ok(self.all_views())
    }

    /// Record a connection test result.
    ///
    /// A test is a one-shot health probe and never *claims* a connection by
    /// itself: it leaves a Disconnected server Disconnected. The one exception
    /// is an already-Connected server: a fresh test that fails must not leave
    /// a stale "Connected" displayed, so it drops to Error. Likewise, a
    /// successful test on an Error'd server restores it to Disconnected.
    pub fn note_test(&mut self, id: &str, result: ConnectionTest) -> Result<ServerView, String> {
        self.ensure_loaded()?;
        self.config(id)?;
        let failed = !result.connected;
        let runtime = self.runtime.entry(id.to_string()).or_default();

        match (runtime.state, failed) {
            (ConnectionState::Connected, true) => {
                if self.active_id.as_deref() == Some(id) {
                    self.active_id = None;
                }
                runtime.state = ConnectionState::Error;
                runtime.degraded = false;
            }
            (ConnectionState::Connected, false) => {
                runtime.degraded = result.degraded;
            }
            (ConnectionState::Error, true) => {}
            (ConnectionState::Error, false) => {
                runtime.state = ConnectionState::Disconnected;
                runtime.degraded = false;
            }
            _ => {}
        }
        runtime.last_check = Some(result.clone());
        runtime.message = Some(result.message);
        Ok(self.view(id).expect("server still present"))
    }

    /// Mark a server as connecting (transient, set before the probe runs).
    pub fn note_connecting(&mut self, id: &str) -> Result<ServerView, String> {
        self.ensure_loaded()?;
        self.config(id)?;
        let runtime = self.runtime.entry(id.to_string()).or_default();
        runtime.state = ConnectionState::Connecting;
        runtime.message = None;
        Ok(self.view(id).expect("server still present"))
    }

    /// Apply the outcome of a connect attempt.
    ///
    /// "Connected" is only claimed when the health probe actually succeeded
    /// (outcome Healthy or Degraded) and the Control API responded according
    /// to its health contract. Anything else yields an Error state.
    pub fn note_connect(&mut self, id: &str, result: ConnectionTest) -> Result<ServerView, String> {
        self.ensure_loaded()?;
        self.config(id)?;

        match result.outcome {
            TestOutcome::Healthy | TestOutcome::Degraded => {
                if let Some(prev) = self.active_id.take() {
                    if prev != id {
                        if let Some(rt) = self.runtime.get_mut(&prev) {
                            rt.state = ConnectionState::Disconnected;
                            rt.degraded = false;
                            rt.message = None;
                        }
                    }
                }
                self.active_id = Some(id.to_string());
                let runtime = self.runtime.entry(id.to_string()).or_default();
                runtime.state = ConnectionState::Connected;
                runtime.degraded = result.outcome == TestOutcome::Degraded;
                runtime.message = Some(result.message.clone());
                runtime.last_check = Some(result);
            }
            _ => {
                if self.active_id.as_deref() == Some(id) {
                    self.active_id = None;
                }
                let runtime = self.runtime.entry(id.to_string()).or_default();
                runtime.state = ConnectionState::Error;
                runtime.degraded = false;
                runtime.message = Some(result.message.clone());
                runtime.last_check = Some(result);
            }
        }

        Ok(self.view(id).expect("server still present"))
    }

    /// Explicitly disconnect a server.
    pub fn disconnect(&mut self, id: &str) -> Result<ServerView, String> {
        self.ensure_loaded()?;
        self.config(id)?;
        if self.active_id.as_deref() == Some(id) {
            self.active_id = None;
        }
        let runtime = self.runtime.entry(id.to_string()).or_default();
        runtime.state = ConnectionState::Disconnected;
        runtime.degraded = false;
        runtime.message = None;
        Ok(self.view(id).expect("server still present"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn test_manager(dir: &Path) -> ServerManager {
        ServerManager::new(dir.to_path_buf())
    }

    fn tmp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("homehub-servers-test-{name}-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn input(server_id: Option<String>, name: &str, url: &str, enabled: bool) -> ServerConfigInput {
        ServerConfigInput {
            server_id,
            name: name.to_string(),
            url: url.to_string(),
            enabled,
        }
    }

    fn healthy() -> ConnectionTest {
        let mut t = ConnectionTest::new(
            "http://127.0.0.1:8000".to_string(),
            "http://127.0.0.1:8000/api/v1/health".to_string(),
        );
        t.outcome = TestOutcome::Healthy;
        t.connected = true;
        t.message = "Control API is healthy".to_string();
        t.version = "0.1.0".to_string();
        t
    }

    fn unreachable() -> ConnectionTest {
        let mut t = ConnectionTest::new(
            "http://127.0.0.1:9999".to_string(),
            "http://127.0.0.1:9999/api/v1/health".to_string(),
        );
        t.outcome = TestOutcome::Unreachable;
        t.message = "Control API is unreachable".to_string();
        t
    }

    #[test]
    fn first_launch_seeds_default_server() {
        let dir = tmp_dir("seed");
        let mut mgr = test_manager(&dir);
        let list = mgr.list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, DEFAULT_SERVER_NAME);
        assert_eq!(list[0].url, config::control_api_url());
        assert!(list[0].enabled);
        assert_eq!(list[0].status, ConnectionState::Disconnected);
        // Seeded default is persisted so restart restores it.
        assert!(mgr.registry_path().exists());
    }

    #[test]
    fn save_endpoint_persists_and_restores_after_restart() {
        let dir = tmp_dir("restart");
        let saved_name = "Home".to_string();
        let saved_url = "http://192.168.1.50:8000";
        {
            let mut mgr = test_manager(&dir);
            let saved = mgr.save(input(None, &saved_name, saved_url, true)).unwrap();
            assert_eq!(saved.name, saved_name);
            assert_eq!(saved.url, saved_url);
            assert!(!saved.server_id.is_empty());
            assert!(saved.enabled);
        }
        // Simulate a restart: a brand new manager over the same directory.
        let mut mgr = test_manager(&dir);
        let list = mgr.list().unwrap();
        let restored = list
            .iter()
            .find(|v| v.name == saved_name)
            .expect("saved endpoint restored after restart");
        assert_eq!(restored.url, saved_url);
        assert!(restored.enabled);
    }

    #[test]
    fn invalid_protocol_is_rejected_on_save() {
        let dir = tmp_dir("invalid");
        let mut mgr = test_manager(&dir);
        let err = mgr
            .save(input(None, "Bad", "javascript:alert(1)", true))
            .unwrap_err();
        assert!(err.contains("unsupported protocol"), "{err}");
        let err = mgr
            .save(input(None, "Bad", "file:///etc/passwd", true))
            .unwrap_err();
        assert!(err.contains("unsupported protocol"), "{err}");
        let err = mgr.save(input(None, "Bad", "not a url", true)).unwrap_err();
        assert_eq!(err, "Endpoint URL is invalid.");
        // Nothing was persisted for the rejected endpoints (only the seed).
        assert_eq!(mgr.list().unwrap().len(), 1);
    }

    #[test]
    fn update_keeps_server_id_and_changes_url() {
        let dir = tmp_dir("update");
        let mut mgr = test_manager(&dir);
        let first = mgr
            .save(input(None, "Old", "http://127.0.0.1:8000", true))
            .unwrap();
        let updated = mgr
            .save(input(
                Some(first.server_id.clone()),
                "New",
                "http://10.0.0.7:8000",
                false,
            ))
            .unwrap();
        assert_eq!(updated.server_id, first.server_id);
        assert_eq!(updated.name, "New");
        assert_eq!(updated.url, "http://10.0.0.7:8000");
        assert!(!updated.enabled);
        assert_eq!(updated.status, ConnectionState::Disabled);
        // The saved record is replaced, not duplicated; the seed remains.
        let list = mgr.list().unwrap();
        assert_eq!(list.len(), 2);
        let matches = list
            .iter()
            .filter(|v| v.server_id == first.server_id)
            .count();
        assert_eq!(matches, 1);
    }

    #[test]
    fn update_unknown_id_is_rejected() {
        let dir = tmp_dir("unknown");
        let mut mgr = test_manager(&dir);
        let err = mgr
            .save(input(
                Some("missing".to_string()),
                "X",
                "http://127.0.0.1:8000",
                true,
            ))
            .unwrap_err();
        assert_eq!(err, "Server not found.");
    }

    #[test]
    fn remove_deletes_the_server() {
        let dir = tmp_dir("remove");
        let mut mgr = test_manager(&dir);
        let first = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let second = mgr
            .save(input(None, "B", "http://127.0.0.1:8001", true))
            .unwrap();
        let list = mgr.remove(&first.server_id).unwrap();
        assert_eq!(list.len(), 2); // seed + second
        assert!(list.iter().any(|v| v.server_id == second.server_id));
        assert!(!list.iter().any(|v| v.server_id == first.server_id));
    }

    #[test]
    fn connect_success_transitions_to_connected() {
        let dir = tmp_dir("connect-ok");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let view = mgr.note_connect(&server.server_id, healthy()).unwrap();
        assert_eq!(view.status, ConnectionState::Connected);
        assert!(!view.degraded);
        assert_eq!(mgr.active_id.as_deref(), Some(server.server_id.as_str()));
    }

    #[test]
    fn connect_failure_never_claims_connected() {
        let dir = tmp_dir("connect-fail");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:9999", true))
            .unwrap();
        let view = mgr.note_connect(&server.server_id, unreachable()).unwrap();
        assert_eq!(view.status, ConnectionState::Error);
        assert!(view.error.is_some());
        assert_eq!(mgr.active_id, None);
    }

    #[test]
    fn connect_switches_active_server() {
        let dir = tmp_dir("connect-switch");
        let mut mgr = test_manager(&dir);
        let a = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let b = mgr
            .save(input(None, "B", "http://127.0.0.1:8001", true))
            .unwrap();
        mgr.note_connect(&a.server_id, healthy()).unwrap();
        let b_view = mgr.note_connect(&b.server_id, healthy()).unwrap();
        assert_eq!(b_view.status, ConnectionState::Connected);
        assert_eq!(mgr.active_id.as_deref(), Some(b.server_id.as_str()));
        let a_view = mgr.view(&a.server_id).unwrap();
        assert_eq!(a_view.status, ConnectionState::Disconnected);
    }

    #[test]
    fn degraded_connect_is_connected_but_degraded() {
        let dir = tmp_dir("connect-degraded");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let mut t = healthy();
        t.outcome = TestOutcome::Degraded;
        t.degraded = true;
        t.status = "degraded".to_string();
        let view = mgr.note_connect(&server.server_id, t).unwrap();
        assert_eq!(view.status, ConnectionState::Connected);
        assert!(view.degraded);
    }

    #[test]
    fn disconnect_clears_connected_state() {
        let dir = tmp_dir("disconnect");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        mgr.note_connect(&server.server_id, healthy()).unwrap();
        let view = mgr.disconnect(&server.server_id).unwrap();
        assert_eq!(view.status, ConnectionState::Disconnected);
        assert_eq!(mgr.active_id, None);
    }

    #[test]
    fn disabled_server_is_blocked_from_connection() {
        let dir = tmp_dir("disabled");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", false))
            .unwrap();
        assert_eq!(server.status, ConnectionState::Disabled);
    }

    #[test]
    fn note_test_does_not_change_connection_state() {
        let dir = tmp_dir("note-test");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let view = mgr.note_test(&server.server_id, healthy()).unwrap();
        assert_eq!(view.status, ConnectionState::Disconnected);
        assert!(view.last_check.is_some());
    }

    #[test]
    fn failed_test_on_connected_server_drops_to_error() {
        let dir = tmp_dir("note-test-fail-connected");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        mgr.note_connect(&server.server_id, healthy()).unwrap();
        let view = mgr
            .note_test(&server.server_id, unreachable())
            .unwrap();
        assert_eq!(view.status, ConnectionState::Error);
        assert!(!view.degraded);
        assert!(view.error.is_some());
        assert_eq!(mgr.active_id, None);
    }

    #[test]
    fn healthy_test_on_connected_server_refreshes_and_stays_connected() {
        let dir = tmp_dir("note-test-healthy-connected");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let mut degraded = healthy();
        degraded.outcome = TestOutcome::Degraded;
        degraded.degraded = true;
        mgr.note_connect(&server.server_id, degraded).unwrap();
        let view = mgr.note_test(&server.server_id, healthy()).unwrap();
        assert_eq!(view.status, ConnectionState::Connected);
        assert!(!view.degraded);
    }

    #[test]
    fn healthy_test_recovers_errored_server_to_disconnected() {
        let dir = tmp_dir("note-test-recover");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        mgr.note_connect(&server.server_id, unreachable()).unwrap();
        let view = mgr.note_test(&server.server_id, healthy()).unwrap();
        assert_eq!(view.status, ConnectionState::Disconnected);
        assert!(view.error.is_none());
    }

    #[test]
    fn failed_test_on_disconnected_server_stays_disconnected() {
        let dir = tmp_dir("note-test-fail-disconnected");
        let mut mgr = test_manager(&dir);
        let server = mgr
            .save(input(None, "A", "http://127.0.0.1:8000", true))
            .unwrap();
        let view = mgr.note_test(&server.server_id, unreachable()).unwrap();
        assert_eq!(view.status, ConnectionState::Disconnected);
        assert!(view.error.is_none());
        assert!(view.last_check.is_some());
    }
}
