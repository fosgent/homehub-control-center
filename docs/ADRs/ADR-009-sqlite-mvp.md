# ADR-009: SQLite MVP persistence

**Status:** Accepted

## Context
Phase 1 is a single-node Control Center. The system needs durable local state for users, sessions, RBAC assignments, registry, operation state, deployments, scheduler state, audit records, and secret metadata without requiring a separate database service.

## Decision
Use SQLite for the single-node MVP with WAL mode and SQLAlchemy as the persistence abstraction. Use Alembic for all schema changes. Use short transactions, explicit concurrency controls, and durable operation/deployment state. The application must treat SQLite as an implementation detail behind repositories/domain services.

Backups are file-consistent database backups produced through an approved SQLite-safe backup procedure, followed by integrity verification and restore testing. Database backup is included in the HomeHub backup model, while secret ciphertext and key-recovery material remain subject to ADR-007 separation.

Expected concurrency is a single Control API instance plus Agent and scheduler activity on one host. The design does not target multi-writer horizontal API scaling, high write throughput, or cross-host shared SQLite. WAL improves reader/writer concurrency but does not remove SQLite's single-writer constraints.

## Alternatives
PostgreSQL from day one; raw SQLite access without ORM boundary; Redis as primary persistence.

## Consequences
SQLite keeps MVP deployment small and operationally simple. The architecture must avoid SQLite-specific SQL, locking assumptions, or filesystem access in domain logic so that PostgreSQL can replace it later. PostgreSQL becomes the preferred target when HA, multi-instance Control API deployment, fleet scale, or sustained concurrency exceeds SQLite's operating envelope.

## Security implications
Database files and WAL/journal files require strict filesystem permissions and backup handling. Database corruption or integrity-check failure is a hard failure for restore/upgrade workflows. No secret master key is stored in the database.

## Compatibility evidence
Current SQLAlchemy 2.0 documentation (2.0.52) supports SQLite and the `aiosqlite` async dialect. FastAPI Users' SQLAlchemy integration documents SQLite/aiosqlite and recommends Alembic for production schema migration.
