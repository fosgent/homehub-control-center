# HomeHub Control Center

Central management and control-plane system for HomeHub servers and the projects they host.

## What it is

HomeHub Control Center is designed as a control plane, not merely a dashboard. It will provide a shared Control API for a Windows desktop client and Web UI, a server-side Agent for privileged operations, project inventory, deployment/update orchestration, scheduling, health monitoring, logs, secrets, security/audit, and multi-server management.

## Architecture

```text
Windows Client ─┐
Web UI ──────────┼──> Control API ──> HomeHub Agent ──> allowlisted adapters
                 │                         ├── Docker
                 │                         ├── Nginx
                 │                         └── System / Filesystem
                 └── Authentication / Authorization
```

The Control API must never expose arbitrary shell execution. Privileged work is delegated to narrowly defined Agent operations.

## Security model

- Application authentication and authorization are mandatory.
- Managed-server privileges are isolated in the Agent.
- No `POST /execute` or equivalent arbitrary command API.
- Secrets are encrypted at rest, masked in UI, excluded from logs and Git, and access is audited.
- Destructive operations require explicit confirmation and audit events.
- Update and deployment flows include validation, health checks, and rollback planning.

## Repository structure

- `docs/PRODUCT.md` — product scope and vision
- `docs/ARCHITECTURE.md` — system architecture and flows
- `docs/SECURITY.md` — security architecture and threat model
- `docs/API.md` — versioned API contract (implemented vs planned)
- `docs/AGENT.md` — Agent boundary and privilege model
- `docs/PROJECT-REGISTRY.md` — extensible project registry
- `docs/DEPLOYMENT.md` — deployment lifecycle and rollback
- `docs/UPDATES.md` — Control Center and project update strategy
- `docs/SCHEDULER.md` — scheduled jobs
- `docs/LOGGING.md` — application/system/audit logging
- `docs/SECRETS.md` — secret storage and access model
- `docs/MULTI-SERVER.md` — control-plane/server model
- `docs/OPERATIONS.md` — operational procedures
- `docs/CLIENTS.md` — Windows and Web clients
- `docs/DEVELOPMENT.md` — development conventions
- `docs/TESTING.md` — test strategy
- `docs/ROADMAP.md` — phased implementation roadmap
- `docs/ADRs/` — architecture decision records

## Development status

**Foundation only.** This repository was empty when the documentation foundation was created. No backend, Agent, client, database schema, deployment engine, scheduler, or production API is claimed to be implemented by this repository.

Architectural items that still require validation are explicitly marked `Proposed` or `Planned` in the documentation.

## Roadmap

The current foundation roadmap is in `docs/ROADMAP.md`. Implementation begins only after the foundation documents and proposed decisions are reviewed and agreed.
