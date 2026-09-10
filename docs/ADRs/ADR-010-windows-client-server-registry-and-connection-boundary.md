# ADR-010: Windows Client Server Registry and Connection Boundary

**Status: Accepted**

## Context

Phase 1 Tasks 2–4 built the Windows client. It needed to configure Control API servers,
persist that configuration, probe server health, and present connection state. Two prior
decisions constrained the solution: ADR-009 keeps persistence backend-only (no database
introduced in the Windows client), and ADR-006/007 reserve credentials for a dedicated
auth design that does not exist yet. The client therefore needs a persistence mechanism
and a strict I/O boundary for the connection features that exist today.

## Decision

The Windows client:

- Persists a **versioned JSON server registry** (`servers.json`, schema version 1) in the
  Tauri application configuration directory (`%APPDATA%\com.homehub.controlcenter` /
  `$XDG_CONFIG_HOME`), written **atomically** (temp file + rename). No database, SQL or
  otherwise, is introduced in the client.
- Seeds a default server on first launch from `HOMEHUB_CONTROL_API_URL`, defaulting to
  `http://127.0.0.1:8000`. The client is not limited to that endpoint; any http/https
  Control API URL can be configured.
- Persists **configuration only**: `server_id`, `name`, `url`, `enabled`. Runtime
  connection state (`disconnected`, `connecting`, `connected`, `error`, `disabled`) is
  never persisted — it is derived, not configured.
- Performs **all network access in native Rust** using `ureq` health probes. The
  frontend never fetches arbitrary server endpoints.
- Exposes the UI through a **typed allowlist of Tauri commands** only
  (`get_app_version`, `list_servers`, `save_server`, `remove_server`, `test_connection`,
  `connect_server`, `disconnect_server`, `check_for_updates`). There is no arbitrary
  command, shell, or subprocess capability.
- Validates URLs **authoritatively in Rust**: only http/https, trailing slash
  normalization, embedded credentials rejected, unsupported schemes (`file:`,
  `javascript:`, `data:`, …) rejected. Frontend validation is UX only and is never a
  security boundary.
- Implements **connection semantics** as a product contract:
  - **Test** performs a one-shot health probe and never claims Connected; a successful
    test on a disconnected or error server restores/stays Disconnected.
  - **Connect** performs a probe and establishes Connected only when the outcome is
    `healthy` or `degraded`; anything else yields `error`.
  - A degraded API displays **Connected (degraded)**.
  - A fresh failed test on a previously connected server drops the state to **Error**
    (no stale "Connected" is shown).
  - Disconnect explicitly clears the active connection state.
  - The client tracks a single active connection.

## Alternatives considered

- **SQLite in the client.** Rejected: ADR-009 scopes database persistence to the backend;
  a JSON registry is sufficient, versionable, auditable, and supports schema migration on
  bump.
- **Frontend-driven HTTP.** Rejected: WebView CSP is `connect-src 'self' ipc:
  http://ipc.localhost`; keeping network in native Rust confines the attack surface and
  keeps validation in one authoritative place.
- **Generic command/dispatch.** Rejected: an arbitrary execution surface violates the
  security model (docs/SECURITY.md, no `POST /execute` equivalent).
- **Persisting connected state.** Rejected: runtime state must not survive a restart and
  must never create the false impression of an established session.

## Consequences

- Future client work (richer server metadata, capability discovery, authentication)
  must extend the versioned registry with a migration bump (`REGISTRY_VERSION` in
  `server.rs`) and must not regress the connection semantics above.
- Introducing real authentication later (ADR-006/007) requires explicit redesign of the
  probe/connect flow; today's probes are unauthenticated by design.

## Security implications

- No secrets, tokens, or credentials are stored in the client or the registry file.
- The WebView has no network path to configured servers; the native layer mediates all
  requests and validates schemes/credentials before any I/O.
- No shell, PowerShell, or arbitrary subprocess exists in the client.
- The command allowlist and the CSP are architectural boundaries; keep them tight when
  extending the client.