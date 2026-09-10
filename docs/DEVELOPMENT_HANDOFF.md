# Development Handoff

**Read this before changing code.**

This document is the canonical on-boarding point for a future developer or AI agent
working on HomeHub Control Center. It describes the current repository state, the
implemented architecture, verified commands, preserved invariants, and where work
continues next. It is a living document that must be updated when the project state
changes.

## Read before changing code

Recommended reading order:

1. `README.md` — project purpose and current status.
2. `docs/DEVELOPMENT_HANDOFF.md` — this document.
3. `docs/ARCHITECTURE.md` — target architecture and trust boundaries.
4. `docs/ADRs/` — architecture decision records (start with `docs/ADRs/README.md`).
5. Implementation files relevant to the requested task.
6. Existing tests before modifying behavior.

## Current repository state

- **Branch:** `main` (single working branch; releases are cut from tags).
- **Latest stable release:** `v0.1.0` (Windows NSIS installer, published from tag by
  GitHub Actions).
- **Latest implemented phase/task:** Phase 1, Task 4 — Server Connection Foundation
  (**COMPLETE**).
- **Next work:** none started. Must be explicitly selected before implementation
  (see `docs/ROADMAP.md` and the "How to continue" section below).

### Architecture summary (as implemented)

```text
Windows Client (Tauri 2 + Rust + React/TypeScript)
   │  typed allowlisted Tauri commands; native health probes (ureq); no auth yet
   ▼
HomeHub Control API (FastAPI, layered: domain / application / api / infrastructure)
   │  HTTP/JSON on http://127.0.0.1:8000 (dev default)
   ▼
Linux Server / Future Agents / Infrastructure   ← target/planned, NOT implemented
```

- The Windows client is a **thin, unprivileged UI**. All I/O happens through a typed
  allowlist of native Tauri commands registered in `src-tauri/src/lib.rs` and implemented
  in `src-tauri/src/commands.rs`.
- Server configuration is persisted by the client in a **versioned JSON registry**
  (`servers.json`, schema version 1) in the Tauri application config directory
  (`%APPDATA%\com.homehub.controlcenter\servers.json` on Windows), written atomically
  (tmp + rename). First launch seeds `http://127.0.0.1:8000` (override at first launch
  via the `HOMEHUB_CONTROL_API_URL` environment variable).
- The Control API (`src/homehub/`) exposes `GET /api/v1/health`
  (`{status, database, version, timestamp, checks}`). Its domain/application layers are
  persistence-independent; only infrastructure references SQLAlchemy/SQLite (ADR-009).
- The client is not limited to the default endpoint: any http/https Control API URL can
  be configured.

### Current implementation status

| Area | State |
|---|---|
| Control API (FastAPI, /api/v1/health) | Implemented |
| SQLite + Alembic baseline | Implemented |
| Windows client UI (React/TS) | Implemented |
| Windows client native layer (Rust) | Implemented |
| Server registry + connection UI | Implemented |
| NSIS Windows installer | Implemented |
| CI build + tag-based GitHub Release | Implemented (`v0.1.0` published) |
| Authentication / RBAC | **Not implemented** |
| Agent / command execution / deployments / backups / scheduler / monitoring / secrets | **Not implemented** |

## Verified commands

All commands below have been run successfully from a Windows machine in this state.
Install dependencies once (`pip install -e ".[dev]"` at the repo root, `npm ci` in
`frontend/`, `cargo` via rust-toolchain in `src-tauri/` — see the CI workflow).

### Backend (from repository root)

```text
python -m pytest -q
python -m ruff check .
python -m ruff format --check .
python -m mypy src
```

### Frontend (working directory: `frontend/`)

```text
npm run typecheck
npm run build
```

### Rust (working directory: `src-tauri/`)

```text
cargo check
cargo test
```

### Windows installer (from repository root — mandatory)

`frontend/` and `src-tauri/` are **sibling directories**. The build must be launched from
the repository root so Tauri resolves `frontend/dist`:

```text
node frontend/node_modules/@tauri-apps/cli/tauri.js build --bundles nsis
```

Outputs (release profile):

- `src-tauri/target/release/homehub-client.exe`
- `src-tauri/target/release/bundle/nsis/HomeHub Control Center_<version>_x64-setup.exe`

### Run the Control API locally (from repository root, after `pip install -e ".[dev]"`)

```text
python -m uvicorn homehub.main:app --host 127.0.0.1 --port 8000
```

