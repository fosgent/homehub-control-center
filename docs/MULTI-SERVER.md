# Multi-Server

**Status: Accepted identity and isolation contract**

The Control Plane and Managed Servers are separate security domains.

```text
Control Plane
 ├── Server A -> Agent identity A
 ├── Server B -> Agent identity B
 └── Future VPS -> Agent identity C
```

## Server identity
Each server has a stable `server_id` plus a unique cryptographic Agent identity. Registration binds the identity to one authorized server record. Revocation invalidates the Agent identity and prevents new privileged operations.

Server capabilities (Docker, Nginx, filesystem resource classes, backups, health checks) constrain what the Agent can execute. A capability is not user authorization.

## Registration
Only an admin can register or revoke a server. Enrollment is an explicit authenticated process and records identity fingerprint, server metadata, capabilities, version, and audit event. A server cannot self-authorize by presenting a network address or Tailscale identity.

## Communication
Phase 1 is single-node and uses local UDS. Remote Agent transport is deferred; the accepted future direction is mTLS over private Tailscale connectivity with certificate identity bound to `server_id`.

## Isolation
Authorization is evaluated for the selected server and project. Credentials, project roots, operation state, and audit scope do not cross server boundaries implicitly. Compromise of Server A does not grant a principal or Agent permission on Server B.

## Offline/stale state
A disconnected server is unavailable, not implicitly authorized. Last-seen is distinct from current health. Privileged operations are not queued indefinitely. Any future queued operation must have an expiration, idempotency key, target server identity, and explicit revalidation before execution.

## Revocation and compromise
On suspected compromise, revoke the server identity, stop accepting privileged work, preserve audit/evidence, rotate affected credentials, and recover/reinstall the Agent from a trusted artifact. Re-enrollment creates a new cryptographic identity rather than silently restoring the revoked identity.
