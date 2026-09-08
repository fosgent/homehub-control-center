# ADR-006: Authentication and authorization

**Status:** Accepted

## Context
The Control Center exposes operational data and privileged actions. Authentication alone is insufficient; access must be scoped to roles, servers, and projects.

## Decision
Use application authentication with FastAPI Users 15.x for user lifecycle and password-reset integration, SQLAlchemy persistence, and `pwdlib` recommended Argon2 password hashing. Web uses revocable server-side sessions carried in Secure/HttpOnly/SameSite=Lax cookies with CSRF protection. Windows uses 15-minute access credentials plus rotating refresh credentials stored in Windows Credential Manager. Refresh reuse revokes the refresh family. All recovery/reset actions invalidate active sessions and refresh families.

RBAC roles are `admin`, `operator`, and `viewer`, with explicit global/server/project scope. The complete operation matrix is part of `SECURITY.md`. Tailscale identity is defense-in-depth, never the application authorization system. All authorization is server-side.

FastAPI Users current documentation (v15.0.5) provides cookie/bearer transports, database-backed invalidatable authentication tokens, user management, and reset-password routes. Its database strategy does not itself define the required rotating desktop refresh-token family, so that rotation/reuse state remains an explicit HomeHub application service rather than being delegated blindly to the library.

## Alternatives
Network location as sole authentication; long-lived bearer tokens; JWT-only sessions without revocation; client-side authorization; Tailscale identity as the only application identity.

## Consequences
Session/refresh persistence, revocation, CSRF, rate limiting, recovery, audit, and step-up confirmation for sensitive actions are required components.

## Security implications
UI permissions are never trusted. Compromised client credentials are limited by server-side role/scope. Passwords are never stored plaintext. Recovery revokes existing credentials.
