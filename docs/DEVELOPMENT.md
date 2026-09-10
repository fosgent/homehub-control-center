# Development

**Status: Implemented for Phase 1 · verify and extend per component**

## Implemented stack
- Backend: Python + FastAPI + Pydantic + SQLAlchemy (async) + Alembic — implemented.
- Windows client: Tauri 2 + Rust + React + TypeScript + Vite — implemented for the
  server-connection foundation.
- Web UI: React + TypeScript + Vite + Tailwind + shadcn/ui — **planned, not implemented**.
- Database: SQLite for the MVP (ADR-009); PostgreSQL later is a proposal. The persistence
  boundary must support migration without coupling domain logic to one engine.

## Local development
Keep Control API, Agent, clients, and infrastructure adapters independently testable. Use safe local fixtures and never place production credentials in development files.

## Environment
Document required environment variables only by name and purpose. Secret values belong in local secret storage/environment mechanisms and must not be committed.

## Quality gates
Before merge: formatting, linting, type checks, unit tests, integration/API tests, security tests, and documentation consistency appropriate to the changed component.

The exact verified commands for backend, frontend, Rust, and the Windows installer are in `docs/DEVELOPMENT_HANDOFF.md` ("Verified commands"). The task-level workflow and preserved invariants are described there as well.

## Workflow
Use small focused changes, explicit architecture decisions, review security-sensitive changes, and keep implementation status honest. Documentation must distinguish implemented behavior from planned contracts.
