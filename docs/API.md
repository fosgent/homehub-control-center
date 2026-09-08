# API Contract

**Status: Accepted architectural contract; endpoints are not implemented.**

Base path: `/api/v1`.

## Authentication and sessions
Web: server-side session with `Secure; HttpOnly; SameSite=Lax` cookie, 12-hour idle timeout, 7-day absolute timeout, CSRF token on state-changing requests, and explicit logout/revocation. Windows: 15-minute access credential plus rotating refresh credential stored in Windows Credential Manager. Refresh reuse revokes the refresh family. Password reset/admin recovery invalidates all active sessions and refresh families.

Authentication library direction: FastAPI Users 15.x for user lifecycle and password-reset flows, SQLAlchemy persistence, and `pwdlib` recommended Argon2 password hashing. Current FastAPI Users documentation documents cookie/bearer transports and database-backed invalidatable tokens.

## Authorization
Every protected endpoint performs server-side RBAC and resource-scope evaluation. Roles are `admin`, `operator`, `viewer`; operation rules are defined in `SECURITY.md`. Client-provided role/scope fields are never authoritative. The UI only hides or disables controls.

## Endpoints
```text
GET  /health
GET  /me
POST /auth/login
POST /auth/logout
POST /auth/refresh
POST /auth/forgot-password
POST /auth/reset-password

GET  /projects
GET  /projects/{id}
POST /projects/{id}/start
POST /projects/{id}/stop
POST /projects/{id}/restart
POST /projects/{id}/pause
POST /projects/{id}/resume
POST /projects/{id}/deploy
POST /projects/{id}/backup
POST /projects/{id}/restore
POST /projects/{id}/health-check
POST /projects/{id}/rollback

GET  /servers
GET  /servers/{id}
POST /servers/{id}/health-check
POST /servers/{id}/register
POST /servers/{id}/revoke

GET  /deployments
GET  /deployments/{id}
GET  /logs
GET  /scheduler/jobs
POST /scheduler/jobs/{id}/run
GET  /scheduler/jobs/{id}/history
GET  /audit

GET  /secrets
GET  /secrets/{id}/metadata
POST /secrets
PUT  /secrets/{id}
DELETE /secrets/{id}
```

Secret values are not returned by ordinary list/detail endpoints. A separately authorized secret-read operation is required and is admin-only by the Phase 1 matrix.

## Agent protocol
The API uses HTTP/JSON over the local Unix socket defined in `AGENT.md`. The protocol is versioned and includes request ID, idempotency key, deadline, operation, typed parameters, and structured response/error data. The API is the user authorization authority; the Agent independently enforces its operation/capability/resource boundary.

## Errors
API errors use a stable envelope containing `code`, safe `message`, `request_id`, optional `operation_state_id`, and `retryable`. Stack traces, credentials, internal filesystem paths not needed by the client, and secrets are never returned. HTTP status is transport metadata, not the operation's durable truth.

## Idempotency
Mutating endpoints require an idempotency key. The Control Plane persists operation state. Repeated requests with the same key and equivalent operation parameters return the existing operation state/result. Different parameters under the same key are rejected. After timeout, clients query operation state before retrying tracked operations.

## API security
No generic command endpoint exists. Operation payloads are typed Pydantic schemas with bounded fields. Credentials never appear in URLs. Collections are paginated. Timestamps are ISO-8601 UTC. Resource IDs are stable. Optimistic concurrency is required for registry/system configuration mutations. CORS is explicit allowlist-only when required.
