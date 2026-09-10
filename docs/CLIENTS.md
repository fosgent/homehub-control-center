# Clients

**Status: Accepted security contract. The Windows client connection foundation is
implemented (Phase 1 / Tasks 2–4); authentication and the Web UI remain planned.**

## Shared contract
Web UI and Windows Desktop Client consume the same versioned Control API. Business rules and authorization belong in the Control Plane, not in clients.

## Web UI
**Not implemented.** Candidate stack: React, TypeScript, Vite, Tailwind, shadcn/ui. The UI may hide or disable controls based on server-provided permissions, but those controls are never a security boundary. Cookie credentials are HttpOnly and unavailable to page JavaScript. CSRF protection is mandatory for state-changing requests.

The UI must not store access/session credentials in localStorage, sessionStorage, URLs, logs, analytics payloads, or frontend bundles. CSP and anti-clickjacking headers are required at deployment.

## Windows Desktop Client

**Implemented today (Phase 1 / Tasks 2–4):** Tauri 2 + Rust with a React + TypeScript
frontend. The client lists/adds/edits/removes configured Control API servers, tests,
connects and disconnects, shows live status, and persists the server registry
(versioned JSON in the app config dir, atomic writes; see ADR-010). All network access
(health probes) happens in native Rust behind a typed command allowlist; the frontend
never fetches arbitrary endpoints. URL validation is authoritative in Rust: only
http/https, embedded credentials rejected, dangerous schemes rejected. There is no
Agent credential, Docker socket, root/admin credential, database key, or privileged
local fallback.

**Not implemented (planned contract):** the authentication contract below. Today's
health probes are unauthenticated by design; ADR-006/ADR-007 define the target model.

Authentication contract (fixed): short-lived access credential, rotating refresh
credential stored in Windows Credential Manager, explicit logout, and server-side
revocation. Refresh-token reuse is treated as credential compromise: the server revokes
the refresh family and requires full re-authentication.

## Failure handling
Clients treat API operation state as authoritative. Network timeout does not mean operation failure; tracked operations are queried by operation/deployment ID before retry. Clients never retry destructive operations with a new idempotency key automatically.
