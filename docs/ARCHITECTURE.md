# Architecture

**Status: Proposed**

## Control-plane model

```text
Windows Client ─┐
Web UI ──────────┼─ HTTPS ─> Control API ── secure local channel ─> Agent
                 │                                      │
                 └─ AuthN/AuthZ                          ├─ Docker adapter
                                                        ├─ Nginx adapter
                                                        └─ System/Filesystem adapters
```

The Control Plane owns identity, authorization, registry, orchestration, scheduling, audit, and persistence. A Managed Server owns host execution through its Agent.

## Trust boundaries
1. Internet/Tailnet to Control API.
2. Client to authenticated API.
3. Control API to privileged Agent channel.
4. Agent to host resources.
5. Control plane database to secrets material.

## Request flow
1. Client authenticates.
2. API authenticates the session/token and authorizes the requested operation.
3. API resolves the project/server from the registry.
4. API creates a correlation ID and audit intent.
5. API asks the Agent for a specific allowlisted operation.
6. Agent validates operation, project identity, paths, capabilities, and parameters.
7. Adapter performs the operation without exposing a general shell.
8. Result is sanitized, persisted/audited, and returned to the client.

## Components
- **Control API:** FastAPI candidate; public application boundary.
- **Agent:** server-side privileged executor with explicit operation catalog.
- **Project Registry:** canonical project metadata and capabilities.
- **Deployment Engine:** versioned deployment state machine with backup, validation, health check, and rollback.
- **Scheduler:** durable scheduled-job orchestration with locks and history.
- **Health Monitor:** server, infrastructure, and project checks.
- **Log service:** structured operational logs with retention and masking.
- **Secrets service:** encrypted-at-rest storage and audited access.
- **Clients:** Web UI and Windows desktop client sharing API/business contracts.

## Database
SQLite is a proposed initial option for single-node development; PostgreSQL is a proposed later option. The migration boundary must remain explicit and be decided by ADR before production requirements are fixed.

## Infrastructure
The current target server layout is `/srv/apps`, `/srv/data`, `/srv/configs`, `/srv/backups`, and `/srv/infra`. Concrete host automation must be implemented through adapters and not encoded as arbitrary user-supplied shell commands.

## Explicit security invariant
There is no generic `POST /execute` operation. Every privileged action is a typed, allowlisted operation with authorization, validation, bounded parameters, and an audit event.
