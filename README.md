# HomeHub Control Center

Central management and control-plane system for HomeHub servers and the projects they host.

## What it is

HomeHub Control Center is a private personal infrastructure/control platform:

```text
Windows Client
     │
     ▼
HomeHub Control API
     │
     ▼
Linux Server / Future Agents / Infrastructure
```

The long-term goal is a centralized control layer for multiple servers and projects.
Potential future capabilities include server management, project status, deployments,
integrations with infrastructure platforms (such as Coolify), backups, logs, scheduled
jobs, monitoring, secrets/credentials, security controls, and audit history.

> Those are **future capabilities**. Only what is listed under "Implemented today" exists
> in this repository. Planned vs implemented is always made explicit in the docs.

## Status

| | |
|---|---|
| Latest release | **v0.1.0** (Windows NSIS installer, published via GitHub Actions from tag `v0.1.0`) |
| Branch | `main` |
| Latest completed work | **Phase 1 / Task 4 — Server Connection Foundation** |
| Next work | Not started — must be explicitly selected (see `docs/ROADMAP.md`) |

## Implemented today

### Phase 1 / Task 1 — Control API Foundation

- FastAPI Control API (`src/homehub/`) with layered architecture: domain
  (`homehub.domain`), application (`homehub.application`), interface/API (`homehub.api`),
  infrastructure (`homehub.infrastructure`).
- `GET /api/v1/health` returns `{status, database, version, timestamp, checks}`; reports
  `degraded` when the database probe fails (transport success is not treated as truth).
- SQLite MVP persistence boundary: SQLAlchemy (async) + aiosqlite, backed by an
  Alembic baseline migration (`migrations/versions/0001_initial_baseline.py`). The
  domain/application layers are persistence-independent (ADR-009).
- Settings via `HOMEHUB_*` environment variables (`src/homehub/core/config.py`).

**Not implemented:** authentication, authorization/RBAC, agents, command execution,
deployments, backups, scheduler, secrets, monitoring.

### Phase 1 / Tasks 2–3 — Windows Client + CI/Release

- Tauri 2 + Rust native layer (`src-tauri/`).
- React + TypeScript frontend (`frontend/`, Vite).
- NSIS Windows installer (bundle target `nsis`).
- A small allowlist of typed native Tauri commands (no arbitrary shell/subprocess).
- GitHub Actions workflow (`.github/workflows/windows-build.yml`): backend, frontend and
  Rust verification, then a Tauri NSIS build; pushing a `v*` tag additionally creates a
  GitHub Release with the installer.
- Current release: v0.1.0.

### Phase 1 / Task 4 — Server Connection Foundation

- Full server list management in the Windows client: list, add, edit, remove, test
  connection, connect, disconnect, live status display, and persistence across restarts.
- Versioned JSON server registry (`servers.json`) in the Tauri application config
  directory, written atomically. First launch seeds
  `http://127.0.0.1:8000` — the client is **not** limited to that endpoint.
- Connection semantics: **Test** is a one-shot health probe and does not claim
  Connected; **Connect** probes and establishes Connected; a degraded API shows
  "Connected (degraded)"; a fresh failed test on a connected server drops to Error;
  runtime connection state is never persisted.
- Security boundary: network access to configured Control APIs happens in **native
  Rust**; the frontend cannot fetch arbitrary endpoints; URL validation is authoritative
  in Rust (rejects `file:`, `javascript:`, `data:`, and embedded credentials).

See `docs/ADRs/ADR-010-windows-client-server-registry-and-connection-boundary.md` for the
recorded decisions and `docs/DEVELOPMENT_HANDOFF.md` for the preserved semantics.

## Architecture

```text
Windows Client (Tauri 2 + Rust + React/TypeScript)
   │  typed allowlisted commands · native health probes (no auth yet)
   ▼
HomeHub Control API (FastAPI · domain/application/infrastructure)
   │  HTTP/JSON
   ▼
Linux Server / Future Agents / Infrastructure (planned)
```

Forward architecture, security model, and decision records live in
`docs/ARCHITECTURE.md`, `docs/SECURITY.md`, and `docs/ADRs/`.

## Repository layout

- `src/homehub/` — Control API (FastAPI, layered).
- `tests/` — backend tests.
- `migrations/` — Alembic migrations.
- `src-tauri/` — Windows client native layer (Rust/Tauri).
- `frontend/` — Windows client UI (React/TypeScript/Vite).
- `.github/workflows/windows-build.yml` — CI + tag-based release automation.
- `docs/` — handoff, architecture, ADRs, contracts.

## Quick start

Full build, test, run and release instructions: `docs/DEVELOPMENT_HANDOFF.md`.

**Critical build-layout rule** — `frontend/` and `src-tauri/` are sibling directories, so
the full Tauri build must be launched from the **repository root**:

```text
node frontend/node_modules/@tauri-apps/cli/tauri.js build --bundles nsis
```

## Roadmap / where to continue

See `docs/ROADMAP.md`. The next proposed area is **Phase 2 — Server Management
Foundation**; nothing has been started. Future capabilities (auth/RBAC, agents,
deployments, backups, scheduler, monitoring, secrets, audit, multi-server, Web UI) are
explicitly out of scope until selected and approved.

## Reading order for contributors

1. `README.md` — this file.
2. `docs/DEVELOPMENT_HANDOFF.md` — read before changing code.
3. `docs/ARCHITECTURE.md` and `docs/ADRs/` — architecture and decisions.
4. Implementation files relevant to the task.
5. Existing tests before changing behavior.