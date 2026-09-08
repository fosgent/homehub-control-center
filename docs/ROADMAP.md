# Roadmap

**Status: Accepted implementation sequence**

## Phase 0 — Foundation
Product, architecture, security model, API contract, Agent model, registry, deployment/update model, scheduler, logging, secrets, multi-server model, ADRs, development/testing standards.

## Phase 1 — API + Agent Foundation
Authentication, authorization, SQLite/Alembic persistence, Control API skeleton, local Agent identity/channel, typed operation catalog, operation state/idempotency, health, audit foundation. **Gate: READY only after the Architecture Review v3 checklist is fully accepted.**

## Phase 2 — Windows Client Foundation
Tauri validation, client shell, secure authentication, Windows Credential Manager integration, API client, connectivity and server status.

## Phase 3 — Dashboard + Project Registry
Dashboard metrics, registry persistence, project detail, server/project health, controlled lifecycle operations.

## Phase 4 — Service Management
Docker/Nginx/system adapters, capability model, richer lifecycle, backup operations.

## Phase 5 — Logs + Monitoring
Structured logs, health monitor, filtering/search, retention, security/audit views.

## Phase 6 — Deployment / Update Engine
Release discovery, immutable artifacts, backup/staging, deployment state machine, health verification, rollback.

## Phase 7 — Scheduler
Durable jobs, locks, retries, history, run-now, health and maintenance tasks.

## Phase 8 — Secrets + Security + Audit
Encrypted secrets, systemd credential bootstrap/recovery, RBAC hardening, rate limiting, threat-model-driven tests, audit integrity.

## Phase 9 — Windows Self-Update
Signed/verified client update flow, recovery, compatibility checks, release policy.

## Phase 10 — Multi-Server + Web UI + Polish
Multiple managed servers, remote Agent registration/revocation, richer Web UI, operational polish, notifications as appropriate.

## Gate
Do not begin implementation of Phase 1 until all security-critical decisions in the Architecture Review v3 gate are accepted and testable.
