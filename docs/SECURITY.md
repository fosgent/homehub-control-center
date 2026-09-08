# Security

**Status: Accepted foundation; implementation-level security contract**

## Security objectives
Protect the Control Plane, managed servers, project data, credentials, and operational history while keeping privileged operations narrowly scoped, fail-safe, and auditable.

## Authorization matrix
Authorization is deny-by-default and evaluated server-side. Scope is enforced after identity and role evaluation; a UI permission never grants access by itself.

| Operation | admin | operator | viewer | Scope |
|---|---|---|---|---|
| View dashboard | yes | yes | yes | global, filtered to allowed resources |
| View projects | yes | yes | yes | project/server |
| View servers | yes | yes | yes | server |
| View logs | yes | yes | yes | server/project |
| View audit | yes | yes | no | global for admin; server/project for operator |
| View deployment history | yes | yes | yes | server/project |
| Start project | yes | yes | no | global for admin; project for operator |
| Stop project | yes | yes | no | global for admin; project for operator |
| Restart project | yes | yes | no | global for admin; project for operator |
| Pause project | yes | yes | no | global for admin; project for operator |
| Resume project | yes | yes | no | global for admin; project for operator |
| Deploy | yes | yes | no | global for admin; project for operator |
| Rollback | yes | yes | no | global for admin; project for operator |
| Backup | yes | yes | no | global for admin; project for operator |
| Restore | yes | no | no | global for admin |
| Health check | yes | yes | no | server/project |
| Modify project registry | yes | no | no | global |
| Modify scheduler | yes | no | no | global; server-scoped jobs remain server-scoped |
| Run scheduled job manually | yes | yes | no | global for admin; server/project for operator |
| View secret metadata | yes | yes | no | global for admin; server/project for operator |
| Read secret value | yes | no | no | global for admin; only explicitly scoped secret |
| Create secret | yes | no | no | global/project/server |
| Update secret | yes | no | no | global/project/server |
| Delete secret | yes | no | no | global/project/server |
| Manage users | yes | no | no | global |
| Manage roles | yes | no | no | global |
| Register server | yes | no | no | global |
| Revoke server | yes | no | no | global |
| Change system configuration | yes | no | no | global |

Operators may be granted narrower resource assignments than the role maximum. No role may escape the server/project assignments attached to the principal. Destructive operations require explicit confirmation and produce an audit event.

## Authentication
Implementation direction: FastAPI Users 15.x for user lifecycle/password-reset integration, with SQLAlchemy persistence. Its documented cookie transport supports Secure/HttpOnly/SameSite settings and its database strategy supports server-side token invalidation; current official documentation identifies v15.0.5 as the current release. Password hashing uses `pwdlib` with its recommended Argon2 configuration rather than Passlib.

Web authentication uses a server-side session identifier in a Secure, HttpOnly cookie named `hh_session`, SameSite=Lax. Sessions have a 12-hour idle timeout and 7-day absolute lifetime. Logout revokes the server-side session and clears the cookie. Password recovery and administrative recovery revoke all sessions and refresh-token families for the affected user.

Windows authentication uses a short-lived bearer access credential (15 minutes) and a rotating refresh credential. Refresh credentials are stored only in Windows Credential Manager. Refresh rotation is single-use: successful refresh invalidates the prior credential; reuse of an already-consumed credential revokes the entire refresh family and requires re-authentication. The access credential is never persisted to disk by the client.

Tailscale identity is network trust/defense-in-depth, not the application authorization system.

## Web security
Cookie-authenticated state-changing requests require a CSRF token and Origin/Referer validation. SameSite=Lax is defense-in-depth, not the sole CSRF control. HTTPS is mandatory in deployed environments; Secure cookies are mandatory. CORS is explicit allowlist-only when cross-origin requests are required; credentialed wildcard origins are forbidden. CSP, frame-ancestors, restrictive referrer policy, and anti-clickjacking headers are required. Tokens/secrets are never placed in localStorage, URLs, logs, or bundled frontend code.

## Recovery
Initial administrator bootstrap is a one-time, explicit installation action. There is no public self-registration of an administrator. Forgot-password uses a generic response to avoid account enumeration, short-lived single-use reset tokens, and post-reset invalidation of all active sessions/refresh families. Admin recovery requires a separate authenticated administrative procedure and creates an audit event. Passwords are never stored plaintext.

## Agent security
The Agent is local-only and exposes typed operations. Its socket is protected by Unix ownership/mode and Linux peer credentials. It validates protocol version, server identity, operation schema, resource references, capability, deadline, idempotency key, and registry-derived project identity. It never accepts arbitrary shell commands or executable paths.

## Filesystem security
The Agent receives resource references, not arbitrary absolute filesystem paths. Registry roots are the only authority for managed filesystem resources. Relative paths are decoded and validated once, absolute paths and NUL bytes are rejected, traversal components are rejected, cross-project roots are impossible, and symlink/magic-link traversal is denied for sensitive operations. Where the platform supports it, Linux `openat2()` with `RESOLVE_BENEATH`/`RESOLVE_NO_SYMLINKS` (and `RESOLVE_NO_MAGICLINKS`) is preferred to reduce TOCTOU exposure; file-descriptor-relative operations and atomic rename/temp-file patterns are used for writes.

## Secrets
Secrets use authenticated encryption and are stored as ciphertext plus non-secret metadata. Operational key material is provided through Ubuntu systemd service credentials, not the database or ordinary configuration. The database never contains the master key. See `SECRETS.md` and ADR-007.

## Update integrity
Production deployments use immutable, identity-pinned artifacts. Signed artifacts/provenance are preferred; an MVP may additionally require an exact SHA-256 digest. Any verification failure fails closed. No deployment activates an unverified or mutable branch artifact.

## Audit
Audit records contain timestamp, actor/principal, source client, server, project, operation, request/correlation ID, result, error code where applicable, and safe metadata. Secret values, credentials, stack traces, and raw sensitive command output are prohibited. Audit storage is append-only at the application contract level and tamper-evident/forwardable for higher-assurance deployments.

## Security invariants
1. No arbitrary command execution API.
2. UI is never an authorization boundary.
3. Authorization is enforced server-side.
4. Agent is not directly Internet/Tailnet exposed.
5. Agent executes only typed allowlisted capabilities.
6. Project filesystem access is registry-derived.
7. Cross-project filesystem access is forbidden.
8. Secrets never appear in logs.
9. Secrets never live in Git.
10. Artifact verification failure fails closed.
11. Deployment failure must produce a deterministic state.
12. Rollback must have a defined rollback point.
13. Server A compromise must not automatically authorize Server B.
14. Capability does not equal authorization.
15. Authentication does not equal authorization.
