# Roadmap

**Status: Proposed implementation sequence**

## Phase 0 — Foundation
Product, architecture, security model, API contract, Agent model, registry, deployment/update model, scheduler, logging, secrets, multi-server model, ADRs, development/testing standards.

## Phase 1 — API + Agent Foundation
Authentication, authorization foundation, persistence, Control API skeleton, Agent identity/channel, typed operation catalog, health, audit foundation.

## Phase 2 — Windows Client Foundation
Tauri validation, client shell, secure authentication/session handling, API client, connectivity and server status.

## Phase 3 — Dashboard + Project Registry
Dashboard metrics, project registry persistence, project detail, server/project health, controlled start/stop/restart.

## Phase 4 — Service Management
Docker/Nginx/system adapters, capability model, richer project lifecycle, backup operations.

## Phase 5 — Logs + Monitoring
Structured logs, health monitor, filtering/search, retention, security/audit views.

## Phase 6 — Deployment / Update Engine
Release discovery, immutable artifacts, backup/staging, deployment state machine, health verification, rollback.

## Phase 7 — Scheduler
Durable jobs, locks, retries, history, run-now, health and maintenance tasks.

## Phase 8 — Secrets + Security + Audit
Encrypted secrets, key management, RBAC hardening, rate limiting, threat-model-driven tests, audit integrity.

## Phase 9 — Windows Self-Update
Signed/verified client update flow, recovery, compatibility checks, release policy.

## Phase 10 — Multi-Server + Web UI + Polish
Multiple managed servers, Agent registration/revocation, richer Web UI, operational polish, notifications as appropriate.

## Gate
Do not begin broad implementation merely because these documents exist. Proposed decisions must be reviewed and converted to accepted ADRs where appropriate.