Options: `HOMEHUB_DATABASE_URL=sqlite+aiosqlite:///:memory:` for an in-memory DB,
`HOMEHUB_CONTROL_API_URL` to change the endpoint the client seeds on first launch.

### Release flow

Push a `v*` tag (e.g. `v0.2.0`). GitHub Actions builds and verifies everything, then the
release job downloads the NSIS installer artifact and runs:

```text
gh release create <tag> dist/*.exe --title <tag> --notes "HomeHub Control Center <tag>"
```

(Defined in `.github/workflows/windows-build.yml` — the artifact is
`src-tauri/target/release/bundle/nsis/*.exe`.)

## Preserved invariants

Do not regress these without an explicit, reviewed product decision. Most are recorded as
decisions in `docs/ADRs/ADR-010-windows-client-server-registry-and-connection-boundary.md`.

- **Domain/application layers must remain infrastructure-independent** — no framework,
  HTTP, or persistence coupling in `homehub.domain` / `homehub.application` (ADR-009).
- **No arbitrary command execution surface** — no shell, no subprocess, no generic
  `execute` in the client or API. Work happens through named, typed operations only.
- **Tauri commands must remain typed and explicitly allowlisted** — add new commands
  intentionally; never a string-based dispatcher (`src-tauri/capabilities/default.json`,
  `commands.rs`).
- **Client URL validation remains authoritative in native Rust** — the frontend is UX
  only. Rust rejects unsupported schemes (`file:`, `javascript:`, `data:`, …) and
  embedded credentials. (`health::validate_url`)
- **Server connection state semantics must not regress** (see `server.rs` `note_test`):
  - Test = one-shot health probe; a successful Test does **not** claim Connected.
  - Connect probes and then claims Connected (healthy or degraded).
  - A degraded API may display **Connected (degraded)**.
  - A fresh failed test on a previously connected server drops the state to **Error**.
  - A successful test after Error restores a non-connected state (Disconnected); it does
    not auto-reconnect.
  - Disconnect explicitly clears the active connection state.
- **Runtime connection state must not be persisted** as connected — `servers.json` stores
  configuration only (`server_id`, `name`, `url`, `enabled`).
- **Credentials must not be added** to source code or plain client configuration without
  an explicit security design (see `docs/SECURITY.md`, ADR-006/007 for the planned model).
- **Release/build artifacts must not be committed** — the NSIS installer and built
  binaries stay untracked.
- **Build-layout rule** — the full Tauri build is launched from the repository root (see
  "Verified commands").

## Connection semantics in detail

The state machine lives in `src-tauri/src/server.rs` (`note_test`, `note_connect`,
`note_connecting`, `disconnect`). Runtime states: `disconnected`, `connecting`,
`connected`, `error`, `disabled`. Degraded/`connected` is rendered as
"Connected (degraded)". Test outcomes: `healthy`, `degraded`, `unreachable`,
`invalid_endpoint`, `http_error`, `invalid_response`. This behavior is exercised by unit
tests and by a live GUI smoke run against the real release executable.

## GUI smoke testing (how Task 4 was verified)

Task 4 verification drove the real release `homehub-client.exe` through WebView2's
Chromium DevTools Protocol (CDP):

1. Launch with
   `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9222 --remote-allow-origins=*"`
   and an isolated `WEBVIEW2_USER_DATA_FOLDER`.
2. Connect a CDP client to `http://127.0.0.1:9222/json`, pick the Tauri page, and drive
   the DOM + `window.__TAURI_INTERNALS__.invoke(...)` with polled assertions.
3. Use a real Control API (uvicorn) and a degrading fixture for end-to-end cases;
   delete `%APPDATA%\com.homehub.controlcenter` and stop stray listeners on the test
   ports before a fresh start.

When modifying client behavior, re-run both `cargo test` and a GUI smoke pass like this
against the built executable.

## How to continue

**CURRENT IMPLEMENTATION STATUS:** Phase 1 / Task 4 — COMPLETE.

**NEXT DEVELOPMENT WORK:** Not started. Must be explicitly selected before implementation.

When handed the instruction *"Read the repository documentation and continue from the
latest completed task"*:

1. Inspect the current git status for uncommitted work.
2. Read this handoff document and `docs/ROADMAP.md`.
3. Read `docs/ARCHITECTURE.md` and the relevant ADRs.
4. Inspect existing tests (backend `tests/`, Rust `src-tauri/src/*.rs` `#[cfg(test)]`).
5. Propose the task scope explicitly (including what is out of scope).
6. Wait for approval before broad architectural changes.

Do not silently start building unapproved features; the roadmap's "future capabilities"
are not in scope until selected.