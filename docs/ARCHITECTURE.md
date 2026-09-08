# Architecture

**Status: Accepted foundation; implementation-level contract for Phase 1**

## Control-plane model

```text
Web UI / Windows Client
        |
        | HTTPS + application authentication
        v
Control API / Control Plane
        |
        | HTTP/JSON over Unix Domain Socket
        v
HomeHub Agent
        |
        | typed allowlisted adapters
        v
Docker / Nginx / Filesystem / System
```

The Control Plane is the policy authority. The Agent is the privileged execution boundary. Clients are never authorization boundaries and never receive privileged fallback paths.

## Trust boundaries
1. Internet/Tailnet -> Control API.
2. Client -> authenticated API principal.
3. Control API -> authenticated local Agent process identity.
4. Agent -> host resources through typed capabilities.
5. Control Plane database -> encrypted secret material and separately protected key material.
6. Managed Server A -> Managed Server B: no implicit trust or authorization inheritance.

## Authorization model
Authentication establishes identity; authorization independently evaluates role, operation, server scope, project scope, and current resource state. Every protected operation is authorized server-side. UI visibility is advisory only.

Roles are `admin`, `operator`, and `viewer`. Maximum scope is global for admin, server/project for operator, and read-only server/project visibility for viewer, subject to explicit operation rules in `SECURITY.md`.

## Agent boundary
The co-located API and Agent communicate using HTTP/JSON over `/run/homehub/agent/agent.sock`. The Agent is not exposed on Internet, LAN, or Tailnet. Linux peer credentials (`SO_PEERCRED`) identify the API process; socket ownership/ACLs provide the first gate, while Agent protocol validation and its typed capability catalog provide the authorization boundary for host execution.

No generic shell, command string, executable path, or arbitrary subprocess API exists.

## Database
SQLite with WAL is the accepted single-node MVP persistence layer. SQLAlchemy is the persistence abstraction and Alembic owns schema migration. Domain/application services must not depend on SQLite-specific APIs. PostgreSQL remains the migration target when concurrency, HA, or fleet scale requires it; see ADR-009.

## Deployment
Deployment is a durable state machine: request -> authorization -> preflight -> lock -> verified backup -> immutable artifact acquisition -> verification -> staging -> compatibility/migration checks -> activation -> health check -> success, or deterministic failure/rollback to an explicit known-good point.

## Security invariants
- No arbitrary command execution API.
- Capability does not equal authorization.
- Authentication does not equal authorization.
- Project filesystem roots come only from the registry.
- Cross-project filesystem access is rejected.
- Artifact verification failure fails closed.
- Secrets are never emitted to logs, URLs, Git, or client telemetry.
