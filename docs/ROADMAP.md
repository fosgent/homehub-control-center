# Roadmap

**Status: living document.** Work is tracked as numbered tasks under phases. This
document distinguishes **completed** work from **next proposed** work and **future
capabilities**. Nothing under "Next proposed work" or "Future capabilities" has been
started; a task must be explicitly selected and approved before implementation.

## Completed

| Task | Scope |
|---|---|
| Phase 1 / Task 1 | **Control API Foundation** — FastAPI API, layered domain/application/infrastructure, `GET /api/v1/health`, SQLite MVP persistence boundary (ADR-009), Alembic baseline, backend tooling (pytest/ruff/mypy). |
| Phase 1 / Task 2 | **Windows Client MVP** — Tauri 2 + Rust native layer, React + TypeScript frontend, typed allowlisted commands, NSIS installer. |
| Phase 1 / Task 3 | **Repository Integration / CI / Release** — GitHub Actions Windows build + verification, tag-based release workflow, v0.1.0 published. |
| Phase 1 / Task 4 | **Server Connection Foundation** — server registry (versioned JSON, atomic writes), add/edit/remove, test/connect/disconnect, status display, connection semantics, native URL validation and security boundary, GUI-verified (ADR-010). |

Implementation details and preserved semantics: `README.md`,
`docs/DEVELOPMENT_HANDOFF.md`, `docs/ADRs/ADR-010-windows-client-server-registry-and-connection-boundary.md`.

## Next proposed work (not started)

**Phase 2 — Server Management Foundation**

Possible scope (keep open until a task is scoped):

- richer server metadata;
- server capability discovery;
- server details/status;
- connection lifecycle (extend what Task 4 built);
- foundation for authenticated server communication.

**Infrastructure Integration (Coolify)**

Investigate and document how HomeHub Control Center should integrate with Coolify.
Architectural principle already agreed: **Coolify may act as an infrastructure/deployment
backend, while HomeHub Control Center remains the user's higher-level centralized
management layer.** Do not implement integration yet unless explicitly assigned.

## Future capabilities (not implemented, not scheduled)

- Project management / project registry.
- Deployment visibility and actions.
- Logs.
- Backups and restore.
- Scheduler (scheduled jobs).
- Monitoring and health trends.
- Secrets / credentials architecture.
- Authentication and authorization / RBAC.
- Audit trail.
- Multi-server remote agents.
- Web UI.

These correspond to the forward contracts in `docs/` (PRODUCT, SECURITY, AGENT, API,
PROJECT-REGISTRY, DEPLOYMENT, UPDATES, SCHEDULER, LOGGING, SECRETS, MULTI-SERVER,
OPERATIONS). They are **not** implemented; a future task must revisit and re-scope them
against the current codebase before any claim of implementation.

## Process gate

Before starting any new task:

1. Inspect current `git status` (uncommitted work belongs to the prior task).
2. Read `docs/DEVELOPMENT_HANDOFF.md` and this roadmap.
3. Read `docs/ARCHITECTURE.md` and relevant ADRs.
4. Inspect existing tests before modifying behavior.
5. Propose the task scope explicitly (including what is out of scope).
6. Wait for approval before broad architectural changes.

Authentication/RBAC (the original Phase 1 hardening items in ADR-006/007 and the API
contract in `docs/API.md`) remain **future work** — authentication is not implemented.