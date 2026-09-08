# Development

**Status: Planned**

## Proposed stack
- Backend: Python + FastAPI + Pydantic + SQLAlchemy + Alembic.
- Web UI: React + TypeScript + Vite + Tailwind + shadcn/ui.
- Windows client: Tauri 2 + React + TypeScript, pending validation.
- Database: SQLite initially is a proposal; PostgreSQL later is also a proposal. The persistence boundary must support migration without coupling domain logic to one engine.

These are architectural candidates, not implemented technology choices.

## Local development
Keep Control API, Agent, clients, and infrastructure adapters independently testable. Use safe local fixtures and never place production credentials in development files.

## Environment
Document required environment variables only by name and purpose. Secret values belong in local secret storage/environment mechanisms and must not be committed.

## Quality gates
Before merge: formatting, linting, type checks, unit tests, integration/API tests, security tests, and documentation consistency appropriate to the changed component.

## Workflow
Use small focused changes, explicit architecture decisions, review security-sensitive changes, and keep implementation status honest. Documentation must distinguish implemented behavior from planned contracts.
