# Clients

**Status: Accepted security contract; UI stack remains implementation candidate**

## Shared contract
Web UI and Windows Desktop Client consume the same versioned Control API. Business rules and authorization belong in the Control Plane, not in clients.

## Web UI
Candidate stack: React, TypeScript, Vite, Tailwind, shadcn/ui. The UI may hide or disable controls based on server-provided permissions, but those controls are never a security boundary. Cookie credentials are HttpOnly and unavailable to page JavaScript. CSRF protection is mandatory for state-changing requests.

The UI must not store access/session credentials in localStorage, sessionStorage, URLs, logs, analytics payloads, or frontend bundles. CSP and anti-clickjacking headers are required at deployment.

## Windows Desktop Client
Candidate stack: Tauri 2 + React + TypeScript remains a proposal pending implementation validation. Authentication contract is fixed: short-lived access credential, rotating refresh credential stored in Windows Credential Manager, explicit logout, and server-side revocation. The client has no Agent credentials, Docker socket, root/admin credential, database key, or privileged local fallback.

Refresh-token reuse is treated as credential compromise: the server revokes the refresh family and requires full re-authentication.

## Failure handling
Clients treat API operation state as authoritative. Network timeout does not mean operation failure; tracked operations are queried by operation/deployment ID before retry. Clients never retry destructive operations with a new idempotency key automatically.
