# ADR-006: Authentication and authorization

**Status:** Proposed

## Context
The Control Center exposes operational data and privileged actions. Authentication alone is insufficient; access must be scoped to roles, servers, and projects.

## Decision
Require application authentication for protected resources and perform server-side authorization for every operation. Introduce RBAC and explicit resource scope. Use secure, revocable sessions/tokens with short-lived credentials where appropriate; exact mechanism requires implementation review.

## Alternatives
Network location as sole authentication; long-lived bearer tokens without revocation; client-side authorization.

## Consequences
Identity/session persistence, revocation, CSRF/XSS protections where applicable, rate limiting, and recovery flows become required components.

## Security implications
Unauthenticated users must not receive project inventory, sensitive server information, or secrets. Failed logins and privileged authorization decisions are auditable without exposing credentials.
