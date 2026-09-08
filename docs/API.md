# API Contract

**Status: Planned architectural contract; endpoints are not implemented.**

Base path: `/api/v1`.

## Health and identity
```text
GET /health
GET /me
```

Health must reveal only safe operational information to unauthenticated callers. Project/server inventory requires authentication.

## Projects
```text
GET  /projects
GET  /projects/{id}
POST /projects/{id}/start
POST /projects/{id}/stop
POST /projects/{id}/restart
POST /projects/{id}/update
POST /projects/{id}/backup
POST /projects/{id}/health-check
POST /projects/{id}/rollback
```

## Servers
```text
GET /servers
GET /servers/{id}
POST /servers/{id}/health-check
```

## Operations
```text
GET /deployments
GET /deployments/{id}
GET /logs
GET /scheduler/jobs
GET /scheduler/jobs/{id}/history
GET /audit
```

## Security contract
There is intentionally no generic execute/command endpoint. Operation payloads must be typed Pydantic schemas with bounded fields. Authorization is evaluated server-side. Destructive operations require confirmation semantics and idempotency where relevant.

## API conventions
Use structured errors, pagination for collections, ISO-8601 timestamps, stable resource IDs, correlation/request IDs, optimistic concurrency where needed, and explicit API versioning. Never return secrets. Never put credentials in URLs.

## Status labels
- **Implemented:** none in this empty repository.
- **Planned:** endpoints above.
- **Proposed:** final authentication, authorization, persistence, and transport details pending ADR decisions.
