# Architecture Review v3

**Repository:** `fosgent/homehub-control-center`
**Branch:** `main`
**Review mode:** documentation-only; no implementation started.

## 1. Permission matrix
**Status: PASS**

A concrete admin/operator/viewer matrix is accepted in `SECURITY.md`. Authorization is server-side, scoped by role plus server/project assignment, and UI permissions are explicitly non-authoritative. Restore, identity administration, role management, registry mutation, secret value reads, and system configuration remain admin-only.

## 2. Authentication
**Status: PASS**

Web uses revocable server-side sessions in Secure/HttpOnly/SameSite=Lax cookies, 12-hour idle and 7-day absolute lifetime, CSRF protection, and explicit logout revocation. Windows uses 15-minute access credentials and rotating refresh credentials in Windows Credential Manager with reuse detection. Recovery invalidates sessions and refresh families.

FastAPI Users 15.x was checked against its current official documentation; it supports cookie/bearer transports, database-backed invalidatable tokens, user lifecycle, and reset-password routes. `pwdlib` provides modern Argon2 password hashing. FastAPI Users does not by itself define the required rotating desktop refresh-token family, so that part is a HomeHub application contract.

## 3. Agent transport
**Status: PASS**

Phase 1 uses HTTP/JSON over `/run/homehub/agent/agent.sock`, with `homehub-api` -> `homehub-agent` ownership/group separation and Linux peer credential validation. The Agent has no Internet/Tailnet listener. Remote mTLS is deferred.

## 4. Agent schemas
**Status: PASS**

Versioned `AgentRequest`/`AgentResponse` schemas define request ID, idempotency key, server ID, operation, typed parameters, deadline, status, typed result, stable error code/message, retryability, duration, and operation state ID. Stack traces and secrets are forbidden.

## 5. Idempotency
**Status: PASS**

Mutating requests persist operation state and idempotency keys. Same key + same operation/parameters returns the original state/result; parameter mismatch is rejected. Unknown outcomes are resolved through durable operation state before retry. Tracked destructive operations never receive an automatic new idempotency key.

## 6. Filesystem security
**Status: PASS**

Agent requests use project/resource references, not arbitrary absolute roots. Registry-derived roots are enforced. Traversal, NUL, encoded traversal, symlink/magic-link escapes, cross-project references, and unsafe root replacement are rejected. Descriptor-relative/atomic operations and `openat2()` are the preferred race-resistant primitives where applicable.

## 7. Secret bootstrap/recovery
**Status: PASS**

A random 256-bit master key is generated once at installation, stored operationally as an encrypted systemd credential, and separately escrowed offline for recovery. Database backups contain ciphertext but not the master key. Key loss without recovery is intentionally unrecoverable. Ubuntu 24.04/systemd 255 provides the required credential primitives.

## 8. Backup/restore
**Status: PASS**

Backup validity, integrity, metadata durability, restore identification, retention, permission preservation, target validation, health verification, and audit requirements are formalized. Deployment backup failure blocks destructive deployment unless a defined non-destructive policy applies.

## 9. SQLite ADR
**Status: PASS**

ADR-009 is Accepted. SQLite + WAL + SQLAlchemy + Alembic is the single-node MVP persistence model with explicit concurrency limits and a PostgreSQL migration boundary. Current SQLAlchemy documentation supports the selected stack.

## 10. Cross-document consistency
**Status: PASS**

Architecture, Security, Agent, API, Registry, Deployment, Updates, Secrets, Multi-server, Clients, Operations, Testing, Roadmap, and ADRs now use the same trust boundaries, authentication direction, UDS transport, capability model, registry-derived filesystem model, deployment states, secret bootstrap, and persistence decision.

## 11. Security invariants
**Status: PASS**

All 15 required invariants are explicitly present in `SECURITY.md` and reflected in the supporting contracts.

## 12. Phase 1 gate
All required architecture decisions are now specified:

- [x] Permission matrix
- [x] Authentication contract
- [x] Recovery model
- [x] Agent transport
- [x] Agent identity
- [x] Agent request/response schema
- [x] Idempotency model
- [x] Filesystem security model
- [x] Secret bootstrap/recovery
- [x] Backup/restore acceptance criteria
- [x] SQLite ADR
- [x] Failure model
- [x] Audit model
- [x] Cross-document consistency

**Phase 1 status: READY**

READY means implementation may begin after this review commit is verified. It does not authorize implementation in this documentation change itself.

## Remaining deferred decisions
These are intentionally not Phase 1 blockers: remote Agent mTLS implementation, PostgreSQL migration, external KMS, full multi-server UI, detailed Windows self-update mechanics, live log streaming, and notifications.